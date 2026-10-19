//! The faceted ground (round 7, journal tab 11): one surface of corners
//! every half cell, cliffs where high ground meets low, a sunken riverbed,
//! and everything that stands on the ground placed at the drawn height.
//! Expected heights are worked out by hand from `scene::level_height`
//! (high ground 2, low ground 0, the river -3/4 and its bed half a unit
//! below that) and the rule that a corner blends only the cells within
//! three quarters of a cell of it.

mod common;

use pots_core::fx::ONE;
use pots_core::map::{Terrain, HIGH, T_RAMP, T_RIVER};
use pots_core::scene::{ground_at, surface_jitter};

fn high(t: &Terrain, x: i32, y: i32) -> bool {
    let i = (y * t.tw + x) as usize;
    t.level[i] == HIGH && t.mark[i] != T_RAMP
}

fn low(t: &Terrain, x: i32, y: i32) -> bool {
    let i = (y * t.tw + x) as usize;
    t.level[i] != HIGH && t.mark[i] != T_RAMP && t.mark[i] != T_RIVER
}

/// A corner's height less its seeded unevenness (a sixteenth of a unit at most).
fn bare(t: &Terrain, k: i32, j: i32) -> i32 {
    let step = t.tree_cell / 2;
    t.surface_corner(k, j) - surface_jitter(k * step, j * step) * ONE / 160
}

#[test]
fn a_plateau_keeps_its_edge_and_drops_to_the_low_ground_within_half_a_cell() {
    let f = common::fixture();
    let t = &f.terrain;
    // a high cell with low ground east of it, and low ground beyond that
    let mut found = None;
    'search: for y in 2..t.tw - 2 {
        for x in 2..t.tw - 3 {
            if high(t, x, y) && high(t, x - 1, y) && low(t, x + 1, y) && low(t, x + 2, y) {
                found = Some((x, y));
                break 'search;
            }
        }
    }
    let (x, y) = found.expect("the map has a cliff facing east");
    // corners: the high cell's centre, the shared edge, the low cell's centre
    let (centre_hi, edge, centre_lo) = (
        bare(t, 2 * x + 1, 2 * y + 1),
        bare(t, 2 * x + 2, 2 * y + 1),
        bare(t, 2 * x + 3, 2 * y + 1),
    );
    assert_eq!(centre_hi, 2 * ONE, "the plateau is at high ground's height");
    assert_eq!(edge, 2 * ONE, "the cliff's top edge stays high");
    assert_eq!(centre_lo, 0, "half a cell out the ground is low");
}

#[test]
fn the_riverbed_sinks_below_the_water() {
    let f = common::fixture();
    let t = &f.terrain;
    let river = |x: i32, y: i32| t.mark[(y * t.tw + x) as usize] == T_RIVER;
    let (x, y) = (2..t.tw - 2)
        .flat_map(|y| (2..t.tw - 2).map(move |x| (x, y)))
        .find(|&(x, y)| (-1..=1).all(|dy| (-1..=1).all(|dx| river(x + dx, y + dy))))
        .expect("the river is three cells wide somewhere");
    // -3/4 for the river's level, half a unit lower for its bed
    assert_eq!(bare(t, 2 * x + 1, 2 * y + 1), -ONE * 5 / 4);
}

#[test]
fn a_point_stands_on_the_surface_between_its_corners() {
    let f = common::fixture();
    let t = &f.terrain;
    let tc = t.tree_cell;
    // a cell centre is a corner, so the height there is that corner's
    let (x, y) = (t.tw / 3, t.tw / 5);
    let c = [x * tc + tc / 2, y * tc + tc / 2];
    assert_eq!(ground_at(t, c), t.surface_corner(2 * x + 1, 2 * y + 1));
    // a quarter cell east of it lies halfway to the next corner east
    let q = [c[0] + tc / 4, c[1]];
    let (a, b) = (
        t.surface_corner(2 * x + 1, 2 * y + 1),
        t.surface_corner(2 * x + 2, 2 * y + 1),
    );
    assert!(
        (ground_at(t, q) - (a + b) / 2).abs() <= 1,
        "halfway between {a} and {b}"
    );
    // and a quarter cell north of it halfway to the next corner north
    let q = [c[0], c[1] + tc / 4];
    let (a, b) = (
        t.surface_corner(2 * x + 1, 2 * y + 1),
        t.surface_corner(2 * x + 1, 2 * y + 2),
    );
    assert!(
        (ground_at(t, q) - (a + b) / 2).abs() <= 1,
        "halfway between {a} and {b}"
    );
}
