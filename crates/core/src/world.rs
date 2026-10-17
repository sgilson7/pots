//! The match: every unit, missile, zone and player, advanced one tick at a
//! time. Everything here is plain data with `Clone`, so rollback copies the
//! world and replays orders on the copy.
//!
//! A tick runs in a fixed order: orders arrive, computer players choose, the
//! clock spawns waves, camps, runes and the boss, every unit regenerates and
//! acts, missiles fly, zones pulse, the dead are counted, vision is redrawn.
//! The same orders in the same ticks give the same match on every machine.

use crate::map::{self, Terrain, Tree};
use crate::path::Grid;
use crate::rng::Rng;
use crate::rules::{Kind, Rules, P, SUB, TPS};

/// A unit's handle: slot in the low 12 bits, generation above. 0 is no unit.
pub type Uid = u32;
pub const NO_UNIT: Uid = 0;
const SLOT_BITS: u32 = 12;
const SLOT_MASK: u32 = (1 << SLOT_BITS) - 1;

pub const SUN: u8 = 0;
pub const MOON: u8 = 1;
pub const NEUTRAL: u8 = 2;
pub const PLAYERS: usize = 10;

pub fn other(team: u8) -> u8 {
    1 - team.min(1)
}

/// A modifier on a unit: a buff, a debuff, a stun, an aura's effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M {
    Stun,
    Silence,
    Hex,
    Root,
    Invulnerable,
    SpellImmune,
    Invisible,
    /// Move speed percent (negative slows).
    MovePct,
    /// Attack speed percent (negative slows).
    AttackPct,
    /// Armor in hundredths (negative reduces).
    Armor,
    /// Flat bonus damage.
    Damage,
    /// Percent of base damage added.
    DamagePct,
    /// Hit points a second in hundredths (negative is damage over time,
    /// magical, credited to the source).
    Regen,
    /// Mana a second in hundredths.
    ManaRegen,
    /// Forced to attack the source (a taunt).
    Taunt,
    /// The rune's double damage.
    DoubleDamage,
    /// A haste: move speed set to the maximum.
    Haste,
    /// Ability-specific state, read by the ability's own rule.
    Tag,
    /// Magic resistance percent.
    MagicResist,
    /// Evasion percent.
    Evasion,
    /// True sight around the unit.
    TrueSight,
    /// Glyph or fountain protection: takes no damage from attacks.
    Glyph,
    /// Cannot attack.
    Disarm,
    /// An illusion: deals `a` percent of its damage and takes `b` percent.
    Illusion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod {
    pub kind: M,
    /// Who applied it.
    pub src: Uid,
    /// The tick it ends; 0 lasts until removed.
    pub until: u32,
    pub a: i32,
    pub b: i32,
    /// The ability or item that applied it (index + 1, or 0), so a reapplied
    /// effect refreshes rather than stacks.
    pub tag: u16,
}

/// What a unit was told to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Idle,
    Move(P),
    Attack(Uid),
    AttackMove(P),
    Hold,
    /// Stand still and attack nothing.
    Stand,
    /// Cast ability `slot` at a unit or a point.
    Cast(u8, Uid, P),
    /// Use the item in inventory slot `slot`.
    Item(u8, Uid, P),
    Follow(Uid),
}

/// What a unit is doing this moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    None,
    /// An attack swing: the hit lands at `hit_at`.
    Swing {
        t: Uid,
        hit_at: u32,
    },
    /// A cast: the effect happens at `fire_at`. `item` marks an item use.
    Cast {
        slot: u8,
        t: Uid,
        at: P,
        fire_at: u32,
        item: bool,
    },
    /// A channel in progress, until `until`.
    Channel {
        slot: u8,
        t: Uid,
        at: P,
        until: u32,
    },
    /// The follow-through after an attack's hit; any order cuts it short.
    Backswing {
        until: u32,
    },
    /// Thrown by the base cannon (All Random All Mid): in the barrel until
    /// `up`, in the air from `from` to `to` until `land`. Map units.
    Flight {
        from: P,
        to: P,
        start: u32,
        up: u32,
        land: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbilState {
    pub level: i32,
    pub ready_at: u32,
    /// A toggle's state, or a counter the ability's rule keeps.
    pub on: bool,
    pub count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemSlot {
    /// Item index + 1; 0 is an empty slot.
    pub item: u16,
    pub charges: i32,
    pub ready_at: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroState {
    pub level: i32,
    pub xp: i32,
    pub points: i32,
    /// Levels taken in the attribute bonus.
    pub bonus: i32,
    /// Kills since last death.
    pub streak: i32,
    /// Enemy heroes who damaged this hero recently: (unit, tick).
    pub damaged_by: Vec<(Uid, u32)>,
    pub respawn_at: u32,
    /// All Random All Mid: the base cannon is loaded for this hero (it
    /// loads in the fountain's range and a throw spends it).
    pub cannon: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub id: Uid,
    pub def: u16,
    pub team: u8,
    /// The player who controls it, or -1.
    pub owner: i8,
    pub alive: bool,
    /// Position in sixteenths of a map unit.
    pub x: i32,
    pub y: i32,
    /// Facing in `fx::TURN` units, counter-clockwise from east.
    pub facing: i32,
    /// Hit points and mana in thousandths.
    pub hp: i32,
    pub mana: i32,
    pub order: Order,
    pub act: Act,
    /// Corners to walk through, map units, nearest last.
    pub path: Vec<P>,
    /// The goal the path was found for.
    pub path_goal: P,
    pub attack_ready: u32,
    /// The unit it is attacking or chasing now.
    pub target: Uid,
    pub mods: Vec<Mod>,
    pub abil: Vec<AbilState>,
    pub items: [ItemSlot; 6],
    pub hero: Option<HeroState>,
    /// Creeps: lane index and the next waypoint.
    pub lane: u8,
    pub waypoint: u8,
    /// Neutrals: the camp index + 1, and home point.
    pub camp: u8,
    pub home: P,
    /// A creep or tower keeps an aggro target until this tick.
    pub aggro_until: u32,
    /// Summons and wards expire at this tick (0 never).
    pub expires: u32,
    /// Animation: what started and when, for drawing only.
    pub anim: u8,
    pub anim_at: u32,
    /// The tick this unit was last hit, and by whom.
    pub hit_at: u32,
    pub last_attacker: Uid,
    /// Upgrade level of a creep when it spawned.
    pub upgrade: i32,
    /// Tower tier, for the rule that a tier falls only after the one before.
    pub tier: i32,
    pub lane_name: u8,
    /// Pseudo-random chance counters for each proc this unit has (bash,
    /// crit, helix, headshot ...): attacks since the last success.
    pub prd: [u8; 6],
    /// The last tick an enemy creep stood near this building (backdoor protection).
    pub quiet: u32,
}

pub const ANIM_STAND: u8 = 0;
pub const ANIM_WALK: u8 = 1;
pub const ANIM_ATTACK: u8 = 2;
pub const ANIM_CAST: u8 = 3;
pub const ANIM_DEATH: u8 = 4;
pub const ANIM_CHANNEL: u8 = 5;

impl Unit {
    pub fn pos(&self) -> P {
        [self.x / SUB, self.y / SUB]
    }

    pub fn has(&self, kind: M) -> bool {
        self.mods.iter().any(|m| m.kind == kind)
    }

    pub fn sum(&self, kind: M) -> i32 {
        self.mods.iter().filter(|m| m.kind == kind).map(|m| m.a).sum()
    }

    pub fn stunned(&self) -> bool {
        self.has(M::Stun)
    }

    /// Cannot move, attack or cast.
    pub fn disabled(&self) -> bool {
        self.has(M::Stun)
    }

    pub fn level(&self) -> i32 {
        self.hero.as_ref().map(|h| h.level).unwrap_or(1)
    }
}

/// A missile in flight: an attack or a spell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Missile {
    pub src: Uid,
    pub team: u8,
    /// Homing target, or NO_UNIT for one that flies to a point.
    pub t: Uid,
    pub x: i32,
    pub y: i32,
    /// Where it is going, sixteenths (for a point missile).
    pub to: P,
    /// Sixteenths a tick.
    pub speed: i32,
    /// What it does on arrival: an attack's damage, or an ability effect.
    pub what: Hit,
    /// Model to draw.
    pub model: u16,
    /// Ticks lived, for missiles that travel a set distance.
    pub age: u32,
    /// Units already hit, for a missile that passes through.
    pub done: Vec<Uid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hit {
    /// An attack carrying its damage roll, whether it is a deny try, and its source's attack type.
    Attack { damage: i32, crit: bool },
    /// An ability's effect, handled by `ability::on_missile`.
    Ability { ability: u16, level: i32 },
    /// An item's effect.
    Item { item: u16 },
}

/// A timed effect on the ground: a delayed blast, a lingering field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub src: Uid,
    pub team: u8,
    pub at: P,
    pub radius: i32,
    pub ability: u16,
    pub level: i32,
    pub start: u32,
    pub until: u32,
    /// Next tick it acts.
    pub next: u32,
    pub a: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub team: u8,
    pub hero: Uid,
    /// Hero roster index, or -1 before picking.
    pub pick: i32,
    pub gold: i32,
    /// Gold that is not lost on death (from hero kills, as in DotA).
    pub reliable: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub last_hits: i32,
    pub denies: i32,
    pub buyback_ready: u32,
    /// Computer player: Some(difficulty index).
    pub bot: Option<u8>,
    /// Whether this seat is in the match.
    pub present: bool,
    /// The bot's memory, kept in the world so rollback replays it.
    pub brain: crate::ai::Brain,
    /// Items bought away from the shop, waiting for the courier: item + 1 and charges.
    pub pending: Vec<(u16, i32)>,
}

/// Something that happened this tick, for the page to show or sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub kind: u8,
    pub a: i32,
    pub b: i32,
    pub at: P,
}

pub const EV_GOLD: u8 = 1;
pub const EV_KILL: u8 = 2;
pub const EV_DENY: u8 = 3;
pub const EV_LEVEL: u8 = 4;
pub const EV_TOWER: u8 = 5;
pub const EV_RUNE: u8 = 6;
pub const EV_BOSS: u8 = 7;
pub const EV_MISS: u8 = 8;
pub const EV_CRIT: u8 = 9;
pub const EV_WIN: u8 = 10;
pub const EV_BUY: u8 = 11;
pub const EV_FIRST_BLOOD: u8 = 12;
pub const EV_HORN: u8 = 13;
pub const EV_CAST: u8 = 14;
pub const EV_RESPAWN: u8 = 15;

/// A moment of an ability worth drawing (round 2, T3): a cast begun, a
/// spell let go, a missile or a blast landing, a passive firing. Only the
/// scene reads marks; no rule does and the checksum leaves them out, so
/// they cannot make two browsers disagree. Each lives `MARK_TICKS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mark {
    pub kind: u8,
    pub ability: u16,
    pub src: Uid,
    pub tgt: Uid,
    /// Where it happens, and where it came from (a beam's or wave's start).
    pub at: P,
    pub from: P,
    pub tick: u32,
}

pub const MK_CAST: u8 = 1;
pub const MK_FIRE: u8 = 2;
pub const MK_LAND: u8 = 3;
pub const MK_PROC: u8 = 4;
pub const MARK_TICKS: u32 = 4 * crate::rules::TPS as u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub tick: u32,
    pub rng: Rng,
    pub units: Vec<Unit>,
    /// Generation counter for each slot.
    pub gens: Vec<u32>,
    pub missiles: Vec<Missile>,
    pub zones: Vec<Zone>,
    pub players: Vec<Player>,
    pub trees: Vec<Tree>,
    /// Bumped whenever a tree is cut or grows back, so the page redraws.
    pub tree_version: u32,
    pub waves: i32,
    pub winner: Option<u8>,
    pub boss_at: u32,
    /// Rune index on the map's list and its kind, or none.
    pub rune: Option<(usize, u8)>,
    pub glyph_ready: [u32; 2],
    /// Vision bits per tree cell: 1 the Sunward sees it, 2 the Moonward.
    pub vision: Vec<u8>,
    pub events: Vec<Event>,
    pub first_blood: bool,
    /// Ticks the match runs before the first wave (the pre-horn).
    pub horn_at: u32,
    /// Each side's courier, and the player it now carries for (-1 none).
    pub couriers: [Uid; 2],
    pub carrying: [i32; 2],
    /// Recent moments of abilities, for the scene only (`Mark`).
    pub marks: Vec<Mark>,
    /// All Random All Mid (round 4).
    pub aram: bool,
    /// The tick each relic returns, Sunward ones first; 0 while it waits.
    pub relics: Vec<u32>,
}

/// The world with the rules and terrain it plays on.
pub struct Ctx<'a> {
    pub rules: &'a Rules,
    pub terrain: &'a Terrain,
}

/// Path cells with this world's trees.
pub struct Walk<'a> {
    pub terrain: &'a Terrain,
    pub trees: &'a [Tree],
}

impl Grid for Walk<'_> {
    fn width(&self) -> i32 {
        self.terrain.pw
    }
    fn cell(&self) -> i32 {
        self.terrain.path_cell
    }
    fn blocked(&self, cx: i32, cy: i32) -> bool {
        let pw = self.terrain.pw;
        if cx < 0 || cy < 0 || cx >= pw || cy >= pw {
            return true;
        }
        let i = (cy * pw + cx) as usize;
        if self.terrain.fixed_block[i] {
            return true;
        }
        matches!(self.trees[self.terrain.tree_of_path(i)], Tree::Standing)
    }
}

pub fn dist2(a: P, b: P) -> i64 {
    let (dx, dy) = ((a[0] - b[0]) as i64, (a[1] - b[1]) as i64);
    dx * dx + dy * dy
}

pub fn within(a: P, b: P, r: i32) -> bool {
    dist2(a, b) <= (r as i64) * (r as i64)
}

pub fn dist(a: P, b: P) -> i32 {
    crate::fx::isqrt(dist2(a, b) as u64) as i32
}

impl World {
    /// An empty world for a match on these rules; `seed` decides every roll.
    pub fn new(ctx: &Ctx, trees: Vec<Tree>, seed: u64) -> World {
        World::new_mode(ctx, trees, seed, false)
    }

    /// A world for a normal match, or for All Random All Mid when `aram`.
    pub fn new_mode(ctx: &Ctx, trees: Vec<Tree>, seed: u64, aram: bool) -> World {
        let aram = aram && ctx.rules.aram.is_some();
        let gold = match (&ctx.rules.aram, aram) {
            (Some(a), true) => a.starting_gold,
            _ => ctx.rules.t.starting_gold,
        };
        let relics = match (&ctx.rules.aram, aram) {
            (Some(a), true) => vec![0; a.relics.len() * 2],
            _ => Vec::new(),
        };
        let cells = (ctx.terrain.tw * ctx.terrain.tw) as usize;
        let mut w = World {
            tick: 0,
            rng: Rng::new(seed),
            units: Vec::new(),
            gens: Vec::new(),
            missiles: Vec::new(),
            zones: Vec::new(),
            players: (0..PLAYERS)
                .map(|p| Player {
                    team: if p < 5 { SUN } else { MOON },
                    hero: NO_UNIT,
                    pick: -1,
                    gold,
                    reliable: 0,
                    kills: 0,
                    deaths: 0,
                    assists: 0,
                    last_hits: 0,
                    denies: 0,
                    buyback_ready: 0,
                    bot: None,
                    present: false,
                    brain: crate::ai::Brain::default(),
                    pending: Vec::new(),
                })
                .collect(),
            trees,
            tree_version: 0,
            waves: 0,
            winner: None,
            boss_at: 0,
            rune: None,
            glyph_ready: [0, 0],
            vision: vec![0; cells],
            events: Vec::new(),
            first_blood: false,
            horn_at: (ctx.rules.t.horn * TPS) as u32,
            couriers: [NO_UNIT; 2],
            carrying: [-1; 2],
            marks: Vec::new(),
            aram,
            relics,
        };
        w.place_buildings(ctx);
        w
    }

    pub fn slot(id: Uid) -> usize {
        (id & SLOT_MASK) as usize
    }

    pub fn get(&self, id: Uid) -> Option<&Unit> {
        if id == NO_UNIT {
            return None;
        }
        self.units.get(Self::slot(id)).filter(|u| u.id == id)
    }

    pub fn get_mut(&mut self, id: Uid) -> Option<&mut Unit> {
        if id == NO_UNIT {
            return None;
        }
        self.units.get_mut(Self::slot(id)).filter(|u| u.id == id)
    }

    /// A living unit.
    pub fn live(&self, id: Uid) -> Option<&Unit> {
        self.get(id).filter(|u| u.alive)
    }

    /// Adds a unit of kind `def` for `team` at `at` (map units).
    pub fn spawn(&mut self, ctx: &Ctx, def: usize, team: u8, at: P) -> Uid {
        let d = &ctx.rules.units[def];
        // a slot is reused once its unit has lain dead three seconds (its
        // fall is drawn meanwhile), and never a player's hero's slot
        let tick = self.tick;
        let heroes: Vec<Uid> = self.players.iter().map(|p| p.hero).collect();
        let free = |u: &Unit| !u.alive && !heroes.contains(&u.id) && tick >= u.anim_at + 3 * TPS as u32;
        let slot = match self.units.iter().position(free) {
            Some(s) => s,
            None => {
                self.units.push(blank());
                self.gens.push(0);
                self.units.len() - 1
            }
        };
        self.gens[slot] += 1;
        let id = (self.gens[slot] << SLOT_BITS) | slot as u32;
        let abil = ctx.rules.unit_abilities[def]
            .iter()
            .map(|_| AbilState::default())
            .collect();
        let hero = if d.kind == Kind::Hero {
            Some(HeroState {
                level: 1,
                xp: 0,
                points: 1,
                bonus: 0,
                streak: 0,
                damaged_by: Vec::new(),
                respawn_at: 0,
                cannon: true,
            })
        } else {
            None
        };
        self.units[slot] = Unit {
            id,
            def: def as u16,
            team,
            owner: -1,
            alive: true,
            x: at[0] * SUB,
            y: at[1] * SUB,
            facing: 0,
            hp: d.hp * 1000,
            mana: d.mana * 1000,
            order: Order::Idle,
            act: Act::None,
            path: Vec::new(),
            path_goal: at,
            attack_ready: 0,
            target: NO_UNIT,
            mods: Vec::new(),
            abil,
            items: [ItemSlot::default(); 6],
            hero,
            lane: 0,
            waypoint: 0,
            camp: 0,
            home: at,
            aggro_until: 0,
            expires: 0,
            anim: ANIM_STAND,
            anim_at: self.tick,
            hit_at: 0,
            last_attacker: NO_UNIT,
            upgrade: 0,
            tier: 0,
            lane_name: 0,
            prd: [0; 6],
            quiet: 0,
        };
        if d.kind == Kind::Hero {
            // a fresh hero starts full
            let s = crate::stats::of(ctx, self, id);
            let u = &mut self.units[slot];
            u.hp = s.max_hp * 1000;
            u.mana = s.max_mana * 1000;
        }
        id
    }

    fn place_buildings(&mut self, ctx: &Ctx) {
        for (team, side) in ctx.rules.map.sides.iter().enumerate() {
            for b in &side.buildings {
                if !crate::aram::builds_lane(self, ctx, &b.lane) {
                    continue;
                }
                let def = ctx.rules.unit(&b.unit).expect("checked when the rules loaded");
                let id = self.spawn(ctx, def, team as u8, b.at);
                let lane = lane_code(&b.lane);
                let u = self.get_mut(id).expect("just spawned");
                u.tier = b.tier;
                u.lane_name = lane;
                u.facing = crate::fx::deg(b.facing);
            }
        }
        if let Some(def) = ctx.rules.unit("shop") {
            for (_, at) in &ctx.rules.map.shops {
                let id = self.spawn(ctx, def, NEUTRAL, *at);
                if let Some(u) = self.get_mut(id) {
                    u.mods.push(Mod {
                        kind: M::Invulnerable,
                        src: id,
                        until: 0,
                        a: 0,
                        b: 0,
                        tag: 0,
                    });
                }
            }
        }
    }

    /// Puts a player's hero on the map at their side's spawn.
    pub fn add_hero(&mut self, ctx: &Ctx, player: usize, roster: usize) -> Uid {
        let team = self.players[player].team;
        let def = ctx.rules.heroes[roster];
        let side = &ctx.rules.map.sides[team as usize];
        // spread the five spawns in a small ring so heroes do not overlap
        let k = (player % 5) as i32;
        let a = crate::fx::TURN * k / 5;
        let at = [
            side.spawn[0] + crate::fx::mul(crate::fx::cos(a), crate::fx::int(120)) / crate::fx::ONE,
            side.spawn[1] + crate::fx::mul(crate::fx::sin(a), crate::fx::int(120)) / crate::fx::ONE,
        ];
        let id = self.spawn(ctx, def, team, at);
        let u = self.get_mut(id).expect("just spawned");
        u.owner = player as i8;
        u.facing = if team == SUN {
            crate::fx::deg(45)
        } else {
            crate::fx::deg(225)
        };
        self.players[player].hero = id;
        self.players[player].pick = roster as i32;
        self.players[player].present = true;
        id
    }

    pub fn event(&mut self, kind: u8, a: i32, b: i32, at: P) {
        self.events.push(Event { kind, a, b, at });
    }

    /// Notes a moment of ability `k` for the scene: where it happens (a
    /// unit's place when `tgt` is one), and where it came from.
    pub fn mark(&mut self, kind: u8, k: usize, src: Uid, tgt: Uid, at: P) {
        let from = self.get(src).map(|u| u.pos()).unwrap_or(at);
        let at = self.get(tgt).map(|u| u.pos()).unwrap_or(at);
        let tick = self.tick;
        self.marks.push(Mark {
            kind,
            ability: k as u16,
            src,
            tgt,
            at,
            from,
            tick,
        });
    }

    /// Living units for which `keep` holds, within `r` of `at`.
    pub fn near(&self, at: P, r: i32, mut keep: impl FnMut(&Unit) -> bool) -> Vec<Uid> {
        let r2 = (r as i64) * (r as i64);
        self.units
            .iter()
            .filter(|u| u.alive && dist2(u.pos(), at) <= r2 && keep(u))
            .map(|u| u.id)
            .collect()
    }

    /// Whether `team` can see this unit now.
    pub fn sees(&self, ctx: &Ctx, team: u8, u: &Unit) -> bool {
        if u.team == team || team == NEUTRAL {
            return true;
        }
        let bit = 1u8 << team.min(1);
        let i = ctx.terrain.tree_index(u.pos()[0], u.pos()[1]);
        let visible = i.map(|i| self.vision[i] & bit != 0).unwrap_or(false);
        if !visible {
            return false;
        }
        if u.has(M::Invisible) {
            // seen only within an enemy true-sight source's radius
            return self.units.iter().any(|s| {
                s.alive
                    && s.team == team
                    && s.mods
                        .iter()
                        .any(|m| m.kind == M::TrueSight && within(s.pos(), u.pos(), m.a))
            });
        }
        true
    }

    /// Is it day? Day comes first.
    pub fn is_day(&self, ctx: &Ctx) -> bool {
        let half = (ctx.rules.t.day_length * TPS) as u32;
        half == 0 || (self.tick / half).is_multiple_of(2)
    }

    /// A short summary of the state, for comparing two copies of a match.
    pub fn checksum(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |v: i64| {
            h ^= v as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        };
        mix(self.tick as i64);
        for u in &self.units {
            if !u.alive {
                continue;
            }
            mix(u.id as i64);
            mix(u.x as i64);
            mix(u.y as i64);
            mix(u.hp as i64);
            mix(u.mana as i64);
            mix(u.facing as i64);
        }
        for p in &self.players {
            mix(p.gold as i64);
            mix(p.reliable as i64);
        }
        for m in &self.missiles {
            mix(m.x as i64);
            mix(m.y as i64);
        }
        mix(self.tree_version as i64);
        mix(self.aram as i64);
        for &r in &self.relics {
            mix(r as i64);
        }
        h
    }

    pub fn walk<'a>(&'a self, terrain: &'a Terrain) -> Walk<'a> {
        Walk {
            terrain,
            trees: &self.trees,
        }
    }

    /// Cuts the tree at tree cell i, if one stands there.
    pub fn cut_tree(&mut self, ctx: &Ctx, i: usize) {
        if matches!(self.trees[i], Tree::Standing) {
            let back = self.tick + (ctx.rules.t.tree_regrow * TPS) as u32;
            self.trees[i] = Tree::Cut(back);
            self.tree_version += 1;
        }
    }

    pub fn cut_trees_in(&mut self, ctx: &Ctx, at: P, r: i32) {
        let tc = ctx.terrain.tree_cell;
        let (x0, y0) = ((at[0] - r) / tc, (at[1] - r) / tc);
        let (x1, y1) = ((at[0] + r) / tc, (at[1] + r) / tc);
        for cy in y0.max(2)..=y1.min(ctx.terrain.tw - 3) {
            for cx in x0.max(2)..=x1.min(ctx.terrain.tw - 3) {
                let i = (cy * ctx.terrain.tw + cx) as usize;
                if within(ctx.terrain.tree_centre(i), at, r) {
                    self.cut_tree(ctx, i);
                }
            }
        }
    }

    /// One tick of the match.
    pub fn step(&mut self, ctx: &Ctx, orders: &[(u8, crate::orders::Cmd)]) {
        self.events.clear();
        let now = self.tick;
        self.marks.retain(|m| m.tick + MARK_TICKS > now);
        if self.winner.is_some() {
            self.tick += 1;
            return;
        }
        for &(p, cmd) in orders {
            crate::orders::apply(self, ctx, p as usize, cmd);
        }
        crate::ai::think(self, ctx);
        crate::lanes::clock(self, ctx);
        crate::stats::regen(self, ctx);
        crate::orders::act_all(self, ctx);
        crate::combat::missiles(self, ctx);
        crate::ability::zones(self, ctx);
        crate::lanes::behave(self, ctx);
        crate::combat::deaths(self, ctx);
        self.expire_mods();
        self.regrow(ctx);
        if self.tick.is_multiple_of(3) {
            crate::vision::update(self, ctx);
        }
        self.tick += 1;
    }

    fn expire_mods(&mut self) {
        let t = self.tick;
        for u in &mut self.units {
            u.mods.retain(|m| m.until == 0 || m.until > t);
        }
    }

    fn regrow(&mut self, ctx: &Ctx) {
        if !self.tick.is_multiple_of(TPS as u32) {
            return;
        }
        let t = self.tick;
        let mut grew = false;
        for i in 0..self.trees.len() {
            if let Tree::Cut(back) = self.trees[i] {
                if back <= t {
                    // a tree does not grow back on a unit standing there
                    let c = ctx.terrain.tree_centre(i);
                    let half = ctx.terrain.tree_cell / 2 + 16;
                    let blocked = self.units.iter().any(|u| {
                        u.alive && (u.pos()[0] - c[0]).abs() < half && (u.pos()[1] - c[1]).abs() < half
                    });
                    if !blocked {
                        self.trees[i] = Tree::Standing;
                        grew = true;
                    }
                }
            }
        }
        if grew {
            self.tree_version += 1;
        }
    }

    /// Adds or refreshes a modifier: one from the same tag and source is
    /// replaced, so recasting refreshes rather than stacks.
    pub fn add_mod(&mut self, id: Uid, m: Mod) {
        if let Some(u) = self.get_mut(id) {
            if m.tag != 0 {
                u.mods
                    .retain(|o| !(o.tag == m.tag && o.kind == m.kind && o.src == m.src));
            }
            u.mods.push(m);
        }
    }

    pub fn remove_mods(&mut self, id: Uid, tag: u16) {
        if let Some(u) = self.get_mut(id) {
            u.mods.retain(|o| o.tag != tag);
        }
    }
}

pub fn lane_code(name: &str) -> u8 {
    match name {
        "top" => 0,
        "mid" => 1,
        "bot" => 2,
        _ => 3,
    }
}

fn blank() -> Unit {
    Unit {
        id: NO_UNIT,
        def: 0,
        team: 0,
        owner: -1,
        alive: false,
        x: 0,
        y: 0,
        facing: 0,
        hp: 0,
        mana: 0,
        order: Order::Idle,
        act: Act::None,
        path: Vec::new(),
        path_goal: [0, 0],
        attack_ready: 0,
        target: NO_UNIT,
        mods: Vec::new(),
        abil: Vec::new(),
        items: [ItemSlot::default(); 6],
        hero: None,
        lane: 0,
        waypoint: 0,
        camp: 0,
        home: [0, 0],
        aggro_until: 0,
        expires: 0,
        anim: 0,
        anim_at: 0,
        hit_at: 0,
        last_attacker: NO_UNIT,
        upgrade: 0,
        tier: 0,
        lane_name: 0,
        prd: [0; 6],
        quiet: 0,
    }
}

/// Builds the ground and an empty world for a match.
pub fn setup(rules: &Rules) -> (Terrain, Vec<Tree>) {
    let mut footprints = Vec::new();
    for side in &rules.map.sides {
        for b in &side.buildings {
            if let Some(d) = rules.unit(&b.unit) {
                let fp = rules.units[d].footprint;
                if fp > 0 {
                    footprints.push((b.at, fp));
                }
            }
        }
    }
    let g = map::generate(&rules.map, &footprints);
    (g.terrain, g.trees)
}
