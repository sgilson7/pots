//! The title screen's backdrop (round 7, after Ape Race and Mage Tower
//! Battle): a match between ten computer players runs behind the menu, and
//! the camera drifts from one fight to the next. It is its own world, apart
//! from `Game::m`, so the arcade and online play never see it. It starts by
//! catching up to the first clash of creeps a few hundred ticks a frame,
//! and is drawn only once it gets there.

use crate::fx;
use crate::rules::{P, SUB, TPS};
use crate::world::{dist, Ctx, Uid, World, NEUTRAL, PLAYERS};

/// Numbers from `data/presentation.json` `backdrop`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackdropLook {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// Seconds of match run before the backdrop is first drawn.
    pub start_s: i32,
    /// Ticks run each frame while catching up to `start_s`.
    pub catch_up: i32,
    /// The computer players' skill (0 to 4).
    pub skill: u8,
    /// Seconds the camera stays with one fight before it looks again.
    pub hold_s: i32,
    /// The camera's top speed, map units a second, and the percent of the
    /// way left it closes each second below that.
    pub pan_speed: i32,
    pub ease_pct: i32,
    /// How near an enemy must be to count toward a fight, map units.
    pub seek: i32,
    /// Map units the camera stands west of the fight, so the fight shows
    /// right of the menu.
    pub west: i32,
    /// Beyond this many map units the camera cuts to a new fight rather
    /// than sweeping over the forest to it.
    pub cut: i32,
    /// A frame slower than this many milliseconds counts as slow; after
    /// `slow_frames` slow frames in a row the backdrop stops for good, so
    /// a machine that cannot draw it smoothly keeps a plain title.
    pub slow_ms: i32,
    pub slow_frames: i32,
}

pub struct Backdrop {
    pub world: World,
    /// Where each unit stood one tick back, for smooth drawing.
    pub prev: Vec<(Uid, i32, i32, i32)>,
    /// Q16 share of the way to the next tick.
    pub alpha: i32,
    acc_us: i64,
    /// The hero the camera follows, and the tick it looks again.
    pub follow: Uid,
    next_look: u32,
    /// Slow frames in a row since it was first drawn.
    slow: i32,
    /// Given up on this machine: never drawn or run again.
    pub stopped: bool,
}

impl Backdrop {
    /// A match with every seat a computer player, from `seed`.
    pub fn new(world: World, skill: u8) -> Backdrop {
        let mut world = world;
        for p in 0..PLAYERS {
            world.players[p].bot = Some(skill);
            world.players[p].present = true;
        }
        Backdrop {
            world,
            prev: Vec::new(),
            alpha: 0,
            acc_us: 0,
            follow: crate::world::NO_UNIT,
            next_look: 0,
            slow: 0,
            stopped: false,
        }
    }

    /// Whether it has caught up to the first clash and may be drawn.
    pub fn ready(&self, lk: &BackdropLook) -> bool {
        !self.stopped && self.world.tick >= (lk.start_s * TPS) as u32
    }

    fn remember(&mut self) {
        self.prev.clear();
        self.prev
            .extend(self.world.units.iter().map(|u| (u.id, u.x, u.y, u.facing)));
    }

    /// One display frame of `dt` microseconds; returns where the camera
    /// should look now, given where it looks (`at`).
    pub fn frame(&mut self, ctx: &Ctx, lk: &BackdropLook, dt: i64, at: P, max_ticks: i32) -> P {
        if !self.ready(lk) {
            let start = (lk.start_s * TPS) as u32;
            for _ in 0..lk.catch_up.max(1) {
                if self.world.tick >= start {
                    break;
                }
                self.world.step(ctx, &[]);
            }
            if !self.ready(lk) {
                return at;
            }
            // first drawn frame: straight to the busiest fight
            self.remember();
            self.follow = busiest(&self.world, lk.seek, 0);
            self.next_look = self.world.tick + (lk.hold_s * TPS) as u32;
            return self.goal(ctx, lk);
        }
        // a machine that draws it slowly gets the plain title instead
        self.slow = if dt > lk.slow_ms as i64 * 1000 {
            self.slow + 1
        } else {
            0
        };
        if self.slow >= lk.slow_frames {
            self.stopped = true;
            return at;
        }
        let tick_us = 1_000_000 / TPS as i64;
        self.acc_us += dt;
        let mut ran = 0;
        while self.acc_us >= tick_us && ran < max_ticks {
            self.acc_us -= tick_us;
            ran += 1;
            self.remember();
            self.world.step(ctx, &[]);
        }
        if ran == max_ticks {
            self.acc_us = 0;
        }
        self.alpha = (self.acc_us * fx::ONE as i64 / tick_us) as i32;
        // look again on the beat, or when the hero followed falls
        if self.world.tick >= self.next_look || self.world.live(self.follow).is_none() {
            let turn = self.world.tick / (lk.hold_s * TPS).max(1) as u32;
            self.follow = busiest(&self.world, lk.seek, turn as usize);
            self.next_look = self.world.tick + (lk.hold_s * TPS) as u32;
        }
        // ease toward the goal, no faster than the top speed
        let goal = self.goal(ctx, lk);
        let d = dist(at, goal);
        if d == 0 || d > lk.cut {
            return goal;
        }
        let speed = (d as i64 * lk.ease_pct as i64 / 100).min(lk.pan_speed as i64);
        let step = (speed * dt / 1_000_000).min(d as i64) as i32;
        [
            at[0] + ((goal[0] - at[0]) as i64 * step as i64 / d as i64) as i32,
            at[1] + ((goal[1] - at[1]) as i64 * step as i64 / d as i64) as i32,
        ]
    }

    /// Where the camera should stand: west of the hero it follows, or the
    /// map's centre with no hero to follow.
    pub fn goal(&self, ctx: &Ctx, lk: &BackdropLook) -> P {
        match self.world.live(self.follow) {
            Some(u) => [u.x / SUB - lk.west, u.y / SUB],
            None => {
                let c = ctx.rules.map.size / 2;
                [c, c]
            }
        }
    }
}

/// The living hero with the most enemies within `seek` of it; ties, and a
/// field with no fight at all, go round the heroes by `turn`, so the camera
/// does not sit on one hero.
pub fn busiest(w: &World, seek: i32, turn: usize) -> Uid {
    let heroes: Vec<_> = w.units.iter().filter(|u| u.alive && u.hero.is_some()).collect();
    if heroes.is_empty() {
        return crate::world::NO_UNIT;
    }
    let n = heroes.len();
    let mut best: Option<(usize, usize)> = None;
    for k in 0..n {
        let h = heroes[(k + turn) % n];
        let near = w
            .units
            .iter()
            .filter(|e| e.alive && e.team != h.team && e.team != NEUTRAL && dist(e.pos(), h.pos()) <= seek)
            .count();
        if best.map(|(c, _)| near > c).unwrap_or(true) {
            best = Some((near, (k + turn) % n));
        }
    }
    heroes[best.map(|b| b.1).unwrap_or(0)].id
}
