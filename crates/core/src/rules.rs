//! The rules of a match, read once from `data/*.json` and never changed
//! during play. `World` holds only what changes, so a rollback clones the
//! world and never the rules.
//!
//! Units in the data are the units the literature review gives (map units,
//! seconds in hundredths, percentages), so a number copied from a source goes
//! into the data unchanged. `Rules::load` turns them into ticks and
//! sixteenths of a map unit once.

use std::collections::BTreeMap;

use serde::Deserialize;

/// Sixteenths of a map unit per map unit: the core's unit of length.
pub const SUB: i32 = 16;
/// Ticks per second.
pub const TPS: i32 = 30;

/// Hundredths of a second to ticks, rounded to nearest.
pub fn ticks(hundredths: i32) -> i32 {
    (hundredths * TPS + 50) / 100
}

/// Seconds to ticks.
pub fn secs(s: i32) -> i32 {
    s * TPS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Hero,
    Creep,
    Tower,
    Barracks,
    Sphere,
    Fountain,
    Shop,
    Neutral,
    Boss,
    Ward,
    Summon,
    Building,
}

impl Kind {
    pub fn is_building(self) -> bool {
        matches!(
            self,
            Kind::Tower | Kind::Barracks | Kind::Sphere | Kind::Fountain | Kind::Shop | Kind::Building
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Attr {
    Str,
    Agi,
    Int,
}

/// One kind of unit: a hero, a creep, a tower, a neutral, the boss.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitDef {
    pub id: String,
    pub kind: Kind,
    /// The model in `data/models.json`.
    pub model: String,
    /// Scale of the model in percent.
    #[serde(default = "hundred")]
    pub scale: i32,
    pub hp: i32,
    /// Health regained a second, in hundredths.
    #[serde(default)]
    pub hp_regen: i32,
    #[serde(default)]
    pub mana: i32,
    /// Mana regained a second, in hundredths.
    #[serde(default)]
    pub mana_regen: i32,
    /// Armor in hundredths.
    #[serde(default)]
    pub armor: i32,
    pub armor_type: String,
    #[serde(default)]
    pub attack_type: String,
    #[serde(default)]
    pub damage: [i32; 2],
    /// Attack range in map units; 0 means the unit does not attack.
    #[serde(default)]
    pub range: i32,
    /// Base attack time in hundredths of a second.
    #[serde(default = "bat")]
    pub bat: i32,
    /// Attack point (the swing before the hit) in hundredths of a second.
    #[serde(default = "attack_point")]
    pub attack_point: i32,
    /// Backswing after the hit, hundredths of a second; any order cuts it short.
    #[serde(default = "backswing")]
    pub backswing: i32,
    /// Projectile speed in map units a second; 0 for a melee hit.
    #[serde(default)]
    pub missile: i32,
    #[serde(default)]
    pub speed: i32,
    /// Turn rate in thousandths of a radian per 0.03 s (Warcraft III's unit).
    #[serde(default = "turn_rate")]
    pub turn: i32,
    /// Collision radius in map units.
    #[serde(default = "radius")]
    pub radius: i32,
    #[serde(default = "acquire")]
    pub acquire: i32,
    #[serde(default = "vision")]
    pub day_vision: i32,
    #[serde(default = "vision")]
    pub night_vision: i32,
    #[serde(default)]
    pub bounty: [i32; 2],
    #[serde(default)]
    pub xp: i32,
    /// Magic resistance in percent.
    #[serde(default)]
    pub magic_resist: i32,
    /// Hero attributes: base and gain a level in hundredths.
    #[serde(default)]
    pub attrs: Option<HeroAttrs>,
    #[serde(default)]
    pub abilities: Vec<String>,
    /// For buildings: the footprint's half-width in map units.
    #[serde(default)]
    pub footprint: i32,
    /// For creeps: hit points added at each upgrade.
    #[serde(default)]
    pub upgrade_hp: i32,
    #[serde(default)]
    pub upgrade_damage: i32,
    /// Free text for the person: which DotA unit or hero this follows.
    #[serde(default, rename = "_after")]
    pub after: String,
}

fn hundred() -> i32 {
    100
}
fn bat() -> i32 {
    170
}
fn attack_point() -> i32 {
    40
}
fn backswing() -> i32 {
    50
}
fn turn_rate() -> i32 {
    500
}
fn radius() -> i32 {
    24
}
fn acquire() -> i32 {
    600
}
fn vision() -> i32 {
    1200
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroAttrs {
    pub primary: Attr,
    /// Strength, agility, intelligence at level one.
    pub base: [i32; 3],
    /// Gain a level, in hundredths.
    pub gain: [i32; 3],
}

/// How an ability is aimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    /// Cast at once, no target.
    None,
    Unit,
    Point,
    /// Unit or point.
    Either,
    Passive,
    /// Toggled on and off.
    Toggle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbilityDef {
    pub id: String,
    /// Which rule in `ability.rs` carries it out.
    pub logic: String,
    pub target: Target,
    /// Whom a unit-target ability may be cast on: "enemy", "ally", "any", "self".
    #[serde(default = "enemy")]
    pub on: String,
    #[serde(default)]
    pub ultimate: bool,
    /// Mana a level.
    #[serde(default)]
    pub mana: Vec<i32>,
    /// Cooldown a level in hundredths of a second.
    #[serde(default)]
    pub cooldown: Vec<i32>,
    /// Cast range a level, map units.
    #[serde(default)]
    pub range: Vec<i32>,
    /// Cast point in hundredths of a second.
    #[serde(default = "cast_point")]
    pub cast_point: i32,
    /// Named numbers a level, read by the logic.
    #[serde(default)]
    pub values: BTreeMap<String, Vec<i32>>,
    /// The icon in `web/art/`.
    #[serde(default)]
    pub icon: String,
    /// The palette role its missiles and areas are drawn in.
    #[serde(default)]
    pub color: String,
    /// The hotkey's code (`KeyQ`), for the HUD.
    #[serde(default)]
    pub key: String,
    #[serde(default, rename = "_after")]
    pub after: String,
}

fn enemy() -> String {
    "enemy".into()
}
fn cast_point() -> i32 {
    30
}

impl AbilityDef {
    pub fn max_level(&self) -> i32 {
        if self.ultimate {
            3
        } else {
            4
        }
    }

    /// The number for `key` at `level` (1-based); the last entry holds for
    /// higher levels, and a missing key is 0.
    pub fn v(&self, key: &str, level: i32) -> i32 {
        at(self.values.get(key).map(|v| v.as_slice()).unwrap_or(&[]), level)
    }

    pub fn mana_at(&self, level: i32) -> i32 {
        at(&self.mana, level)
    }

    pub fn cooldown_at(&self, level: i32) -> i32 {
        at(&self.cooldown, level)
    }

    pub fn range_at(&self, level: i32) -> i32 {
        at(&self.range, level)
    }
}

/// The level-th entry (1-based) of a per-level list; the last holds beyond.
pub fn at(list: &[i32], level: i32) -> i32 {
    if list.is_empty() {
        return 0;
    }
    let i = (level.max(1) as usize - 1).min(list.len() - 1);
    list[i]
}

/// Bonuses an item gives while carried.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Bonus {
    #[serde(default)]
    pub str: i32,
    #[serde(default)]
    pub agi: i32,
    #[serde(default)]
    pub int: i32,
    #[serde(default)]
    pub damage: i32,
    /// Hundredths.
    #[serde(default)]
    pub armor: i32,
    /// Percent attack speed.
    #[serde(default)]
    pub attack_speed: i32,
    #[serde(default)]
    pub move_speed: i32,
    /// Percent movement speed (does not stack with itself, as in DotA).
    #[serde(default)]
    pub move_pct: i32,
    #[serde(default)]
    pub hp: i32,
    #[serde(default)]
    pub mana: i32,
    /// Hundredths a second.
    #[serde(default)]
    pub hp_regen: i32,
    /// Percent of base mana regeneration.
    #[serde(default)]
    pub mana_regen_pct: i32,
    /// Hundredths a second.
    #[serde(default)]
    pub mana_regen: i32,
    #[serde(default)]
    pub magic_resist: i32,
    /// Percent evasion.
    #[serde(default)]
    pub evasion: i32,
    /// Percent of attack damage healed.
    #[serde(default)]
    pub lifesteal: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemDef {
    pub id: String,
    /// Gold. For a recipe item, the price of the recipe scroll alone.
    pub cost: i32,
    /// Items this one is built from; empty for a basic item.
    #[serde(default)]
    pub parts: Vec<String>,
    /// Whether a recipe scroll is part of the build.
    #[serde(default)]
    pub recipe: bool,
    /// "home", "side", "secret": where a basic item is sold.
    #[serde(default = "home")]
    pub shop: String,
    /// The shop tab it is listed under.
    #[serde(default)]
    pub tab: String,
    #[serde(default)]
    pub bonus: Bonus,
    /// The active or passive rule in `item.rs`, if any.
    #[serde(default)]
    pub logic: String,
    #[serde(default)]
    pub target: Option<Target>,
    #[serde(default)]
    pub values: BTreeMap<String, i32>,
    #[serde(default)]
    pub cooldown: i32,
    #[serde(default)]
    pub mana: i32,
    #[serde(default)]
    pub range: i32,
    /// Charges a purchase gives; 0 for an item without charges.
    #[serde(default)]
    pub charges: i32,
    /// Whether charges from another purchase stack into one slot.
    #[serde(default)]
    pub stacks: bool,
    #[serde(default)]
    pub icon: String,
    #[serde(default, rename = "_after")]
    pub after: String,
}

fn home() -> String {
    "home".into()
}

impl ItemDef {
    pub fn val(&self, key: &str) -> i32 {
        self.values.get(key).copied().unwrap_or(0)
    }
}

/// Numbers that hold for the whole match.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tables {
    /// Experience needed to reach each level from 2 up (total, not step).
    pub xp_levels: Vec<i32>,
    pub max_level: i32,
    pub starting_gold: i32,
    /// Gold a player gains every `periodic_every` hundredths of a second.
    pub periodic_gold: i32,
    pub periodic_every: i32,
    /// Seconds before the first creep wave leaves (the horn).
    pub horn: i32,
    pub wave_every: i32,
    /// Every this many waves a siege creep joins.
    pub siege_every: i32,
    pub creeps_melee: i32,
    pub creeps_ranged: i32,
    /// Seconds between creep upgrades.
    pub creep_upgrade_every: i32,
    pub creep_upgrade_max: i32,
    pub neutral_every: i32,
    pub neutral_first: i32,
    pub rune_every: i32,
    pub boss_respawn: [i32; 2],
    pub day_length: i32,
    /// Health per strength, hundredths.
    pub hp_per_str: i32,
    pub hp_regen_per_str: i32,
    pub mana_per_int: i32,
    pub mana_regen_per_int: i32,
    /// Armor per agility, hundredths (100/7 ≈ 14).
    pub armor_per_agi: i32,
    /// Attack speed percent per agility.
    pub as_per_agi: i32,
    pub hero_base_hp: i32,
    pub max_move_speed: i32,
    pub min_move_speed: i32,
    /// Most attack speed percent.
    pub max_attack_speed: i32,
    pub min_attack_speed: i32,
    /// Armor reduction factor in ten-thousandths (0.06 is 600).
    pub armor_factor: i32,
    /// Range within which allied heroes share experience.
    pub xp_radius: i32,
    /// Respawn seconds a hero level.
    pub respawn_per_level: i32,
    pub buyback_base: i32,
    pub buyback_per_level_sq: i32,
    pub buyback_per_minute: i32,
    pub buyback_cooldown: i32,
    /// Hero kill gold: base + per victim level.
    pub kill_gold_base: i32,
    pub kill_gold_per_level: i32,
    /// Gold for ending a streak, by the victim's streak from three kills up.
    pub streak_gold: Vec<i32>,
    /// Gold the killer of the match's first hero gains on top.
    pub first_blood_gold: i32,
    /// Gold lost on death: per own level.
    pub death_gold_per_level: i32,
    /// Gold each other enemy hero near a hero's death gains, by how many
    /// there are (one to five): base and per victim level.
    pub assist_gold: Vec<[i32; 2]>,
    /// Range within which enemy heroes share a hero kill's assist gold.
    pub assist_radius: i32,
    /// Deny allowed below this percent of health.
    pub deny_below: i32,
    /// Experience a deny gives the enemies near it, shared: melee and ranged heroes.
    pub deny_xp_melee: i32,
    pub deny_xp_ranged: i32,
    /// Spell damage heroes resist, percent.
    pub hero_magic_resist: i32,
    /// Extra mana the fountain gives a second, hundredths.
    pub fountain_mana: i32,
    /// Seconds after the first wave when a lane wave gains a melee creep, and a ranged and siege creep.
    pub extra_melee_at: Vec<i32>,
    pub extra_ranged_at: Vec<i32>,
    /// The boss grows every this many seconds by this much health and damage.
    pub boss_growth_every: i32,
    pub boss_growth_hp: i32,
    pub boss_growth_damage: i32,
    /// Gold each player of the side that takes a tower gains.
    pub tower_team_gold: Vec<i32>,
    /// Backdoor protection: seconds without enemy creeps near a building
    /// before it returns, its reach, and its regeneration in hundredths.
    pub backdoor_calm: i32,
    pub backdoor_radius: i32,
    pub backdoor_regen: i32,
    /// Buyback cost is rounded down to a multiple of this.
    pub buyback_round: i32,
    /// Range from which a shop sells.
    pub shop_range: i32,
    /// Fountain regeneration: percent of maximum a second.
    pub fountain_regen: i32,
    pub fountain_range: i32,
    /// Percent miss chance attacking up a cliff.
    pub uphill_miss: i32,
    /// Seconds before a cut tree grows back.
    pub tree_regrow: i32,
    /// Range within which a unit attacking an enemy hero draws creep aggro.
    pub creep_aggro_range: i32,
    /// Seconds a creep keeps a hero it switched to.
    pub creep_aggro_time: i32,
    /// Experience a hero kill gives, by the victim's level.
    pub hero_kill_xp: Vec<i32>,
    /// Bonus experience each enemy hero near a hero kill gains, by how many
    /// share it (one to five): base and per victim level.
    pub hero_kill_bonus: Vec<[i32; 2]>,
    /// Ability points granted: one a level; the attribute bonus adds this much to each attribute.
    pub stat_bonus: i32,
    pub stat_bonus_max: i32,
    /// Glyph: seconds of invulnerability and cooldown.
    pub glyph_time: i32,
    pub glyph_cooldown: i32,
}

/// A point in map units.
pub type P = [i32; 2];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placed {
    pub unit: String,
    pub at: P,
    /// Which lane a tower or barracks guards ("top", "mid", "bot", "base").
    #[serde(default)]
    pub lane: String,
    #[serde(default)]
    pub tier: i32,
    #[serde(default)]
    pub facing: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Camp {
    pub at: P,
    /// Unit lists to choose from, one picked by the seed at each spawn.
    pub groups: Vec<Vec<String>>,
    #[serde(default)]
    pub ancient: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Side {
    pub fountain: P,
    /// Lane paths from this side's base to the other's, by name.
    pub lanes: BTreeMap<String, Vec<P>>,
    pub buildings: Vec<Placed>,
    pub creep_spawn: BTreeMap<String, P>,
    pub creeps: [String; 3],
    /// Where heroes appear.
    pub spawn: P,
    /// Base region: a polygon of high ground.
    pub base: Vec<P>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapDef {
    /// Width and height in map units.
    pub size: i32,
    /// Tree cell size in map units.
    pub tree_cell: i32,
    /// Pathing cell size in map units.
    pub path_cell: i32,
    pub seed: u64,
    pub sides: [Side; 2],
    /// The river's centre line and half-width.
    pub river: Vec<P>,
    pub river_width: i32,
    /// Lines kept clear of trees, with their half-widths: jungle paths.
    pub paths: Vec<(Vec<P>, i32)>,
    /// Round clearings: centre and radius.
    pub clearings: Vec<(P, i32)>,
    /// Lane half-width kept clear of trees.
    pub lane_width: i32,
    /// Ramps from the base's high ground: centre and radius.
    pub ramps: Vec<(P, i32)>,
    pub camps: Vec<Camp>,
    pub runes: Vec<P>,
    pub boss: P,
    pub shops: Vec<(String, P)>,
    /// Percent of open forest cells left without a tree.
    pub gaps: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroPick {
    /// The hero's unit id.
    pub unit: String,
    /// The DotA hero whose abilities it carries (for the person).
    #[serde(rename = "_after")]
    pub after: String,
}

/// The attack-type against armor-type table, in percent.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageTable {
    pub attack: Vec<String>,
    pub armor: Vec<String>,
    /// `percent[attack][armor]`.
    pub percent: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Deserialize)]
struct UnitsFile {
    units: Vec<UnitDef>,
}

#[derive(Debug, Clone, Deserialize)]
struct AbilitiesFile {
    abilities: Vec<AbilityDef>,
}

#[derive(Debug, Clone, Deserialize)]
struct ItemsFile {
    items: Vec<ItemDef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RulesFile {
    pub tables: Tables,
    pub damage: DamageTable,
    #[serde(default)]
    pub aram: Option<AramDef>,
    pub heroes: Vec<HeroPick>,
}

/// All Random All Mid (round 4, T13 and T14): what the mode changes. Points
/// are the Sunward side's; the Moonward's are their mirrors through the
/// map's centre, as the map itself is.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AramDef {
    #[serde(default, rename = "_note")]
    pub note: String,
    /// The lanes whose towers, barracks and creeps are built.
    pub lanes: Vec<String>,
    pub starting_gold: i32,
    /// Rerolls each player may spend before the match starts.
    pub rerolls: i32,
    /// Where the cannon stands.
    pub cannon: P,
    /// Hundredths of a second to climb into the cannon, and in the air.
    pub climb: i32,
    pub flight: i32,
    /// How far past the outer tower a throw may land, map units.
    pub reach_past: i32,
    pub relics: Vec<P>,
    /// Percent of health and mana a relic gives back.
    pub relic_pct: i32,
    /// Seconds until a taken relic returns.
    pub relic_back: i32,
    /// How near a hero must walk to take one, map units.
    pub relic_radius: i32,
    /// The level every hero starts at, and the percent of the usual
    /// experience and of the usual gold per second heroes earn (round 7,
    /// after League of Legends' ARAM, a little slower).
    pub start_level: i32,
    pub xp_pct: i32,
    pub gold_pct: i32,
}

/// Everything fixed for a match, with names resolved to indices.
#[derive(Debug, Clone)]
pub struct Rules {
    pub units: Vec<UnitDef>,
    pub abilities: Vec<AbilityDef>,
    pub items: Vec<ItemDef>,
    pub map: MapDef,
    pub t: Tables,
    pub damage: DamageTable,
    /// Hero unit indices in roster order.
    pub heroes: Vec<usize>,
    /// For each unit: its ability indices.
    pub unit_abilities: Vec<Vec<usize>>,
    /// For each unit: attack-type and armor-type indices.
    pub unit_attack: Vec<usize>,
    pub unit_armor: Vec<usize>,
    /// For each item: its parts' indices, and whether a recipe completes it.
    pub item_parts: Vec<Vec<usize>>,
    /// Computer players' trees and difficulty, when loaded.
    pub ai: Option<crate::ai::AiDef>,
    /// The All Random All Mid mode's numbers.
    pub aram: Option<AramDef>,
}

pub type Fail = String;

fn parse<T: serde::de::DeserializeOwned>(what: &str, text: &str) -> Result<T, Fail> {
    serde_json::from_str(text).map_err(|e| format!("{what} could not be read: {e}"))
}

impl Rules {
    pub fn load(
        rules_json: &str,
        units_json: &str,
        abilities_json: &str,
        items_json: &str,
        map_json: &str,
    ) -> Result<Rules, Fail> {
        let r: RulesFile = parse("rules.json", rules_json)?;
        let u: UnitsFile = parse("units.json", units_json)?;
        let a: AbilitiesFile = parse("abilities.json", abilities_json)?;
        let i: ItemsFile = parse("items.json", items_json)?;
        let map: MapDef = parse("map.json", map_json)?;
        let units = u.units;
        let abilities = a.abilities;
        let items = i.items;
        let find_unit = |id: &str| -> Result<usize, Fail> {
            units
                .iter()
                .position(|d| d.id == id)
                .ok_or_else(|| format!("No unit is called \"{id}\"."))
        };
        let heroes = r
            .heroes
            .iter()
            .map(|h| find_unit(&h.unit))
            .collect::<Result<Vec<_>, _>>()?;
        let mut unit_abilities = Vec::new();
        let mut unit_attack = Vec::new();
        let mut unit_armor = Vec::new();
        for d in &units {
            let mut list = Vec::new();
            for name in &d.abilities {
                let k = abilities
                    .iter()
                    .position(|x| &x.id == name)
                    .ok_or_else(|| format!("Unit \"{}\" names no ability \"{name}\".", d.id))?;
                list.push(k);
            }
            unit_abilities.push(list);
            let atk = if d.attack_type.is_empty() {
                0
            } else {
                r.damage
                    .attack
                    .iter()
                    .position(|x| *x == d.attack_type)
                    .ok_or_else(|| format!("Unit \"{}\": no attack type \"{}\".", d.id, d.attack_type))?
            };
            let arm = r
                .damage
                .armor
                .iter()
                .position(|x| *x == d.armor_type)
                .ok_or_else(|| format!("Unit \"{}\": no armor type \"{}\".", d.id, d.armor_type))?;
            unit_attack.push(atk);
            unit_armor.push(arm);
        }
        let mut item_parts = Vec::new();
        for it in &items {
            let mut list = Vec::new();
            for p in &it.parts {
                let k = items
                    .iter()
                    .position(|x| &x.id == p)
                    .ok_or_else(|| format!("Item \"{}\" names no part \"{p}\".", it.id))?;
                list.push(k);
            }
            item_parts.push(list);
        }
        for s in &map.sides {
            for b in &s.buildings {
                find_unit(&b.unit)?;
            }
            for c in &s.creeps {
                find_unit(c)?;
            }
        }
        for c in &map.camps {
            for g in &c.groups {
                for n in g {
                    find_unit(n)?;
                }
            }
        }
        Ok(Rules {
            units,
            abilities,
            items,
            map,
            t: r.tables,
            damage: r.damage,
            heroes,
            unit_abilities,
            unit_attack,
            unit_armor,
            item_parts,
            ai: None,
            aram: r.aram,
        })
    }

    pub fn unit(&self, id: &str) -> Option<usize> {
        self.units.iter().position(|d| d.id == id)
    }

    pub fn item(&self, id: &str) -> Option<usize> {
        self.items.iter().position(|d| d.id == id)
    }

    pub fn ability(&self, id: &str) -> Option<usize> {
        self.abilities.iter().position(|d| d.id == id)
    }

    /// The total gold an item costs, parts and recipe together.
    pub fn item_total(&self, k: usize) -> i32 {
        let it = &self.items[k];
        let parts: i32 = self.item_parts[k].iter().map(|&p| self.item_total(p)).sum();
        if it.parts.is_empty() {
            it.cost
        } else {
            parts + if it.recipe { it.cost } else { 0 }
        }
    }

    /// Percent damage an attack type deals to an armor type.
    pub fn type_percent(&self, attack: usize, armor: usize) -> i32 {
        self.damage
            .percent
            .get(attack)
            .and_then(|row| row.get(armor))
            .copied()
            .unwrap_or(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_level_lists_hold_their_last_value() {
        assert_eq!(at(&[10, 20, 30], 1), 10);
        assert_eq!(at(&[10, 20, 30], 3), 30);
        assert_eq!(at(&[10, 20, 30], 7), 30);
        assert_eq!(at(&[], 2), 0);
    }

    #[test]
    fn hundredths_become_ticks_rounded_to_nearest() {
        // 1.70 s at 30 ticks a second is 51 ticks; 0.4 s is 12; 0.35 s is 10.5 → 11
        assert_eq!(ticks(170), 51);
        assert_eq!(ticks(40), 12);
        assert_eq!(ticks(35), 11);
    }
}
