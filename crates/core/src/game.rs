//! The game around the match: screens, arcade missions, the camera, the
//! mouse and keyboard, and the buffers the page draws.
//!
//! The page passes raw input (a key's code, a button's number, pointer
//! pixels, the clock) and draws what this returns. Every choice, from what
//! a right-click on an enemy means to how far the camera pans, is made here.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::ai::AiDef;
use crate::fx::{self, ONE};
use crate::map::{Terrain, Tree};
use crate::models::Models;
use crate::orders::Cmd;
use crate::render::{self, Batch};
use crate::rules::{Kind, Rules, Target, P, SUB, TPS};
use crate::scene::{self, Frame, Look, View};
use crate::text::Strings;
use crate::world::{self, Ctx, Uid, World, NEUTRAL, NO_UNIT, PLAYERS, SUN};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mission {
    pub id: String,
    pub allies: usize,
    pub enemies: usize,
    pub ally: u8,
    pub enemy: u8,
    /// "last_hits", "tower", "kills", "boss", "sphere"
    pub goal: String,
    pub target: i32,
    /// Time limit in minutes; 0 for none.
    pub minutes: i32,
    /// A fixed hero, or -1 to choose.
    pub hero: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arcade {
    pub missions: Vec<Mission>,
    pub custom: CustomRange,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomRange {
    pub allies: [usize; 2],
    pub enemies: [usize; 2],
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Keys {
    pub bindings: BTreeMap<String, String>,
    pub learn_keys: BTreeMap<String, u8>,
}

/// What a click will do next, after an ability key or the attack key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aim {
    Ability(u8),
    Item(u8),
    AttackMove,
    /// All Random All Mid: where the base cannon throws the hero.
    Cannon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Won,
    Lost,
}

/// A match in progress, offline or online.
pub struct Match {
    pub world: World,
    /// The local player's seat.
    pub me: usize,
    pub mission: Option<usize>,
    pub paused: bool,
    pub over: Option<Outcome>,
    /// Commands from this browser waiting for the next tick.
    pub pending: Vec<Cmd>,
    pub online: bool,
    /// Kill feed and announcements: (tick, text).
    pub feed: Vec<(u32, String)>,
    /// Gold this player earned, shown floating where it was earned: (tick, place, amount).
    pub popups: Vec<(u32, P, i32)>,
    /// Towers of the other side destroyed, hero kills, bosses slain by my side.
    pub towers_taken: i32,
    pub bosses: i32,
}

/// The arcade setup being chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Setup {
    pub allies: usize,
    pub enemies: usize,
    pub ally: u8,
    pub enemy: u8,
    pub hero: i32,
    pub mission: Option<usize>,
    /// All Random All Mid (round 4): heroes dealt at random, one lane.
    pub aram: bool,
    /// Whether this player has spent their reroll.
    pub rerolled: bool,
}

pub struct Game {
    pub rules: Rules,
    pub terrain: Terrain,
    trees0: Vec<Tree>,
    pub models: Models,
    pub look: Look,
    pub strings: Strings,
    pub glossary: Vec<(String, String)>,
    pub keys: Keys,
    pub arcade: Arcade,
    pub screen: String,
    pub m: Option<Match>,
    pub seed: u64,
    pub view: View,
    /// Canvas size in CSS pixels and the device ratio in hundredths.
    pub width: i32,
    pub height: i32,
    pub dpr: i32,
    pub pointer: (i32, i32),
    pub pointer_in: bool,
    /// Whether the pointer is over the 3D view itself, not the HUD on it.
    pub pointer_view: bool,
    pub hover: Uid,
    pub selected: Uid,
    pub held: Vec<String>,
    pub aim: Option<Aim>,
    pub learn: bool,
    pub shop: bool,
    pub shop_tab: String,
    /// The item whose build tree the shop shows, or -1.
    pub shop_pick: i32,
    pub score: bool,
    pub menu: bool,
    /// Arcade matches started, so each All Random All Mid match deals afresh.
    played: u64,
    /// Microseconds of display time since the game began, for what the
    /// player's own hand shows (the order marker).
    clock_us: i64,
    /// The last order click: where, on whom, whether an attack, and when.
    ping: Option<(P, Uid, bool, i64)>,
    last_us: i64,
    acc_us: i64,
    prev: Vec<(Uid, i32, i32, i32)>,
    pub setup: Setup,
    /// Q16 share of the way to the next tick.
    alpha: i32,
    /// A still scene for screenshots, when set.
    pub stage: Option<Stage>,
    pub online: crate::online::Online,
    /// A message for the page's status line (an error, a refusal).
    pub notice: String,
    /// The scene built by the last `render`.
    pub last: render::Finished,
    /// The match behind the title screen (round 7), made when the title
    /// is first shown.
    pub backdrop: Option<crate::backdrop::Backdrop>,
    /// The reader's browser asks for reduced motion: the backdrop stands
    /// still once drawn.
    pub reduced_motion: bool,
}

/// A still scene set up for a screenshot (`?tile=...`).
#[derive(Debug, Clone)]
pub enum Stage {
    /// Models in a row, turned to face the camera, with the gap between
    /// them in scene units.
    Lineup(Vec<String>, i32),
    /// The map from high above.
    Overview,
}

/// The data files the page fetches and hands over, by name.
pub struct Files<'a> {
    pub strings: &'a str,
    pub glossary: &'a str,
    pub rules: &'a str,
    pub units: &'a str,
    pub abilities: &'a str,
    pub items: &'a str,
    pub map: &'a str,
    pub ai: &'a str,
    pub models: &'a str,
    pub presentation: &'a str,
    pub keys: &'a str,
    pub arcade: &'a str,
}

fn parse<T: serde::de::DeserializeOwned>(what: &str, text: &str) -> Result<T, String> {
    serde_json::from_str(text).map_err(|e| format!("{what} could not be read: {e}"))
}

impl Game {
    pub fn new(f: &Files, seed: u64) -> Result<Game, String> {
        let mut rules = Rules::load(f.rules, f.units, f.abilities, f.items, f.map)?;
        rules.ai = Some(AiDef::load(f.ai)?);
        let (terrain, trees0) = world::setup(&rules);
        let mut models = Models::load(f.models)?;
        for d in &rules.units {
            if models.find(&d.model).is_none() {
                return Err(format!("Unit \"{}\" has no model \"{}\".", d.id, d.model));
            }
        }
        let look: Look = parse("presentation.json", f.presentation)?;
        models.facet_all_but(&look.map_look.keep_smooth)?;
        let strings = Strings::load(f.strings)?;
        let glossary = crate::glossary::parse(f.glossary)
            .map_err(|e| e.message())?
            .into_iter()
            .map(|e| (e.term, e.definition))
            .collect();
        let keys: Keys = parse("keys.json", f.keys)?;
        let arcade: Arcade = parse("arcade.json", f.arcade)?;
        let view = View {
            target: rules.map.sides[0].spawn,
            distance: look.cam_distance * ONE / 100,
            pitch: fx::deg(look.cam_pitch),
            fov: fx::deg(look.cam_fov),
            yaw: 0,
        };
        Ok(Game {
            rules,
            terrain,
            trees0,
            models,
            look,
            strings,
            glossary,
            keys,
            arcade,
            screen: "title".into(),
            m: None,
            seed,
            view,
            width: 1280,
            height: 720,
            dpr: 100,
            pointer: (0, 0),
            pointer_in: false,
            pointer_view: false,
            hover: NO_UNIT,
            selected: NO_UNIT,
            held: Vec::new(),
            aim: None,
            learn: false,
            shop: false,
            shop_tab: "basics".into(),
            shop_pick: -1,
            score: false,
            menu: false,
            played: 0,
            clock_us: 0,
            ping: None,
            last_us: -1,
            acc_us: 0,
            prev: Vec::new(),
            setup: Setup {
                allies: 4,
                enemies: 5,
                ally: 2,
                enemy: 2,
                hero: -1,
                mission: None,
                aram: false,
                rerolled: false,
            },
            alpha: 0,
            stage: None,
            online: crate::online::Online::default(),
            notice: String::new(),
            last: (Vec::new(), Vec::new()),
            backdrop: None,
            reduced_motion: false,
        })
    }

    pub fn ctx(&self) -> Ctx<'_> {
        Ctx {
            rules: &self.rules,
            terrain: &self.terrain,
        }
    }

    pub fn aspect(&self) -> i32 {
        fx::div(fx::int(self.width.max(1)), fx::int(self.height.max(1)))
    }

    /// CSS size and device ratio in; the drawing buffer's size out.
    pub fn resize(&mut self, w: i32, h: i32, dpr: i32) -> [i32; 2] {
        self.width = w.max(1);
        self.height = h.max(1);
        self.dpr = dpr.max(50);
        [self.width * self.dpr / 100, self.height * self.dpr / 100]
    }

    /// The forest every match starts with.
    pub fn trees_for_match(&self) -> Vec<crate::map::Tree> {
        self.trees0.clone()
    }

    /// A fresh world for a match with these seats.
    pub fn new_world(&self, seed: u64) -> World {
        World::new(&self.ctx(), self.trees0.clone(), seed)
    }

    /// Starts an arcade match from the setup: me in seat 0, allies after
    /// me, enemies in the Moonward seats.
    /// The seed the next arcade match deals its heroes from.
    pub fn arcade_seed(&self) -> u64 {
        self.seed ^ self.played.wrapping_mul(0x9e37_79b9_7f4a_7c15)
    }

    /// This mode's deal for the next arcade match: every seat's hero.
    pub fn arcade_deal(&self) -> [u8; PLAYERS] {
        let mut rerolled = [false; PLAYERS];
        rerolled[0] = self.setup.rerolled;
        crate::aram::deal(self.arcade_seed(), self.rules.heroes.len(), &rerolled)
    }

    pub fn start_arcade(&mut self) {
        let s = self.setup;
        let aram = s.aram && s.mission.is_none();
        let deal = self.arcade_deal();
        let mut w = World::new_mode(&self.ctx(), self.trees0.clone(), self.arcade_seed(), aram);
        self.played += 1;
        let t = &self.rules.t;
        // a shorter wait before the first wave in the arcade
        w.horn_at = (t.horn * TPS / 2) as u32;
        w.players[0].present = true;
        for p in 1..=s.allies.min(4) {
            w.players[p].bot = Some(s.ally);
            w.players[p].present = true;
        }
        for p in 5..5 + s.enemies.clamp(1, 5) {
            w.players[p].bot = Some(s.enemy);
            w.players[p].present = true;
        }
        let ctx = self.ctx();
        if aram {
            // every seat takes its dealt hero, so the computer players pick nothing
            for (p, &hero) in deal.iter().enumerate() {
                if w.players[p].present {
                    crate::orders::apply(&mut w, &ctx, p, Cmd::Pick(hero));
                }
            }
        } else {
            let hero = if s.hero >= 0 { s.hero as usize } else { 0 };
            crate::orders::apply(&mut w, &ctx, 0, Cmd::Pick(hero as u8));
        }
        self.begin(w, 0, s.mission, false);
    }

    /// Puts a world on screen as the current match.
    pub fn begin(&mut self, w: World, me: usize, mission: Option<usize>, online: bool) {
        let spawn = self.rules.map.sides[w.players[me].team as usize].spawn;
        self.view.target = spawn;
        self.prev.clear();
        self.m = Some(Match {
            world: w,
            me,
            mission,
            paused: false,
            over: None,
            pending: Vec::new(),
            online,
            feed: Vec::new(),
            popups: Vec::new(),
            towers_taken: 0,
            bosses: 0,
        });
        self.screen = "match".into();
        self.last_us = -1;
        self.acc_us = 0;
        self.aim = None;
        self.shop = false;
        self.selected = NO_UNIT;
    }

    pub fn me_team(&self) -> u8 {
        self.m.as_ref().map(|m| m.world.players[m.me].team).unwrap_or(SUN)
    }

    pub fn my_hero(&self) -> Uid {
        self.m
            .as_ref()
            .map(|m| m.world.players[m.me].hero)
            .unwrap_or(NO_UNIT)
    }

    /// One display frame: run the ticks the clock allows, pan the camera.
    pub fn frame(&mut self, now_us: i64) {
        let dt = if self.last_us < 0 {
            0
        } else {
            (now_us - self.last_us).clamp(0, 250_000)
        };
        self.last_us = now_us;
        self.clock_us += dt;
        self.pan(dt);
        self.follow_throw();
        if self.on_title() {
            self.backdrop_frame(dt);
            return;
        }
        if self.screen != "match" {
            return;
        }
        if self.m.as_ref().map(|m| m.online).unwrap_or(false) {
            crate::online::frame(self, dt);
            return;
        }
        let Some(m) = self.m.as_mut() else { return };
        if m.paused || m.over.is_some() {
            return;
        }
        let tick_us = 1_000_000 / TPS as i64;
        self.acc_us += dt;
        let mut ran = 0;
        while self.acc_us >= tick_us && ran < self.look.max_ticks_per_frame {
            self.acc_us -= tick_us;
            ran += 1;
            self.tick_local();
        }
        if ran == self.look.max_ticks_per_frame {
            // a slow machine drops time rather than spiralling
            self.acc_us = 0;
        }
        self.alpha = (self.acc_us * ONE as i64 / tick_us) as i32;
    }

    /// Whether the title screen shows, with its backdrop behind it.
    pub fn on_title(&self) -> bool {
        self.screen == "title" && self.stage.is_none()
    }

    /// Runs the title screen's backdrop match for one display frame and
    /// moves the camera with it (round 7).
    fn backdrop_frame(&mut self, dt: i64) {
        let mut b = match self.backdrop.take() {
            Some(b) => b,
            None => crate::backdrop::Backdrop::new(
                self.new_world(self.seed ^ 0x7417_1e5c_e4e5),
                self.look.backdrop.skill,
            ),
        };
        let ctx = Ctx {
            rules: &self.rules,
            terrain: &self.terrain,
        };
        if b.stopped || self.reduced_motion && b.ready(&self.look.backdrop) {
            // a still of the match, not a moving one
            self.backdrop = Some(b);
            return;
        }
        self.view.target = b.frame(
            &ctx,
            &self.look.backdrop,
            dt,
            self.view.target,
            self.look.max_ticks_per_frame,
        );
        self.backdrop = Some(b);
    }

    /// Remembers where every unit stands before a tick, for smooth drawing.
    pub fn remember(&mut self) {
        let Some(m) = self.m.as_ref() else { return };
        self.prev.clear();
        self.prev
            .extend(m.world.units.iter().map(|u| (u.id, u.x, u.y, u.facing)));
    }

    fn tick_local(&mut self) {
        self.remember();
        let ctx = Ctx {
            rules: &self.rules,
            terrain: &self.terrain,
        };
        let m = self.m.as_mut().expect("in a match");
        let me = m.me as u8;
        let orders: Vec<(u8, Cmd)> = m.pending.drain(..).map(|c| (me, c)).collect();
        m.world.step(&ctx, &orders);
        Self::after_tick(m, &ctx, &self.strings, &self.arcade);
    }

    /// Feed lines and the arcade's goal, after any tick (local or online).
    pub fn after_tick(m: &mut Match, ctx: &Ctx, strings: &Strings, arcade: &Arcade) {
        let team = m.world.players[m.me].team;
        let events = m.world.events.clone();
        for e in events {
            match e.kind {
                world::EV_KILL => {
                    let name = |p: i32| -> String {
                        if p < 0 {
                            return strings.ui("feed.creeps");
                        }
                        let pick = m.world.players[p as usize].pick;
                        let id = &ctx.rules.units[ctx.rules.heroes[pick.max(0) as usize]].id;
                        strings.hero(id).name
                    };
                    let line = strings
                        .ui("feed.kill")
                        .replace("{killer}", &name(e.a))
                        .replace("{victim}", &name(e.b));
                    m.feed.push((m.world.tick, line));
                }
                world::EV_FIRST_BLOOD => m.feed.push((m.world.tick, strings.ui("feed.first_blood"))),
                world::EV_TOWER => {
                    if e.a as u8 != team {
                        m.towers_taken += 1;
                    }
                    let key = if e.a as u8 == team {
                        "feed.tower_lost"
                    } else {
                        "feed.tower_taken"
                    };
                    m.feed.push((m.world.tick, strings.ui(key)));
                }
                world::EV_BOSS => {
                    if e.a as u8 == team {
                        m.bosses += 1;
                    }
                    m.feed.push((m.world.tick, strings.ui("feed.boss")));
                }
                world::EV_RUNE => m.feed.push((m.world.tick, strings.ui("feed.rune"))),
                world::EV_GOLD if e.b == m.me as i32 && e.at != [0, 0] => {
                    m.popups.push((m.world.tick, e.at, e.a))
                }
                _ => {}
            }
        }
        let now = m.world.tick;
        m.popups.retain(|(t, _, _)| now < t + (3 * TPS / 2) as u32);
        if m.feed.len() > 8 {
            let cut = m.feed.len() - 8;
            m.feed.drain(..cut);
        }
        if let Some(winner) = m.world.winner {
            m.over = Some(if winner == team {
                Outcome::Won
            } else {
                Outcome::Lost
            });
            return;
        }
        if let Some(k) = m.mission {
            let ms = &arcade.missions[k];
            let p = &m.world.players[m.me];
            let reached = match ms.goal.as_str() {
                "last_hits" => p.last_hits >= ms.target,
                "tower" => m.towers_taken >= ms.target,
                "kills" => p.kills >= ms.target,
                "boss" => m.bosses >= ms.target,
                _ => false,
            };
            let clock = m.world.tick.saturating_sub(m.world.horn_at) as i32 / TPS;
            if reached {
                m.over = Some(Outcome::Won);
            } else if ms.minutes > 0 && clock >= ms.minutes * 60 {
                m.over = Some(Outcome::Lost);
            }
        }
    }

    fn pan(&mut self, dt: i64) {
        if self.screen != "match" || self.stage.is_some() {
            return;
        }
        let step = (self.look.pan_speed as i64 * dt / 1_000_000) as i32;
        let (mut dx, mut dy) = (0i32, 0i32);
        for k in &self.held {
            match self.keys.bindings.get(k).map(|s| s.as_str()) {
                Some("pan up") => dy += 1,
                Some("pan down") => dy -= 1,
                Some("pan left") => dx -= 1,
                Some("pan right") => dx += 1,
                _ => {}
            }
        }
        if self.pointer_in {
            let e = self.look.edge;
            let (px, py) = (
                self.pointer.0 * 100 / self.width,
                self.pointer.1 * 100 / self.height,
            );
            if px < e {
                dx -= 1;
            } else if px >= 100 - e {
                dx += 1;
            }
            if py < e {
                dy += 1;
            } else if py >= 100 - e {
                dy -= 1;
            }
        }
        let size = self.rules.map.size;
        self.view.target = [
            (self.view.target[0] + dx.signum() * step).clamp(0, size),
            (self.view.target[1] + dy.signum() * step).clamp(0, size),
        ];
    }

    pub fn center_on_hero(&mut self) {
        let h = self.my_hero();
        if let Some(u) = self.m.as_ref().and_then(|m| m.world.get(h)) {
            self.view.target = u.pos();
        }
    }

    fn pct(&self, x: i32, y: i32) -> (i32, i32) {
        (
            (x as i64 * 100 * ONE as i64 / self.width.max(1) as i64) as i32,
            (y as i64 * 100 * ONE as i64 / self.height.max(1) as i64) as i32,
        )
    }

    fn with_frame<T>(&self, f: impl FnOnce(&Frame) -> T) -> Option<T> {
        let ctx = self.ctx();
        if self.on_title() {
            // the backdrop match, seen by neither side, so nothing is fogged
            let b = self.backdrop.as_ref().filter(|b| b.ready(&self.look.backdrop))?;
            let frame = Frame {
                ctx: &ctx,
                w: &b.world,
                models: &self.models,
                look: &self.look,
                team: crate::world::NEUTRAL,
                alpha: b.alpha,
                prev: &b.prev,
                hover: NO_UNIT,
                selected: NO_UNIT,
                view: self.view,
            };
            return Some(f(&frame));
        }
        let m = self.m.as_ref()?;
        let frame = Frame {
            ctx: &ctx,
            w: &m.world,
            models: &self.models,
            look: &self.look,
            team: m.world.players[m.me].team,
            alpha: self.alpha,
            prev: &self.prev,
            hover: self.hover,
            selected: self.selected,
            view: self.view,
        };
        Some(f(&frame))
    }

    /// The ground point under the pointer.
    pub fn ground_point(&self, x: i32, y: i32) -> P {
        let (sx, sy) = self.pct(x, y);
        let g = scene::view_ground(&self.terrain, &self.view);
        self.view.pick(g, self.aspect(), sx, sy)
    }

    pub fn unit_at(&self, x: i32, y: i32) -> Uid {
        let (sx, sy) = self.pct(x, y);
        let aspect = self.aspect();
        self.with_frame(|f| scene::unit_under(f, aspect, sx, sy))
            .unwrap_or(NO_UNIT)
    }

    /// The pointer moved to (x, y) in the window; `over_view` when it is on
    /// the 3D view rather than the HUD, the minimap or a panel. The edge of
    /// the window pans the camera whatever is under the pointer, so the
    /// bottom and left edges, under the HUD and the minimap, pan too (T4).
    pub fn pointer_move(&mut self, x: i32, y: i32, over_view: bool) {
        self.pointer = (x, y);
        self.pointer_in = true;
        self.pointer_view = over_view;
        if self.screen == "match" {
            self.hover = if over_view { self.unit_at(x, y) } else { NO_UNIT };
        }
    }

    /// The locked pointer moved by (dx, dy): the drawn cursor moves as far,
    /// and stops at the window's edges. Returns where it now is; the page
    /// then reports what lies there with `pointer_move`.
    pub fn nudge(&mut self, dx: i32, dy: i32) -> [i32; 2] {
        let x = (self.pointer.0 + dx).clamp(0, self.width - 1);
        let y = (self.pointer.1 + dy).clamp(0, self.height - 1);
        self.pointer = (x, y);
        [x, y]
    }

    /// The pointer left the window.
    pub fn pointer_out(&mut self) {
        self.pointer_in = false;
        self.pointer_view = false;
        self.hover = NO_UNIT;
    }

    pub fn command(&mut self, c: Cmd) {
        if let Some(m) = self.m.as_mut() {
            if m.over.is_none() {
                m.pending.push(c);
            }
        }
    }

    /// A mouse button on the 3D view: 0 is the main button, 2 the second.
    pub fn click(&mut self, button: i32, x: i32, y: i32) {
        if self.screen != "match" || self.m.is_none() {
            return;
        }
        let at = self.ground_point(x, y);
        let under = self.unit_at(x, y);
        if button == 2 {
            if self.aim.is_some() {
                self.aim = None;
                return;
            }
            self.right_click(under, at);
            return;
        }
        match self.aim.take() {
            Some(Aim::AttackMove) => {
                if under != NO_UNIT && self.is_enemy(under) {
                    self.command(Cmd::Attack(under));
                    self.mark_order(at, under, true);
                } else {
                    self.command(Cmd::AttackMove(at));
                    self.mark_order(at, NO_UNIT, true);
                }
            }
            Some(Aim::Ability(s)) => self.command(Cmd::Cast(s, under, at)),
            Some(Aim::Item(s)) => self.command(Cmd::Item(s, under, at)),
            Some(Aim::Cannon) => self.command(Cmd::Cannon(at)),
            None => self.selected = under,
        }
    }

    fn is_enemy(&self, id: Uid) -> bool {
        let team = self.me_team();
        self.m
            .as_ref()
            .and_then(|m| m.world.get(id))
            .map(|u| u.team != team)
            .unwrap_or(false)
    }

    fn right_click(&mut self, under: Uid, at: P) {
        let team = self.me_team();
        let Some(m) = self.m.as_ref() else { return };
        match m.world.get(under) {
            Some(u) if u.team != team && !matches!(self.rules.units[u.def as usize].kind, Kind::Shop) => {
                self.command(Cmd::Attack(under));
                self.mark_order(at, under, true);
            }
            Some(u) if u.team == team && self.rules.units[u.def as usize].kind == Kind::Creep => {
                // a right-click on an allied creep tries a deny; the rule decides if it may
                self.command(Cmd::Attack(under));
                self.mark_order(at, under, true);
            }
            _ => {
                self.command(Cmd::Move(at));
                self.mark_order(at, NO_UNIT, false);
            }
        }
    }

    /// While my hero is in the air from the cannon, the camera keeps it in view.
    fn follow_throw(&mut self) {
        let hero = self.my_hero();
        let Some(u) = self.m.as_ref().and_then(|m| m.world.get(hero)) else {
            return;
        };
        if let world::Act::Flight { up, .. } = u.act {
            if self.m.as_ref().map(|m| m.world.tick >= up).unwrap_or(false) {
                self.view.target = u.pos();
            }
        }
    }

    /// Remembers an order click, so the scene marks it for a moment (T10).
    fn mark_order(&mut self, at: P, unit: Uid, attack: bool) {
        self.ping = Some((at, unit, attack, self.clock_us));
    }

    /// What the local player's own hand shows this frame (T9, T10).
    pub fn overlay(&self) -> crate::overlay::Overlay {
        use crate::overlay::{ability_area, Aiming, Area, Overlay, Ping};
        let mut o = Overlay::default();
        let life = self.look.ping as i64 * 10_000;
        if let Some((at, unit, attack, when)) = self.ping {
            let gone = self.clock_us - when;
            if gone < life {
                o.ping = Some(Ping {
                    at,
                    unit,
                    attack,
                    age: (gone * ONE as i64 / life.max(1)) as i32,
                });
            }
        }
        let Some(aim) = self.aim else { return o };
        let hero = self.my_hero();
        let Some(m) = self.m.as_ref() else { return o };
        let Some(u) = m.world.get(hero) else { return o };
        let at = self.ground_point(self.pointer.0, self.pointer.1);
        let team = u.team;
        let fits = |on: &str, t: Uid| -> Uid {
            let Some(v) = m.world.live(t) else { return NO_UNIT };
            let ok = match on {
                "ally" => v.team == team,
                "self" => v.id == hero,
                "any" => true,
                _ => v.team != team,
            };
            if ok && !self.rules.units[v.def as usize].kind.is_building() {
                t
            } else {
                NO_UNIT
            }
        };
        let (range, area, color, on) = match aim {
            Aim::Ability(s) => {
                let Some(&k) = self.rules.unit_abilities[u.def as usize].get(s as usize) else {
                    return o;
                };
                let a = &self.rules.abilities[k];
                let level = u.abil.get(s as usize).map(|x| x.level).unwrap_or(1).max(1);
                let col = self
                    .look
                    .fx
                    .get(&a.id)
                    .map(|l| l.color.as_str())
                    .unwrap_or("select");
                (
                    a.range_at(level),
                    ability_area(a, level),
                    self.models.named(col),
                    a.on.clone(),
                )
            }
            Aim::Item(s) => {
                let Some(it) = u.items.get(s as usize).filter(|x| x.item > 0) else {
                    return o;
                };
                let def = &self.rules.items[it.item as usize - 1];
                let area = match (def.logic.as_str(), def.target) {
                    ("tp", _) => Area::Teleport,
                    (_, Some(Target::Unit)) => Area::Unit,
                    _ => Area::Point,
                };
                let col = if def.logic == "tp" {
                    self.models.team_named(team, "team_glow")
                } else {
                    self.models.named("fx_arcane")
                };
                let on = if matches!(def.logic.as_str(), "salve" | "clarity") {
                    "ally"
                } else {
                    "enemy"
                };
                (def.range, area, col, on.to_string())
            }
            Aim::AttackMove => (
                0,
                Area::Point,
                self.models.named("bar_enemy"),
                "enemy".to_string(),
            ),
            Aim::Cannon => (
                0,
                Area::Cannon,
                self.models.team_named(team, "team_glow"),
                "none".to_string(),
            ),
        };
        o.aim = Some(Aiming {
            hero,
            range,
            at,
            under: fits(&on, self.hover),
            area,
            color,
        });
        o
    }

    /// A click on the minimap, in its own pixels.
    pub fn minimap_click(&mut self, button: i32, x: i32, y: i32) {
        let size = self.rules.map.size;
        let p = [
            x * size / crate::draw::WIDTH,
            (crate::draw::HEIGHT - y) * size / crate::draw::HEIGHT,
        ];
        if button == 2 {
            self.command(Cmd::Move(p));
            self.mark_order(p, NO_UNIT, false);
        } else {
            match self.aim.take() {
                Some(Aim::AttackMove) => self.command(Cmd::AttackMove(p)),
                Some(Aim::Cannon) => self.command(Cmd::Cannon(p)),
                other => {
                    self.aim = other;
                    self.view.target = p;
                }
            }
        }
    }

    pub fn wheel(&mut self, dy: i32) {
        let step = self.look.zoom_step * ONE / 100;
        let d = self.view.distance + dy.signum() * step;
        self.view.distance = d.clamp(self.look.zoom_min * ONE / 100, self.look.zoom_max * ONE / 100);
    }

    /// A key went down or up, by its code (`KeyQ`).
    pub fn key(&mut self, code: &str, down: bool) {
        if !down {
            self.held.retain(|k| k != code);
            if self
                .keys
                .bindings
                .get(code)
                .map(|s| s == "score")
                .unwrap_or(false)
            {
                self.score = false;
            }
            return;
        }
        if !self.held.iter().any(|k| k == code) {
            self.held.push(code.to_string());
        }
        if self.screen != "match" {
            return;
        }
        if self.learn {
            if let Some(&slot) = self.keys.learn_keys.get(code) {
                self.command(Cmd::Learn(slot));
                self.learn = false;
                return;
            }
        }
        let Some(action) = self.keys.bindings.get(code).cloned() else {
            return;
        };
        let mut parts = action.split(' ');
        let verb = parts.next().unwrap_or("");
        let arg: i32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        match verb {
            "ability" => self.use_slot(Aim::Ability(arg as u8)),
            "item" => self.use_slot(Aim::Item(arg as u8)),
            "attack" => self.aim = Some(Aim::AttackMove),
            "cannon" => {
                let _ = self.aim_cannon();
            }
            "stop" => self.command(Cmd::Stop),
            "hold" => self.command(Cmd::Hold),
            "learn" => self.learn = !self.learn,
            "shop" => self.shop = !self.shop,
            "score" => self.score = true,
            "center" => self.center_on_hero(),
            "cancel" => {
                self.aim = None;
                self.learn = false;
                self.shop = false;
            }
            "menu" => self.menu = !self.menu,
            "pause" => {
                if let Some(m) = self.m.as_mut() {
                    if !m.online {
                        m.paused = !m.paused;
                    }
                }
            }
            _ => {}
        }
    }

    /// The cannon key or button: aims the base cannon when it may fire.
    pub fn aim_cannon(&mut self) -> Result<(), String> {
        let hero = self.my_hero();
        let ok = self
            .m
            .as_ref()
            .map(|m| crate::aram::may_throw(&m.world, &self.ctx(), hero))
            .unwrap_or(false);
        if !ok {
            return Err(self.strings.ui("cannon.not_ready"));
        }
        self.aim = Some(Aim::Cannon);
        Ok(())
    }

    /// An ability or item key: a target-less one goes off at once, a
    /// targeted one waits for a click.
    pub fn use_slot(&mut self, aim: Aim) {
        let hero = self.my_hero();
        let Some(m) = self.m.as_ref() else { return };
        let Some(u) = m.world.get(hero) else { return };
        let target = match aim {
            Aim::Ability(s) => self.rules.unit_abilities[u.def as usize]
                .get(s as usize)
                .map(|&k| self.rules.abilities[k].target),
            Aim::Item(s) => u
                .items
                .get(s as usize)
                .filter(|it| it.item > 0)
                .and_then(|it| self.rules.items[it.item as usize - 1].target),
            Aim::AttackMove | Aim::Cannon => None,
        };
        match (aim, target) {
            (_, None) | (_, Some(Target::Passive)) => {}
            (Aim::Ability(s), Some(Target::None | Target::Toggle)) => {
                self.command(Cmd::Cast(s, NO_UNIT, [0, 0]))
            }
            (Aim::Item(s), Some(Target::None)) => self.command(Cmd::Item(s, NO_UNIT, [0, 0])),
            _ => self.aim = Some(aim),
        }
    }

    /// The scene's instances for this frame: (values, runs).
    pub fn scene(&self) -> render::Finished {
        let mut b = Batch::new();
        if let Some(stage) = &self.stage {
            return self.stage_scene(stage);
        }
        let overlay = self.overlay();
        self.with_frame(|f| {
            scene::ground(f, &mut b);
            scene::units(f, &mut b);
            scene::effects(f, &mut b);
            crate::overlay::teleports(f, &mut b);
            crate::overlay::aram(f, &mut b);
            crate::overlay::draw(f, &mut b, &overlay);
        });
        b.finish()
    }

    /// Builds this frame's scene once; the page then reads its values and draws.
    pub fn render(&mut self) {
        self.last = self.scene();
    }

    pub fn camera(&self) -> render::Camera {
        let g = scene::view_ground(&self.terrain, &self.view);
        self.view.camera(g)
    }

    pub fn mats(&self) -> Vec<i32> {
        self.camera().mat_values(self.aspect()).to_vec()
    }

    pub fn uniforms(&self) -> Vec<i32> {
        let l = &self.look;
        let c = |v: [i32; 3]| [v[0] * ONE / 255, v[1] * ONE / 255, v[2] * ONE / 255];
        let dir = crate::geom::v3(
            l.light_dir[0] * ONE / 100,
            l.light_dir[1] * ONE / 100,
            l.light_dir[2] * ONE / 100,
        )
        .norm();
        let mut out = vec![dir.x, dir.y, dir.z];
        out.extend_from_slice(&c(l.sky));
        out.extend_from_slice(&c(l.fog));
        let far = self.stage_view().distance;
        out.extend_from_slice(&[
            fx::mul(far, l.fog_range[0] * ONE / 4600),
            fx::mul(far, l.fog_range[1] * ONE / 4600),
        ]);
        out.extend_from_slice(&c(l.light));
        out.extend_from_slice(&c(l.ink));
        out.extend_from_slice(&[
            l.ink_size[0] * ONE / 100,
            l.ink_size[1] * ONE / 100,
            l.ink_size[2] * ONE / 100,
        ]);
        out.extend_from_slice(&[
            l.bands[0] * ONE / 100,
            l.bands[1] * ONE / 100,
            l.bands[2] * ONE / 100,
            l.bands[3] * ONE / 100,
        ]);
        out.extend_from_slice(&[
            l.ambient[0] * ONE / 100,
            l.ambient[1] * ONE / 100,
            l.ambient[2] * ONE / 100,
        ]);
        out.extend_from_slice(&[
            l.ground[0] * ONE / 100,
            l.ground[1] * ONE / 100,
            l.ground[2] * ONE / 100,
        ]);
        // uQ: one Q16 step; uEps: a length too small to divide by
        out.push(1);
        out.push(ONE / 10_000);
        // uPasses: solid, ink, tint, and a half for comparing pass numbers
        use crate::render::{PASS_INK, PASS_SOLID, PASS_TINT};
        out.extend_from_slice(&[
            fx::int(PASS_SOLID as i32),
            fx::int(PASS_INK as i32),
            fx::int(PASS_TINT as i32),
            ONE / 2,
        ]);
        // uGlow: the glow style's number and a half
        out.extend_from_slice(&[fx::int(crate::scene::STYLE_GLOW), ONE / 2]);
        out
    }

    pub fn clear_color(&self) -> Vec<i32> {
        let c = self.look.sky;
        vec![c[0] * ONE / 255, c[1] * ONE / 255, c[2] * ONE / 255]
    }

    /// Sets up a still scene from a tile spec: `lineup:a,b,c`, `overview`,
    /// `match:<seconds>` (bots play that long), `map:<x>,<y>`.
    pub fn set_stage(&mut self, spec: &str) -> Result<(), String> {
        let (kind, arg) = spec.split_once(':').unwrap_or((spec, ""));
        match kind {
            "lineup" => {
                // `lineup:12:a,b` sets a gap of 12 scene units; `lineup:a,b` uses 7
                let (gap, list) = match arg.split_once(':') {
                    Some((g, rest)) if g.parse::<i32>().is_ok() => (g.parse::<i32>().unwrap_or(7), rest),
                    _ => (7, arg),
                };
                let names: Vec<String> = list.split(',').map(|s| s.to_string()).collect();
                for n in &names {
                    if self.models.find(n).is_none() {
                        return Err(format!("There is no model \"{n}\"."));
                    }
                }
                self.stage = Some(Stage::Lineup(names, gap));
            }
            "overview" => {
                self.stage = Some(Stage::Overview);
            }
            "match" | "map" => {
                let nums: Vec<i32> = arg.split(',').filter_map(|s| s.parse().ok()).collect();
                let mut w = self.new_world(self.seed);
                for p in 0..PLAYERS {
                    w.players[p].bot = Some(2);
                    w.players[p].present = true;
                }
                let secs = if kind == "match" {
                    nums.first().copied().unwrap_or(120)
                } else {
                    0
                };
                let ctx = self.ctx();
                for _ in 0..secs * TPS {
                    w.step(&ctx, &[]);
                }
                let me = nums.get(1).copied().unwrap_or(0).clamp(0, 9) as usize;
                let hero = w.players[me].hero;
                self.begin(w, me, None, false);
                if kind == "map" {
                    self.view.target = [
                        nums.first().copied().unwrap_or(4000),
                        nums.get(1).copied().unwrap_or(4000),
                    ];
                } else if let Some(u) = self.m.as_ref().and_then(|m| m.world.get(hero)) {
                    self.view.target = u.pos();
                }
                if let Some(m) = self.m.as_mut() {
                    m.paused = true;
                }
                self.stage = None;
            }
            "cast" => {
                // cast:<ability id>,<ticks>: its hero, at full levels and mana,
                // casts it at three enemy creeps beside an enemy hero, with an
                // ally at its side; the picture is `ticks` after the order
                let (id, t) = arg.split_once(',').unwrap_or((arg, "30"));
                let ticks: u32 = t.parse().map_err(|_| format!("\"{t}\" is not a number."))?;
                let w = self.cast_world(id, ticks)?;
                let hero = w.players[0].hero;
                self.begin(w, 0, None, false);
                if let Some(u) = self.m.as_ref().and_then(|m| m.world.get(hero)) {
                    self.view.target = [u.pos()[0] + 250, u.pos()[1]];
                }
                if let Some(m) = self.m.as_mut() {
                    m.paused = true;
                }
                self.stage = None;
            }
            "volley" => {
                // volley:<ticks>: every kind of shooter fires across open
                // ground, so the shots can be seen side by side (T6)
                let ticks: u32 = arg.parse().map_err(|_| format!("\"{arg}\" is not a number."))?;
                let w = self.volley_world(ticks)?;
                let hero = w.players[0].hero;
                self.begin(w, 0, None, false);
                if let Some(u) = self.m.as_ref().and_then(|m| m.world.get(hero)) {
                    self.view.target = [u.pos()[0] + 350, u.pos()[1] + 200];
                }
                if let Some(m) = self.m.as_mut() {
                    m.paused = true;
                }
                self.stage = None;
            }
            _ => return Err(format!("There is no stage \"{kind}\".")),
        }
        Ok(())
    }

    /// The `volley:` stage's world: the ranged heroes, a ranged and a siege
    /// creep of each side and a tower each attack a row of enemy creeps,
    /// `ticks` after the order.
    pub fn volley_world(&self, ticks: u32) -> Result<World, String> {
        let ctx = self.ctx();
        let mut w = self.new_world(self.seed);
        let spot = [self.rules.map.size / 2 - 1400, self.rules.map.size / 2 - 1200];
        let place = |w: &mut World, id: Uid, at: P| {
            let u = w.get_mut(id).expect("just added");
            u.x = at[0] * SUB;
            u.y = at[1] * SUB;
            u.path.clear();
        };
        let unit = |name: &str| self.rules.unit(name).ok_or(format!("no unit \"{name}\""));
        // a column of targets for each side
        let mut targets = [Vec::new(), Vec::new()];
        for i in 0..6 {
            for side in 0..2u8 {
                let def = unit(if side == 0 {
                    "creep_melee_moon"
                } else {
                    "creep_melee_sun"
                })?;
                let x = spot[0] + 600 + side as i32 * 200;
                let id = w.spawn(&ctx, def, 1 - side, [x, spot[1] + i * 150]);
                w.get_mut(id).expect("spawned").order = world::Order::Stand;
                targets[side as usize].push(id);
            }
        }
        let ranged: Vec<usize> = (0..self.rules.heroes.len())
            .filter(|&r| self.rules.units[self.rules.heroes[r]].missile > 0)
            .collect();
        let mut shooters = Vec::new();
        for (i, &r) in ranged.iter().enumerate().take(5) {
            w.players[i].present = true;
            let h = w.add_hero(&ctx, i, r);
            place(&mut w, h, [spot[0], spot[1] + i as i32 * 170]);
            shooters.push((h, 0usize));
        }
        if let Some(&r) = ranged.get(5) {
            w.players[5].present = true;
            let h = w.add_hero(&ctx, 5, r);
            place(&mut w, h, [spot[0] + 1400, spot[1] + 400]);
            shooters.push((h, 1));
        }
        let near = [
            [spot[0] + 250, spot[1] - 60],
            [spot[0] + 250, spot[1] - 240],
            [spot[0] + 200, spot[1] + 880],
        ];
        for (k, name) in ["creep_ranged_sun", "creep_siege_sun", "tower1_sun"]
            .iter()
            .enumerate()
        {
            let id = w.spawn(&ctx, unit(name)?, 0, near[k]);
            shooters.push((id, 0));
        }
        for (k, name) in ["creep_ranged_moon", "creep_siege_moon"].iter().enumerate() {
            let id = w.spawn(
                &ctx,
                unit(name)?,
                1,
                [spot[0] + 1450, spot[1] + 100 + k as i32 * 500],
            );
            shooters.push((id, 1));
        }
        for (n, &(id, side)) in shooters.iter().enumerate() {
            let list = &targets[side];
            let t = list[n % list.len()];
            let u = w.get_mut(id).expect("added");
            u.order = world::Order::Attack(t);
            u.attack_ready = 0;
        }
        for _ in 0..ticks {
            w.step(&ctx, &[]);
        }
        Ok(w)
    }

    /// The `cast:` stage's world: ability `id`'s hero casts it `ticks`
    /// before the picture. Tests read it too.
    pub fn cast_world(&self, id: &str, ticks: u32) -> Result<World, String> {
        let k = self
            .rules
            .abilities
            .iter()
            .position(|a| a.id == id)
            .ok_or_else(|| format!("There is no ability \"{id}\"."))?;
        let (roster, slot) = (0..self.rules.heroes.len())
            .find_map(|r| {
                self.rules.unit_abilities[self.rules.heroes[r]]
                    .iter()
                    .position(|&a| a == k)
                    .map(|s| (r, s))
            })
            .ok_or_else(|| format!("No hero casts \"{id}\"."))?;
        let ctx = self.ctx();
        let mut w = self.new_world(self.seed);
        let n = self.rules.heroes.len();
        let spot = [self.rules.map.size / 2 - 900, self.rules.map.size / 2 - 900];
        let put = |w: &mut World, player: usize, r: usize, at: P| {
            w.players[player].present = true;
            let h = w.add_hero(&ctx, player, r);
            let u = w.get_mut(h).expect("just added");
            u.x = at[0] * SUB;
            u.y = at[1] * SUB;
            u.path.clear();
            h
        };
        let me = put(&mut w, 0, roster, spot);
        put(&mut w, 1, (roster + 2) % n, [spot[0] - 120, spot[1] + 260]);
        let foe = put(&mut w, 5, (roster + 1) % n, [spot[0] + 650, spot[1] + 120]);
        let def = self.rules.unit("creep_melee_moon").ok_or("no Moonward creep")?;
        let mut creeps = Vec::new();
        for i in 0..3 {
            creeps.push(w.spawn(&ctx, def, 1, [spot[0] + 420, spot[1] - 160 + i * 160]));
        }
        {
            let ults: Vec<bool> = self.rules.unit_abilities[self.rules.heroes[roster]]
                .iter()
                .map(|&a| self.rules.abilities[a].ultimate)
                .collect();
            let u = w.get_mut(me).expect("added");
            for (s, ult) in ults.iter().enumerate() {
                u.abil[s].level = if *ult { 3 } else { 4 };
            }
            u.facing = 0;
        }
        // level 16, so the ultimate's mana is in the pool, then one tick so
        // the side sees its targets
        crate::combat::give_xp(&mut w, &ctx, me, 100_000);
        w.step(&ctx, &[]);
        let max = crate::stats::of(&ctx, &w, me).max_mana;
        w.get_mut(me).expect("added").mana = max * 1000;
        let t = if self.rules.abilities[k].on == "ally" {
            w.players[1].hero
        } else {
            creeps[1]
        };
        let at = w.get(t).map(|u| u.pos()).unwrap_or(spot);
        let cmd = match self.rules.abilities[k].target {
            crate::rules::Target::Passive => Cmd::Attack(creeps[1]),
            _ => Cmd::Cast(slot as u8, t, at),
        };
        let _ = foe;
        w.step(&ctx, &[(0, cmd)]);
        for _ in 1..ticks {
            w.step(&ctx, &[]);
        }
        Ok(w)
    }

    fn stage_scene(&self, stage: &Stage) -> render::Finished {
        use crate::geom::{v3, Affine};
        use crate::models::{pose_parts, Pose};
        let mut b = Batch::new();
        match stage {
            Stage::Lineup(names, gap) => {
                let gap = ONE * *gap;
                let n = names.len() as i32;
                let floor = Affine::translate(v3(0, -ONE / 10, 0)).then(&Affine::scale(v3(
                    gap * (n + 1),
                    ONE / 5,
                    ONE * 12,
                )));
                b.push(
                    render::LAYER_PLAIN,
                    crate::mesh::Mesh::Cube,
                    &floor,
                    self.models.named("stone_sun"),
                    6,
                );
                for (i, name) in names.iter().enumerate() {
                    let Some(mi) = self.models.find(name) else {
                        continue;
                    };
                    let x = (i as i32 * 2 - (n - 1)) * gap / 2;
                    let scale = self.look.unit_scale * ONE / 100;
                    // turned three-quarters toward the camera
                    let place = Affine::translate(v3(x, 0, 0))
                        .then(&Affine::rot_y(fx::deg(-60)))
                        .then(&Affine::scale(v3(scale, scale, scale)));
                    let team = if name.ends_with("_moon") { 1 } else { 0 };
                    pose_parts(&self.models.list[mi], &Pose::STILL, &place, |p, a| {
                        let layer = if p.plain {
                            render::LAYER_PLAIN
                        } else {
                            render::LAYER_INKED
                        };
                        b.push(layer, p.mesh, &a, self.models.color(team, p.color), p.style);
                    });
                }
            }
            Stage::Overview => {
                let w = self.new_world(self.seed);
                let ctx = self.ctx();
                let look = Look {
                    reach_x: self.rules.map.size,
                    reach_south: self.rules.map.size,
                    reach_north: self.rules.map.size,
                    ..self.look.clone()
                };
                let f = Frame {
                    ctx: &ctx,
                    w: &w,
                    models: &self.models,
                    look: &look,
                    team: NEUTRAL,
                    alpha: 0,
                    prev: &[],
                    hover: NO_UNIT,
                    selected: NO_UNIT,
                    view: self.stage_view(),
                };
                scene::ground(&f, &mut b);
                scene::units(&f, &mut b);
            }
        }
        b.finish()
    }

    /// The camera a still scene uses.
    pub fn stage_view(&self) -> View {
        match &self.stage {
            Some(Stage::Lineup(names, gap)) => View {
                target: [0, 0],
                distance: ONE * (12 + gap * 5 / 7 * names.len() as i32),
                pitch: fx::deg(16),
                fov: fx::deg(40),
                yaw: 0,
            },
            Some(Stage::Overview) => View {
                target: [self.rules.map.size / 2, self.rules.map.size / 2],
                distance: ONE * 620,
                pitch: fx::deg(89),
                fov: fx::deg(50),
                yaw: 0,
            },
            None => self.view,
        }
    }

    /// The model lineup's camera looks at the models' middle height.
    pub fn stage_camera(&self) -> render::Camera {
        let v = self.stage_view();
        let lift = match &self.stage {
            Some(Stage::Lineup(_, gap)) => ONE * 3 * *gap / 7,
            _ => 0,
        };
        v.camera(lift)
    }

    pub fn stage_mats(&self) -> Vec<i32> {
        self.stage_camera().mat_values(self.aspect()).to_vec()
    }

    /// Positions to the page's pixels: whether the hero is under the pointer, for the cursor.
    pub fn cursor(&self) -> &'static str {
        match self.aim {
            Some(_) => "aim",
            None if self.hover != NO_UNIT && self.is_enemy(self.hover) => "attack",
            None => "",
        }
    }

    /// Map units of the camera's corners on the ground, for the minimap frame.
    pub fn view_corners(&self) -> [P; 4] {
        let g = scene::view_ground(&self.terrain, &self.view);
        let a = self.aspect();
        let pick = |x: i32, y: i32| self.view.pick(g, a, x * ONE, y * ONE);
        [pick(0, 0), pick(100, 0), pick(100, 100), pick(0, 100)]
    }

    pub fn tick_alpha(&self) -> i32 {
        self.alpha
    }

    pub fn sub_to_map(x: i32) -> i32 {
        x / SUB
    }
}

impl Game {
    /// Every button and setting comes back through here: `name` says what,
    /// `value` which. An unknown action is an error the page shows.
    pub fn act(&mut self, name: &str, value: &str) -> Result<(), String> {
        let num = || {
            value
                .parse::<i32>()
                .map_err(|_| format!("\"{value}\" is not a number."))
        };
        self.notice.clear();
        match name {
            "motion" => {
                self.reduced_motion = value == "reduce";
            }
            "screen" => {
                match value {
                    "title" | "arcade" | "heroes" | "help" | "pick" => {}
                    v if v.starts_with("online") => crate::online::enter(self, v),
                    other => return Err(format!("There is no screen \"{other}\".")),
                }
                if value == "title" || value == "arcade" {
                    if let Some(m) = self.m.as_ref() {
                        if m.online {
                            crate::online::leave(self);
                        }
                    }
                    self.m = None;
                }
                self.screen = value.to_string();
                self.stage = None;
            }
            "mission" => {
                let k = num()? as usize;
                let ms = self
                    .arcade
                    .missions
                    .get(k)
                    .ok_or("There is no such mission.")?
                    .clone();
                self.setup = Setup {
                    allies: ms.allies,
                    enemies: ms.enemies,
                    ally: ms.ally,
                    enemy: ms.enemy,
                    hero: ms.hero,
                    mission: Some(k),
                    aram: false,
                    rerolled: false,
                };
                self.screen = "pick".into();
            }
            "custom" => {
                self.setup.mission = None;
                self.setup.hero = -1;
                self.setup.rerolled = false;
                if self.setup.aram {
                    self.setup.hero = self.arcade_deal()[0] as i32;
                }
                self.screen = "pick".into();
            }
            "setup.mode" => self.setup.aram = !self.setup.aram,
            "reroll" => {
                let spare = self.rules.aram.as_ref().map(|a| a.rerolls > 0).unwrap_or(false);
                if !(self.setup.aram && self.setup.mission.is_none() && spare) {
                    return Err(self.strings.ui("pick.no_reroll"));
                }
                if self.setup.rerolled {
                    return Err(self.strings.ui("pick.rerolled"));
                }
                self.setup.rerolled = true;
                self.setup.hero = self.arcade_deal()[0] as i32;
            }
            "setup.allies" | "setup.enemies" | "setup.ally" | "setup.enemy" => {
                let up = value == "+";
                let levels = self.rules.ai.as_ref().map(|a| a.difficulty.len()).unwrap_or(1) as i32;
                let step = |v: i32, lo: i32, hi: i32| if up { (v + 1).min(hi) } else { (v - 1).max(lo) };
                let c = &self.arcade.custom;
                match name {
                    "setup.allies" => {
                        self.setup.allies =
                            step(self.setup.allies as i32, c.allies[0] as i32, c.allies[1] as i32) as usize
                    }
                    "setup.enemies" => {
                        self.setup.enemies = step(
                            self.setup.enemies as i32,
                            c.enemies[0] as i32,
                            c.enemies[1] as i32,
                        ) as usize
                    }
                    "setup.ally" => self.setup.ally = step(self.setup.ally as i32, 0, levels - 1) as u8,
                    _ => self.setup.enemy = step(self.setup.enemy as i32, 0, levels - 1) as u8,
                }
                self.setup.mission = None;
            }
            "pick" => {
                let k = num()?;
                if k < 0 || k as usize >= self.rules.heroes.len() {
                    return Err("There is no such hero.".into());
                }
                let online_aram = self.screen.starts_with("online") && self.online.aram;
                if (self.setup.aram && self.setup.mission.is_none() && self.screen == "pick") || online_aram {
                    return Err(self.strings.ui("pick.random"));
                }
                self.setup.hero = k;
                if self.screen.starts_with("online") {
                    crate::online::pick(self, k as u8);
                }
            }
            "start" => {
                if self.setup.hero < 0 {
                    return Err(self.strings.ui("pick.hint"));
                }
                self.start_arcade();
            }
            "again" => {
                if self.m.as_ref().map(|m| m.online).unwrap_or(false) {
                    self.screen = "title".into();
                } else {
                    self.start_arcade();
                }
            }
            "resume" => {
                self.menu = false;
                if let Some(m) = self.m.as_mut() {
                    m.paused = false;
                }
            }
            "quit" => {
                self.menu = false;
                let online = self.m.as_ref().map(|m| m.online).unwrap_or(false);
                if online {
                    crate::online::leave(self);
                }
                self.m = None;
                self.screen = if online { "title".into() } else { "arcade".into() };
            }
            "ability" => self.use_slot(Aim::Ability(num()? as u8)),
            "item" => self.use_slot(Aim::Item(num()? as u8)),
            "learn" => {
                self.command(Cmd::Learn(num()? as u8));
                self.learn = false;
            }
            "buy" => self.command(Cmd::Buy(num()? as u16)),
            "sell" => self.command(Cmd::Sell(num()? as u8)),
            "buyback" => self.command(Cmd::Buyback),
            "autopilot" => {
                // the computer plays this player's hero, at normal skill, or hands it back
                let on = self
                    .m
                    .as_ref()
                    .map(|m| m.world.players[m.me].bot.is_some())
                    .unwrap_or(false);
                self.command(Cmd::Autopilot(if on { 255 } else { 2 }));
                self.menu = false;
                if let Some(m) = self.m.as_mut() {
                    m.paused = false;
                }
            }
            "glyph" => self.command(Cmd::Glyph),
            "shop.toggle" => self.shop = !self.shop,
            "shop.tab" => self.shop_tab = value.to_string(),
            "shop.pick" => {
                let k = num()?;
                if k < 0 || k as usize >= self.rules.items.len() {
                    return Err(format!("no item {k}"));
                }
                self.shop_pick = k;
            }
            "score.toggle" => self.score = !self.score,
            "cannon" => self.aim_cannon()?,
            // the page puts the code or link on the clipboard; the core says so
            "copy" | "copy.link" => self.notice = self.strings.ui("online.copied"),
            n if n.starts_with("online.") => crate::online::act(self, n, value)?,
            other => return Err(format!("Unknown action \"{other}\".")),
        }
        Ok(())
    }

    /// The view model as JSON.
    pub fn ui_json(&self) -> String {
        serde_json::to_string(&crate::ui::view(self)).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    /// The glossary's terms and definitions, for the page's hover hints.
    pub fn hints_json(&self) -> String {
        serde_json::to_string(&self.glossary).unwrap_or_default()
    }
}

/// Sizes and layout the page needs to upload and draw: every stride and
/// offset, so `gl.js` holds none of its own.
pub fn numbers() -> String {
    use crate::mesh::VERTEX_VALUES;
    use crate::render::*;
    let b = BYTES_PER_VALUE;
    serde_json::json!({
        "vertex_values": VERTEX_VALUES,
        "vertex": {"stride": VERTEX_VALUES * b, "attributes": [["aPos", 3, 0], ["aNormal", 3, 3 * b]]},
        "instance": {"stride": INSTANCE_VALUES * b, "attributes": [["iA", 4, 0], ["iB", 4, 4 * b], ["iC", 4, 8 * b], ["iD", 4, 12 * b]]},
        "buffers": {"scenery": BUF_SCENERY, "dynamic": BUF_DYNAMIC},
        "passes": {"solid": PASS_SOLID, "ink": PASS_INK, "tint": PASS_TINT},
        "draw_values": DRAW_VALUES,
        "mat_values": MAT_VALUES,
        "view_values": 16,
        "mesh_count": MESH_COUNT,
        "uniforms": [["uLightDir", 3], ["uSky", 3], ["uFog", 3], ["uFogRange", 2], ["uLight", 3], ["uInk", 3], ["uInkSize", 3], ["uBands", 4], ["uAmbient", 3], ["uGround", 3], ["uQ", 1], ["uEps", 1], ["uPasses", 4], ["uGlow", 2]],
        "ui_every_ms": 100,
        "minimap": crate::draw::assets(),
    })
    .to_string()
}

impl Game {
    /// The minimap's ground: one square for each block of the map, drawn
    /// again only when trees change.
    pub fn minimap_ground(&self) -> Vec<i32> {
        use crate::draw::{Color, DrawList, HEIGHT, WIDTH};
        let mut d = DrawList::new();
        d.clear(Color::Grass);
        let t = &self.terrain;
        let trees = self.m.as_ref().map(|m| &m.world.trees).unwrap_or(&self.trees0);
        let cells = 64;
        let px = WIDTH / cells;
        let per = t.tw / cells;
        for cy in 0..cells {
            for cx in 0..cells {
                // the block's look: the most common of its tree cells
                let mut count = [0i32; 5];
                for y in 0..per {
                    for x in 0..per {
                        let i = ((cy * per + y) * t.tw + cx * per + x) as usize;
                        let k = if matches!(trees[i], Tree::Standing) {
                            1
                        } else {
                            match t.mark[i] {
                                crate::map::T_RIVER => 2,
                                crate::map::T_LANE => 3,
                                crate::map::T_BASE | crate::map::T_RAMP => 4,
                                _ => 0,
                            }
                        };
                        count[k] += 1;
                    }
                }
                let best = (0..5).max_by_key(|&k| (count[k], k as i32)).unwrap_or(0);
                let color = [
                    Color::Grass,
                    Color::Forest,
                    Color::River,
                    Color::Lane,
                    Color::Base,
                ][best];
                if color != Color::Grass {
                    d.rect(cx * px, HEIGHT - (cy + 1) * px, px, px, color);
                }
            }
        }
        d.finish()
    }

    /// The minimap's moving layer: units the local side sees, the camera's frame.
    pub fn minimap_units(&self) -> Vec<i32> {
        use crate::draw::{Color, DrawList, Scale, Shape, HEIGHT, WIDTH};
        let mut d = DrawList::new();
        d.clear(Color::None);
        let Some(m) = self.m.as_ref() else {
            return d.finish();
        };
        let w = &m.world;
        let ctx = self.ctx();
        let size = self.rules.map.size;
        let to = |p: P| (p[0] * WIDTH / size, HEIGHT - p[1] * HEIGHT / size);
        let team = w.players[m.me].team;
        // fog: dim what the side cannot see, in coarse blocks
        let t = &self.terrain;
        let cells = 32;
        let per = t.tw / cells;
        let px = WIDTH / cells;
        for cy in 0..cells {
            for cx in 0..cells {
                let i = ((cy * per + per / 2) * t.tw + cx * per + per / 2) as usize;
                if w.vision[i] & (1 << team) == 0 {
                    d.rect(cx * px, HEIGHT - (cy + 1) * px, px, px, Color::Fog);
                }
            }
        }
        if let Some((spot, _)) = w.rune {
            let (x, y) = to(self.rules.map.runes[spot]);
            d.shape(Shape::Diamond, x, y, Scale::Small, Color::Rune, Color::Frame, 1);
        }
        // All Random All Mid: the relics waiting, and while the cannon is
        // aimed, the band it can reach (a click on the minimap lands there)
        if w.aram {
            for (i, spot) in crate::aram::relic_spots(&ctx).iter().enumerate() {
                if w.relics.get(i) == Some(&0) {
                    let (x, y) = to(*spot);
                    d.shape(Shape::Square, x, y, Scale::Small, Color::Rune, Color::Frame, 1);
                }
            }
            if self.aim == Some(Aim::Cannon) {
                if let Some((a, b, half)) = crate::aram::band(&ctx, team) {
                    let width = (half * 2 * WIDTH / size).max(2);
                    d.line(&[to(a), to(b)], Color::Me, width);
                }
            }
        }
        for u in &w.units {
            if !u.alive {
                continue;
            }
            let def = &self.rules.units[u.def as usize];
            if matches!(def.kind, Kind::Shop) {
                continue;
            }
            let building = def.kind.is_building();
            if u.team != team && !building && !w.sees(&ctx, team, u) {
                continue;
            }
            let (x, y) = to(u.pos());
            let color = if u.owner == m.me as i8 && u.hero.is_some() {
                Color::Me
            } else if u.team == 0 {
                Color::Sun
            } else if u.team == 1 {
                Color::Moon
            } else {
                Color::Neutral
            };
            if u.hero.is_some() {
                d.shape(Shape::Diamond, x, y, Scale::Tile, color, Color::Frame, 1);
            } else if building {
                d.shape(Shape::Square, x, y, Scale::Small, color, Color::Frame, 1);
            } else {
                d.rect(x - 1, y - 1, 3, 3, color);
            }
        }
        let c = self.view_corners();
        let pts: Vec<(i32, i32)> = c.iter().chain(c.first()).map(|p| to(*p)).collect();
        d.line(&pts, Color::Frame, 1);
        d.finish()
    }

    /// Bumps when the minimap's ground must be drawn again.
    pub fn ground_version(&self) -> u32 {
        self.m.as_ref().map(|m| m.world.tree_version + 1).unwrap_or(0)
    }
}

/// An action's result as the page reads it.
pub fn answer(r: Result<(), String>) -> String {
    match r {
        Ok(()) => "{\"ok\":true}".into(),
        Err(e) => serde_json::json!({ "error": e }).to_string(),
    }
}

pub fn values(f: render::Finished) -> Vec<i32> {
    f.0
}

/// The draws for a finished scene (everything is in the dynamic buffer).
pub fn draw_list(f: &render::Finished) -> Vec<u32> {
    render::draws(&[], &f.1)
}

pub fn mesh(id: u32) -> Vec<i32> {
    crate::mesh::Mesh::ALL
        .get(id as usize)
        .map(|m| crate::mesh::vertices(*m))
        .unwrap_or_default()
}

impl Game {
    /// The camera's matrices: the still scene's when one is set.
    pub fn view_mats(&self) -> Vec<i32> {
        if self.stage.is_some() {
            self.stage_mats()
        } else {
            self.mats()
        }
    }

    pub fn net_out(&mut self) -> String {
        crate::online::take_out(self)
    }

    pub fn net_in(&mut self, peer: &str, text: &str) {
        crate::online::receive(self, peer, text)
    }

    pub fn net_event(&mut self, kind: &str, peer: &str) {
        crate::online::event(self, kind, peer)
    }

    /// A summary for the browser check and the online runners: screen, tick,
    /// checksum, the local seat and its hero's state.
    pub fn probe(&self) -> String {
        let m = self.m.as_ref();
        let hero = self.my_hero();
        let u = m.and_then(|m| m.world.get(hero));
        serde_json::json!({
            "screen": self.screen,
            "tick": m.map(|m| m.world.tick).unwrap_or(0),
            "sum": m.map(|m| m.world.checksum().to_string()).unwrap_or_default(),
            "me": m.map(|m| m.me).unwrap_or(0),
            "alive": u.map(|u| u.alive).unwrap_or(false),
            "level": u.map(|u| u.level()).unwrap_or(0),
            "hp": u.map(|u| u.hp / 1000).unwrap_or(0),
            "x": u.map(|u| u.pos()[0]).unwrap_or(0),
            "y": u.map(|u| u.pos()[1]).unwrap_or(0),
            "gold": m.map(|m| m.world.players[m.me].gold).unwrap_or(0),
            "winner": m.and_then(|m| m.world.winner),
            "over": m.map(|m| m.over.is_some()).unwrap_or(false),
            // All Random All Mid, for the browser check (round 4)
            "aram": m.map(|m| m.world.aram).unwrap_or(false),
            "flying": u.map(|u| matches!(u.act, world::Act::Flight { .. })).unwrap_or(false),
            "heroes": m.map(|m| m.world.players.iter().filter(|p| p.hero != NO_UNIT).count()).unwrap_or(0),
            "side_towers": m
                .map(|m| {
                    m.world
                        .units
                        .iter()
                        .filter(|v| {
                            v.alive && self.rules.units[v.def as usize].kind == Kind::Tower && matches!(v.lane_name, 0 | 2)
                        })
                        .count()
                })
                .unwrap_or(0),
            // where the computer players' heroes stand, for the checks
            "bots": m
                .map(|m| {
                    m.world
                        .players
                        .iter()
                        .filter(|p| p.bot.is_some())
                        .filter_map(|p| m.world.get(p.hero).map(|u| u.pos()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            "net": crate::online::probe(self),
        })
        .to_string()
    }
}
