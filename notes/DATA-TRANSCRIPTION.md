# Data transcription notes (6.72/6.73)

What `tools/unitgen.py`, `tools/abilitygen.py` and `tools/itemgen.py` now hold, measured against `notes/LITERATURE.md`. Heroes A (Sven, Earthshaker, Pudge, Juggernaut, Drow) use the 6.72 column. Heroes B (Sniper, Crystal Maiden, Lina, Lion, Zeus) use 6.73. Items use column [C]. Section 1 lists values the review does not give or marks UNVERIFIED, and what the data uses instead. Section 2 lists DotA mechanics the Rust core does not model.

## 1. Values not given or UNVERIFIED in the review

### Heroes (all ten)
- **Turn rate.** The review gives only Earthshaker's (0.4, so 400). Every other hero keeps the generator default of 500.
- **Base armor.** Derived as level-1 armor minus agi × 0.14. Playdota rounded the 6.72 figures:
  - Sven 2 → base −1
  - Pudge 2 → base 0 (6.78 shows 0.96, which would mean base −1)
  - Juggernaut 3.9 → base 1 (3.8 in 6.78)
  - Drow 2.1 → base −1
- **Base damage.** Level-1 damage minus the level-1 primary attribute.
- **Base HP regen.** 0.25 for every hero except Juggernaut, which is 0.75 per the review's exception list.
- **Collision radius 24 and acquire range 800.** Both are UNVERIFIED (generator values).
- **Crystal Maiden backswing.** Set to 0, as playdota lists it ("0.55 / 0").

### Creeps, buildings, neutrals, Roshan
- **Lane-creep armor type.** Set to `unarmored`, following playdota. The Dota 2 port says Medium. The review flags this as UNVERIFIED, and the `after` text notes the disagreement.
- **Mega creeps.** Vision, acquire range, attack point and backswing are not in the review. They copy the normal creep of the same kind. The ranged mega creep is given the normal ranged creep's 300 mana.
- **Siege creep vision.** 850/850, as in the 2.3 table.
- **Fountain.** HP regen (999/s) is not in the review and keeps the old value. Armor is 0, as the review gives.
- **Neutrals.**
  - Attack point 0.5, HP regen 0.5, acquire 400 and vision 800 are UNVERIFIED (generator defaults).
  - Armor type is Heavy, as listed. Black Drake and Black Dragon have no armor type in the review, so they also use Heavy.
  - Black Dragon's attack range is not given. It uses 300, the same as Black Drake.
  - Missile speed 900 for the ranged neutrals is from the brief, not the review.
- **Roshan.** Acquire range 250 and collision radius 60 are UNVERIFIED.
- **Healing Ward unit.** Its HP and armor are UNVERIFIED (the data uses 1 and 0). Move speed is 300 (6.72).
- **Wards.** Observer: 200 HP. Sentry: 200 HP and 70 vision ([C]).

### Abilities
- **Cast points.** The review gives one casting animation per hero, not per ability, so every active ability uses its hero's value:
  - Sven, Pudge, Juggernaut, Crystal Maiden, Lion, Sniper: 0.3
  - Earthshaker: 0.69
  - Drow, Zeus: 0.4
  - Lina: 0.45
  - Assassinate: 1.7
  - Exception: Warcry has its 6.72 cast time (it was removed in 6.73).
- **Missile and line speeds.** UNVERIFIED and kept from the old data:
  - Storm Bolt: 1000
  - Meat Hook: 1600
  - Dragon Slave: 1200
  - Assassinate: 2500
- **Dragon Slave length.** Set to 1075. The review says it "can hit units up to 1225 away" with an end width of 150, and 1225 − 150 = 1075. The table also prints "distance 800".
- **Fissure length.** The review marks it UNVERIFIED. It is set equal to the cast range, 1200.
- **Laguna Blade and Finger of Death delay.** Kept at 0.25 s. The review lists no delay for Finger and calls one UNVERIFIED.
- **Freezing Field.**
  - The inner explosion radius (195) is UNVERIFIED.
  - The old +20 armor (a Dota 2 feature) is set to 0, because the review lists no armor bonus.
- **Ministuns.** The review gives no duration for Lightning Bolt's or Assassinate's ministun. Both use 0.1 s.
- **Storm Bolt stun.** 2 s at every level, per the 6.72 table.
- **Mana Drain break range.** 800, per 6.73.
- **Roshan's Bash.** 15%, +50 damage, 1.65 s, matching the review.

### Items
- **Mask of Death.** The review gives no lifesteal %. The data keeps 15.
- **Buckler and Headdress.** The review gives cost only. Their bonuses are kept from the old data.
- **Cranium Basher.** The review gives cost only. Its bonuses are kept: +40 damage, +6 str, 25% melee / 10% ranged, 1.4 s stun.
- **Items added to complete the 6.72 recipes:**
  - Stout Shield (250)
  - Helm of Iron Will (950, +5 armor, +3 regen)
  - Oblivion Staff (1675)
  - Crystalys (2150, scroll 500)
  - Helm of the Dominator (1850)
- **Stats not in the review for the added items:**
  - Oblivion Staff uses +6 int, +15 attack speed, +10 damage, +75% mana regen.
  - Crystalys uses +30 damage with a 20% ×1.75 crit.
  - Helm of the Dominator uses +5 armor, +20 damage, 15% lifesteal.
  - All three come from general knowledge and are UNVERIFIED.
- **Dagon.**
  - Its [C] parts sum to 2720 against the review's 2730 total; the review flags this too.
  - "+13 Intelligence; +3 All Attributes" is read as 13 int total plus 3 str and 3 agi, which is UNVERIFIED.
- **Hand of Midas.** `xp_pct` is 250, meaning the normal XP plus a 1.5× bonus. It assumes the core grants no normal kill XP for the transmuted creep.
- **Manta Style.** The data uses the melee cooldown of 35 s. The ranged values (28%/400%, 50 s) are stored but unused.
- **Item icons.** The new items name icons (`items/stout_shield`, `items/helm_iron_will`, `items/oblivion_staff`, `items/crystalys`, `items/helm_dominator`) that may not exist yet in `web/art`.

## 2. Mechanics the Rust core does not model
Value keys that are written to the data but not read by the core are shown in brackets.

### Heroes and abilities
- **Trueshot Aura.**
  - 6.72 gives ranged allies a percentage of their own base damage.
  - `ability.rs` instead gives `agi × pct / 100` of Drow's agility, which is the 6.76+ rule.
  - Its radius should affect ranged units only, and the code does check for a missile.
- **Frost Arrows.**
  - Does not apply the attack-speed slow [`attack_slow`].
  - Does not use the 7 s duration on creeps [`creep_duration`].
- **Frostbite.** Does not use the 10 s duration on creeps of level 5 or lower [`creep_duration`]. The core spreads `damage` (the total) over the duration correctly.
- **Dismember.**
  - Does not use the 6 s duration on creeps [`creep_duration`].
  - The Aghanim's STR bonus and heal are not modeled [`scepter_str_pct`].
- **Aghanim's upgrades.** None are modeled:
  - Freezing Field, Laguna Blade and Thundergod's Wrath [`scepter_damage`, `scepter_range`]
  - Finger of Death [`scepter_damage`, `scepter_cooldown`]
  - Omnislash [`scepter_jumps`, `scepter_cooldown`]
  - Echo Slam (double echo)
- **Echo Slam.** The echo search radius of 550 is separate from the 500 initial AoE [`echo_radius`]. The core uses `radius` for both.
- **Dragon Slave.** Its width tapers from 275 to 150 [`end_width`]. The core uses a constant 275 radius.
- **Line widths treated as half-widths.** The core uses the review's "AoE (width)" figures as a half-width or radius: Fissure 225, Impale 125, Hook 100. Whether playdota's number is a radius or a full width is UNVERIFIED.
- **Impale.** The 0.52 s airborne time before the stun is not modeled [`airborne`]. The `speed` key is also unused, because the core applies Impale instantly.
- **Shrapnel.**
  - DotA hits 9 times (once on landing, then every second); the core deals continuous damage over 8 s [`hits`].
  - Buildings take 1/3 damage in DotA; the core does not [`building_pct`].
- **Lightning Bolt and Thundergod's Wrath.** True sight and vision around the target are not modeled [`sight`, `vision`, `sight_duration`].
- **Static Field.** DotA lands it before the spell's damage. The core's order is `on_cast` relative to `effect`; check whether that matches.
- **Mana Drain.** DotA drains in 0.25 s ticks [`tick`]; the core drains continuously. It does not destroy illusions after 0.25 s.
- **Rot.** Its slow does not pierce magic immunity in 6.72; the core matches this.
- **Flesh Heap.** The pickup radius is hard-coded at 400 in `hero_died_near`. The review says 450 [`radius`]. Retroactive recount is implicit, because stacks × the current level's value.
- **Healing Ward.** "Does not stack" and "not on mechanical units" are not modeled.
- **Blade Fury.** The core disarms Juggernaut. In 6.72 he could still attack units Blade Fury does not affect.
- **Omnislash.** The jump range in the data is `radius` 425.
- **Headshot.** Damage type is physical (hero physical in DotA). Magic immunity blocking the stun is handled by the core's `stun()`.
- **Roshan.**
  - Not modeled: Spell Block, Slam, and the growth of +500 HP and +10 damage every 5 min with the armor aura.
  - The core's Bash damage is Magic.

### Items
- **Javelin.** The 20% chance of +40 damage is not modeled [`chance`, `damage`].
- **Vanguard.** Damage Block is not modeled [`block_*`]. Neither is Stout Shield's block; the item has no values.
- **Heart of Tarrasque.** The regeneration of 2% of max HP per second, paused for 6 s (4 s melee) after damage, is not modeled [`regen_pct`, `delay`, `melee_delay`]. Heart therefore currently gives no regeneration. The old flat +15/s was removed because the review does not support it.
- **Black King Bar.** The duration and cooldown that shrink with each use are not modeled [`min_*`, `step_*`]. The core always uses 10 s and 80 s.
- **Mekansm.**
  - The +4 HP regen aura in 750 is not modeled [`aura_regen`, `aura_radius`].
  - The 25 s re-heal lockout is not modeled [`lockout`].
- **Shiva's Guard.**
  - The −40 attack speed aura in 900 is not modeled [`aura_*`].
  - The core applies the Arctic Blast wave instantly [`speed`].
- **Assault Cuirass.** The −5 enemy armor aura is not modeled [`enemy_armor`].
- **Linken's Sphere.** Spell Block (every 17 s) is not modeled [`block_cooldown`].
- **Sange and Sange and Yasha.** Maim is not modeled [`maim_*`].
- **Mjollnir.** Static Charge is not modeled [`shield_*`].
- **Maelstrom and Mjollnir.** The core hard-codes the 500 bounce range [`radius`].
- **Phase Boots.** The Phase active is not modeled [`move_pct`, `duration`, `cooldown`].
- **Power Treads.** Switch Attribute is not modeled. Treads always give +8 str.
- **Monkey King Bar.** True Strike is not modeled.
- **Orchid Malevolence.** The 30% damage amplification is not modeled; the core silences only [`amp`].
- **Dagon.** Levels 2–5 are not modeled.
- **Magic Stick and Magic Wand.** The core's charge radius is not read from data [`radius` 1200].
- **Kelen's Dagger (Blink).** The 3 s lock after taking damage is not modeled [`damage_lock`].
- **Scroll of Town Portal.**
  - The 3 s channel and the extra time per recent allied teleport [`channel`] are not read from data.
  - The 525 offset from the building [`offset`] is not read from data either.
- **Force Staff.** The core applies the push instantly [`time`].
- **Aegis.**
  - The core keeps it on its 5 s revive.
  - Expiry after 6 min is not modeled [`expires`].
- **Orb effects.** Orb-effect and buff-placer conflicts are not modeled.
- **Item cleave.** Battle Fury's cleave correctly skips ranged carriers.
