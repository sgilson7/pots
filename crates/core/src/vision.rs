//! What each side can see: a bit per tree cell, redrawn every few ticks.
//!
//! Every living unit lights a disc of its day or night vision. A unit on
//! lower ground does not see up onto higher ground (Warcraft III's cliff
//! rule); flying vision is not in this game. Standing trees block sight
//! beyond them: a cell is lit only when the line from the unit to it crosses
//! no standing tree, checked by stepping the line in tree cells.

use crate::map::Tree;
use crate::rules::Kind;
use crate::world::{Ctx, World, NEUTRAL};

pub fn update(w: &mut World, ctx: &Ctx) {
    let tr = ctx.terrain;
    for v in w.vision.iter_mut() {
        *v = 0;
    }
    let day = w.is_day(ctx);
    let tw = tr.tw;
    let tc = tr.tree_cell;
    for u in &w.units {
        if !u.alive || u.team == NEUTRAL {
            continue;
        }
        let d = &ctx.rules.units[u.def as usize];
        if matches!(d.kind, Kind::Shop) {
            continue;
        }
        let r = if day { d.day_vision } else { d.night_vision };
        let bit = 1u8 << u.team;
        let p = u.pos();
        let (cx, cy) = (p[0] / tc, p[1] / tc);
        let rc = r / tc;
        let own = tr.level_at(p[0], p[1]);
        let flying = d.kind == Kind::Ward;
        for dy in -rc..=rc {
            for dx in -rc..=rc {
                if dx * dx + dy * dy > rc * rc {
                    continue;
                }
                let (x, y) = (cx + dx, cy + dy);
                if x < 0 || y < 0 || x >= tw || y >= tw {
                    continue;
                }
                let i = (y * tw + x) as usize;
                if w.vision[i] & bit != 0 {
                    continue;
                }
                if !flying && tr.level[i] > own {
                    continue;
                }
                if !flying && !clear(w, tw, (cx, cy), (x, y)) {
                    continue;
                }
                w.vision[i] |= bit;
            }
        }
    }
}

/// Whether the line between two tree cells crosses a standing tree before
/// the last cell (the tree itself is seen).
fn clear(w: &World, tw: i32, a: (i32, i32), b: (i32, i32)) -> bool {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let n = dx.abs().max(dy.abs());
    if n <= 1 {
        return true;
    }
    // round half up, the same way for negative steps
    let rdiv = |p: i32, q: i32| (2 * p + q).div_euclid(2 * q);
    for s in 1..n {
        let x = a.0 + rdiv(dx * s, n);
        let y = a.1 + rdiv(dy * s, n);
        if matches!(w.trees[(y * tw + x) as usize], Tree::Standing) {
            return false;
        }
    }
    true
}
