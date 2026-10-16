//! Finding a way around trees and cliffs: A* on the path grid, then the
//! corners pulled tight so a hero walks in straight lines between them.
//!
//! A* runs on 64-unit cells with eight neighbours and octile costs (10 for a
//! side step, 14 for a diagonal). A diagonal step needs both sides open, so a
//! path never squeezes between two trees touching at a corner, which is how
//! a tree line holds in Warcraft III. The search stops after `MAX_EXPANDED`
//! cells and walks toward the closest cell it reached, so an order to a
//! walled-off point still moves the hero as near as it can get.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::rules::P;

/// The most cells one search may expand.
pub const MAX_EXPANDED: usize = 24_000;

/// What the search needs to know about the grid.
pub trait Grid {
    fn width(&self) -> i32;
    fn cell(&self) -> i32;
    fn blocked(&self, cx: i32, cy: i32) -> bool;
}

fn octile(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
    let (dx, dy) = ((ax - bx).abs(), (ay - by).abs());
    10 * dx.max(dy) + 4 * dx.min(dy)
}

/// The nearest open cell to (cx, cy), searching outward ring by ring.
pub fn nearest_open<G: Grid>(g: &G, cx: i32, cy: i32, max_ring: i32) -> Option<(i32, i32)> {
    let w = g.width();
    let open = |x: i32, y: i32| x >= 0 && y >= 0 && x < w && y < w && !g.blocked(x, y);
    if open(cx, cy) {
        return Some((cx, cy));
    }
    for r in 1..=max_ring {
        let mut best: Option<(i32, i32, i32)> = None;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs() != r && dy.abs() != r {
                    continue;
                }
                let (x, y) = (cx + dx, cy + dy);
                if open(x, y) {
                    let d = dx * dx + dy * dy;
                    if best.map(|b| d < b.2).unwrap_or(true) {
                        best = Some((x, y, d));
                    }
                }
            }
        }
        if let Some((x, y, _)) = best {
            return Some((x, y));
        }
    }
    None
}

/// Whether a straight walk between two cell centres crosses no blocked
/// cell. Steps along the line in quarter cells and checks each cell touched,
/// and both sides of a diagonal crossing.
pub fn clear_line<G: Grid>(g: &G, a: (i32, i32), b: (i32, i32)) -> bool {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let steps = 4 * dx.abs().max(dy.abs());
    if steps == 0 {
        return !g.blocked(a.0, a.1);
    }
    let mut last = a;
    for s in 0..=steps {
        // quarter-cell steps from the centre of a, rounded to the cell
        let fx = a.0 * 4 + 2 + dx * 4 * s / steps;
        let fy = a.1 * 4 + 2 + dy * 4 * s / steps;
        let c = (fx.div_euclid(4), fy.div_euclid(4));
        if g.blocked(c.0, c.1) {
            return false;
        }
        if c.0 != last.0 && c.1 != last.1 && (g.blocked(c.0, last.1) || g.blocked(last.0, c.1)) {
            return false;
        }
        last = c;
    }
    true
}

/// A path from `from` to `to` in map units, as the corners to walk through
/// (the start left out, the end included). Empty when already there.
pub fn find<G: Grid>(g: &G, from: P, to: P) -> Vec<P> {
    let cell = g.cell();
    let w = g.width();
    let to_cell = |p: P| ((p[0] / cell).clamp(0, w - 1), (p[1] / cell).clamp(0, w - 1));
    let start = to_cell(from);
    let start = nearest_open(g, start.0, start.1, 3).unwrap_or(start);
    let goal_raw = to_cell(to);
    let Some(goal) = nearest_open(g, goal_raw.0, goal_raw.1, 12) else {
        return Vec::new();
    };
    if start == goal {
        return vec![to_open(to, goal_raw == goal, goal, cell)];
    }
    if clear_line(g, start, goal) {
        return vec![to_open(to, goal_raw == goal, goal, cell)];
    }
    let n = (w * w) as usize;
    let idx = |c: (i32, i32)| (c.1 * w + c.0) as usize;
    let mut came: Vec<u32> = vec![u32::MAX; n];
    let mut cost: Vec<i32> = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();
    cost[idx(start)] = 0;
    heap.push(Reverse((
        octile(start.0, start.1, goal.0, goal.1),
        0i32,
        idx(start) as u32,
    )));
    let mut best = (octile(start.0, start.1, goal.0, goal.1), idx(start));
    let mut expanded = 0usize;
    let mut reached = false;
    while let Some(Reverse((_, g_cost, at))) = heap.pop() {
        let at = at as usize;
        if g_cost > cost[at] {
            continue;
        }
        let c = ((at as i32) % w, (at as i32) / w);
        if c == goal {
            reached = true;
            break;
        }
        expanded += 1;
        if expanded > MAX_EXPANDED {
            break;
        }
        for (dx, dy) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let (nx, ny) = (c.0 + dx, c.1 + dy);
            if nx < 0 || ny < 0 || nx >= w || ny >= w || g.blocked(nx, ny) {
                continue;
            }
            if dx != 0 && dy != 0 && (g.blocked(c.0 + dx, c.1) || g.blocked(c.0, c.1 + dy)) {
                continue;
            }
            let step = if dx != 0 && dy != 0 { 14 } else { 10 };
            let ni = idx((nx, ny));
            let nc = g_cost + step;
            if nc < cost[ni] {
                cost[ni] = nc;
                came[ni] = at as u32;
                let h = octile(nx, ny, goal.0, goal.1);
                if h < best.0 {
                    best = (h, ni);
                }
                heap.push(Reverse((nc + h, nc, ni as u32)));
            }
        }
    }
    let end = if reached { idx(goal) } else { best.1 };
    let mut cells = vec![end];
    let mut at = end;
    while came[at] != u32::MAX {
        at = came[at] as usize;
        cells.push(at);
    }
    cells.reverse();
    let pos = |i: usize| ((i as i32) % w, (i as i32) / w);
    // pull the corners tight: from each kept corner, skip ahead to the
    // farthest cell still in a clear line
    let mut corners = Vec::new();
    let mut i = 0;
    while i + 1 < cells.len() {
        let mut j = cells.len() - 1;
        while j > i + 1 && !clear_line(g, pos(cells[i]), pos(cells[j])) {
            j -= 1;
        }
        corners.push(pos(cells[j]));
        i = j;
    }
    let last = corners.len().saturating_sub(1);
    corners
        .iter()
        .enumerate()
        .map(|(k, &c)| {
            if k == last && reached {
                to_open(to, goal_raw == goal, goal, cell)
            } else {
                [c.0 * cell + cell / 2, c.1 * cell + cell / 2]
            }
        })
        .collect()
}

/// The goal point itself when its cell was open, else the open cell's centre.
fn to_open(to: P, exact: bool, goal: (i32, i32), cell: i32) -> P {
    if exact {
        to
    } else {
        [goal.0 * cell + cell / 2, goal.1 * cell + cell / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        w: i32,
        wall: Vec<bool>,
    }

    impl Grid for Test {
        fn width(&self) -> i32 {
            self.w
        }
        fn cell(&self) -> i32 {
            64
        }
        fn blocked(&self, x: i32, y: i32) -> bool {
            x < 0 || y < 0 || x >= self.w || y >= self.w || self.wall[(y * self.w + x) as usize]
        }
    }

    fn grid(rows: &[&str]) -> Test {
        let w = rows.len() as i32;
        let mut wall = vec![false; (w * w) as usize];
        // rows are written top (high y) first
        for (r, line) in rows.iter().enumerate() {
            let y = w - 1 - r as i32;
            for (x, ch) in line.chars().enumerate() {
                wall[(y * w + x as i32) as usize] = ch == '#';
            }
        }
        Test { w, wall }
    }

    fn c(x: i32, y: i32) -> P {
        [x * 64 + 32, y * 64 + 32]
    }

    #[test]
    fn an_open_field_is_one_straight_leg() {
        let g = grid(&[".....", ".....", ".....", ".....", "....."]);
        assert_eq!(find(&g, c(0, 0), c(4, 4)), vec![c(4, 4)]);
    }

    #[test]
    fn a_wall_is_walked_around_its_open_end() {
        // A wall at x = 2 from y = 0 to y = 3; the gap is at the top (y = 4).
        let g = grid(&[".....", "..#..", "..#..", "..#..", "..#.."]);
        let p = find(&g, c(0, 0), c(4, 0));
        assert_eq!(*p.last().unwrap(), c(4, 0));
        assert!(p.iter().any(|q| q[1] >= c(0, 4)[1]), "goes over the top: {p:?}");
        // every leg is clear
        let mut at = (0, 0);
        for q in &p {
            let next = (q[0] / 64, q[1] / 64);
            assert!(clear_line(&g, at, next), "{at:?} to {next:?}");
            at = next;
        }
    }

    #[test]
    fn two_trees_touching_at_a_corner_hold() {
        // # at (1,1) and (2,2): the diagonal (1,2)->(2,1) must not squeeze through.
        let g = grid(&["....", "..#.", ".#..", "...."]);
        assert!(!clear_line(&g, (1, 2), (2, 1)));
    }

    #[test]
    fn a_walled_off_goal_leads_to_the_nearest_open_cell() {
        let g = grid(&["..#.#", "..###", ".....", ".....", "....."]);
        // goal (4,4) is open but enclosed; the search ends as near as it can
        let p = find(&g, c(0, 0), c(4, 4));
        assert!(!p.is_empty());
    }
}
