//! The heroes screen's inspector (round 7, T34): one hero drawn in the
//! game's own renderer, standing alone for the reader to turn the camera
//! around, or playing a demonstration of an attack or an ability on loop.
//! Like the title's backdrop it is its own world, apart from `Game::m`, so
//! no match ever sees it; the demonstrations are the cast stage's worlds
//! (`Game::cast_world`), so what plays is what the ability does in a match.

use crate::fx;
use crate::rules::{P, SUB, TPS};
use crate::scene::View;
use crate::world::{Ctx, Uid, World};

/// Numbers from `data/presentation.json` `showcase`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowcaseLook {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// Camera distance, hundredths of a scene unit: standing alone, in a
    /// demonstration, and the nearest and farthest a wheel takes it.
    pub idle_distance: i32,
    pub demo_distance: i32,
    pub min_distance: i32,
    pub max_distance: i32,
    /// Hundredths of a scene unit each wheel notch moves the camera.
    pub zoom_step: i32,
    /// The camera's angle below the horizon at the start, and its limits, degrees.
    pub pitch: i32,
    pub min_pitch: i32,
    pub max_pitch: i32,
    /// The camera's turn at the start, degrees, and degrees per 100 pixels dragged.
    pub yaw: i32,
    pub drag_deg: i32,
    /// Degrees a second the camera turns by itself while nobody drags it.
    pub drift_deg: i32,
    /// Seconds a demonstration plays before it starts again, and longer
    /// ones by ability id.
    pub demo_s: i32,
    #[serde(default)]
    pub demo_s_for: std::collections::BTreeMap<String, i32>,
    /// Map units east of the hero a demonstration's camera looks at, where
    /// its targets stand.
    pub demo_ahead: i32,
    /// Percent of the camera's distance it looks to the subject's left,
    /// so the subject shows right of the panel.
    pub shift_pct: i32,
    /// Hundredths of a scene unit above the ground the camera looks at,
    /// standing alone (the hero's middle) and in a demonstration.
    pub idle_lift: i32,
    pub demo_lift: i32,
    /// Map units round a demonstration's spot whose trees are cut, so the
    /// camera meets no tree however it turns.
    pub open: i32,
}

/// What the inspector plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Demo {
    /// The hero alone, standing.
    Stand,
    /// The hero attacks a creep: its swing or its shot.
    Attack,
    /// Ability `k` (an index into the rules' abilities).
    Ability(usize),
}

pub struct Showcase {
    /// The hero, by its place on the roster.
    pub hero: usize,
    pub demo: Demo,
    pub world: World,
    /// Where each unit stood one tick back, for smooth drawing.
    pub prev: Vec<(Uid, i32, i32, i32)>,
    /// Q16 share of the way to the next tick.
    pub alpha: i32,
    acc_us: i64,
    /// The tick the world started at, and the tick it starts again (0: never).
    pub begun: u32,
    pub again: u32,
    /// The camera, angle units and Q16 scene units.
    pub yaw: i32,
    pub pitch: i32,
    pub distance: i32,
    /// Where a drag on the view began, in pixels, while one is under way.
    pub drag: Option<(i32, i32)>,
    /// Ticks run since the last drag ended, so the camera turns by itself
    /// only once the reader has let go for a while.
    still: u32,
    /// The hero's numbers at level 1, whatever level the demonstration plays at.
    pub level1: crate::stats::Stats,
}

impl Showcase {
    pub fn new(
        hero: usize,
        demo: Demo,
        world: World,
        lk: &ShowcaseLook,
        again: u32,
        level1: crate::stats::Stats,
    ) -> Showcase {
        let begun = world.tick;
        let mut s = Showcase {
            hero,
            demo,
            world,
            prev: Vec::new(),
            alpha: 0,
            acc_us: 0,
            begun,
            again: if again == 0 { 0 } else { begun + again },
            yaw: fx::deg(lk.yaw),
            pitch: fx::deg(lk.pitch),
            distance: 0,
            drag: None,
            still: 0,
            level1,
        };
        s.distance = s.rest_distance(lk);
        s.remember();
        s
    }

    /// The camera distance this demonstration starts at.
    pub fn rest_distance(&self, lk: &ShowcaseLook) -> i32 {
        let d = if self.demo == Demo::Stand {
            lk.idle_distance
        } else {
            lk.demo_distance
        };
        d * fx::ONE / 100
    }

    /// Keeps the camera of an earlier showcase of the same hero, so
    /// choosing another demonstration does not throw the view back.
    pub fn keep_camera(&mut self, old: &Showcase) {
        self.yaw = old.yaw;
        self.pitch = old.pitch;
        if old.demo == Demo::Stand && self.demo != Demo::Stand {
            return;
        }
        if old.demo != Demo::Stand && self.demo == Demo::Stand {
            return;
        }
        self.distance = old.distance;
    }

    fn remember(&mut self) {
        self.prev.clear();
        self.prev
            .extend(self.world.units.iter().map(|u| (u.id, u.x, u.y, u.facing)));
    }

    /// Whether the demonstration has played out and should start again.
    pub fn done(&self) -> bool {
        self.again != 0 && self.world.tick >= self.again
    }

    /// One display frame of `dt` microseconds.
    pub fn frame(&mut self, ctx: &Ctx, lk: &ShowcaseLook, dt: i64, max_ticks: i32) {
        let tick_us = 1_000_000 / TPS as i64;
        self.acc_us += dt;
        let mut ran = 0;
        while self.acc_us >= tick_us && ran < max_ticks {
            self.acc_us -= tick_us;
            ran += 1;
            self.remember();
            self.world.step(ctx, &[]);
            if self.drag.is_none() {
                self.still += 1;
            }
        }
        if ran == max_ticks {
            self.acc_us = 0;
        }
        self.alpha = (self.acc_us * fx::ONE as i64 / tick_us) as i32;
        // the camera drifts round the hero once the reader has let go a while
        if self.demo == Demo::Stand && self.drag.is_none() && self.still > TPS as u32 * 3 {
            self.yaw += (fx::deg(lk.drift_deg) as i64 * dt / 1_000_000) as i32;
        }
    }

    /// A drag on the view moved to (x, y): the camera turns with it.
    pub fn drag_to(&mut self, lk: &ShowcaseLook, x: i32, y: i32) {
        let Some((x0, y0)) = self.drag else { return };
        let per = fx::deg(lk.drag_deg) / 100;
        self.yaw -= (x - x0) * per;
        self.pitch = (self.pitch + (y - y0) * per).clamp(fx::deg(lk.min_pitch), fx::deg(lk.max_pitch));
        self.drag = Some((x, y));
        self.still = 0;
    }

    pub fn zoom(&mut self, lk: &ShowcaseLook, dy: i32) {
        let d = self.distance + dy.signum() * lk.zoom_step * fx::ONE / 100;
        self.distance = d.clamp(lk.min_distance * fx::ONE / 100, lk.max_distance * fx::ONE / 100);
    }

    /// The height the camera looks at, Q16 scene units.
    pub fn lift(&self, lk: &ShowcaseLook) -> i32 {
        let l = if self.demo == Demo::Stand {
            lk.idle_lift
        } else {
            lk.demo_lift
        };
        l * fx::ONE / 100
    }

    /// The hero's unit, when it stands in the world.
    pub fn hero_unit(&self) -> Option<Uid> {
        let h = self.world.players[0].hero;
        self.world.get(h).map(|u| u.id)
    }

    /// The camera: on the hero, or ahead of it where a demonstration's
    /// targets stand, moved to the subject's left so it shows right of the
    /// panel on the left of the screen.
    pub fn view(&self, lk: &ShowcaseLook, fov: i32) -> View {
        let at: P = self
            .hero_unit()
            .and_then(|h| self.world.get(h))
            .map(|u| [u.x / SUB, u.y / SUB])
            .unwrap_or([0, 0]);
        let ahead = if self.demo == Demo::Stand {
            0
        } else {
            lk.demo_ahead
        };
        // the screen's right, on the map: (cos yaw, sin yaw)
        let units = fx::mul(self.distance, fx::int(crate::scene::GAME_UNITS_PER_SCENE)) / fx::ONE;
        let left = units * lk.shift_pct / 100;
        let (c, s) = (fx::cos(self.yaw), fx::sin(self.yaw));
        View {
            target: [
                at[0] + ahead - fx::mul(c, fx::int(left)) / fx::ONE,
                at[1] - fx::mul(s, fx::int(left)) / fx::ONE,
            ],
            distance: self.distance,
            pitch: self.pitch,
            fov,
            yaw: self.yaw,
        }
    }
}
