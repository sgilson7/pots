//! The ground: its height, its trees and where a unit can walk.
//!
//! `data/map.json` gives the lanes, the river, the bases, jungle paths and
//! clearings. Everything else is forest: every tree cell that no lane, path,
//! clearing or base clears gets a tree, less a seeded share of gaps. The same
//! seed gives every player the same forest, tree for tree.
//!
//! Two grids. Tree cells (128 map units, Warcraft III's tree footprint)
//! carry the height level and the trees. Path cells (64 units) carry what
//! blocks walking: a tree, a building's footprint, a cliff edge, the map's
//! rim. Trees can be cut; `Ground` keeps them in a flat vector of states so
//! the world can clone it cheaply for rollback.

use crate::rng::Rng;
use crate::rules::{MapDef, P};

/// Height levels of a tree cell.
pub const RIVER: u8 = 0;
pub const LOW: u8 = 1;
pub const HIGH: u8 = 2;

/// Terrain marks for drawing and for rules that care (the river's runes,
/// a lane's ground).
pub const T_GRASS: u8 = 0;
pub const T_LANE: u8 = 1;
pub const T_RIVER: u8 = 2;
pub const T_BASE: u8 = 3;
pub const T_RAMP: u8 = 4;
pub const T_PATH: u8 = 5;

/// Squared distance from point p to the segment a–b, in map units squared.
pub fn dist2_to_segment(p: P, a: P, b: P) -> i64 {
    let (px, py) = (p[0] as i64, p[1] as i64);
    let (ax, ay) = (a[0] as i64, a[1] as i64);
    let (bx, by) = (b[0] as i64, b[1] as i64);
    let (dx, dy) = (bx - ax, by - ay);
    let len2 = dx * dx + dy * dy;
    let (qx, qy) = if len2 == 0 {
        (ax, ay)
    } else {
        let t = ((px - ax) * dx + (py - ay) * dy).clamp(0, len2);
        (ax + dx * t / len2, ay + dy * t / len2)
    };
    (px - qx) * (px - qx) + (py - qy) * (py - qy)
}

pub fn dist2_to_line(p: P, line: &[P]) -> i64 {
    let mut best = i64::MAX;
    for w in line.windows(2) {
        best = best.min(dist2_to_segment(p, w[0], w[1]));
    }
    if line.len() == 1 {
        let (dx, dy) = ((p[0] - line[0][0]) as i64, (p[1] - line[0][1]) as i64);
        best = dx * dx + dy * dy;
    }
    best
}

/// Whether p lies inside the polygon (even-odd rule).
pub fn inside(p: P, poly: &[P]) -> bool {
    let mut c = false;
    let n = poly.len();
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) {
            let x = a[0] as i64 + (b[0] - a[0]) as i64 * (p[1] - a[1]) as i64 / (b[1] - a[1]) as i64;
            if (p[0] as i64) < x {
                c = !c;
            }
        }
        j = i;
    }
    c
}

/// A tree's state: standing, or cut and growing back at a tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tree {
    None,
    Standing,
    /// Cut; grows back at this tick.
    Cut(u32),
}

/// The fixed part of the ground: heights and terrain, never changed in play.
#[derive(Debug, Clone)]
pub struct Terrain {
    pub size: i32,
    pub tree_cell: i32,
    pub path_cell: i32,
    /// Tree cells across.
    pub tw: i32,
    /// Path cells across.
    pub pw: i32,
    pub level: Vec<u8>,
    pub mark: Vec<u8>,
    /// Path cells blocked by something that never changes (cliff, rim,
    /// building footprint).
    pub fixed_block: Vec<bool>,
}

impl Terrain {
    pub fn tree_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return None;
        }
        Some(((y / self.tree_cell) * self.tw + x / self.tree_cell) as usize)
    }

    pub fn path_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return None;
        }
        Some(((y / self.path_cell) * self.pw + x / self.path_cell) as usize)
    }

    /// Height level at a point in map units (outside the map: low ground).
    pub fn level_at(&self, x: i32, y: i32) -> u8 {
        self.tree_index(x, y).map(|i| self.level[i]).unwrap_or(LOW)
    }

    pub fn mark_at(&self, x: i32, y: i32) -> u8 {
        self.tree_index(x, y).map(|i| self.mark[i]).unwrap_or(T_GRASS)
    }

    /// The centre of tree cell i, in map units.
    pub fn tree_centre(&self, i: usize) -> P {
        let (cx, cy) = (i as i32 % self.tw, i as i32 / self.tw);
        [
            cx * self.tree_cell + self.tree_cell / 2,
            cy * self.tree_cell + self.tree_cell / 2,
        ]
    }

    pub fn path_centre(&self, i: usize) -> P {
        let (cx, cy) = (i as i32 % self.pw, i as i32 / self.pw);
        [
            cx * self.path_cell + self.path_cell / 2,
            cy * self.path_cell + self.path_cell / 2,
        ]
    }

    /// The tree cell holding path cell i.
    pub fn tree_of_path(&self, i: usize) -> usize {
        let p = self.path_centre(i);
        ((p[1] / self.tree_cell) * self.tw + p[0] / self.tree_cell) as usize
    }
}

/// The whole ground for a match: the fixed terrain and the trees' states.
pub struct Generated {
    pub terrain: Terrain,
    pub trees: Vec<Tree>,
}

/// Builds the ground from the map. `footprints` are building centres and
/// half-widths, which block walking and clear trees.
pub fn generate(map: &MapDef, footprints: &[(P, i32)]) -> Generated {
    let tc = map.tree_cell;
    let tw = map.size / tc;
    let pw = map.size / map.path_cell;
    let n = (tw * tw) as usize;
    let mut level = vec![LOW; n];
    let mut mark = vec![T_GRASS; n];
    let mut trees = vec![Tree::None; n];
    let mut rng = Rng::new(map.seed);
    let sq = |r: i32| (r as i64) * (r as i64);
    let river_w2 = sq(map.river_width);
    let lane_w2 = sq(map.lane_width);
    for i in 0..n {
        let p = [(i as i32 % tw) * tc + tc / 2, (i as i32 / tw) * tc + tc / 2];
        let mut clear = false;
        // base high ground
        for s in &map.sides {
            if inside(p, &s.base) {
                level[i] = HIGH;
                mark[i] = T_BASE;
                clear = true;
            }
        }
        if dist2_to_line(p, &map.river) <= river_w2 {
            level[i] = RIVER;
            mark[i] = T_RIVER;
            clear = true;
        }
        for (c, r) in &map.ramps {
            if dist2_to_line(p, &[*c]) <= sq(*r) {
                mark[i] = T_RAMP;
                level[i] = HIGH;
                clear = true;
            }
        }
        for s in &map.sides {
            for lane in s.lanes.values() {
                if dist2_to_line(p, lane) <= lane_w2 {
                    if mark[i] == T_GRASS {
                        mark[i] = T_LANE;
                    }
                    clear = true;
                }
            }
        }
        for (line, w) in &map.paths {
            if dist2_to_line(p, line) <= sq(*w) {
                if mark[i] == T_GRASS {
                    mark[i] = T_PATH;
                }
                clear = true;
            }
        }
        for (c, r) in &map.clearings {
            if dist2_to_line(p, &[*c]) <= sq(*r) {
                clear = true;
            }
        }
        for c in &map.camps {
            if dist2_to_line(p, &[c.at]) <= sq(320) {
                clear = true;
            }
        }
        for (c, r) in footprints {
            if dist2_to_line(p, &[*c]) <= sq(*r + tc) {
                clear = true;
            }
        }
        let cx = i as i32 % tw;
        let cy = i as i32 / tw;
        let rim = cx < 2 || cy < 2 || cx >= tw - 2 || cy >= tw - 2;
        // The roll is drawn for every cell, so a change to one clearing does
        // not reshuffle the gaps across the whole forest.
        let gap = rng.below(100) < map.gaps as u32;
        if rim || (!clear && !gap) {
            trees[i] = Tree::Standing;
        }
    }
    let mut fixed_block = vec![false; (pw * pw) as usize];
    let terrain_probe = Terrain {
        size: map.size,
        tree_cell: tc,
        path_cell: map.path_cell,
        tw,
        pw,
        level: level.clone(),
        mark: mark.clone(),
        fixed_block: Vec::new(),
    };
    for (j, block) in fixed_block.iter_mut().enumerate() {
        let p = terrain_probe.path_centre(j);
        let here = terrain_probe.tree_of_path(j);
        let (cx, cy) = (j as i32 % pw, j as i32 / pw);
        if cx == 0 || cy == 0 || cx == pw - 1 || cy == pw - 1 {
            *block = true;
            continue;
        }
        // A cliff: a high cell next to a lower one, unless either is a ramp.
        let lv = level[here];
        let ramp = mark[here] == T_RAMP;
        if lv == HIGH && !ramp {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let q = [p[0] + dx * map.path_cell, p[1] + dy * map.path_cell];
                if let Some(k) = terrain_probe.tree_index(q[0], q[1]) {
                    if level[k] < HIGH && mark[k] != T_RAMP {
                        *block = true;
                    }
                }
            }
        }
        for (c, r) in footprints {
            if (p[0] - c[0]).abs() < *r && (p[1] - c[1]).abs() < *r {
                *block = true;
            }
        }
    }
    Generated {
        terrain: Terrain {
            size: map.size,
            tree_cell: tc,
            path_cell: map.path_cell,
            tw,
            pw,
            level,
            mark,
            fixed_block,
        },
        trees,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_distance_is_worked_by_hand() {
        // (0,5) to the segment (0,0)–(10,0) is 5 away: 25 squared
        assert_eq!(dist2_to_segment([0, 5], [0, 0], [10, 0]), 25);
        // past the end: (13,4) to (10,0) is 5 away
        assert_eq!(dist2_to_segment([13, 4], [0, 0], [10, 0]), 25);
    }

    #[test]
    fn a_point_is_inside_a_square_and_outside_beyond_it() {
        let sq = [[0, 0], [10, 0], [10, 10], [0, 10]];
        assert!(inside([5, 5], &sq));
        assert!(!inside([15, 5], &sq));
    }
}
