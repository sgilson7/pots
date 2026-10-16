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
        let models = Models::load(f.models)?;
        for d in &rules.units {
            if models.find(&d.model).is_none() {
                return Err(format!("Unit \"{}\" has no model \"{}\".", d.id, d.model));
            }
        }
        let look: Look = parse("presentation.json", f.presentation)?;
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
            },
            alpha: 0,
            stage: None,
            online: crate::online::Online::default(),
            notice: String::new(),
            last: (Vec::new(), Vec::new()),
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

    /// A fresh world for a match with these seats.
    pub fn new_world(&self, seed: u64) -> World {
        World::new(&self.ctx(), self.trees0.clone(), seed)
    }

    /// Starts an arcade match from the setup: me in seat 0, allies after
    /// me, enemies in the Moonward seats.
    pub fn start_arcade(&mut self) {
        let s = self.setup;
        let mut w = self.new_world(self.seed);
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
        let hero = if s.hero >= 0 { s.hero as usize } else { 0 };
        crate::orders::apply(&mut w, &ctx, 0, Cmd::Pick(hero as u8));
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
        self.pan(dt);
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
        let m = self.m.as_ref()?;
        let ctx = self.ctx();
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

    pub fn pointer_move(&mut self, x: i32, y: i32, inside: bool) {
        self.pointer = (x, y);
        self.pointer_in = inside;
        if self.screen == "match" {
            self.hover = self.unit_at(x, y);
        }
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
                } else {
                    self.command(Cmd::AttackMove(at));
                }
            }
            Some(Aim::Ability(s)) => self.command(Cmd::Cast(s, under, at)),
            Some(Aim::Item(s)) => self.command(Cmd::Item(s, under, at)),
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
                self.command(Cmd::Attack(under))
            }
            Some(u) if u.team == team && self.rules.units[u.def as usize].kind == Kind::Creep => {
                // a right-click on an allied creep tries a deny; the rule decides if it may
                self.command(Cmd::Attack(under))
            }
            _ => self.command(Cmd::Move(at)),
        }
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
        } else if let Some(Aim::AttackMove) = self.aim.take() {
            self.command(Cmd::AttackMove(p));
        } else {
            self.view.target = p;
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
            Aim::AttackMove => None,
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
        self.with_frame(|f| {
            scene::ground(f, &mut b);
            scene::units(f, &mut b);
            scene::effects(f, &mut b);
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
            _ => return Err(format!("There is no stage \"{kind}\".")),
        }
        Ok(())
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
                };
                self.screen = "pick".into();
            }
            "custom" => {
                self.setup.mission = None;
                self.setup.hero = -1;
                self.screen = "pick".into();
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
                self.shop_pick = k as i32;
            }
            "score.toggle" => self.score = !self.score,
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
        "uniforms": [["uLightDir", 3], ["uSky", 3], ["uFog", 3], ["uFogRange", 2], ["uLight", 3], ["uInk", 3], ["uInkSize", 3], ["uBands", 4], ["uAmbient", 3], ["uQ", 1], ["uEps", 1], ["uPasses", 4], ["uGlow", 2]],
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
            "net": crate::online::probe(self),
        })
        .to_string()
    }
}
