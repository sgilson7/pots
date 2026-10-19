# OGDota literature review: DotA Allstars (WC3), 6.48–6.83

This is the source of truth for the game's data files. Original names are kept here so the facts can be checked; the remake renames everything.

Compiled 2026-10-16 from web research. Every number has a source. Where nothing could be confirmed, the value is marked **UNVERIFIED** instead of guessed.

**Baseline version.** Unless stated otherwise, numbers are **DotA 6.72/6.73** (2011), taken from IceFrog's official site playdota.com through the Wayback Machine. Version deltas from 6.48 to 6.79c come from IceFrog's changelogs. The 6.81–6.83 changelog pages were not archived; late-6.8x values come only from 2014/2016 playdota snapshots.

**Access notes.**
- fandom.com (dota2 and dota wikis), Liquipedia and Reddit returned 402/403/429 to direct fetches during this research.
- The Wayback Machine worked.
- Some claims from those blocked sites are given as search-engine summaries and are labelled that way.
- No Reddit quotes could be retrieved. The forum quotes come from TeamLiquid, Hive Workshop and Steam instead.

## Contents
1. Sources
2. Core numbers (map, buildings, creeps, neutrals, Roshan, runes, gold, XP, respawn, attributes, armor, attack/move, denying, inventory, disables, modes)
3. Items (shops, basic items, ~50 key recipes)
4. Ten MVP heroes
5. "Feel" details with player quotes
6. What to simplify for an MVP vs. what never to simplify

## 1. Sources

| # | Title | URL | Type | Used for | Reliability |
|---|---|---|---|---|---|
| 1 | PlayDotA "DotA Mechanics" pages (Attributes, Attack Speed, Damage and Armor, Gold, Experience, Buildings, Unit Statistics, Roshan, Runes, Terrain and Vision, Day and Night, Fountains, HP/MP, Item Sharing, Disables, Cooldown, Cleave, Invisibility, Neutral Creeps, Illusions, Targeting, Factions) | https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics (subpages listed in §2 keys) | reference (official DotA 1 site) | Nearly all core formulas and unit stats | **High.** Official IceFrog-era site, snapshot Dec 2011 to Jan 2012 (6.72/6.73). A few typos; some content is community-written. |
| 2 | PlayDotA unit statistics (towers, creeps, neutrals, buildings, summons, with damage point/backswing, acquisition range, bounty dice) | https://web.archive.org/web/20111215055159/http://www.playdota.com:80/mechanics/unitstats | reference | §2.2–2.5 tables | High. It shows engine-level fields (unit IDs, dice). |
| 3 | IceFrog changelogs 6.48–6.79c (mirrored on playdota) | https://web.archive.org/web/2013/http://www.playdota.com/changelogs/6.79 (and `/6.48` … `/6.79c`) | primary source | Version deltas (Glyph, backdoor, buyback, XP AoE, day/night, Smoke, Roshan respawn, …) | **Highest.** The developer's own notes. Each page also shows the previous version, so the earliest page with a change is taken as its version. |
| 4 | PlayDotA hero pages (per-hero stats, animation points, full ability tables) | e.g. https://web.archive.org/web/20110613072647/http://www.playdota.com/heroes/rogue-knight | reference | §4 heroes | High. Snapshots at 2010, 2011 (6.72), 2013 and 2016 were diffed against each other. |
| 5 | PlayDotA item database `cache/items.js` and Items/Shops pages | https://web.archive.org/web/20100116065848/http://www.playdota.com/cache/items.js?242=242 ; https://web.archive.org/web/20111229165805/http://www.playdota.com/cache/items.js?117=117 ; https://web.archive.org/web/20111215050913/http://www.playdota.com:80/mechanics/shopsanditems ; https://web.archive.org/web/20111227082539/http://www.playdota.com:80/mechanics/Orb_Effects | reference (raw data) | §3 items | High. These are machine data dumps used by the official site. |
| 6 | PlayDotA "Learn" guides (Game Modes & Commands, Lane Control, Runes, Animations) | https://web.archive.org/web/2011/http://www.playdota.com:80/learn/commands | guide (official site) | Game modes, creep aggro, rune control, animation cancelling | Medium-high. Written by community authors and published on the official site. |
| 7 | Blizzard Classic Battle.net: WC3 Armor and Weapon Types | http://classic.battle.net/war3/basics/armorandweapontypes.shtml | reference (engine) | Armor formula; stock WC3 attack/armor table (which DotA overrides) | High for stock WC3. **Not** DotA's table. |
| 8 | Dota 2 Wiki (Gamepedia) 2013 snapshots: Armor, Lane creeps, Aggro, Denying, Movement speed, Turn rate, Glyph, Runes, Buildings, Shops | https://web.archive.org/web/2013/http://dota2.gamepedia.com/Armor | wiki | Cross-check; tower targeting order; MS cap; turn-rate units | Medium. Dota 2 at 6.7x parity, not DotA 1. Marked "(Dota 2 port)" wherever used. |
| 9 | Dota 2 Wiki Armor/Changelogs (DotA 6.xx lines) | https://web.archive.org/web/2019/https://dota2.gamepedia.com/Armor/Changelogs | wiki | History of the attack/armor table | Medium. One line conflicts with another archive (Pierce vs Heavy, 6.44). |
| 10 | GosuGamers, "6.83b launched … respawn time changes" | https://www.gosugamers.net/dota2/news/29587-6-83b-launched-with-minor-nerfs-and-respawn-time-changes | news | Old respawn formula 4×Level | Medium. Dota 2 6.83b news, but it states the pre-6.83b (DotA parity) formula. |
| 11 | D1Stats "DotA Allstars v7.00" changelog (unofficial post-IceFrog WC3 continuation) | https://d1stats.ru/dota-700-changelog/?lang=en | changelog (community) | Context only (its "from" values mirror late 6.8x/Dota 2) | Low-medium. Not IceFrog. |
| 12 | Dota 2 Wiki 5.61 changelog (Guinsoo-era DotA Allstars) | https://dota2.fandom.com/wiki/5.61 | wiki | Deny below 50% HP rule | Medium. Read via search summary; fandom blocked the direct fetch. |
| 13 | "What is DotA?" (UNBC personal page) | https://web.unbc.ca/~cchan/about.htm | personal page | Ultimate at 6/11/16 | Low-medium. Consistent with the hero tables. |
| 14 | TeamLiquid "Little differences in Dota2 beta vs DotA" (2011) | https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota | forum | Player quotes on feel (animations, blocking, selection) | Medium as opinion; high as a record of player sentiment. |
| 15 | TeamLiquid blog "Dota 2 interface concerns" (2011) | https://tl.net/blogs/295350-dota-2-interface-concerns | forum/blog | Quotes on animations, balance, input latency | Opinion |
| 16 | TeamLiquid blog "DotA back to basics" (2011) | https://tl.net/blogs/282827-dota-back-to-basics | guide | Runes, stacking, day/night ganking | Medium (Dota 2 beta at 6.7x parity) |
| 17 | Hive Workshop "DotA vs DotA 2" (2014) | https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/ | forum | Nostalgia and comparison quotes | Opinion |
| 18 | Hive Workshop "DotA All-Stars gets a comeback" (community 6.85 notes, 2017) | https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/ | forum/changelog | WC3 engine quirks that 6.85 "fixed" (global tree regrowth, fountain, random order delay) | Medium. A community patch describing 6.83 behaviour. |
| 19 | Hive Workshop threads on damage point/backswing and cast point | https://www.hiveworkshop.com/threads/dps-and-attack-animation-damage-point.247190/ ; https://www.hiveworkshop.com/threads/cast-point-and-backswing-point.265781/ | reference (WC3 modding) | Animation-cancel engine rules | Medium-high |
| 20 | Steam guide on tower/creep aggro (2014) | https://steamcommunity.com/sharedfiles/filedetails/?id=288488800 | guide | Tower aggro priorities, 500 aggro vs 700 attack range | Medium (Dota 2) |
| 21 | Liquipedia Dota 2: Denying, Attack Modifier, Lane Creeps, Vision, Attack Animation, Creep control techniques, Archive:Version 6.70 | https://liquipedia.net/dota2/Denying (etc.) | wiki | Deny XP by version, orb walking, creep aggro radius (Dota 2), vision rules | Medium-high. Has DotA 1 version-history lines. Fetched via API by a sub-researcher; some entries are search snippets. |
| 22 | DotA-Blog command/mode guide (2008–2010) | http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html | guide | Chat commands and the version each was added | Medium |
| 23 | warcraft.wiki.gg "Defense of the Ancients: Allstars" | https://warcraft.wiki.gg/wiki/Defense_of_the_Ancients:_Allstars | wiki | Leaver handling, banlists | Medium (fetch summary) |
| 24 | Bayes Esports map coordinate doc | https://docs.bayesesports.com/docs-live-data/message-schema/misc/map_coord | reference | Dota 2 map size (context only) | Not DotA 1 |
| 25 | Liquipedia game versions / Softpedia changelog page | https://www.liquipedia.net/dota2/Game_Versions/6.70_to_6.88f ; https://games.softpedia.com/progChangelog/Warcraft-III-DotA-Allstars-Map-Changelog-12121.html | reference | Release dates used to date snapshots | Medium (search summaries) |

The per-section source lists in §3, §4 and §5 give the exact snapshot URL behind each number.


## 2. Core numbers

**How to read this section.** The main source is the official DotA 1 site playdota.com (IceFrog era), read through the Wayback Machine. Its "DotA Mechanics" pages were captured in Dec 2011 to Jan 2012, which is map version **6.72/6.73**. The second source is IceFrog's own changelogs for 6.48 to 6.79c, mirrored on playdota.com/changelogs. Where a value changed between 6.48 and 6.83, the version is given. Pages from the 2013 Dota 2 wiki are used only where no DotA 1 page exists. Each of those uses is labelled **(Dota 2 port)**, because Dota 2 copied 6.7x parity but is not DotA 1. **Baseline for the data files: 6.72/6.73 unless stated otherwise.** The reference keys in square brackets, such as [PD-Gold], resolve to the URLs listed at the end of this section and in the Sources table.

### 2.1 Map and layout

| Item | Value | Source |
|---|---|---|
| Factions | Sentinel at bottom-left, Scourge at top-right. Lane creeps are Treant/Druid of the Talon/Glaive Thrower (Sentinel) and Ghoul/Necromancer/Meat Wagon (Scourge). Their stats are identical. | [PD-Factions] |
| Lanes | 3 lanes (top, middle, bottom). Each lane has 3 towers per side, plus 2 towers beside each Ancient, for **11 towers per side**. The minimum number of towers needed to win is 5. | [PD-Buildings] |
| River | Runs from near the north-west corner to near the south-east corner. Both rune spots are in it. | [PD-Terrain] |
| Rune spots | 2. One is north-west of the map centre and one is south-east, both in the river. | [PD-Runes] |
| Jungle (per side) | 1 easy, 2 moderate, 2 hard and 1 ancient camp, for 12 camps on the map. Their numbered positions are in [PD-Neutrals]. Version notes: 6.65 "Added a new small neutral camp" [CL-6.65]. 6.69 added the Thunder Lizard ancient camp [CL-6.69]. 6.79 swapped the medium pullable camp with the small camp [CL-6.79]. | [PD-Neutrals] |
| Roshan pit | Roshan can be attacked only from inside his spawn area ("protected area"). Vision into the pit is restricted, and wards cannot be placed inside it. | [PD-Roshan], [CL-6.48] ("error message when attacking Roshan from a disallowed area") |
| Shops | Each base has its faction's shops (names and items are in section 3), plus 2 Secret Shops and 2 side shops (Goblin Laboratory and Goblin Merchant from 6.60). | [CL-6.60], items section |
| Fountain | Well of Life (Sentinel) and Defiled Fountain of Life (Scourge). | [PD-Fountain] |
| Map dimensions | **UNVERIFIED.** No DotA 1 source gives the World Editor size. A third-party "Dota All Star Privatemap" listing says 128x128 with 116x116 playable, but that is not the official map ([Softpedia](https://games.softpedia.com/get/Mods-Addons/Warcraft-3-Map-Dota-All-Star-Privatemap.shtml)). Modern Dota 2 is about 17664 x 16643 units ([Bayes docs](https://docs.bayesesports.com/docs-live-data/message-schema/misc/map_coord)), but its map was later redesigned. Recommendation: lay out to scale from distances such as tower range 700 and XP range 1200, and open the 6.83d .w3x in the World Editor to confirm. | — |
| Terrain | Trees block vision. Dead trees respawn every 5 minutes from the first creep spawn, but not where a unit is nearby. Higher ground sees farther. A ranged unit attacking uphill has a **25% chance to miss**. | [PD-Terrain], [PD-Time] |

### 2.2 Towers, barracks, Ancient, fountain (6.72/6.73 values, from [PD-Units])

| Building | HP | Armor (type) | Damage (type), range | BAT | Projectile | Vision day/night | Last-hit bounty | Notes |
|---|---|---|---|---|---|---|---|---|
| Tower T1 (Ancient Protector / Spirit Tower lvl 1) | 1300 | 18 (Fortified) | 100-120 (Siege), 700 | 1.0 | 750 | 1900/800 | 452-470 | True Sight 900 AoE. Damage point/backswing 0.4/0.6, acquisition range 700. |
| Tower T2 | 1600 | 18 | 120-140, 700 | 0.95 | 750 | 1900/800 | 502-520 | |
| Tower T3 | 1600 | 25 | 122-182, 700 | 0.95 | 750 | 1900/800 | 552-570 | |
| Tower T4 (Ancient guards, x2) | 1600 | 30 | 122-182, 700 | 0.95 | 750 | 1900/800 | 602-620 | |
| Melee barracks (Ancient of War / Crypt) | 1500 | 15 (Fort) | — | — | — | 900/600 | 352-370 | HP regen 2.5 |
| Ranged barracks (Ancient of Lore / Temple of the Damned) | 1500 | 5 (Fort) | — | — | — | 900/600 | 352-370 | HP regen 2.5 |
| World Tree / Frozen Throne (Ancient) | **4250** | 15 (Fort) | — | — | — | 1800/1800 | 0 | HP regen 3, True Sight 900 |
| Fountain | 50000 | 0 (Fort) | 190-199 (Chaos), 800 | 0.15 | 1400-1500 | 1300/1300 | 0 | 825 AoE: 4% HP/s, 4% mana/s, +14 mana/s |

Version notes and disagreements:
- 6.60: the first tower's damage type was changed to Siege "like the rest of the towers" [CL-6.60].
- 6.78: Level 1 and 2 tower armor 18 → 20. The tower last hit now gives the hero 100-200 bonus gold, and team bounty is uniform [CL-6.78]. 6.79: last-hit bonus 150-250 [CL-6.79].
- 6.53: ranged barracks armor 15 → 5 and regen 2.5 → 20 [CL-6.53]. 6.60: "Range rax no longer has extra regeneration" [CL-6.60]. 6.79: melee barracks regen 2.5 → 5, ranged barracks HP 1500 → 1200, armor 5 → 10, regen 0 [CL-6.79].
- Tower vision: playdota lists **1900/800**. The Dota 2 port says 1800/800 [D2W13-Buildings]. Use 1900/800 for DotA 1.
- The fountains page calls fountain damage "physical" [PD-Fountain], but the unit table says Chaos [PD-Units]. The fountain heal is percentage-based, so items with flat mana speed up refills [PD-Fountain].
- Team gold for a tower kill (as opposed to the last-hitter bounty above): **UNVERIFIED** for 6.72. The changelogs say "Global team tower bounty increases a little per tower level" (6.55) and "Denied towers give 50% team gold rather than 0%" (6.55) [CL-6.55].

**Invulnerability chain.** Each tower is invulnerable until the tower before it in the lane dies. The 2 Ancient-side towers must die before the Ancient becomes vulnerable [PD-Buildings]. In the Dota 2 port, the T4s become vulnerable once any T3 is down, without needing barracks [D2W13-Buildings]. DotA 1 confirmation of that detail: UNVERIFIED.

**Backdoor protection ("regen armor").** Introduced in 6.60 [CL-6.60]. Verbatim rules:
- The "regen armor" is stripped away whenever there are nearby creeps (or corpses). It comes back once 15 seconds pass with no nearby unit.
- If a tower or barracks takes damage without the regen armor, that life can never be naturally regenerated. With the regen armor, it regenerates 40 HP/s and takes only 25% damage from illusions.
- Flagged areas are everything in or near the base and the second tower of each lane.

Later changes: 6.64/6.65 regen 40 → 55 HP/s [CL-6.65]. 6.68 "Increased Anti-Backdoor regeneration rate" [CL-6.68]. The playdota page states **90 HP/s** in late 2011 [PD-Buildings]. 6.78 adds a 25% incoming damage reduction and fixes the Ancients not having protection [CL-6.78]. Towers do not regenerate from allied (deny) damage (6.60) [CL-6.60].

**Glyph of Fortification.**
- 6.55: a consumable "item" with no cost and a 6-minute cooldown that "gives Old TP Scroll style bonus armor to all allied buildings for 4 seconds" [CL-6.55].
- 6.57: moved to an ability on the Circle of Power [CL-6.57].
- 6.63: the cooldown is reset at the first creep spawn [CL-6.65].
- 6.69: cooldown 6 → 5 minutes [CL-6.69].
- 6.75: duration 4 → 5 s [CL-6.75].
- The Dota 2 port notes that "in earlier versions of DotA, Glyph did not grant magic immunity" [D2W13-Glyph].

**Tower targeting.** The priority list below comes from the Dota 2 port, which inherited the DotA AI. It is listed lowest to highest priority, so the closest enemy hero is the lowest priority and a unit attacking a friendly hero is the highest:

1. Closest enemy hero.
2. Closest catapult.
3. Closest enemy unit.
4. Closest unit attacking any friendly unit.
5. Closest unit attacking the tower.
6. Closest unit attacking a friendly hero.

A tower switches target only when its target dies, when an enemy targets a friendly hero, or when the hero it is attacking manually attacks a friendly unit [D2W13-Aggro]. A DotA 1 changelog confirms that towers defend heroes ("Fixed a bug with Towers AI not defending heroes properly", 6.59 [CL-6.59]). The full DotA 1 priority order is UNVERIFIED.

**Tower deny.** Allowed in DotA 1 (6.55 denied-tower gold rule [CL-6.55]). The DotA 1 HP threshold is **UNVERIFIED**. The Dota 2 port uses ≤10% HP [D2W13-Denying].

### 2.3 Lane creeps (6.72/6.73, [PD-Units])

| Unit | Level / XP | HP | Regen | Damage (type), range | Armor (type) | MS | BAT | Dmg point/backswing | Missile | Vision | Acq. | Gold |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Melee (Treant/Ghoul) | 3 / 62 | 550 | 0.5 | 19-23 (Normal), 100 | 2 (playdota prints "Unarmored") | 325 | 1.0 | 0.467/0.533 (Treant), 0.39/0.44 (Ghoul) | — | 850/800 | 500 | 38-48 (37+1d11) |
| Ranged (Druid/Necromancer) | 2 / 41 | 300 | 2 | 21-26 (Piercing), 500 | 0 | 325 | 1.0 | 0.5/0.67 (Druid), 0.53/0.47 (Necro) | 900 | 850/800 | 800 / 600 | 43-53 (42+1d11) |
| Siege (Glaive Thrower/Meat Wagon) | 4 / 88 | 500 | — | 35-46 (Siege), 690 | 0 (Fortified) | 325 | 2.7 | 0.1/1.9 (GT), 0.7/1.3 (MW) | 1100 | 850/850 | 800 | 66-80 (65+1d15) |
| Mega melee | 1 / 25 | 700 | 0.5 | 36-44 | 3 | 325 | 1.0 | | | | | 18-26 |
| Mega ranged | 1 / 25 | 475 | 2 | 41-46 | 1 | 325 | 1.0 | | 900 | | | 18-26 |
| Mega siege | 4 / 88 | 500 | — | 51-62 | 0 (Fort) | 325 | 2.7 | | 1100 | | | 66-80 |

Ranged creeps have 300 mana and 0.75 mana regen. Siege creeps have the "SiegeUnit" ability, which makes them automatically attack any tower within 900 [PD-Units].

**Waves.**
- A wave spawns every **30 s**. A siege creep comes every **7th wave**.
- The base wave is **3 melee + 1 ranged** (+1 siege on every 7th wave).
- **+1 melee** at 999, 1999 and 2999 s after the first wave. **+1 ranged and +1 siege** at 2699 s [PD-Units].
- In 6.55 "the game timer now counts down to 0, at which point the creeps and powerups spawn. 30 seconds later the neutrals spawn" [CL-6.55].
- **Upgrades** apply every 7:30 from the first spawn, up to 30 levels. Normal melee gets +10 HP and +1 damage per level. Normal ranged gets +10 HP and +2 damage. Mega melee gets +19 HP and +2 damage. Mega ranged gets +18 HP and +3 damage. Siege creeps do not upgrade [PD-Units].
- Killing a barracks makes that lane spawn the Mega version (melee barracks for melee, ranged barracks for ranged and siege). Killing all barracks sets the creep upgrades to level 30 [PD-Units], [PD-Buildings].
- Creep vision: 1400/800 → 1100/800 in 6.68 [CL-6.68] → 850 in 6.70 [CL-6.70]. The playdota table shows 850/800, which matches.
- **Super Creeps** (Siege Golem, Scary Fish, Ancient Hydra), one every 10 minutes, exist only in the **-sc** game mode [PD-Units], [PD-Cmd].
- **Armor type disagreement.** Playdota labels lane creeps "Unarmored". The Dota 2 port's creep armor type was "Medium" until 6.86 changed it [D2W-ArmorCL]. The two are numerically different (Unarmored takes 150% from Pierce, Medium takes 75%). **Flag for verification against the .w3x.** Both labels can be right if the World Editor's "Normal" armor shows in game as "Medium". UNVERIFIED.

### 2.4 Neutral creeps

- **Spawning.** First spawn at **0:30**, then at the start of every minute (XX:00). A camp spawns only if its spawn box is empty of units. Exceptions that do not block: Scarab, Spin Web, Remote Mine, Exorcism ghosts, Psionic Trap, Swarm beetles, dummies [PD-Neutrals]. The Dota 2 port adds wards to the blockers [D2W13-LaneCreeps].
- **Stacking.** Stacking follows from the spawn rule: pull a camp out of its box before XX:00 and a fresh camp spawns alongside it. Leash time: the Dota 2 port says aggroed neutrals follow for 7 s before returning [D2W13-LaneCreeps]. **UNVERIFIED** for DotA 1.
- **6.79 changes.** Non-ancient neutrals split XP with all heroes in range. A camp never spawns the same set twice in a row. XP bounties of the Satyr Soulstealer, Satyr Hellcaller, Mud Golem and Dark Troll Warlord were reduced [CL-6.79].

| Camp tier | Possible groups (one is chosen at random) [PD-Neutrals], [PD-Units] |
|---|---|
| Easy | Kobold x3 + Kobold Tunneler + Kobold Taskmaster. Or Gnoll Assassin x3. Or Forest Troll Berserker x2 + Forest Troll High Priest. Or Harpy Scout x2 + Harpy Storm. Or Forest Troll Berserker x2 + Kobold Taskmaster. Or Fel Beast x2 + Ghost. |
| Moderate | Ogre Mauler x2 + Ogre Magi. Or Giant Wolf x2 + Alpha Wolf. Or Mud Golem x2. Or Satyr Trickster x2 + Satyr Soulstealer x2. Or Centaur Outrunner + Centaur Khan. |
| Hard | Centaur Outrunner + Centaur Khan. Or Dark Troll x2 + Dark Troll Warlord. Or Polar Furbolg Champion + Polar Furbolg Ursa Warrior. Or Satyr Trickster + Satyr Soulstealer + Satyr Hellcaller. Or Wildkin x2 + Enraged Wildkin. |
| Ancient | Black Drake x2 + Black Dragon. Or Jungle Stalker x2 + Elder Jungle Stalker. Or Blue Dragonspawn Sorcerer + Blue Dragonspawn Overseer x2. Or Rock Golem x2 + Granite Golem. Or Thunder Lizard (big) + Thunder Lizard (small) x2 (from 6.69). |

Selected neutral stat blocks from [PD-Units]. Format: level/XP, HP, damage (type) and range, armor (type), MS, BAT, gold.
- Kobold: L1/25xp, 240, 10-11 (Normal) 100, 0 (Heavy), 270, 1.35, 7-9.
- Kobold Tunneler: L1/25, 325, 14-15, 1 (Medium), 270, 1.35, 17-19.
- Kobold Taskmaster: L2/41, 400, 24-27, 0 (Heavy), 330, 1.35, 23-29.
- Centaur Outrunner: L2/41, 550, 18-21, 2 (Heavy), 350, 1.35, 20-24.
- Centaur Khan: L5/119, 1100, 49-55 (Chaos), 4 (Heavy), 320, 1.65, 66-78. War Stomp: 250 AoE, 25 damage, 2 s hero stun / 3 s unit stun, 20 s CD, 100 mana.
- Gnoll Assassin: L2/41, 370, 30-36 (Pierce) 500, 1 (Medium), 270, 1.6, 21-29. Envenomed Weapon 2 dps.
- Ghost: L3/62, 500, 30-35 (Pierce) 400, 1 (Medium), 320, 1.0, 30-40. Frost Attack 20% slow 1.5 s.
- Ogre Mauler: L2/41, 850, 24-27, 1 (Heavy), 270, 1.35, 23-47.
- Ogre Magi: L3/62, 600, 24-27, 0 (Heavy), 270, 1.35, 43-61. Frost Armor +8 armor.
- Giant Wolf: L3/62, 500, 21-24, 1 (Heavy), 350, 1.45, 22-26. 20% x2 crit.
- Alpha Wolf: L4/88, 600, 30-33, 3 (Heavy), 350, 1.35, 37-45. Command Aura +30% damage, 900 AoE. Crit.
- Mud Golem: L5/119, 800, 29-33, 4 (Heavy), 270, 1.35, 54-62.
- Satyr Trickster: L2/41, 240, 7-10 (Pierce) 600, 0 (Heavy), 300, 1.8, 15-17. Purge.
- Satyr Soulstealer: L4/88, 600, 24-27, 1 (Heavy), 270, 1.35, 27-33. Mana Burn 100.
- Satyr Hellcaller: L6/155, 1100, 49-55 (Chaos), 0 (Heavy), 290, 1.35, 97-111. Shockwave 100 damage, Unholy Aura +3 HP/s.
- Dark Troll: L3/62, 500, 24-27 (Pierce) 500, 0 (Heavy), 270, 1.35, 26-33.
- Dark Troll Warlord: L6/155, 1100, 49-54 (Pierce) 500, 1 (Heavy), 320, 1.35, 54-62. Ensnare 1.5 s, Raise Dead.
- Polar Furbolg Champion: L4/88, 950, 39-44 (Chaos), 5 (Heavy), 320, 1.35, 60-70.
- Polar Furbolg Ursa Warrior: L5/119, 1100, 49-55 (Chaos), 4, 320, 1.35, 76-88. Thunder Clap 150 damage, 300 AoE, 25% slow.
- Wildkin: L1/25, 350, 20-25, 2 (Heavy), 300, 1.35, 15-20.
- Enraged Wildkin: L5/119, 950, 50-56 (Chaos), 4, 320, 1.35, 67-87. Toughness Aura +3 armor, Tornado.
- Forest Troll Berserker: L2/41, 500, 28-37 (Pierce) 500, 1 (Medium), 270, 1.6, 22-26.
- Forest Troll High Priest: L2/41, 450, 25-32 (Pierce) 600, 0 (Medium), 290, 1.8, 21-25. Heal 15.
- Harpy Scout: L2/41, 400, 28-37 (Pierce) 300, 1 (Medium), 280, 1.6, 24-27.
- Harpy Storm: L3/62, 550, 30-37 (Pierce) 450, 2 (Medium), 310, 1.6, 33-37. Chain Lightning 140.
- Fel Beast: L2/41, 400, 14-15, 1 (Heavy), 350, 1.5, 20-23.
- Ancients (type "Ancient"):
  - Black Drake: L3/62, 950, 34-45 (Pierce) 300, 2, 350, 1.8, 44-56.
  - Black Dragon: L6/155, 2000, 48-81 (Chaos, splash), 6, 300, 1.5, 164-234. Spell Immunity.
  - Jungle Stalker: L5/119, 1300, 46-64, 2, 320, 1.35, 52-70.
  - Elder Jungle Stalker: L6/155, 2000, 56-78 (Chaos), 4, 320, 1.35, 152-170.
  - Blue Dragonspawn Sorcerer: L3/62, 675, 33-38, 0, 270, 1.35, 74-82. 15% evasion.
  - Blue Dragonspawn Overseer: L5/119, 1200, 59-65 (Chaos), 1, 300, 1.35, 86-98. Evasion, +3 armor aura.
  - Rock Golem: L5/119, 800, 29-33, 4, 270, 1.35, 54-62.
  - Granite Golem: L6/155, 2000, 77-87 (Chaos), 8, 270, 1.35, 107-121.
  - Thunder Lizard stats: not captured. UNVERIFIED.

### 2.5 Roshan and Aegis ([PD-Roshan], 6.72/6.73)

| Stat | Value |
|---|---|
| Level / XP | 30 / 1789 |
| HP / regen | 7500 / 20. **+500 HP every 5 min** from the first creep spawn, to 12000 at 45:00. |
| Damage | 65 (Chaos), melee 128 range. +10 every 5 min, to 155. A separate air attack is used only on flyers. |
| Armor | 3 (Heavy) plus a Devotion-style aura: +0.5 at start, +0.5 every 5 min, max +5 (8 total). |
| MS / BAT | 270 / 1.0. Damage point/backswing 0.3/0.3. |
| Vision | 1400/1400 |
| Bounty | 105-600 gold |
| Spell Block | Blocks one targeted spell every 15 s |
| Spell resistance | 75% |
| Bash | 15% chance: +50 damage, 1.65 s stun |
| Slam | 250 AoE, 70 damage, attack/move slow 2 s on heroes (4 s on units). Cast only when 3 or more units are near. |
| Illusions | Destroyed instantly when they attack Roshan |
| Respawn | Exactly **10 min** after death (6.72/6.73) [PD-Roshan]. **6.79: random 8-11 min** [CL-6.79]. Cannot be blocked by standing in the pit. |
| Aegis | Always dropped. Cannot be dropped or given away once picked up. On death, the holder revives 5 s later at the tombstone with full HP and mana [PD-Roshan]. 6.55: revive where you died, undroppable, Roshan reclaims it when he respawns [CL-6.55]. 6.75: reclaim time 10 → 6 min [CL-6.75]. 6.79: when it expires unused it heals the holder fully over 5 s [CL-6.79]. |
| Cheese | Dropped from the 3rd Roshan kill onward. Shareable, sells for 500. Instantly restores 2500 HP and 1000 mana [PD-Roshan]. |
| Growth | Since 6.55 Roshan gets stronger over time instead of after each death [CL-6.55]. 6.76: Roshan no longer has or needs mana [CL-6.76]. |

### 2.6 Runes and Bottle

- **Spawn timing.** One rune spawns every even minute from the first creep spawn (0:00, 2:00, ...) at one of the 2 river spots, and only if no rune is already sitting at either spot [PD-Runes], [PD-RuneGuide].
- **5 types** [PD-Runes]:

| Rune | Effect (6.72/6.73) |
|---|---|
| Double Damage | +100% of base damage (base, base attributes and bonus attributes; not raw +damage items) for **45 s**. Also applies to your illusions within 500. |
| Haste | +100% MS, which means the maximum **522**, for **30 s** |
| Illusion | 2 illusions (2 since 6.58 [CL-6.59]; 1 before). They deal **35%** damage, take **300%** damage, and last **75 s**. |
| Invisibility | **45 s**, with a **2 s** fade during which you can still act. Attacking, casting or using items breaks it. The Dota 2 port uses a 1 s fade. |
| Regeneration | Restores up to 3000 HP and 2000 mana over **30 s**. Ends on any damage or when both HP and mana are full (6.60 [CL-6.60]). |

- **Bottle.** It stores a rune. Using a stored rune activates it and refills the bottle to 3/3 charges [PD-RuneGuide]. Buying a Bottle fills it to 2/3 (6.49 [CL-6.49]). Usage got a cooldown in 6.50 [CL-6.50]. Since 6.62 it auto-refills at the fountain if not fully used [CL-6.62]. A rune with no expiry inside the Bottle was treated as a bug ("captured rune not expiring", 6.53 [CL-6.53]). The DotA 1 expiry timer itself is UNVERIFIED; the Dota 2 port uses 2 min [D2W13-Runes]. Charge values are in section 3.
- **Mode -er (6.59).** In this experimental mode runes spawn on both sides every 3 min. It is not the default [CL-6.59].

### 2.7 Gold ([PD-Gold], 6.72/6.73)

| Rule | Value / formula |
|---|---|
| Team starting pool | 4265 gold, split among the players on the team (853 each with 5 players) |
| Effective start after buying a hero | **603** (853 − 250 hero cost). Random modes skip the 250 cost (−random is free). **6.79: 625**, and the random bonus was cut by 50 [CL-6.79]. |
| Hero purchase / -random str/agi/int | 250 / 100 |
| Periodic gold | **1 unreliable gold every 0.875 s** (8 gold per 7 s; 2 per tick in -em). 6.79 changelog: "1 per 0.6 seconds, up from 1 per 0.8 seconds" [CL-6.79]. The 0.875 vs 0.8 disagreement is unresolved for 6.6x-6.78. Income starts with the first creep wave (6.50 [CL-6.50]). |
| Reliable vs unreliable | Hero-kill bounty (including the AoE assist gold) is reliable; everything else is unreliable. Death loses only unreliable gold. Purchases spend unreliable gold first, buyback spends reliable first (added 6.68 [CL-6.68]). |
| Hero kill (killer) | **200 + 5 × victim level + victim's streak value**. 6.75: changed to 200 + 9 × level [CL-6.75]. |
| Streak value of victim | 0 for fewer than 3 kills. Killing Spree (3) 50, Dominating (4) 100, Mega Kill (5) 150, Unstoppable (6) 200, Wicked Sick (7) 250, Monster Kill (8) 300, Godlike (9) 350, Beyond Godlike (10+) 400. 6.78: ending a streak pays 75 to 600 [CL-6.78]. 6.79: 125 to 1000 [CL-6.79]. |
| First Blood | +200 reliable to the killer |
| AoE assist gold (non-killers within 1000) | 1 hero: 95 + 9 × level. 2 heroes: 20 + 8L each. 3: 10 + 6L. 4 or 5: 6L. 6.75 retuned these (for example "kills with 2 heroes around … from 9*Level+95 to 12*Level+125") [CL-6.75]. The 6.75 wording counts heroes differently from playdota, so the exact mapping is UNVERIFIED. 6.78: the AoE became 1200 [CL-6.78]. |
| Kills by creeps or towers | With 1 assister, the kill goes to that hero. With several, the gold is split among the assisters, not the whole team. |
| Death gold loss | **30 × your level** of unreliable gold. Each Bloodstone charge reduces it by 25. |
| Sell back | 50% of item price; 80% for recipes |
| Buyback cost | **100 + level² × 1.5 + game-time-minutes × 15**, rounded down to a multiple of 50 [PD-Gold]. Done at the Circle of Power (6.55). Cost scaling reworked in 6.64/6.65. **Cooldown 4 min (6.70)** [CL-6.70], **5 min (6.73)** [CL-6.73]. 6.79: no unreliable gold until the normal respawn would have ended, and 25% of the remaining respawn time is added to the next death [CL-6.79]. |
| Creep and neutral bounty | See the tables above. Gold goes only to the last hitter. A denied creep gives no gold. |

### 2.8 Experience ([PD-XP], 6.72/6.73)

- **XP radius.** Normal XP is shared equally among allied heroes within **1200** (1000 before 6.69 [CL-6.69]; **1300 from 6.79** [CL-6.79]). A hero kill outside the radius still gives XP to the killer. Heroes keep taking a share at level 25. Buildings give XP.
- **Deny XP.** A denied unit gives the enemies **36/n XP to each melee hero and 18/n to each ranged hero** within range, where n is the number of enemy heroes in range. That range was 1000 → 1200 in 6.78 [CL-6.78]. 6.79: ranged heroes get the same as melee [CL-6.79].
- **Summons** give 50% XP. **-em** gives 150%.
- **Bonus hero-kill XP** (allies of the killer within 1000, by number of heroes):
  - 1: 120 + 20L
  - 2: 90 + 15L
  - 3: 30 + 7L
  - 4: 15 + 6L
  - 5: 10 + 5L
  - 6.78 changed 4 heroes to 20 + 5L and 5 heroes to 15 + 4L [CL-6.78].
- **Formulas.**
  - XP to reach level x: f(x) = f(x−1) + 100x, with f(2) = 200.
  - Hero kill XP: 100, 120, 160, 220, 300, then +100 per level.
  - Unit XP by unit level: f(1) = 25, f(x) = f(x−1) + 5x + 6.

| Lvl | Total XP | Hero-kill XP | Unit-kill XP (by unit level) |
|---|---|---|---|
| 1 | 0 | 100 | 25 |
| 2 | 200 | 120 | 41 |
| 3 | 500 | 160 | 62 |
| 4 | 900 | 220 | 88 |
| 5 | 1400 | 300 | 119 |
| 6 | 2000 | 400 | 155 |
| 7 | 2700 (2600 from 6.75) | 500 | 196 |
| 8 | 3500 (3200 from 6.75) | 600 | 242 |
| 9 | 4400 | 700 | 293 |
| 10 | 5400 | 800 | 349 |
| 11 | 6500 (6000 from 6.78) | 900 | 410 |
| 12 | 7700 (8200 from 6.78) | 1000 | 476 |
| 13 | 9000 | 1100 | 547 |
| 14 | 10400 | 1200 | 623 |
| 15 | 11900 | 1300 | 704 |
| 16 | 13500 | 1400 | 790 |
| 17 | 15200 | 1500 | 881 |
| 18 | 17000 | 1600 | 977 |
| 19 | 18900 | 1700 | 1078 |
| 20 | 20900 | 1800 | 1184 |
| 21 | 23000 | 1900 | 1295 |
| 22 | 25200 | 2000 | 1411 |
| 23 | 27500 | 2100 | 1532 |
| 24 | 29900 | 2200 | 1658 |
| 25 | 32400 | 2300 | 1789 |

Level changes are from [CL-6.75] and [CL-6.78]. The level cap is 25 (the table ends there, and Roshan, a level-30 unit, gives 1789, the level-25 unit value [PD-XP], [PD-Roshan]).

### 2.9 Respawn and death

- **Respawn time = 4 × hero level** seconds (4 s at level 1, 100 s at level 25). Dota 2 6.83b changed it to 5 + 3.8 × level and quoted "4*Level" as the old formula ([GosuGamers 6.83b](https://www.gosugamers.net/dota2/news/29587-6-83b-launched-with-minor-nerfs-and-respawn-time-changes)). That is the Dota 2 side of 6.83 parity. A DotA-1-only statement of 4 × level is **UNVERIFIED**, but it is consistent with the 6.83b "old" table.
- **-fr** halves respawn time (6.55 [CL-6.55]). Bloodstone reduces it by 3 s per charge (4 s later) [CL-6.73].
- Heroes respawn with full mana from 6.73 [CL-6.73]. Couriers respawn 3 min after death (6.73) [CL-6.73].

### 2.10 Attributes, HP/mana, damage, armor

| Rule | Value | Source |
|---|---|---|
| STR | +19 max HP and +0.03 HP/s per point | [PD-Attr], [PD-HPMP] |
| AGI | +0.14 armor (1/7) and +1 IAS per point | [PD-Attr], [PD-AS] |
| INT | +13 max mana and +0.04 mana/s per point | [PD-Attr], [PD-HPMP] |
| Primary attribute | +1 attack damage per point | [PD-Attr] |
| Base HP / base mana (every hero) | **150 / 0** | [PD-HPMP] |
| Base HP regen | 0.25 for most heroes. Exceptions: Axe 1.25. Gondar, Faceless Void, Juggernaut, Krobelus, Naga Siren, Spiritbreaker, Stealth Assassin, Venomancer, Brewmaster and Phantom Lancer 0.75. Shadow Fiend and Lone Druid 0.5. | [PD-HPMP] |
| Base mana regen | 0.01 for all heroes (Techies 0.02) | [PD-HPMP] |
| Mana regen formula | (base + INT × 0.04) × (1 + sum of % bonuses) + flat Brilliance Aura | [PD-HPMP] |
| Attribute Bonus ("stats") | A 5th skill on every hero except Invoker, +2 to all attributes per level. Max level **10** is inferred: 25 levels − (4+4+4+3) = 10. UNVERIFIED as stated text. | [PD-Attr] |
| Ability levels | 3 normal skills × 4 levels. The ultimate has 3 levels, available at hero level **6/11/16**. One skill point per level. | ["What is DotA?" (UNBC)](https://web.unbc.ca/~cchan/about.htm). The per-hero tables in section 4 confirm 4/4/4/3 levels. |
| Armor reduction | **(0.06 × armor) / (1 + 0.06 × armor)**. Each armor point adds 6% effective HP. | [PD-DA], [BNet] |
| Negative armor | Damage increase = **1 − 0.94^(−armor)**, capped at −20 armor (71%) | [PD-DA]. The Blizzard page writes it as 2 − 0.94^(−armor), i.e. the multiplier [BNet]. |
| Which damage armor reduces | Only damage-type "Normal" (attacks and a few spells). "Magic" damage ignores armor and is blocked completely by magic immunity. "Universal" ignores armor and is NOT blocked by magic immunity. Most spells are attack-type "Spells" with damage-type "Magic". | [PD-DA] |
| % damage bonuses | Command Aura, Double Damage and similar count only base damage plus primary attribute, never raw +damage | [PD-DA], [PD-Runes] |

**Attack type vs armor type (DotA 6.72/6.73)** [PD-DA]:

| Attack \ Armor | Light | Medium | Heavy | Fortified | Hero | Unarmored |
|---|---|---|---|---|---|---|
| Normal | 100% | 150% | 125% | 70% | 75% | 100% |
| Pierce | 200% | 75% | 75% | 35% | 50% | 150% |
| Siege | 100% | 50% | 125% | 150% | 75% | 100% |
| Chaos | 100% | 100% | 100% | 40% | 100% | 100% |
| Hero | 100% | 100% | 100% | 50% | 100% | 100% |
| Spells | 100% | 100% | 100% | 100% | **75%** | 100% |

- **Heroes' innate 25% magic resistance** is the "Spells vs Hero 75%" cell [PD-DA]. The Dota 2 port states it outright [D2W13-Armor].
- **Disagreements and history.**
  - The table differs from stock Warcraft III (TFT: Normal→Hero 100%, Siege→Hero 50%, Chaos→Fort 100%, Spells→Hero 70%) [BNet]. DotA edits the gameplay constants, so do not use the stock table.
  - Chaos vs Fortified was 100% → 50% (6.28) → 40% (6.39) [D2W-ArmorCL].
  - Pierce vs Heavy was changed 75%→100% (6.44) per one archive, or 100%→75% per another [D2W-ArmorCL], [D2W16-DamageTypes]. Playdota's 6.72/6.73 table shows **75%**; use that.
  - Siege vs Hero went from 75% to 85% only in 6.84 (Dota 2) [D2W-ArmorCL]. Keep 75%.
- **Who uses what** [PD-Units], [PD-DA]:
  - Heroes: Hero attack / Hero armor.
  - Towers: Siege attack / Fortified armor.
  - Barracks and Ancients: Fortified armor.
  - Fountain: Chaos attack.
  - Melee lane creeps: Normal attack. Ranged lane creeps: Pierce attack. Lane creep armor: see the 2.3 disagreement.
  - Siege creeps: Siege attack / Fortified armor.
  - Neutrals: Normal, Pierce or Chaos attack, mostly Heavy or Medium armor (listed above).
  - Roshan: Chaos attack / Heavy armor.
  - Illusions: deal and take the percentages listed per illusion source [PD-Illusions].
- **Cleave** is reduced by armor type but not armor value. It does not apply orbs (except crit). Multiple cleaves stack independently [PD-Cleave].

### 2.11 Attack speed, animation, movement, turning, collision, vision

| Rule | Value | Source |
|---|---|---|
| Attack rate | **Attacks/s = (1 + IAS/100) / BAT**. IAS from all sources is additive and clamped to **[−80, +400]**. | [PD-AS] |
| Base attack time | **1.7 for almost all heroes** (see the hero tables). Creeps 1.0, siege 2.7, towers 1.0/0.95, Roshan 1.0. | [PD-AS], [PD-Units] |
| Attack animation | Each attack has a damage point (when damage lands or the projectile leaves) and a backswing. The backswing "can and should be cancelled" by a new order right after the damage point. | [PD-Anim] |
| Cast animation | Spells have a cast point and cast backswing. Interrupting during the cast point costs no mana and no cooldown. Channelling spells spend mana and cooldown at the start. | [PD-Target], [PD-Cool] |
| Projectile spells | Can be dodged by blinking or going invisible before impact. Forced movement does not dodge them. | [PD-Target], [PD-Invis] |
| Movement speed | Min **100**, max **522** (Haste rune = 522) | [D2W13-MS] (Dota 2 port), [PD-Runes], [CL-6.62] |
| MS stacking | Boots bonuses do not stack with other boots. Yasha-line % bonuses do not stack with each other. Different sources stack: (base + flat) × (1 + Σ%). | [D2W13-MS] (Dota 2 port) |
| Turn rate | Expressed in radians per 0.03 s. Hero values are mostly 0.4-0.6 in the 2013 Dota 2 port, which inherited DotA values with later tweaks. Example: Earthshaker 0.4 before 6.78 (see the hero section). DotA 1 changelogs tuned turn rates individually (6.58 Templar Assassin, Roshan). Per-hero DotA 1 values: UNVERIFIED except where the hero tables cite them. | [D2W13-TurnRate], [CL-6.59] |
| Collision size | **UNVERIFIED for DotA 1.** Dota 2 heroes are 24, with a few at 8. WC3 heroes use the editor's per-unit collision size. Read it from the .w3x. | [D2W-Collision] |
| Vision | Most heroes see 1800 by day and 800 by night. Exceptions (6.72/6.73): Enchantress 1800/1800, Sniper 1800/1000, Treant 1800/1200, Bounty Hunter 1800/1000, Alchemist 1800/1400, Priestess of the Moon 1800/1200, Faerie Dragon 1800/1200, Tormented Soul 1800/900, Night Stalker 1200/1800, Geomancer 1800/1800, Ancient Apparition 1800/1400, Nightcrawler 1800/1800, Drow 1800/1700. 6.79 moved most of these to 800 at night. Flying units' vision is unobstructed. | [PD-Terrain], [CL-6.79] |
| True Sight | Towers, the Ancient and the fountain (900 AoE). Sentry Wards. Gem of True Sight (on a courier it loses true sight, 6.69). Dust of Appearance: 1250 AoE, 12 s. | [PD-Terrain], [PD-Units], [CL-6.69] |
| Day/night | Before 6.68: UNVERIFIED (6.68 made the cycle "25% faster", which implies 8 + 8 min). 6.68-6.78: **6 min day + 6 min night**, starting at day. 6.79: 4 + 4 min. Darkness and Eclipse pause the clock. | [PD-Time], [CL-6.68], [CL-6.79] |
| Invisibility | Permanent invisibility does not auto-attack. Wind Walk lets you pass through units. Invisibility is broken or prevented by silence, pause and hex in some cases. | [PD-Invis] |

### 2.12 Denying, last hits, aggro

- **Denying creeps.** You may attack your own creeps only when they are below **50% HP**. This rule dates from Guinsoo-era DotA Allstars **5.61** ("You may only attack friendly creeps with HP less than 50%", [Dota 2 wiki 5.61 changelog](https://dota2.fandom.com/wiki/5.61), via search summary because fandom blocked the direct fetch). A denied creep gives its enemies only the reduced deny XP and no gold [PD-XP].
- **Denying heroes.** Only under specific debuffs. The Dota 2 port lists Doom, Shadow Strike and Venomous Gale [D2W13-Denying]. 6.58 fixed a bug that let you deny an ally healed by Shadow Word [CL-6.59]. The DotA 1 list is UNVERIFIED.
- **Creep aggro.** If you issue an attack order on an enemy hero, nearby enemy creeps switch to attacking you. Spell casts do not draw aggro, and neither do manually cast arrow "orbs" such as Drow's or Viper's [PD-Lane]. The aggro radius and duration in DotA 1 are **UNVERIFIED**. Lane creeps prioritise enemy creeps, then heroes, then siege [D2W13-Aggro] (Dota 2 port).

### 2.13 Inventory, items, courier, shops (mechanics only; the item data is in section 3)

- **6 inventory slots** ("you only can carry 6 items at a time") [PD-DA].
- **No stash.** From 6.55, items bought in base with a full inventory drop on the floor at your Circle of Power [CL-6.55]. Couriers carry items. From 6.60 they have "Transfer Items" and "Return Home" [CL-6.60]. 6.57: the courier costs 200 [CL-6.57]. 6.74: couriers are invulnerable near their fountain [CL-6.74].
- **Item sharing** is disabled by default (enabled in -pm).
  - Fully shareable: consumables, namely Clarity, Tango, Salve, Cheese, TP, Dust, Observer and Sentry wards, and the couriers.
  - Partially shareable (usable, but not sellable or combinable): Gem, Ring of Regen, Sobi Mask, Ring of Health, Void Stone, Perseverance, Bottle, Divine Rapier.
  - Everything else, including all recipes and recipe-made items except the Flying Courier, is unusable by others.
  - Some items are disabled on couriers (Dominate, Unholy Strength, Arctic Blast, Gem's true sight).

  [PD-Share]
- **Cooldowns.** Copies of the same item share a cooldown, and the Dagon levels share one too. A cooldown cannot be partly reduced; it can only be fully refreshed (Refresher) [PD-Cool].
- **Smoke of Deceit EXISTS in DotA 1.** It was added in **6.70**: 100 gold, stock 3, restocks every 10 min, gives invisibility, +10% MS and minimap hiding to allied units within 900 [CL-6.70]. The brief's assumption that "smoke is not in DotA 1" holds only for versions before 6.70.

### 2.14 Disables and spell immunity ([PD-Disables], [PD-DA])

| Disable | Behaviour |
|---|---|
| Stun | Prevents all actions |
| Entangle | Like a stun, but the target can still turn and cast |
| Ensnare | Prevents movement only |
| Silence | Prevents spells; items still usable |
| Slow | Reduces movement and/or attack speed |
| Hex | Sheep: the target can only move |
| Shackle | A stun that the caster must channel |
| Cyclone | Fully disabled and invulnerable |
| Sleep | Ends on damage |
| Ethereal | Cannot attack or be attacked; amplifies magic damage taken |
| Doom | Silence plus muted items plus some passives disabled |

- **Stacking.** The same bash or stun does not stack with itself in many cases. Evasion stacks diminishingly from 6.79 [CL-6.79]. Crit and spell resistance stack diminishingly [CL-6.79].
- **Magic immunity** blocks Magic-type damage and most spells. Universal damage passes through it [PD-DA]. Example: from 6.72, Finger of Death and Laguna Blade can no longer target magic-immune units [CL-6.72].

### 2.15 Game modes (brief, [PD-Cmd])

Modes are entered on one line by the host, e.g. `-apemomsc`.

| Mode | Meaning |
|---|---|
| **-ap** | All Pick (any tavern) |
| **-ar** | All Random |
| **-rd** | Random Draft (20 heroes; 22 from 6.75 [CL-6.75]) |
| **-sd** | Single Draft (choice of 3 random heroes; 6.50) |
| **-cd** | Captains Draft |
| **-cm** | Captains Mode (6.55) |
| **-em** | Easy Mode: weaker towers, more gold and XP |
| **-dm** | Death Match |

Others:
- -rs (Random Side)
- -du (Duplicate)
- -sp (Shuffle Players)
- -xl (Extended League)
- -om (Only Mid)
- -sc (Super Creeps)
- -lm (League Mode)
- -mm (Mirror Mode)
- -tr (Team Random)
- -mr (Mode Random)
- -vr (Vote Random)
- -rv (Reverse)
- -sh (Same Hero)
- -aa/-ai/-as (All Agility/Intelligence/Strength)
- -id (Item Drop)
- -np (No Powerups)
- -nt/-nm/-nb (No Top/Mid/Bot)
- -ns (No Swap)
- -nr (No Repick)
- -pm (Pooling Mode)
- -mi (Mini Heroes)
- -fr (Fast Respawn)
- -mo/-ro (Melee/Ranged Only)
- -er (Experimental Runes)
- -so (Switch On)
- -zm (Zoom Mode)
- -cp (Capture Point)
- -wtf (no cooldowns or mana costs)
- -tt (TagTeam, 6.76)

Default "normal mode" restricts picks to your faction's taverns (plus neutral taverns from 6.60) [PD-Factions], [CL-6.60].

#### Reference keys for section 2
- [PD-Attr]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/Attributes
- [PD-AS]: https://web.archive.org/web/20111215054858/http://www.playdota.com:80/mechanics/attackspeed
- [PD-DA]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/damagearmor
- [PD-Gold]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/Gold
- [PD-XP]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/experience
- [PD-Buildings]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/buildings
- [PD-Units]: https://web.archive.org/web/20111215055159/http://www.playdota.com:80/mechanics/unitstats
- [PD-Roshan]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/roshan
- [PD-Runes]: https://web.archive.org/web/20111218233131/http://www.playdota.com:80/mechanics/runes
- [PD-Terrain]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/terrainvision
- [PD-Time]: https://web.archive.org/web/20111222030025/http://www.playdota.com:80/mechanics/timedevents
- [PD-Fountain]: https://web.archive.org/web/20111228074216/http://www.playdota.com:80/mechanics/fountains
- [PD-HPMP]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/Hitpoints_and_Manapoints
- [PD-Share]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/shareditems
- [PD-Disables]: https://web.archive.org/web/20120101120851/http://www.playdota.com:80/mechanics/disables
- [PD-Cool]: https://web.archive.org/web/20120101120841/http://www.playdota.com:80/mechanics/Cooldown
- [PD-Cleave]: https://web.archive.org/web/20111225172955/http://www.playdota.com:80/mechanics/physattackmods
- [PD-Invis]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/Invisibility
- [PD-Factions]: https://web.archive.org/web/2011/http://www.playdota.com:80/mechanics/factions
- [PD-Neutrals]: https://web.archive.org/web/2012/http://www.playdota.com/mechanics/neutralcreeps
- [PD-Illusions]: https://web.archive.org/web/2012/http://www.playdota.com/mechanics/illusions
- [PD-Target]: https://web.archive.org/web/2012/http://www.playdota.com/mechanics/Targeting
- [PD-Cmd]: https://web.archive.org/web/2011/http://www.playdota.com:80/learn/commands
- [PD-Lane]: https://web.archive.org/web/2011/http://www.playdota.com:80/learn/lanecontrol
- [PD-RuneGuide]: https://web.archive.org/web/2011/http://www.playdota.com:80/learn/runeinfo
- [PD-Anim]: https://web.archive.org/web/2011/http://www.playdota.com:80/learn/animations
- [CL-6.xx]: https://web.archive.org/web/2013/http://www.playdota.com/changelogs/6.xx (replace xx with the version, e.g. `/changelogs/6.79`). Each page shows its own version first, followed by earlier versions.
- [BNet]: http://classic.battle.net/war3/basics/armorandweapontypes.shtml
- [D2W13-Armor]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Armor
- [D2W-ArmorCL]: https://web.archive.org/web/2019/https://dota2.gamepedia.com/Armor/Changelogs
- [D2W16-DamageTypes]: https://web.archive.org/web/2016/https://dota2.gamepedia.com/Damage_types
- [D2W13-Aggro]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Aggro
- [D2W13-Buildings]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Backdoor_protection
- [D2W13-Glyph]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Glyph_of_Fortification
- [D2W13-Denying]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Denying
- [D2W13-Runes]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Runes
- [D2W13-MS]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Movement_speed
- [D2W13-TurnRate]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Turn_rate
- [D2W13-LaneCreeps]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Lane_creeps
- [D2W-Collision]: https://web.archive.org/web/2013/http://dota2.gamepedia.com/Collision_size (the archive resolved to a later, post-2016 snapshot)

## 3. Items

Compiled from playdota's item database (three snapshots, 6.6x to 6.72/6.73). The baseline column in this section is **6.6x [A]**, with the 6.72/6.73 column [C] next to it. Use [C] if the game is pinned to 6.72/6.73 as §2 recommends.

### DotA Allstars items (6.6x baseline, with 6.7x deltas)

**How this was sourced.** Every number below comes from playdota.com's item database (the site IceFrog ran for the map), read from its data file `cache/items.js` in three Wayback Machine snapshots:

- **[A]** 2010-01-16: https://web.archive.org/web/20100116065848/http://www.playdota.com/cache/items.js?242=242. This is the 6.6x baseline. Liquipedia dates 6.67 to 2010-03-24, so this snapshot reflects 6.65 or 6.66. Which of the two it is: UNVERIFIED.
- **[B]** 2010-12-29: https://web.archive.org/web/20101229100347/http://www.playdota.com/cache/items.js?36=36. This is after 6.69c (2010-11-05). Whether it is 6.69c or 6.70: UNVERIFIED.
- **[C]** 2011-12-29: https://web.archive.org/web/20111229165805/http://www.playdota.com/cache/items.js?117=117. This is the 6.72/6.73 era. The exact version is UNVERIFIED.

Human-readable page that loads the same data: https://web.archive.org/web/20111229165805/http://www.playdota.com/items (one page per item at `playdota.com/items/<slug>`).

Version dates come from the Liquipedia search snippet for https://liquipedia.net/dota2gameru/Версии_игры/0.00_до_6.69c (6.67 2010-03-24, 6.68 2010-07-26, 6.69 2010-10-11, 6.69c 2010-11-05). A direct fetch was rate-limited.

Unless a cell says otherwise, values are **[A]**. The playdota text is transcribed close to word for word, including its inconsistent capitalisation.

### Shops

playdota uses the **Sentinel** shop names. The Scourge base has a mirror shop for each one with the same stock but a different name. The Scourge-side shop names are **UNVERIFIED**: no source found in this pass. Note: "Leragas The Vile" is the **Secret Shop**, not a Scourge consumables shop [M2].

| Shop (playdota name) | Kind | What it sells [A] |
|---|---|---|
| Ancient of Wonders | Base shop, consumables | Tango, Healing Salve, Clarity, Empty Bottle, Observer and Sentry Wards, Dust, Animal Courier, Town Portal. Flying Courier recipe moved here from Supportive Vestments by [C]. Smoke of Deceit by [B]. |
| Weapons Dealer | Base shop, basic weapons and armor | Blades of Attack, Broadsword, Quarterstaff, Claymore, Ring of Protection, Stout Shield, Javelin, Mithril Hammer, Chain Mail, Helm of Iron Will, Plate Mail, Quelling Blade |
| Sena The Accesorizer (sic) | Base shop, attribute items | Gauntlets, Mantle, Slippers, Ironwood Branch, Belt, Robe, Boots of Elvenskin, Circlet, Ogre Axe, Staff of Wizardry, Blade of Alacrity, Ultimate Orb |
| Cache of Quel-thelan | Base shop, misc and utility | Gloves of Haste, Mask of Death, Ring of Regeneration, Kelen's Dagger, Sobi Mask, Boots of Speed, Gem of True Sight, Planewalker's Cloak, Magic Stick, Talisman of Evasion, Ghost Scepter. Shadow Amulet by [C]. |
| Marketplace = **Leragas The Vile** (the Secret Shops) | Secret shop. playdota "Shops and Items" [M2]: "the Leragas The Vile shops, these also sell items that do not require any recipe to make, but they are generally much more expensive and powerful"; there are "6 recipe item shops" and "2 Goblin Shops" that "sell a combination of base items from the base and from the Leragas The Vile shops". The items.js database labels this shop "Marketplace". | Demon Edge, Eaglehorn, Messerschmidt's Reaver, Sacred Relic, Hyperstone, Ring of Health, Void Stone, Mystic Staff, Energy Booster, Point Booster, Vitality Booster. Orb of Venom by [C]. This matches the list in the request exactly. |
| Gateway Relics | Recipe shop | Power Treads, Perseverance, Oblivion Staff, Phase Boots, Boots of Travel, Poor Man's Shield, Magic Wand, Hand of Midas, Null Talisman, Bracer, Wraith Band. Soul Ring by [C]. |
| Supportive Vestments | Recipe shop | Headdress, Nathrezim Buckler, Ring of Basilius, Pipe, Mekansm, Vladmir's, Urn of Shadows, Arcane Ring, Flying Courier. By [C]: Arcane Boots, Tranquil Boots, Ring of Aquila, Drum (Ancient Janggo), Medallion. |
| Protectorate | Recipe shop | BKB, Assault Cuirass, Shiva's, Bloodstone, Blade Mail, Soul Booster, Hood of Defiance, Vanguard, Linken's, Heart, Aegis (listed here, but it drops from Roshan). Manta moved here by [C]. |
| Arcane Sanctum | Recipe shop | Guinsoo, Orchid, Eul's, Force Staff, Dagon, Necronomicon, Aghanim's, Refresher. By [C]: Rod of Atos, Veil of Discord. |
| Enchanted Artifacts | Recipe shop | Sange, Yasha, S&Y, Mask of Madness, Helm of the Dominator, Satanic, Diffusal, Maelstrom, Mjollnir, Desolator, Eye of Skadi. Heaven's Halberd by [C]. |
| Ancient Weaponary (sic) | Recipe shop | MKB, Radiance, Butterfly, Buriza, Cranium Basher, Battle Fury, Crystalys, Armlet, Lothar's Edge, Divine Rapier. Manta is here in [A] and moves to Protectorate by [C]. Ethereal Blade by [B]. Abyssal Blade by [C]. |
| Side shops (Goblin Merchant, Goblin Laboratory) | Neutral lane shops | These are not separate catalogues. playdota marks individual items "Buyable from Goblin Merchant/Laboratory" (column in the table below). In [A], Ring of Health and Energy Booster are both Secret Shop and Goblin Laboratory items. |

**Stock and restock [A].** Observer Wards: "Maximum stock of 2 in store | Store stock takes 6 minutes to replenish". By [C]: "Initial stock of 1". Sentry Wards: no stock text in [A], [B] or [C], so a restock value is UNVERIFIED. Gem of True Sight [C]: "Maximum stock of 1 in store | Store stock takes 10 minutes to replenish". [A] gives no stock for Gem.

**Name mapping (DotA 1 to DotA 2), from general knowledge only, not looked up (UNVERIFIED):**

| DotA 1 | DotA 2 |
|---|---|
| Kelen's Dagger | Blink Dagger |
| Buriza-do Kyanon | Daedalus |
| Stygian Desolator | Desolator |
| Guinsoo's Scythe of Vyse | Scythe of Vyse |
| Messerschmidt's Reaver | Reaver |
| Ancient Tango of Essifation | Tango |
| Empty Bottle | Bottle |
| Nathrezim Buckler | Buckler |
| Headdress of Rejuvenation | Headdress |
| Khadgar's Pipe of Insight | Pipe of Insight |
| Planewalker's Cloak | Cloak |
| Ancient Janggo of Endurance | Drum of Endurance |
| Lothar's Edge | Shadow Blade (later DotA 2 also has Silver Edge) |
| Cranium Basher | Skull Basher |
| Arcane Ring | removed; Arcane Boots replaced it |

### Notes on mechanics

These come from the item text in [A]/[C] except where marked.

- **Orb effects.** playdota marks these as "Orb effect":
  - Mask of Death, Mask of Madness, Helm of the Dominator and Satanic lifesteal
  - Sange, Yasha and S&Y maim
  - Maelstrom and Mjollnir Chain Lightning
  - Stygian Desolator Corruption
  - Diffusal Blade Feedback
  - Eye of Skadi (melee only; ranged Skadi is a "buff placer")
  - [C] also has Orb of Venom

  By [C] some are reworded. Maelstrom and Mjollnir: "Orb effect on regular attacks, orb effect and buff placer on proc". Desolator: "Orb effect and buff placer". Sange [C] no longer says "Orb effect". The general rule is as stated on playdota "Orb Effects" (snapshot 2011-12-27, [M1]): "Orb Effects do not stack (you will only get the effect of one 'orb' skill)" and "You can only place one buff per attack". When orbs conflict, the winner depends on the pair: "Place" means the item in the highest-priority inventory slot wins (Top Left 1 > Top Right 2 > Mid Left 3 > Mid Right 4 > Bottom Left 5 > Bottom Right 6); "Latest" means the most recently acquired wins; some pairs work "Both"; Chain Lightning "Overrides on proc". Example [M1] (melee): Lifesteal + Corruption -> "Place"; Lifesteal + Chain Lightning -> "Place"; Corruption + Chain Lightning -> "Overrides on proc". Cleave is "not considered Orb Effects or Buff Placers" but is an attack modifier. When two Critical Strikes proc together, only the last-acquired works. Two bashes on a melee hero both deal damage and the longer stun applies; on a ranged hero only the last-acquired bash works. Ranged-unit crits and bashes are buff placers [M1]. The full pair-by-pair matrix is on the page; its row and column labels did not survive text extraction, so only the rules above are transcribed.
- **Bottle.** Rune Capture is in [A]: "Stores the target rune in the Bottle… When a rune is used from the Bottle, the Bottle is refilled to 3/3 full | Stored runes are automatically used after 120 seconds | Can only be used when Bottle is empty | Bottle is not drop-able when it has a rune stored". By [C]: "Couriers move 30% slower when carrying empty bottles".
- **Black King Bar.** [A] and [B]: "Duration is 10/9/8/7/6/5 seconds, and cooldown is 80/75/70/65/60/55 seconds. After 5 uses it stays at 5 second duration and 55 second cooldown". [C] (6.7x): 10/9/8/7/6/5/4 s duration and 80/75/70/65/60/55/50 s cooldown, floor 4 s, plus "Cannot be sold".
- **Kelen's Dagger** is a basic item (2150) at Cache of Quel-thelan and the Goblin Merchant, and "cannot be held by Butcher or Vengeful Spirit".
- **Divine Rapier** [A]: drops on death and cannot be destroyed. If someone other than the owner picks it up, they cannot drop it manually. [C] adds the Original/Free Rapier states.
- **Aegis** [A]: removed when Roshan respawns, "10 minutes after he was killed". [C]: 6 minutes.


### Basic items table

Primary column = playdota.com items database, snapshot 2010-01-16 [A] (6.6x, pre-6.67). "Dec 2011" column = snapshot 2011-12-29 [C] (6.72/6.73 era). Stats are [A] unless noted. "Side shop" = the item text says "Buyable from Goblin Merchant/Goblin Laboratory" (the two neutral side shops by the lanes).

| Item | Shop (6.6x) | Cost 6.6x [A] | Cost Dec 2010 [B] | Cost Dec 2011 [C] | Stats / effect [A] | Side shop? | Builds into [A] |
|---|---|---|---|---|---|---|---|
| Ancient Tango of Essifation | Ancient of Wonders | 90 | 90 | 90 | Eat Tree (active) | - |  |
| Animal Courier | Ancient of Wonders | 200 | 170 | 150 | Summon Animal Courier (active) | - | Flying Courier |
| Clarity Potion | Ancient of Wonders | 50 | 50 | 50 | Regenerate Mana (active) | - |  |
| Dust of Appearance | Ancient of Wonders | 180 | 180 | 180 | Reveal (active) | - |  |
| Empty Bottle | Ancient of Wonders | 600 | 600 | 600 | Regenerate (active); Rune Capture (active) | - |  |
| Healing Salve | Ancient of Wonders | 100 | 100 | 100 | Regenerate Health (active) | - |  |
| Observer Wards | Ancient of Wonders | 200 | 200 | 150 | Place Ward (active) | - |  |
| Scroll of Town Portal | Ancient of Wonders | 135 | 135 | 135 | Teleport (active) | Goblin Laboratory |  |
| Sentry Wards | Ancient of Wonders | 200 | 200 | 200 | Place Ward (active) | - |  |
| Smoke of Deceit | Ancient of Wonders | n/a | 100 | 100 | (not in 6.6x snapshot; Dec 2011 data) Smoke of Deceit (active) | - |  |
| Blades of Attack | Weapons Dealer | 500 | 450 | 450 | +9 damage | Goblin Merchant | Armlet of Mordiggian, Crystalys, Dagon, Phase Boots |
| Broadsword | Weapons Dealer | 1200 | 1200 | 1200 | +18 damage | - | Battle Fury, Blade Mail, Crystalys |
| Chain Mail | Weapons Dealer | 550 | 550 | 550 | +5 armor | - | Assault Cuirass, Blade Mail, Nathrezim Buckler |
| Claymore | Weapons Dealer | 1400 | 1400 | 1400 | +21 damage | - | Battle Fury, Lothar's Edge |
| Helm of Iron Will | Weapons Dealer | 950 | 950 | 950 | +5 armor; +3 HP regeneration | Goblin Laboratory | Armlet of Mordiggian, Helm of the Dominator, Hood of Defiance |
| Javelin | Weapons Dealer | 1500 | 1500 | 1500 | +21 damage; 20% chance to deal 40 bonus damage | - | Cranium Basher, Monkey King Bar |
| Mithril Hammer | Weapons Dealer | 1600 | 1600 | 1600 | +24 damage | - | Black King Bar, Maelstrom, Stygian Desolator |
| Plate Mail | Weapons Dealer | 1400 | 1400 | 1400 | +10 armor | - | Assault Cuirass, Shiva's Guard |
| Quarterstaff | Weapons Dealer | 900 | 900 | 900 | +10 damage; +10 attack speed | Goblin Merchant | Force Staff, Lothar's Edge, Oblivion Staff, The Butterfly |
| Quelling Blade | Weapons Dealer | 225 | 225 | 225 | Quell (passive); Tree Chop (active) | Goblin Merchant |  |
| Ring of Protection | Weapons Dealer | 175 | 175 | 175 | +2 armor | - | Arcane Ring, Ring of Basilius |
| Stout Shield | Weapons Dealer | 250 | 250 | 250 | Damage Block (passive) | Goblin Laboratory | Poor Man's Shield, Vanguard |
| Belt of Giant Strength | Sena The Accesorizer | 450 | 450 | 450 | +6 Strength | Goblin Merchant | Cranium Basher, Necronomicon, Power Treads, Sange |
| Blade of Alacrity | Sena The Accesorizer | 1000 | 1000 | 1000 | +10 Agility | - | Aghanim's Scepter, Diffusal Blade, Yasha |
| Boots of Elvenskin | Sena The Accesorizer | 450 | 450 | 450 | +6 Agility | Goblin Merchant | Power Treads, Yasha |
| Circlet of Nobility | Sena The Accesorizer | 185 | 185 | 185 | +2 all attributes | - | Bracer, Null Talisman, Wraith Band |
| Gauntlets of Ogre Strength | Sena The Accesorizer | 150 | 150 | 150 | +3 Strength | - | Bracer, Urn of Shadows |
| Ironwood Branch | Sena The Accesorizer | 53 | 53 | 53 | +1 all attributes | - | Headdress of Rejuvenation, Magic Wand, Nathrezim Buckler |
| Mantle of Intelligence | Sena The Accesorizer | 150 | 150 | 150 | +3 Intelligence | - | Null Talisman |
| Ogre Axe | Sena The Accesorizer | 1000 | 1000 | 1000 | +10 Strength | - | Aghanim's Scepter, Black King Bar, Sange |
| Robe of the Magi | Sena The Accesorizer | 450 | 450 | 450 | +6 Intelligence | Goblin Merchant | Blade Mail, Diffusal Blade, Oblivion Staff, Power Treads |
| Slippers of Agility | Sena The Accesorizer | 150 | 150 | 150 | +3 Agility | Goblin Merchant | Poor Man's Shield, Wraith Band |
| Staff of Wizardry | Sena The Accesorizer | 1000 | 1000 | 1000 | +10 Intelligence | - | Aghanim's Scepter, Dagon, Eul's Scepter of Divinity, Force Staff, Necronomicon |
| Ultimate Orb | Sena The Accesorizer | 2100 | 2100 | 2100 | +10 all attributes | Goblin Merchant | Eye of Skadi, Guinsoo's Scythe of Vyse, Linken's Sphere, Manta Style |
| Boots of Speed | Cache of Quel-thelan | 500 | 500 | 450 | +50 movement speed | Goblin Laboratory | Boots of Travel, Phase Boots, Power Treads |
| Gem of True Sight | Cache of Quel-thelan | 700 | 700 | 850 | True Sight (passive) | - |  |
| Ghost Scepter | Cache of Quel-thelan | 1500 | 1600 | 1600 | +7 all attributes; Ghost Form (active) | - |  |
| Gloves of Haste | Cache of Quel-thelan | 500 | 500 | 500 | +15 Attack Speed | Goblin Merchant | Armlet of Mordiggian, Hand of Midas, Maelstrom, Power Treads |
| Kelen's Dagger | Cache of Quel-thelan | 2150 | 2150 | 2150 | Blink (active) | Goblin Merchant |  |
| Magic Stick | Cache of Quel-thelan | 200 | 200 | 200 | Energy Charge (active) | Goblin Laboratory | Magic Wand |
| Mask of Death | Cache of Quel-thelan | 900 | 900 | 900 | Lifesteal (passive) | Goblin Laboratory | Helm of the Dominator, Mask of Madness, Vladmir's Offering |
| Planeswalker's Cloak | Cache of Quel-thelan | n/a | n/a | 550 | (not in 6.6x snapshot; Dec 2011 data) +15% spell resistance | Goblin Laboratory |  |
| Planewalker's Cloak | Cache of Quel-thelan | 500 | 550 | n/a (removed) | +15% spell resistance | Goblin Laboratory | Hood of Defiance |
| Ring of Regeneration | Cache of Quel-thelan | 350 | 350 | 350 | +2 HP regeneration | Goblin Laboratory | Headdress of Rejuvenation, Hood of Defiance, Vladmir's Offering |
| Shadow Amulet | Cache of Quel-thelan | n/a | n/a | 1600 | (not in 6.6x snapshot; Dec 2011 data) +30 attack speed; Fade (Active) | - |  |
| Sobi Mask | Cache of Quel-thelan | 325 | 325 | 325 | +50% Mana regeneration | Goblin Laboratory | Eul's Scepter of Divinity, Oblivion Staff, Ring of Basilius, Urn of Shadows |
| Talisman of Evasion | Cache of Quel-thelan | 1800 | 1800 | 1800 | 25% evasion | Goblin Merchant | The Butterfly |
| Demon Edge | Marketplace | 2400 | 2400 | 2400 | +46 damage | - | Buriza-do Kyanon, Divine Rapier, Monkey King Bar |
| Eaglehorn | Marketplace | 3300 | 3300 | 3300 | +25 Agility | - | The Butterfly |
| Energy Booster | Marketplace | 1000 | 1000 | 1000 | +250 mana | Goblin Laboratory | Arcane Ring, Soul Booster |
| Hyperstone | Marketplace | 2100 | 2100 | 2100 | +55 attack speed | - | Assault Cuirass, Mjollnir |
| Messerschmidt's Reaver | Marketplace | 3200 | 3200 | 3200 | +25 Strength | - | Heart of Tarrasque, Satanic |
| Mystic Staff | Marketplace | 2700 | 2700 | 2700 | +25 Intelligence | - | Guinsoo's Scythe of Vyse, Shiva's Guard |
| Orb of Venom | Marketplace | n/a | 500 | 275 | (not in 6.6x snapshot; Dec 2011 data) Adds a poison effect to your attacks | Goblin Laboratory |  |
| Point Booster | Marketplace | 1200 | 1200 | 1200 | +200 HP; +150 mana | - | Aghanim's Scepter, Eye of Skadi, Soul Booster |
| Ring of Health | Marketplace | 875 | 875 | 875 | +5 HP regeneration | Goblin Laboratory | Hood of Defiance, Perseverance, Vanguard |
| Sacred Relic | Marketplace | 3800 | 3800 | 3800 | +60 damage | - | Divine Rapier, Radiance |
| Vitality Booster | Marketplace | 1100 | 1100 | 1100 | +250 HP | - | Heart of Tarrasque, Soul Booster, Vanguard |
| Void Stone | Marketplace | 875 | 875 | 875 | +100% mana regeneration | - | Eul's Scepter of Divinity, Guinsoo's Scythe of Vyse, Perseverance |
| Aegis of the Immortal | Protectorate | 0 | 0 | 0 | Reincarnation (passive) | - |  |

### Recipe (combined) items by shop

| Item | Recipe shop | Total 6.6x [A] | Total Dec 2011 [C] | Components 6.6x (price) [A] |
|---|---|---|---|---|
| Boots of Travel | Gateway Relics | 2700 | 2450 | Boots of Speed (500), Boots of Travel (2200) |
| Bracer | Gateway Relics | 510 | 525 | Circlet of Nobility (185), Gauntlets of Ogre Strength (150), Bracer (175) |
| Hand of Midas | Gateway Relics | 1900 | 1900 | Gloves of Haste (500), Hand of Midas (1400) |
| Magic Wand | Gateway Relics | 509 | 509 | Magic Stick (200), 3 X Ironwood Branch (53), Magic Wand (150) |
| Null Talisman | Gateway Relics | 485 | 470 | Circlet of Nobility (185), Mantle of Intelligence (150), Null Talisman (150) |
| Oblivion Staff | Gateway Relics | 1675 | 1675 | Quarterstaff (900), Robe of the Magi (450), Sobi Mask (325) |
| Perseverance | Gateway Relics | 1750 | 1750 | Ring of Health (875), Void Stone (875) |
| Phase Boots | Gateway Relics | 1500 | 1350 | Boots of Speed (500), 2 X Blades of Attack (500) |
| Poor Man's Shield | Gateway Relics | 550 | 550 | Stout Shield (250), 2 X Slippers of Agility (150) |
| Power Treads | Gateway Relics | 1450 | 1400 | Boots of Speed (500), Gloves of Haste (500), Belt of Giant Strength (450), Robe of the Magi (450), Boots of Elvenskin (450) |
| Soul Ring | Gateway Relics | n/a | 800 | (added after Jan 2010; Dec 2011 data) Ring of Regeneration (350), Sobi Mask (325), Soul Ring (125) |
| Wraith Band | Gateway Relics | 460 | 485 | Circlet of Nobility (185), Slippers of Agility (150), Wraith Band (125) |
| Ancient Janggo of Endurance | Supportive Vestments | n/a | 1775 | (added after Jan 2010; Dec 2011 data) Bracer (525), Robe of the Magi (450), Ancient Janggo of Endurance  (800) |
| Arcane Boots | Supportive Vestments | n/a | 1450 | (added after Jan 2010; Dec 2011 data) Energy Booster (1000), Boots of Speed (450) |
| Arcane Ring | Supportive Vestments | 1700 | n/a (removed) | Energy Booster (1000), Ring of Protection (175), Arcane Ring (525) |
| Flying Courier | Supportive Vestments | 400 | 370 | Animal Courier (200), Flying Courier (200) |
| Headdress of Rejuvenation | Supportive Vestments | 603 | 603 | Ring of Regeneration (350), Ironwood Branch (53), Headdress of Rejuvenation (200) |
| Khadgar's Pipe of Insight | Supportive Vestments | 3653 | 3628 | Hood of Defiance (2150), Headdress of Rejuvenation (603), Khadgar's Pipe of Insight (1000) |
| Medallion of Courage | Supportive Vestments | n/a | 1075 | (added after Jan 2010; Dec 2011 data) Chain Mail (550), Sobi Mask (325), Medallion of Courage (200) |
| Mekansm | Supportive Vestments | 2306 | 2306 | Nathrezim Buckler (803), Headdress of Rejuvenation (603), Mekansm (900) |
| Nathrezim Buckler | Supportive Vestments | 803 | 803 | Chain Mail (550), Ironwood Branch (53), Nathrezim Buckler (200) |
| Ring of Aquila | Supportive Vestments | n/a | 985 | (added after Jan 2010; Dec 2011 data) Ring of Basilius (500), Wraith Band (485) |
| Ring of Basilius | Supportive Vestments | 500 | 500 | Sobi Mask (325), Ring of Protection (175) |
| Tranquil Boots | Supportive Vestments | n/a | 975 | (added after Jan 2010; Dec 2011 data) Boots of Speed (450), Ring of Regeneration (350), Ring of Protection (175) |
| Urn of Shadows | Supportive Vestments | 875 | 875 | Sobi Mask (325), 2 X Gauntlets of Ogre Strength (150), Urn of Shadows (250) |
| Vladmir's Offering | Supportive Vestments | 2050 | 2050 | Mask of Death (900), Ring of Basilius (500), Ring of Regeneration (350), Vladmir's Offering (300) |
| Assault Cuirass | Protectorate | 5550 | 5350 | Hyperstone (2100), Plate Mail (1400), Chain Mail (550), Assault Cuirass (1500) |
| Black King Bar | Protectorate | 3900 | 3900 | Mithril Hammer (1600), Ogre Axe (1000), Black King Bar (1300) |
| Blade Mail | Protectorate | 2200 | 2200 | Broadsword (1200), Chain Mail (550), Robe of the Magi (450) |
| Bloodstone | Protectorate | 5050 | 5050 | Soul Booster (3300), Perseverance (1750) |
| Heart of Tarrasque | Protectorate | 5500 | 5500 | Messerschmidt's Reaver (3200), Vitality Booster (1100), Heart of Tarrasque (1200) |
| Hood of Defiance | Protectorate | 2150 | 2125 | Helm of Iron Will (950), Planewalker's Cloak (500), 2 X Ring of Regeneration (350), Ring of Health (875) |
| Linken's Sphere | Protectorate | 5175 | 5175 | Ultimate Orb (2100), Perseverance (1750), Linken's Sphere (1325) |
| Shiva's Guard | Protectorate | 4700 | 4700 | Mystic Staff (2700), Plate Mail (1400), Shiva's Guard (600) |
| Soul Booster | Protectorate | 3300 | 3300 | Point Booster (1200), Vitality Booster (1100), Energy Booster (1000) |
| Vanguard | Protectorate | 2225 | 2225 | Vitality Booster (1100), Ring of Health (875), Stout Shield (250) |
| Aghanim's Scepter | Arcane Sanctum | 4300 | 4200 | Point Booster (1200), Ogre Axe (1000), Blade of Alacrity (1000), Staff of Wizardry (1000), Aghanim's Scepter (1100) |
| Dagon | Arcane Sanctum | 2850 | 2730 | Staff of Wizardry (1000), Blades of Attack (500), Dagon (1350) |
| Eul's Scepter of Divinity | Arcane Sanctum | 2800 | 2700 | Staff of Wizardry (1000), Sobi Mask (325), Void Stone (875), Eul's Scepter of Divinity (600) |
| Force Staff | Arcane Sanctum | 2200 | 2250 | Staff of Wizardry (1000), Quarterstaff (900), Force Staff (300) |
| Guinsoo's Scythe of Vyse | Arcane Sanctum | 5675 | 5675 | Mystic Staff (2700), Ultimate Orb (2100), Void Stone (875) |
| Necronomicon | Arcane Sanctum | 2700 | 2700 | Staff of Wizardry (1000), Belt of Giant Strength (450), Necronomicon (1250) |
| Orchid Malevolence | Arcane Sanctum | 5025 | 4125 | 3 X Oblivion Staff (1675) |
| Refresher Orb | Arcane Sanctum | 5300 | 5300 | Oblivion Staff (1675), Perseverance (1750), Refresher Orb (1875) |
| Rod of Atos | Arcane Sanctum | n/a | 3100 | (added after Jan 2010; Dec 2011 data) 2 X Staff of Wizardry (1000), Vitality Booster (1100) |
| Veil of Discord | Arcane Sanctum | n/a | 2650 | (added after Jan 2010; Dec 2011 data) Helm of Iron Will (950), Robe of the Magi (450), Veil of Discord (1250) |
| Diffusal Blade | Enchanted Artifacts | 3300 | 3300 | 2 X Blade of Alacrity (1000), Robe of the Magi (450), Diffusal Blade (850) |
| Eye of Skadi | Enchanted Artifacts | 6150 | 5675 | 2 X Ultimate Orb (2100), Point Booster (1200), Eye of Skadi (750) |
| Heaven's Halberd | Enchanted Artifacts | n/a | 3850 | (added after Jan 2010; Dec 2011 data) Sange (2050), Talisman of Evasion (1800) |
| Helm of the Dominator | Enchanted Artifacts | 1850 | 1850 | Helm of Iron Will (950), Mask of Death (900) |
| Maelstrom | Enchanted Artifacts | 3000 | 2700 | Mithril Hammer (1600), Gloves of Haste (500), Maelstrom (900) |
| Mask of Madness | Enchanted Artifacts | 1900 | 1900 | Mask of Death (900), Mask of Madness (1000) |
| Mjollnir | Enchanted Artifacts | 5500 | 5400 | Hyperstone (2100), Maelstrom (3000), Mjollnir (400) |
| Sange | Enchanted Artifacts | 2250 | 2050 | Ogre Axe (1000), Belt of Giant Strength (450), Sange (800) |
| Sange and Yasha | Enchanted Artifacts | 4500 | 4100 | Sange (2250), Yasha (2250) |
| Satanic | Enchanted Artifacts | 6150 | 6150 | Messerschmidt's Reaver (3200), Helm of the Dominator (1850), Satanic (1100) |
| Stygian Desolator | Enchanted Artifacts | 4400 | 4100 | 2 X Mithril Hammer (1600), Stygian Desolator (1200) |
| Yasha | Enchanted Artifacts | 2250 | 2050 | Blade of Alacrity (1000), Boots of Elvenskin (450), Yasha (800) |
| Abyssal Blade | Ancient Weaponary | n/a | 6750 | (added after Jan 2010; Dec 2011 data) Sacred Relic (3800), Cranium Basher (2950) |
| Armlet of Mordiggian | Ancient Weaponary | 2850 | 2600 | Helm of Iron Will (950), Gloves of Haste (500), Blades of Attack (500), Armlet of Mordiggian (900) |
| Battle Fury | Ancient Weaponary | 4350 | 4350 | Claymore (1400), Broadsword (1200), Perseverance (1750) |
| Buriza-do Kyanon | Ancient Weaponary | 5800 | 5550 | Demon Edge (2400), Crystalys (2200), Buriza-do Kyanon (1200) |
| Cranium Basher | Ancient Weaponary | 3100 | 2950 | Javelin (1500), Belt of Giant Strength (450), Cranium Basher (1150) |
| Crystalys | Ancient Weaponary | 2200 | 2150 | Broadsword (1200), Blades of Attack (500), Crystalys (500) |
| Divine Rapier | Ancient Weaponary | 6200 | 6200 | Sacred Relic (3800), Demon Edge (2400) |
| Ethereal Blade | Ancient Weaponary | n/a | 4900 | (added after Jan 2010; Dec 2011 data) Eaglehorn (3300), Ghost Scepter (1600) |
| Lothar's Edge | Ancient Weaponary | 3400 | 3000 | Claymore (1400), Quarterstaff (900), Lothar's Edge (1100) |
| Manta Style | Ancient Weaponary | 4850 | 5050 | Ultimate Orb (2100), Yasha (2250), Manta Style (500) |
| Monkey King Bar | Ancient Weaponary | 5400 | 5400 | 2 X Javelin (1500), Demon Edge (2400) |
| Radiance | Ancient Weaponary | 5150 | 5150 | Sacred Relic (3800), Radiance (1350) |
| The Butterfly | Ancient Weaponary | 6000 | 6000 | Eaglehorn (3300), Talisman of Evasion (1800), Quarterstaff (900) |

### Key recipes

Reading the tables: in **Components**, "Recipe scroll (N)" is the scroll's own price. "2 X Item (p each)" means two copies. Dagon levels 2-5 are each made by buying the Dagon recipe again: "Buy recipe to upgrade (5 levels)". The scroll is 1350 in [A] and 1250 in [C]. Dagon's total cost by level is not stated, so it is UNVERIFIED; by arithmetic it would be 2850 + 1350 x (level - 1) in [A]. Hyperstone, Kelen's Dagger, the consumables, Gem and Aegis are basic items, so they have no Components row. DotA 2 values were not collected in this pass. Treat the [C] column as the 6.7x reference.


#### Boots of Speed

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Cache of Quel-thelan | Cache of Quel-thelan |
| Total cost | 500 | 450 |
| Bonuses | +50 movement speed | +50 movement speed |
| Ability text | Buyable from Goblin Laboratory; Movement speed bonus does not stack with Boots of Travel, Phase Boots, Power Treads, or Boots of Speed | Buyable from Goblin Laboratory; Movement speed bonus does not stack with other boots (Boots of Speed, Power Treads, Boots of Travel, Arcane Boots, Phase Boots, Tranquil Boots) |

#### Power Treads

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 1450 | 1400 |
| Components | Boots of Speed (500) + Gloves of Haste (500) + ONE of Belt of Giant Strength / Robe of the Magi / Boots of Elvenskin (450); no recipe scroll (500+500+450 = 1450). The "pick one" reading comes from the total price; playdota lists all three. | Boots of Speed (450) + Gloves of Haste (500) + ONE of Belt / Robe / Elvenskin (450) = 1400 |
| Bonuses | +60 movement speed; +10 selected attribute; +25 attack speed; Switch Attribute (active) | +55 movement speed; +8 selected attribute; +30 attack speed; Switch Attribute (active) |
| Ability text | Can only carry one; Movement speed bonus does not stack with Boots of Travel, Phase Boots, Power Treads, or Boots of Speed; Switch Attribute; Switches the attribute that receives a bonus from Power Treads; Switches from Strength to Intelligence, Intelligence to Agility, and Agility to Strength | Can only carry one; Movement speed bonus does not stack with other boots (Boots of Speed, Power Treads, Boots of Travel, Arcane Boots, Phase Boots, Tranquil Boots); Switch Attribute; Switches the attribute that receives a bonus from Power Treads; Switches from Strength to Intelligence, Intelligence to Agility, and Agility to Strength |

#### Phase Boots

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 1500 | 1350 |
| Components | Boots of Speed (500) + 2 X Blades of Attack (500 each) | Boots of Speed (450) + 2 X Blades of Attack (450 each) |
| Bonuses | +70 movement speed; +24 damage; Phase (active) | +55 movement speed; +24 damage; Phase (active) |
| Ability text | Movement speed bonus does not stack with Boots of Travel, Phase Boots, Power Treads, or Boots of Speed; Phase; Gives 12% movement speed bonus and lets you move through units; Using items or abilities cancels Phase; Lasts 5 seconds, has a 10 second cooldown | Movement speed bonus does not stack with other boots (Boots of Speed, Power Treads, Boots of Travel, Arcane Boots, Phase Boots, Tranquil Boots); Phase; Gives 16% movement speed bonus and lets you move through units; Using items or abilities cancels Phase; Lasts 4 seconds, has a 8 second cooldown |

#### Boots of Travel

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 2700 | 2450 |
| Components | Boots of Speed (500) + Recipe scroll (2200) | Boots of Speed (450) + Recipe scroll (2000) |
| Bonuses | +95 movement speed; Teleport (active) | +100 movement speed; Teleport (active) |
| Ability text | Movement speed bonus does not stack with Boots of Travel, Phase Boots, Power Treads, or Boots of Speed; Teleport; Teleports you to an allied non-hero unit; Takes 3 seconds to teleport, costs 75 mana and has a 60 seccond cooldown; Shares cooldown with Scroll of Town Portal | Movement speed bonus does not stack with other boots (Boots of Speed, Power Treads, Boots of Travel, Arcane Boots, Phase Boots, Tranquil Boots); Teleport; Teleports you to an allied non-hero unit; Takes 3 seconds to teleport, costs 75 mana and has a 60 second cooldown; Shares cooldown with Scroll of Town Portal |

#### Bracer

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 510 | 525 |
| Components | Circlet of Nobility (185) + Gauntlets of Ogre Strength (150) + Recipe scroll (175) | Circlet of Nobility (185) + Gauntlets of Ogre Strength (150) + Recipe scroll (190) |
| Bonuses | +6 Strength; +3 Agility; +3 Intelligence | +3 damage; +6 Strength; +3 Agility; +3 Intelligence |
| Ability text | none | none |

#### Wraith Band

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 460 | 485 |
| Components | Circlet of Nobility (185) + Slippers of Agility (150) + Recipe scroll (125) | Circlet of Nobility (185) + Slippers of Agility (150) + Recipe scroll (150) |
| Bonuses | +6 Agility; +3 Strength; +3 Intelligence | +3 damage; +6 Agility; +3 Strength; +3 Intelligence |
| Ability text | none | none |

#### Null Talisman

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 485 | 470 |
| Components | Circlet of Nobility (185) + Mantle of Intelligence (150) + Recipe scroll (150) | Circlet of Nobility (185) + Mantle of Intelligence (150) + Recipe scroll (135) |
| Bonuses | +6 Intelligence; +3 Strength; +3 Agility | +3 damage; +6 Intelligence; +3 Strength; +3 Agility |
| Ability text | none | none |

#### Magic Stick

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Cache of Quel-thelan | Cache of Quel-thelan |
| Total cost | 200 | 200 |
| Bonuses | Energy Charge (active) | Energy Charge (active) |
| Ability text | Buyable from Goblin Laboratory; Gains 1 charge every time an enemy unit casts an ability in a 1600 AoE; Maximum of 10 charges; Certain abilities, such as arrow abilites (Frost Arrow, Poison Attack, etc) and item abilities, will not add a charge; Energy Charge; Restores 15 HP and mana for each charge | Buyable from Goblin Laboratory; Gains 1 charge every time a visible enemy unit casts an ability in a 1200 AoE; Maximum of 10 charges; Certain abilities, such as arrow abilites (Frost Arrow, Poison Attack, etc) and item abilities, will not add a charge; Energy Charge; Restores 15 HP and mana for each charge |

#### Magic Wand

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 509 | 509 |
| Components | Magic Stick (200) + 3 X Ironwood Branch (53 each) + Recipe scroll (150) | Magic Stick (200) + 3 X Ironwood Branch (53 each) + Recipe scroll (150) |
| Bonuses | +3 all attributes; Energy Charge (active) | +3 All Attributes; Energy Charge (active) |
| Ability text | Gains 1 charge every time an enemy unit casts an ability in a 1600 AoE; Maximum of 15 charges; Certain abilities, such as arrow abilites (Frost Arrow, Poison Attack, etc) and item abilities, will not add a charge; Energy Charge; Restores 15 HP and mana for each charge | Gains 1 charge every time a visible enemy unit casts an ability in a 1200 AoE; Maximum of 15 charges; Certain abilities, such as arrow abilites (Frost Arrow, Poison Attack, etc) and item abilities, will not add a charge; Energy Charge; Restores 15 HP and mana for each charge |

#### Empty Bottle

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 600 | 600 |
| Bonuses | Regenerate (active); Rune Capture (active) | Regenerate (active); Rune Capture (active) |
| Ability text | Refills to 3/3 full when you are near the fountain; Regenerate; Restores 135 HP and 70 Mana over 3 seconds; Effect ends if you take damage; When used, 3/3 full Bottle changes to 2/3 full, 2/3 Bottle changes to 1/3 full, and 1/3 full Bottle changes to Empty Bottle; Cannot be used when Bottle is empty; Has a 0.5 second cooldown; Rune Capture; Stores the target rune in the Bottle, the rune can be used by using the Bottle; When a rune is used from the Bottle, the Bottle is refilled to 3/3 full; Stored runes are automatically used after 120 seconds; Can only be used when Bottle is empty; Bottle is not drop-able when it has a rune stored | Refills to 3/3 full when you are near the fountain; Couriers move 30% slower when carrying empty bottles; ; Regenerate; Restores 135 HP and 70 Mana over 3 seconds; Effect ends if you take damage; When used, 3/3 full Bottle changes to 2/3 full, 2/3 Bottle changes to 1/3 full, and 1/3 full Bottle changes to Empty Bottle; Cannot be used when Bottle is empty; Has a 0.5 second cooldown; Rune Capture; Stores the target rune in the Bottle, the rune can be used by using the Bottle; When a rune is used from the Bottle, the Bottle is refilled to 3/3 full; Stored runes are automatically used after 120 seconds; Can only be used when Bottle is empty; Bottle is not drop-able when it has a rune stored |

#### Black King Bar

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 3900 | 3900 |
| Components | Mithril Hammer (1600) + Ogre Axe (1000) + Recipe scroll (1300) | Mithril Hammer (1600) + Ogre Axe (1000) + Recipe scroll (1300) |
| Bonuses | +10 Strength; +24 damage; Avatar (active) | +10 Strength; +24 damage; Avatar (active) |
| Ability text | Avatar; Gives magic immunity; Duration and cooldown decrease each time it is used; Duration is 10/9/8/7/6/5 seconds, and cooldown is 80/75/70/65/60/55 seconds. After 5 uses it stays at 5 second duration and 55 second cooldown | Cannot be sold; Avatar; Gives magic immunity; Duration and cooldown decrease each time it is used; Duration is 10/9/8/7/6/5/4 seconds, and cooldown is 80/75/70/65/60/55/50 seconds. After 6 uses it stays at 4 second duration and 50 second cooldown |

#### Battle Fury

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 4350 | 4350 |
| Components | Claymore (1400) + Broadsword (1200) + Perseverance (1750) | Claymore (1400) + Broadsword (1200) + Perseverance (1750) |
| Bonuses | +65 damage; +6 HP/sec regeneration; +150% mana regeneration; Cleave (passive) | +65 damage; +6 HP/sec regeneration; +150% mana regeneration; Cleave (passive) |
| Ability text | Cleave; 35% Cleave on attack; 225 AoE; Cleave does not work on ranged units | Cleave; 35% Cleave on attack; 225 AoE; Cleave does not work on ranged units |

#### The Butterfly

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 6000 | 6000 |
| Components | Eaglehorn (3300) + Talisman of Evasion (1800) + Quarterstaff (900) | Eaglehorn (3300) + Talisman of Evasion (1800) + Quarterstaff (900) |
| Bonuses | +30 Agility; +30 Damage; 30% Evasion; +30 Attack Speed | +30 Agility; +30 Damage; 35% Evasion; +30 Attack Speed |
| Ability text | none | none |

#### Buriza-do Kyanon

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 5800 | 5550 |
| Components | Demon Edge (2400) + Crystalys (2200) + Recipe scroll (1200) | Demon Edge (2400) + Crystalys (2150) + Recipe scroll (1000) |
| Bonuses | +75 damage; Critical Strike (passive) | +81 damage; Critical Strike (passive) |
| Ability text | Critical Strike; Gives a 20% chance to deal 2.4x normal damage on an attack | Critical Strike; Gives a 25% chance to deal 2.4x normal damage on an attack |

#### Monkey King Bar

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 5400 | 5400 |
| Components | 2 X Javelin (1500 each) + Demon Edge (2400) | 2 X Javelin (1500 each) + Demon Edge (2400) |
| Bonuses | +80 damage; +15 attack speed; True Strike (passive); Mini-Bash (passive) | +88 damage; +15 attack speed; True Strike (passive); Mini-Bash (passive) |
| Ability text | Clicking the item will toggle True Strike on and off; True Strike; Prevents your attacks from missing (except due to exceeding Movement Buffer Range on melee units); Interferes with some attack modifiers on ranged units; Mini-Bash; Gives a 35% chance to bash on attacks; Deals 100 bonus damage and stuns for 0.01 seconds | Clicking the item will toggle True Strike on and off; True Strike; Prevents your attacks from missing (except due to exceeding Movement Buffer Range on melee units); Interferes with some attack modifiers on ranged units; Mini-Bash; Gives a 35% chance to bash on attacks; Deals 100 bonus damage and stuns for 0.01 seconds |

#### Sange

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 2250 | 2050 |
| Components | Ogre Axe (1000) + Belt of Giant Strength (450) + Recipe scroll (800) | Ogre Axe (1000) + Belt of Giant Strength (450) + Recipe scroll (600) |
| Bonuses | +16 Strength; +10 damage; Lesser Maim (passive) | +16 Strength; +10 damage; Lesser Maim (passive) |
| Ability text | Lesser Maim; Gives a 15% chance on attack to slow movement speed by 20% for 4 seconds; Orb effect | Lesser Maim; Gives a 15% chance on attack to slow movement speed and attack speed by 20% for 4 seconds |

#### Yasha

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 2250 | 2050 |
| Components | Blade of Alacrity (1000) + Boots of Elvenskin (450) + Recipe scroll (800) | Blade of Alacrity (1000) + Boots of Elvenskin (450) + Recipe scroll (600) |
| Bonuses | +16 Agility; +15 attack speed; +10% movement speed | +16 Agility; +15 attack speed; +10% movement speed |
| Ability text | Movespeed bonus does not stack with Yasha, Sange and Yasha, or Manta Style | Movespeed and attack speed bonus does not stack with Yasha, Sange and Yasha, or Manta Style. |

#### Sange and Yasha

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 4500 | 4100 |
| Components | Sange (2250) + Yasha (2250) | Sange (2050) + Yasha (2050) |
| Bonuses | +16 Agility; +16 Strength; +12 damage; +15 attack speed; Greater Maim (passive); +12% movement speed | +16 Agility; +16 Strength; +16 damage; +16 attack speed; Greater Maim (passive); +12% movement speed |
| Ability text | Movespeed bonus does not stack with Yasha, Sange and Yasha, or Manta Style; Greater Maim; Gives a 15% chance on attack to slow movement speed by 30% and attack speed by 15 for 4 seconds; Orb effect | Movespeed and attack speed bonus does not stack with Yasha, Sange and Yasha, or Manta Style; Can be disassembled; Greater Maim; Gives a 16% chance on attack to slow movement speed and attack speed by 30% for 4 seconds |

#### Manta Style

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Protectorate |
| Total cost | 4850 | 5050 |
| Components | Ultimate Orb (2100) + Yasha (2250) + Recipe scroll (500) | Ultimate Orb (2100) + Yasha (2050) + Recipe scroll (900) |
| Bonuses | +26 Agility; +10 Strength; +10 Intelligence; +15 attack speed; +10% movement speed; Mirror Image (active) | +26 Agility; +10 Strength; +10 Intelligence; +15 attack speed; +10% movement speed; Mirror Image (active) |
| Ability text | Movespeed bonus does not stack with Yasha, Sange and Yasha, or Manta Style; Can be disassembled; Mirror Image; Creates 2 images that deal 33% damage and take 350% damage; Has a 0.1 second animation delay for which you are invulnerable; Lasts 20 seconds, costs 165 mana, and has a 50 second cooldown | Movespeed and attack speed bonus does not stack with Yasha, Sange and Yasha, or Manta Style; Can be disassembled; Mirror Image (melee); Creates 2 images that deal 33% damage and take 350% damage; Has a 0.1 second animation delay for which you are invulnerable; Lasts 20 seconds, costs 165 mana, and has a 35 second cooldown; Mirror Image (ranged); Creates 2 images that deal 28% damage and take 400% damage; Has a 0.1 second animation delay for which you are invulnerable; Lasts 20 seconds, costs 165 mana, and has a 50 second cooldown |

#### Heart of Tarrasque

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 5500 | 5500 |
| Components | Messerschmidt's Reaver (3200) + Vitality Booster (1100) + Recipe scroll (1200) | Messerschmidt's Reaver (3200) + Vitality Booster (1100) + Recipe scroll (1200) |
| Bonuses | +40 strength; +300 hit points; Health Regeneration (passive) | +40 Strength; +300 HP; Health Regeneration (passive) |
| Ability text | Health Regeneration; Regenerates 2% of max health each second; Only regenerates health when the bearer has not taken damage from a player owned source in the last 8 seconds; Self damage does not disable Health Regeneration, but damage from Roshan does; Does not stack with itself | Health Regeneration; Regenerates 2% of the bearer's total hit points per second; Disabled if the bearer took damage from a player or from Roshan in the last 6 seconds (4 for melee); Does not stack with itself |

#### Satanic

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 6150 | 6150 |
| Components | Messerschmidt's Reaver (3200) + Helm of the Dominator (1850) + Recipe scroll (1100) | Messerschmidt's Reaver (3200) + Helm of the Dominator (1850) + Recipe scroll (1100) |
| Bonuses | +25 Strength; +20 damage; +5 armor; Lifesteal (passive); Unholy Rage (active) | +25 Strength; +20 damage; +5 armor; Lifesteal (passive); Unholy Rage (active) |
| Ability text | Lifesteal; Gives 25% Lifesteal on attacks; Orb effect; Unholy Rage; Gives +150% Lifesteal for 3.5 seconds; Has a 35 second cooldown | Lifesteal; Gives 25% Lifesteal on attacks; Orb effect; Unholy Rage; Gives +175% Lifesteal for 3.5 seconds; Has a 35 second cooldown |

#### Divine Rapier

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 6200 | 6200 |
| Components | Sacred Relic (3800) + Demon Edge (2400) | Demon Edge (2400) + Sacred Relic (3800) |
| Bonuses | +250 damage | +300 damage |
| Ability text | Drops on death; Cannot be destroyed by attacking; If a player other than the actual owner of the Divine Rapier picks it up, they will be unable to manually drop it (drop on death will still occur) | Drops on death.; Cannot be destroyed.; Can switch between 2 states, being in the "Original Rapier" mode when assembled.; Original Rapier; Can be dropped freely by its wearer; Can be picked up by allies, but only its original owner benefits from it (unshareable); Will switch permanently to the "Free Rapier" mode when picked up by an enemy of the original owner; Free Rapier; Cannot be dropped once picked up; Anyone wearing it will benefit from it (fully shareable) |

#### Aghanim's Scepter

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 4300 | 4200 |
| Components | Point Booster (1200) + Ogre Axe (1000) + Blade of Alacrity (1000) + Staff of Wizardry (1000) + Recipe scroll (1100) [playdota lists the scroll at 1100, but the components alone sum to 4200 against a 4300 total, so the scroll should be 100. Scroll cost UNVERIFIED] | Point Booster (1200) + Ogre Axe (1000) + Blade of Alacrity (1000) + Staff of Wizardry (1000), no recipe = 4200 |
| Bonuses | +10 all attributes; +200 HP; +150 mana; Ultimate Upgrade (passive) | +10 All Attributes; +200 HP; +150 mana; Ultimate Upgrade (passive) |
| Ability text | Can be disassembled; Ultimate Upgrade; Upgrades the ultimate of certain heroes when acquired, details can be found on the appropriate hero page; Only affects Undying, Earthshaker, Bane Elemental, Tormented Soul, Lord of Avernus, Night Stalker, Lich, Pandaren Brewmaster, Prophet, Crystal Maiden, Lord of Olympia, Vengeful Spirit, Juggernaut, Shadow Shaman, Venomancer, Sand King, Demon Witch, Faceless Void, Doom Bringer, Goblin Techies, Queen of Pain, Spiritbreaker, Oblivion, Necrolyte, Moon Rider, Slayer, Butcher, Holy Knight, Witch Doctor, Twin Head Dragon, Shadow Priest, Sacred Warrior, Invoker, Clockwerk Goblin, and Windrunner | Can be disassembled; Ultimate Upgrade; Upgrades the ultimate of certain heroes when acquired, details can be found on the appropriate hero page; Only affects Beastmaster, Earthshaker, Pandaren Brewmaster, Stone Giant, Clockwerk Goblin, Sacred Warrior, Axe, Doom Bringer, Lord of Avernus, Night Stalker, Butcher, Undying, Spiritbreaker, Sand King, Juggernaut, Moon Rider, Gyrocopter, Vengeful Spirit, Venomancer, Netherdrake, Geomancer, Lightning Revenant, Faceless Void, Crystal Maiden, Enchantress, Faerie Dragon, Holy Knight, Lord of Olympus, Prophet, Slayer, Windrunner, Ogre Magi, Goblin Techies, Twin Head Dragon, Shadow Shaman, Bane Elemental, Dark Seer, Demon Witch, Lich, Necrolyte, Oblivion, Obsidian Destroyer, Queen of Pain, Warlock, Shadow Priest, Invoker, Necro'lic, Tormented Soul, Witch Doctor |

#### Kelen's Dagger

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Cache of Quel-thelan | Cache of Quel-thelan |
| Total cost | 2150 | 2150 |
| Bonuses | Blink (active) | Blink (active) |
| Ability text | Kelen's Dagger cannot be held by Butcher or Vengeful Spirit; Buyable from Goblin Merchant; Blink; Teleports the caster to a nearby location instantly; 1200 range; Cannot be used if the carrier was damaged by an enemy player-controlled unit in the last 3 seconds; Self damage does not disable Blink, but damage from Roshan does; Costs 75 mana, has an 14 second cooldown | Kelen's Dagger cannot be held by Butcher or Vengeful Spirit; Buyable from Goblin Merchant; Blink; Teleports the caster to a nearby location instantly; 1200 range; Targeting an item in your inventory will blink you in the direction of that item's pick-up location; Cannot be used if the carrier was damaged by an enemy player-controlled unit in the last 3 seconds; Self damage does not disable Blink, but damage from Roshan does; Costs 75 mana, has a 12 second cooldown |

#### Mekansm

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Supportive Vestments | Supportive Vestments |
| Total cost | 2306 | 2306 |
| Components | Nathrezim Buckler (803) + Headdress of Rejuvenation (603) + Recipe scroll (900) | Nathrezim Buckler (803) + Headdress of Rejuvenation (603) + Recipe scroll (900) |
| Bonuses | +5 all attributes; +5 armor; Mekansm Aura (passive); Heal (active) | +5 All Attributes; +5 armor; Mekansm Aura (passive); Heal (active) |
| Ability text | Mekansm Aura; +3 HP regeneration; 500 AoE; Restore; Heals 250 HP and gives +2 armor in 450 AoE; Armor bonus lasts 25 seconds; Heal does not affect units that have been healed by any Restore in the last 25 seconds; Costs 150 mana, and has a 45 second cooldown | Mekansm Aura; +4 HP regeneration; 750 AoE; Restore; Heals 250 HP and gives +2 armor in 750 AoE; Armor bonus lasts 25 seconds; Heal does not affect units that have been healed by any Restore in the last 25 seconds; Costs 150 mana, and has a 45 second cooldown |

#### Vanguard

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 2225 | 2225 |
| Components | Vitality Booster (1100) + Ring of Health (875) + Stout Shield (250) | Ring of Health (875) + Vitality Booster (1100) + Stout Shield (250) |
| Bonuses | +300 HP; +6 HP/sec regeneration; Damage Block (passive) | +6 HP/sec regeneration; +250 HP; Damage Block (passive) |
| Ability text | Damage Block; Gives a 70% chance to block 40 attack damage if wielder is melee, or 70% chance to block 20 damage if wielder is ranged | Damage Block; Gives a 70% chance to block 40 damage if the wielder is melee, or a 70% chance to block 20 damage if the wielder is ranged |

#### Hand of Midas

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 1900 | 1900 |
| Components | Gloves of Haste (500) + Recipe scroll (1400) | Gloves of Haste (500) + Recipe scroll (1400) |
| Bonuses | +30 attack speed; Transmute (active) | +30 attack speed; Transmute (active) |
| Ability text | Transmute; Kills a target non-hero non-ancient unit, giving bonus gold; Gives the unit's normal bounty twice, as well as an additional gold bonus that varies from unit to unit; Transmute gold ranges are viewable in the Unit Statistics guide; Costs 25 mana and has a 100 second cooldown | Transmute; Kills a target non-hero non-ancient unit, giving bonus gold and experience; Gives 1.5 times the unit's normal experience as additional bonus experience; Gives 190 reliable gold (you do not get the normal creep bounty); Has a 100 second cooldown |

#### Linken's Sphere

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 5175 | 5175 |
| Components | Ultimate Orb (2100) + Perseverance (1750) + Recipe scroll (1325) | Perseverance (1750) + Ultimate Orb (2100) + Recipe scroll (1325) |
| Bonuses | +6 HP/sec regeneration; +150% mana; regeneration; +15 Strength; +15 Agility; +15 Intelligence; Blocks most targeted spells once every 20 seconds | +10 damage; +15 All Stats; +6 HP/sec regeneration; +150% mana regeneration; Spell Block (passive) |
| Ability text | none | Spell Block; Blocks most targeted spells once every 17 seconds |

#### Stygian Desolator

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 4400 | 4100 |
| Components | 2 X Mithril Hammer (1600 each) + Recipe scroll (1200) | 2 X Mithril Hammer (1600 each) + Recipe scroll (900) |
| Bonuses | +60 damage; Corruption (passive) | +60 damage; Corruption (passive) |
| Ability text | Corruption; Causes attacks to place a buff that reduces armor by 6; Buff lasts 5 seconds; Orb effect | Corruption; Causes attacks to place a buff that reduces armor by 7; Buff lasts 15 seconds; Works on buildings; Orb effect and buff placer |

#### Maelstrom

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 3000 | 2700 |
| Components | Mithril Hammer (1600) + Gloves of Haste (500) + Recipe scroll (900) | Mithril Hammer (1600) + Gloves of Haste (500) + Recipe scroll (600) |
| Bonuses | +24 damage; +25 attack speed; Chain Lighting (passive) | +24 damage; +25 attack speed; Chain Lighting (passive) |
| Ability text | Chain Lightning; Gives a 20% chance on attack to release a Chain Lightning that hits several units; Hits 4 units, dealing 150 damage to each, 500 bounce range; Orb effect | Chain Lightning; Gives a 25% chance on attack to release a Chain Lightning that hits several units; Hits 4 units, dealing 120 magical damage to each, 500 bounce range; Orb effect on regular attacks, orb effect and buff placer on proc |

#### Mjollnir

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Enchanted Artifacts | Enchanted Artifacts |
| Total cost | 5500 | 5400 |
| Components | Hyperstone (2100) + Maelstrom (3000) + Recipe scroll (400) | Hyperstone (2100) + Maelstrom (2700) + Recipe scroll (600) |
| Bonuses | +70 attack speed; +24 damage; Static Charge (active); Chain Lightning (passive) | +80 attack speed; +24 damage; Static Charge (active); Chain Lightning (passive) |
| Ability text | Can be disassembled.; Static Charge; Places a charged shield on a target unit, when that unit is attacked, there is a 20% chance that lightning will strike the attacker and 2 nearby enemies; Lightning deals 200 damage, the attacker cannot be more than 600 range away, the secondary targets cannot be more than 900 units away, the 3 targets hit must be within a 300 AoE of each other; Cannot trigger more than once per second; Will have no effect if cast on a unit that already has a Static Charge active on it; Lasts 20 seconds, costs 50 mana, has a 35 second cooldown, and 600 cast range; Chain Lightning; Gives a 20% chance on attack to release a Chain Lightning that hits several units; Hits 4 units, dealing 200 damage to each, 500 bounce range; Orb effect | Can be disassembled.; Static Charge; Places a charged shield on a target unit, when that unit is damaged, there is a 20% chance that lightning will strike the source of the damage and 2 nearby enemies; Lightning deals 200 magical damage, the damage source and secondary targets cannot be more than 900 range away, the 3 targets hit must be in a cone with 125 starting AoE, 475 distance and 300 final AoE; Cannot trigger more than once per second; Will have no effect if cast on a unit that already has a Static Charge active on it; Double clicking on the item will cast Static Charge on yourself; Lasts 20 seconds, costs 50 mana, has a 35 second cooldown, and 600 cast range; Chain Lightning; Gives a 25% chance on attack to release a Chain Lightning that hits several units; Hits 8 units, dealing 160 magical damage to each, 500 bounce range; Orb effect on regular attacks, orb effect and buff placer on proc |

#### Radiance

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 5150 | 5150 |
| Components | Sacred Relic (3800) + Recipe scroll (1350) | Sacred Relic (3800) + Recipe scroll (1350) |
| Bonuses | +60 damage; Burn Damage (passive) | +60 damage; Burn Damage (passive) |
| Ability text | Clicking Radiance will toggle Burn Damage on and off; Burn Damage; Deals 40 damage per second in a 650 AoE | Clicking Radiance will toggle Burn Damage on and off; Burn Damage; Deals 50 magical damage per second in a 650 AoE |

#### Shiva's Guard

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 4700 | 4700 |
| Components | Mystic Staff (2700) + Plate Mail (1400) + Recipe scroll (600) | Mystic Staff (2700) + Plate Mail (1400) + Recipe scroll (600) |
| Bonuses | +30 Intelligence; +15 armor; Freezing Aura (passive, enemies); Arctic Blast (active) | +30 Intelligence; +15 armor; Freezing Aura (passive, enemies); Arctic Blast (active) |
| Ability text | Can be disassembled; Freezing Aura; -25 attack speed; 1000 AoE; Arctic Blast; Emits a freezing wave around the caster; Enemies hit by the wave take 200 magic damage and have their movement speed slowed by 40% for 4 seconds; The wave extends at a speed of 300 to a max size of 744; Costs 100 mana and has a 30 second cooldown | Can be disassembled; Freezing Aura; -40 attack speed; 900 AoE; Arctic Blast; Emits a freezing wave around the caster; Enemies hit by the wave take 200 magical damage and have their movement speed slowed by 40% for 4 seconds; The wave extends at a speed of 300 to a max size of 719; Costs 100 mana and has a 30 second cooldown |

#### Guinsoo's Scythe of Vyse

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 5675 | 5675 |
| Components | Mystic Staff (2700) + Ultimate Orb (2100) + Void Stone (875) | Mystic Staff (2700) + Ultimate Orb (2100) + Void Stone (875) |
| Bonuses | +35 Intelligence; +10 Strength; +10 Agility; +200% mana regeneration; Hex (active) | +35 Intelligence; +10 Strength; +10 Agility; +150% mana regeneration; Hex (active) |
| Ability text | Hex; Turns a target unit into a sheep; Hex disables damage block and evasion, and reduces the units base movement speed to 100; Affected unit retains its normal health, mana, armor value, and armor type; If cast on an illusion, it is immediately destroyed; Lasts 3.5 seconds, costs 100 mana, has a 35 second cooldown, and an 800 cast range | Hex; Turns a target unit into a sheep; Hex disables damage block and evasion, and reduces the units base movement speed to 100; Affected unit retains its normal health, mana, armor value, and armor type; If cast on an illusion, it is immediately destroyed; Lasts 3.5 seconds, costs 100 mana, has a 35 second cooldown, and an 800 cast range |

#### Dagon

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 2850 | 2730 |
| Components | Staff of Wizardry (1000) + Blades of Attack (500) + Recipe scroll (1350) | Staff of Wizardry (1000) + Null Talisman (470) + Recipe scroll (1250) [sums to 2720 against a 2730 total. One price is off by 10: UNVERIFIED] |
| Bonuses | +12/14/16/18/20 Intelligence; +3 All Atributes; +9 damage; Energy Burst (active) | +13/15/17/19/21 Intelligence; +3 All Attributes; +9 damage; Energy Burst (active) |
| Ability text | Buy recipe to upgrade (5 levels); Energy Burst; Deals 400/500/600/700/800 damage to a target unit; Costs 180/160/140/120/100 mana, has a 40/36/32/28/24 second cooldown, and a 600/650/700/750/800 cast range | Buy recipe to upgrade (5 levels); Energy Burst; Deals 400/500/600/700/800 magical damage to a target unit; Costs 180/160/140/120/100 mana, has a 35/30/25/20/15 seconds cooldown, and a 600/650/700/750/800 cast range |

#### Eul's Scepter of Divinity

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 2800 | 2700 |
| Components | Staff of Wizardry (1000) + Sobi Mask (325) + Void Stone (875) + Recipe scroll (600) | Staff of Wizardry (1000) + Sobi Mask (325) + Void Stone (875) + Recipe scroll (500) |
| Bonuses | +10 Intelligence; +125% mana regeneration; +25 movement speed; Cyclone (active) | +10 Intelligence; +150% mana regeneration; +35 movement speed; Cyclone (active) |
| Ability text | Movement speed bonus does not stack with other Eul's Scepters of Divinity; Cyclone; Cyclones a target unit, making it invulnerable and unable to act; Can target enemies or self; Lasts 2.5 seconds, costs 75 mana, has a 30 second cooldown, and a 600 cast range | Movement speed bonus does not stack with other Eul's Scepters of Divinity; Cyclone; Cyclones a target unit, making it invulnerable and unable to act; Double clicking the item will make cyclone be used on yourself.; Can target enemies or self; Lasts 2.5 seconds, costs 75 mana, has a 30 second cooldown, and a 700 cast range |

#### Force Staff

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 2200 | 2250 |
| Components | Staff of Wizardry (1000) + Quarterstaff (900) + Recipe scroll (300) | Staff of Wizardry (1000) + Ring of Regeneration (350) + Recipe scroll (900) |
| Bonuses | +10 Intelligence; +10 damage; +10% attack speed; Force (Active) | +10 Intelligence; +3 HP Regeneration; Force (Active) |
| Ability text | Force pushes any unit 600 units in the direction it is facing.; Force has 25 manacost and 25 seconds cooldown; Force does not interrupt the target's actions; Double clicking on Force Staff will act as if you had targeted yourself | Force; Pushes any unit 600 units in the direction it is facing over a 0.3s duration.; Double clicking the item will make Force be used on yourself.; Does not interrupt the target's actions.; The -disablehelp command will prevent an ally from casting this spell on you.; Will not work on a unit inside Chronosphere or Black Hole.; Costs 25 mana, has a 20 second cooldown, and a 800 cast range. |

#### Orchid Malevolence

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Arcane Sanctum | Arcane Sanctum |
| Total cost | 5025 | 4125 |
| Components | 3 X Oblivion Staff (1675 each) | 2 X Oblivion Staff (1675 each) + Recipe scroll (775) |
| Bonuses | +20 Intelligence; +30 attack speed; +45 damage; +225% mana regeneration; Soul Burn (active) | +25 Intelligence; +30 attack speed; +30 damage; +150% mana regeneration; Soul Burn (active) |
| Ability text | Soul Burn; Silences a target unit and amplifies the damage it takes by 20%; Amplified damage is dealt at the end of the duration as magic damage; Lasts 5 seconds, costs 100 mana, has a 25 second cooldown, and a 900 cast range | Soul Burn; Silences a target unit and amplifies the damage it takes by 30%; Amplified damage is dealt at the end of the duration as magical damage; Lasts 5 seconds, costs 100 mana, has a 18 second cooldown, and a 900 cast range |

#### Assault Cuirass

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 5550 | 5350 |
| Components | Hyperstone (2100) + Plate Mail (1400) + Chain Mail (550) + Recipe scroll (1500) | Plate Mail (1400) + Hyperstone (2100) + Chain Mail (550) + Recipe scroll (1300) |
| Bonuses | +40 attack speed; +10 armor; Attack Speed Aura (passive); Positive Armor Aura (passive); Negative Armor Aura (passive, enemies) | +10 armor; Positive Armor Aura (passive); Negative Armor Aura (passive, enemies); +35 attack speed; Attack Speed Aura (passive) |
| Ability text | Attack Speed Aura; +15 attack speed; 900 AoE; Positive Armor Aura; +5 armor; 900 AoE; Affects buildings; Does not stack with armor auras from Assault Cuirass, Ring of Basilius, or Vladmir's Offering; Negative Armor Aura; 5 armor; 900 AoE; Affects buildings | Positive Armor Aura; +5 armor; 900 AoE; ; Negative Armor Aura; -5 armor; 900 AoE; ; Attack Speed Aura; +20 attack speed; 900 AoE |

#### Hyperstone

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Marketplace | Marketplace |
| Total cost | 2100 | 2100 |
| Bonuses | +55 attack speed | +55 attack speed |
| Ability text | none | none |

#### Observer Wards

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 200 | 150 |
| Bonuses | Place Ward (active) | Place Ward (active) |
| Ability text | Maximum stock of 2 in store; Store stock takes 6 minutes to replenish; Stacks with itself in inventory; 2 charges; Place Ward; Places an Observer Ward to give sight of the surrounding area; Observer Ward has 1600 vision, 200 HP, and is invisible; Observer Ward lasts 6 minutes | Maximum stock of 2 in store; Initial stock of 1; Store stock takes 6 minutes to replenish; Stacks with itself in inventory; 2 charges; Place Ward; Places an Observer Ward to give sight of the surrounding area; Observer Ward has 1600 vision, 200 HP, and is invisible; Observer Ward lasts 6 minutes |

#### Sentry Wards

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 200 | 200 |
| Bonuses | Place Ward (active) | Place Ward (active) |
| Ability text | Stacks with itself in inventory; 2 charges; Place Ward; Places a Sentry Ward to give True Sight of the surrounding area; Sentry Ward has 200 vision, 800 AoE True Sight, 200 HP, and is invisible; Sentry Ward lasts 3 minutes | Stacks with itself in inventory; 2 charges; Place Ward; Places a Sentry Ward to give True Sight of the surrounding area; Sentry Ward has 70 vision, 800 AoE True Sight, 200 HP, and is invisible; Sentry Ward lasts 3 minutes |

#### Ancient Tango of Essifation

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 90 | 90 |
| Bonuses | Eat Tree (active) | Eat Tree (active) |
| Ability text | Stacks with itself in inventory; 3 charges; Eat Tree; Eats a tree to restore 115 HP over 16 seconds; Purge will end the effect even though the status icon will remain | Stacks with itself in inventory; 3 charges; Eat Tree; Eats a tree to restore 115 HP over 16 seconds; Purge will end the effect even though the status icon will remain |

#### Healing Salve

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 100 | 100 |
| Bonuses | Regenerate Health (active) | Regenerate Health (active) |
| Ability text | Stacks with itself in inventory; 1 charge; Regenerate Health; Restores 400 HP over 10 seconds; Effect ends if affected unit takes damage; You can target an ally or yourself to receive the effect | Stacks with itself in inventory; 1 charge; Regenerate Health; Restores 400 HP over 10 seconds; Effect ends if affected unit takes damage; You can target an ally or yourself to receive the effect; Double clicking the item causes it to be used on yourself |

#### Clarity Potion

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 50 | 50 |
| Bonuses | Regenerate Mana (active) | Regenerate Mana (active) |
| Ability text | Stacks with itself in inventory; 1 charge; Regenerate Mana; Restores 100 mana over 30 seconds; Effect ends if affected unit takes damage; You can target an ally or yourself to receive the effect | Stacks with itself in inventory; 1 charge; Regenerate Mana; Restores 100 mana over 30 seconds; Effect ends if affected unit takes damage; You can target an ally or yourself to receive the effect; Double clicking the item will cause it be used on yourself |

#### Scroll of Town Portal

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 135 | 135 |
| Bonuses | Teleport (active) | Teleport (active) |
| Ability text | Buyable from Goblin Laboratory; Stacks with itself in inventory; 1 charge; Double clicking on the item will automatically target your fountain; Teleport; Teleports you to an allied building; Can be targeted at a point up to 525 range away from the building instead of directly on it, it will teleport you to that spot instead; If the targeted point is closer than 50 range to the tower, 70 will be used as the distance from the tower instead; When you use Teleport, the minimap is pinged for your allies at the target location of your teleport, and you are given 200 AoE vision at the target location for the duration of the teleport; When the teleport finishes, trees are destroyed around the landing spot in a 480x480 rectangle; The casting time of the teleport is determined by how many allies have targeted a Scroll of Town Portal in a 1100 AoE around your target location in the last 25 seconds; For each recent allied teleport in the area, time is added to your Teleport's casting time (2 seconds for the first recent allied teleport, 1 second for each additional one), the default casting time is 3 seconds; Recent allied teleports have no effect if the target is in the fountain area; Shares cooldown with Boots of Travel; Costs 75 Mana, and has a 65 second cooldown | Buyable from Goblin Laboratory; Stacks with itself in inventory; 1 charge; Double clicking on the item will automatically target your fountain; Teleport; Teleports you to an allied building; Can be targeted at a point up to 525 range away from the building instead of directly on it, it will teleport you to that spot instead; If the targeted point is closer than 50 range to the tower, 70 will be used as the distance from the tower instead; When you use Teleport, the minimap is pinged for your allies at the target location of your teleport, and you are given 200 AoE vision at the target location for the duration of the teleport; When the teleport finishes, trees are destroyed around the landing spot in a 480x480 square; The casting time of the teleport is determined by how many allies have targeted a Scroll of Town Portal in a 1100 AoE around your target location in the last 25 seconds; For each recent allied teleport in the area, time is added to your Teleport's casting time (2 seconds for the first recent allied teleport, 1 second for each additional one), the default casting time is 3 seconds; Recent allied teleports have no effect if the target is in the fountain area; Shares cooldown with Boots of Travel; Costs 75 Mana, and has a 65 second cooldown |

#### Animal Courier

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 200 | 150 |
| Bonuses | Summon Animal Courier (active) | Summon Animal Courier (active) |
| Ability text | 1 charge; Summon Animal Courier; Summons an Animal Courier to carry items for you; Animal Courier is not affected by most spells; Items an Animal Courier is carrying will drop to the ground if it dies; Costs 5 mana; Full stats for the Animal Courier can be found at the Unit Statistics page | 1 charge; Summon Animal Courier; Summons an Animal Courier to carry items for you; Animal Courier is not affected by most spells; Costs 5 mana; Full stats for the Animal Courier can be found at the Unit Statistics page |

#### Flying Courier

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Supportive Vestments | Ancient of Wonders |
| Total cost | 400 | 370 |
| Components | Animal Courier (200) + Recipe scroll (200) | Animal Courier (150) + Recipe scroll (220) |
| Bonuses | Summon Flying Courier (active) | Summon Flying Courier (active) |
| Ability text | An Animal Courier can be upgraded to a Flying Courier if they carry a Flying Courier recipe; Summon Flying Courier; Creates a Flying Courier to carry items; Flying Courier has 150 HP, 10 armor, Burst (moves at max movespeed for a short time), Courier Shield (invulnerable for a short time), and magic immunity; Flying Courier cannot carry Gem of True Sight or Aegis of the Immortal; Costs 5 mana; Full unit statistics available on the Unit Statistics page | An Animal Courier will be upgraded to a Flying Courier if they carry a Flying Courier recipe; Summon Flying Courier; Creates a Flying Courier to carry items; Flying Courier has 150 HP, 10 armor, Burst (+50% movespeed for a short time) and magic immunity; Flying Courier cannot carry Aegis of the Immortal; Costs 5 mana; Full unit statistics available on the Unit Statistics page |

#### Gem of True Sight

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Cache of Quel-thelan | Cache of Quel-thelan |
| Total cost | 700 | 850 |
| Bonuses | True Sight (passive) | True Sight (passive) |
| Ability text | Drops on death; True Sight; Gives True Sight in 1100 AoE | Maximum stock of 1 in store; Store stock takes 10 minutes to replenish; Drops on death; Cannot be destroyed; ; True Sight; Gives True Sight in 1100 AoE; Disabled while carried by a courier or flying courier |

#### Aegis of the Immortal

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Protectorate | Protectorate |
| Total cost | 0 | 0 |
| Bonuses | Reincarnation (passive) | Reincarnation (passive) |
| Ability text | Drops from Roshan; Reincarnation; Brings you to life with full health and mana 5 seconds after you die, at the location where you died; Aegis of the Immortal is removed from your inventory after reincarnating you once; Aegis of the Immortal is also removed from your inventory when Roshan respawns (10 minutes after he was killed) | Drops from Roshan; Reincarnation; Brings you to life with full health and mana 5 seconds after you die, at the location where you died; Aegis of the Immortal is removed from your inventory after reincarnating you once; Aegis of the Immortal is also removed from your inventory when Roshan respawns (6 minutes after he was killed) |

#### Ring of Basilius

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Supportive Vestments | Supportive Vestments |
| Total cost | 500 | 500 |
| Components | Sobi Mask (325) + Ring of Protection (175) | Sobi Mask (325) + Ring of Protection (175) |
| Bonuses | +6 damage; Mana Aura (passive); Armor Aura (passive) | +6 damage; +1 armor; Mana Aura (passive); Armor Aura (passive) |
| Ability text | Clicking the item will toggle whether Armor Aura affects heroes and units, or just heroes; Can be disassembled; Mana Aura; +0.65 mana regeneration; 900 AoE; Armor Aura; +3 armor; 900 AoE; Does not stack with armor auras from Assault Cuirass, Ring of Basilius, or Vladmir's Offering | Clicking the item will toggle whether Armor Aura affects heroes and units, or just heroes; Can be disassembled; Mana Aura; +0.65 mana regeneration; 900 AoE; Does not stack with mana auras from Ring of Basilius or Ring of Aquila; ; Armor Aura; +2 armor; 900 AoE; Does not stack with armor auras from Assault Cuirass, Ring of Basilius, Ring of Aquila, or Vladmir's Offering |

#### Perseverance

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Gateway Relics | Gateway Relics |
| Total cost | 1750 | 1750 |
| Components | Ring of Health (875) + Void Stone (875) | Ring of Health (875) + Void Stone (875) |
| Bonuses | +10 damage; +5 HP/sec regeneration; +125% mana regeneration | +10 damage; +5 HP/sec regeneration; +125% mana regeneration |
| Ability text | Can be disassembled | Can be disassembled |

#### Lothar's Edge

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient Weaponary | Ancient Weaponary |
| Total cost | 3400 | 3000 |
| Components | Claymore (1400) + Quarterstaff (900) + Recipe scroll (1100) | Claymore (1400) + Shadow Amulet (1600) |
| Bonuses | +38 damage; +10 attack speed; Wind Walk (active) | +22 damage; +30 attack speed; Wind Walk (active) |
| Ability text | Wind Walk; Makes you invisible until the duration ends, or until you attack or use an ability; While invisible you move 20% faster and can move through units; Has a 0.3 second fade time; Attacking to break the invisibility will cause you to deal 125 bonus physical damage on your attack; Lasts 9 seconds, costs 75 mana, and has an 18 second cooldown | Wind Walk; Makes you invisible until the duration ends, or until you attack or use an ability; While invisible you move 20% faster and can move through units; Has a 0.3 second fade time; Attacking to break the invisibility will cause you to deal 150 bonus physical damage on your attack; Lasts 12 seconds, costs 75 mana, and has an 18 second cooldown |

#### Dust of Appearance

| Field | 6.6x (Jan 2010) [A] | 6.72/6.73 (Dec 2011) [C] |
|---|---|---|
| Shop | Ancient of Wonders | Ancient of Wonders |
| Total cost | 180 | 180 |
| Bonuses | Reveal (active) | Reveal (active) |
| Ability text | 2 charges; Reveal; Places a buff on enemy units in 1250 AoE that prevents them from becoming invisible; Lasts 12 seconds, costs 5 mana, and has a 60 second cooldown | Stacks with itself in inventory; 2 charges; Reveal; Places a buff on enemy units in 1050 AoE that prevents them from becoming invisible; Invisible units affected by Dust are slowed by 10%.; Lasts 12 seconds, costs 5 mana, and has a 60 second cooldown |


### Sources

- [A] playdota.com items DB, 2010-01-16 (6.6x, pre-6.67): https://web.archive.org/web/20100116065848/http://www.playdota.com/cache/items.js?242=242
- [B] playdota.com items DB, 2010-12-29: https://web.archive.org/web/20101229100347/http://www.playdota.com/cache/items.js?36=36
- [C] playdota.com items DB, 2011-12-29: https://web.archive.org/web/20111229165805/http://www.playdota.com/cache/items.js?117=117 (rendered at https://web.archive.org/web/20111229165805/http://www.playdota.com/items)
- [M1] playdota Mechanics, Orb Effects, 2011-12-27: https://web.archive.org/web/20111227082539/http://www.playdota.com:80/mechanics/Orb_Effects
- [M2] playdota Mechanics, Shops and Items, 2011-12-15: https://web.archive.org/web/20111215050913/http://www.playdota.com:80/mechanics/shopsanditems
- Version dates: Liquipedia (RU), Версии игры/0.00 до 6.69c, from a search-result snippet because the page fetch was rate-limited: https://liquipedia.net/dota2gameru/%D0%92%D0%B5%D1%80%D1%81%D0%B8%D0%B8_%D0%B8%D0%B3%D1%80%D1%8B/0.00_%D0%B4%D0%BE_6.69c
- Not reachable: dota.fandom.com and dota-allstars.fandom.com returned HTTP 402/403. liquipedia.net returned 429 (rate limit) on direct fetch.

## 4. Ten MVP heroes

**Why these ten.** Together they cover each attribute, both attack styles and the main roles, and all ten have simple, readable kits:

| Hero (DotA 1 name) | Attribute | Attack | Role taught |
|---|---|---|---|
| Sven (Rogue Knight) | STR | Melee | Carry, initiator (AoE stun, cleave) |
| Earthshaker (Raigor) | STR | Melee | Initiator, disabler (Fissure terrain block, Echo Slam) |
| Pudge (Butcher) | STR | Melee | Ganker (skillshot hook, Dismember channel) |
| Juggernaut (Yurnero) | AGI | Melee | Carry (spin magic immunity, Healing Ward, Omnislash) |
| Drow Ranger (Traxex) | AGI | Ranged | Ranged carry (orb attack, silence, aura) |
| Sniper (Kardel) | AGI | Ranged | Ranged carry (range scaling, bash proc, long-cast nuke) |
| Crystal Maiden (Rylai) | INT | Ranged | Support (global mana aura, root, slow) |
| Lina (Slayer) | INT | Ranged | Nuker (line and delayed-AoE skillshots) |
| Lion (Demon Witch) | INT | Ranged | Disabler (hex, line stun, nuke) |
| Zeus (Lord of Olympia) | INT | Ranged | Nuker (global ultimate, % HP passive) |

The two halves below come from separate passes over the same official playdota hero pages. Heroes A uses the **6.72** (Jun 2011) snapshot as its main column. Heroes B uses **6.73** (Jan 2012). Both list the later changes per hero. The 6.72 → 6.73 deltas for Heroes A are in each hero's changelog list.

### Heroes A: Sven, Earthshaker, Pudge, Juggernaut, Drow Ranger (DotA 1 data)

**How this was sourced.** Every number below was taken from the official DotA-Allstars hero database at playdota.com (the IceFrog-era site), read through Wayback Machine snapshots. I pulled the raw HTML for each hero at several dates and diffed them to find when values changed. The pages do not print a map version, so versions are inferred from snapshot dates against release dates. 6.72 came out 2011-04-27 and 6.72d on 2011-07-17 ([Liquipedia game versions](https://www.liquipedia.net/dota2/Game_Versions/6.70_to_6.88f), via search summary). 6.73 is dated 2011-12-24 or 2012-01-12, 6.75 2012-10-04, 6.77 2012-12-19 and 6.78b 2013-06-04 (6.78b date from [Softpedia changelog](https://games.softpedia.com/progChangelog/Warcraft-III-DotA-Allstars-Map-Changelog-12121.html)).

| Snapshot used | Label in this file | Inferred map version |
|---|---|---|
| 2 Jan 2010 | **6.6x** | about 6.64/6.65 (UNVERIFIED exact sub-version) |
| 13 Jun 2011 | **6.72 (baseline)** | 6.72, confirmed by one anchor: Sven Warcry AoE 450→700 is listed as a 6.72c change, and this snapshot shows 700 |
| Nov 2012 (Drow only) | **6.75/6.76** | inferred |
| Jun 2013 | **6.78** | inferred |
| May 2014 (Drow, Pudge) | **6.79–6.81** | inferred |
| May–Sep 2016 | **late 6.8x (last playdota state)** | about 6.81–6.83 (UNVERIFIED; playdota may lag map patches) |

**Version pinning via the official changelogs.** The archived playdota changelog pages (`https://web.archive.org/web/2013/http://www.playdota.com/changelogs/<ver>`) confirm the attributions in each hero's "DotA 1 changelog" list below. Each page contains version N's section first, then the previous version's section. A change is assigned to version N when it first appears on page N. These lists confirm that the 13 Jun 2011 snapshot is 6.72 (it already contains every 6.72 change and none of the 6.73 changes). The Jun 2013 "6.78" snapshot is only partly updated: it has 6.77 and some 6.78 changes, but not the 6.78 Flesh Heap STR change. The changelog pages for 6.81–6.83 were not archived (404), so the late-6.8x values that do not appear in the 6.79/6.80 changelogs have no changelog confirmation.

The **6.72 (baseline)** values are the main tables. Columns or notes give the other snapshots where values differ.

**Conventions (from playdota pages).** "Damage" and "Armor" are the in-game level-1 values, including primary-attribute and agility contributions. Sight Range is day / night. Attack Animation and Casting Animation are point / backswing in seconds. Playdota rounded armor inconsistently in 2010–2011 (for example, Sven "2", which the 2013 page shows as 1.94).

---

#### Sven (Rogue Knight), Sentinel, Strength

Source pages: [6.6x](https://web.archive.org/web/20100102191948/http://www.playdota.com/heroes/rogue-knight), [6.72](https://web.archive.org/web/20110613072647/http://www.playdota.com/heroes/rogue-knight), [6.78](https://web.archive.org/web/20130604082518/http://www.playdota.com/heroes/rogue-knight), [late 6.8x](https://web.archive.org/web/20160913233506/http://www.playdota.com/heroes/rogue-knight)

| Stat | 6.72 | Changes in other snapshots |
|---|---|---|
| Primary | STR | |
| STR | 23 + 2.7 | same in all snapshots |
| AGI | 21 + 2.0 | same |
| INT | 14 + 1.3 | same |
| Damage (lvl 1) | 54–56 | late 6.8x: 60–66 |
| Armor (lvl 1) | 2 (shown as 1.94 in 6.78) | 6.6x: 1.3; late 6.8x: 4.94 |
| Move speed | 295 | |
| Attack range | 128 (melee) | 6.6x page: 125 |
| Base attack time | 1.7 | |
| Attack anim (point / backswing) | 0.4 / 0.3 | |
| Cast anim | 0.3 / 0.51 | |
| Projectile | Instant (melee) | |
| Vision day / night | 1800 / 800 | |

**Storm Bolt** (Active, unit-targeted AoE, hotkey T). Magical damage. The AoE reticle must be centred on an enemy unit. The projectile cannot be dodged by blinking or moving (6.72 note).

| Lvl | Mana | CD | Range | AoE | Stun | Damage |
|---|---|---|---|---|---|---|
| 1 | 140 | 15 | 600 | 255 | 2 s | 100 |
| 2 | 140 | 15 | 600 | 255 | 2 s | 175 |
| 3 | 140 | 15 | 600 | 255 | 2 s | 250 |
| 4 | 140 | 15 | 600 | 255 | 2 s | 325 |

Other snapshots: 6.6x had CD 14 and AoE 250. Late 6.8x has CD 13, and the "can't be dodged" note is removed.

**Great Cleave** (Passive, hotkey C). Cleave is reduced by armor type but not armor value, goes through magic immunity, and hits a circular area in front of Sven.

| Lvl | Cleave AoE | % of attack damage splashed |
|---|---|---|
| 1 | 200 | 20% |
| 2 | 200 | 30% |
| 3 | 200 | 40% |
| 4 | 200 | 50% |

Other snapshots: 6.6x had AoE 175 with the same percentages. 6.78 and late 6.8x have AoE 300 and 20/35/50/65%. The in-between values come from the changelogs: 30/40/50/60% at 200 AoE in 6.73–6.74, then 25/40/55/70% at 300 AoE in 6.75–6.76.

**Warcry** (Active, instant, hotkey W). Buffs Sven and nearby allies. Stacks fully with item armor auras and other movement-speed bonuses.

| Lvl | Mana | CD | AoE | Duration | Bonus armor | Bonus move speed |
|---|---|---|---|---|---|---|
| 1 | 25 | 36 | 700 | 6 s | +3 | +12% |
| 2 | 25 | 30 | 700 | 6 s | +6 | +12% |
| 3 | 25 | 24 | 700 | 6 s | +9 | +12% |
| 4 | 25 | 18 | 700 | 6 s | +12 | +12% |

Other snapshots: 6.6x had AoE 450 and was otherwise the same. 6.78 has AoE 900, duration 7 s and armor +4/8/12/16. Late 6.8x keeps those values with CD 32/26/20/14.

**God's Strength** (Ultimate, instant self-buff, hotkey R). Adds a percentage of base damage, meaning base damage plus primary attribute; raw bonus damage is not increased. There is a 0.3 s transformation time, and stuns can be dodged while transforming. In 6.72 it can only be removed by Purge (Diffusal, Stone Gaze, Unstable Current, Satyr Trickster). The purge note is gone by 6.78.

| Lvl | Mana | CD | Duration | Bonus damage |
|---|---|---|---|---|
| 1 | 100 | 80 | 25 s | +100% |
| 2 | 150 | 80 | 25 s | +125% |
| 3 | 200 | 80 | 25 s | +150% |

Other snapshots: 6.6x had the same values. 6.78 and late 6.8x have +100/150/200%.

**Sven: DotA 1 changelog (playdota changelogs)**
- 6.72: melee attack range set to 128 (Sven was 125 before). Great Cleave AoE 175→200.
- 6.72c (lower section of the 6.73 page): Warcry AoE 450→700.
- 6.73: Great Cleave 20/30/40/50%→30/40/50/60%. God's Strength no longer purgeable, +100/125/150%→+100/140/180%. Warcry duration 6→8 s, and its cast time was removed.
- 6.75: Great Cleave AoE 200→300 with 25/40/55/70%. God's Strength 100/150/200%. Warcry AoE 900 and armor 4/8/12/16.
- 6.77: Great Cleave 20/35/50/65%. Warcry duration 8→7 s.
- 6.79: Storm Bolt CD 15→13, and the bolt is now dodgeable.
- 6.80: base armor +3. Warcry CD 36/30/24/18→32/26/20/14.

---

#### Earthshaker (Raigor Stonehoof), Sentinel, Strength

Source pages: [6.6x](https://web.archive.org/web/20100102202845/http://www.playdota.com/heroes/earthshaker), [6.72](https://web.archive.org/web/20110613071932/http://www.playdota.com/heroes/earthshaker), [6.78](https://web.archive.org/web/20130604085226/http://www.playdota.com/heroes/earthshaker), [late 6.8x](https://web.archive.org/web/20160921025908/http://www.playdota.com/heroes/earthshaker)

| Stat | 6.72 | Changes in other snapshots |
|---|---|---|
| Primary | STR | |
| STR | 22 + 2.5 | late 6.8x: 22 + 2.9 |
| AGI | 12 + 1.4 | |
| INT | 16 + 1.8 | |
| Damage (lvl 1) | 46–56 | |
| Armor (lvl 1) | 2.7 (2.68) | |
| Move speed | 300 | late 6.8x: 310 |
| Attack range | 128 (melee) | |
| Base attack time | 1.7 | |
| Attack anim | 0.467 / 0.863 | |
| Cast anim | 0.69 / 0.5 | |
| Projectile | Instant (melee) | |
| Vision day / night | 1800 / 800 | |

**Fissure** (Active, point-targeted, hotkey F; 6.78 page says unit or point). Magical damage. Creates an impassable crevasse.

| Lvl | Mana | CD | Range | AoE (width) | Stun | Crevasse duration | Damage |
|---|---|---|---|---|---|---|---|
| 1 | 125 | 15 | 1200 | 225 | 1.0 s | 8 s | 125 |
| 2 | 140 | 15 | 1200 | 225 (page says 255, probably a typo) | 1.25 s | 8 s | 175 |
| 3 | 155 | 15 | 1200 | 225 | 1.5 s | 8 s | 225 |
| 4 | 170 | 15 | 1200 | 225 | 1.75 s | 8 s | 275 |

Other snapshots: 6.6x had AoE 250. 6.78 has range 1300. Late 6.8x has range 1400. Fissure length is UNVERIFIED (playdota lists only cast range and width).

**Enchant Totem** (Active, instant self-buff, hotkey E). Lasts 14 s or 1 attack. Increases base damage plus primary-attribute damage only, and does not stack with the Double Damage rune.

| Lvl | Mana | CD | Duration | Bonus damage |
|---|---|---|---|---|
| 1 | 50 | 7 | 14 s / 1 attack | +75% |
| 2 | 50 | 7 | 14 s / 1 attack | +150% |
| 3 | 50 | 7 | 14 s / 1 attack | +225% |
| 4 | 50 | 7 | 14 s / 1 attack | +300% |

Other snapshots: 6.6x had +50/100/150/200%. 6.78 has CD 5. Late 6.8x has CD 5 and +100/200/300/400%.

**Aftershock** (Passive, hotkey A). Magical damage plus a stun around Raigor whenever he casts a spell. Using items does not trigger it.

| Lvl | AoE | Stun | Damage |
|---|---|---|---|
| 1 | 300 | 0.3 s | 25 |
| 2 | 300 | 0.7 s | 45 |
| 3 | 300 | 1.2 s | 75 |
| 4 | 300 | 1.5 s | 115 |

Other snapshots: 6.6x had AoE 280 and was otherwise the same. 6.78 and late 6.8x have stun 0.6/0.9/1.2/1.5 s and damage 50/75/100/125.

**Echo Slam** (Ultimate, instant, hotkey C). Magical damage. The initial damage pierces magic immunity; the echo damage does not. Does not damage ethereal units. The initial blast hits a 500 AoE. Each unit within 550 of the blast emits an echo that damages units around it, including itself. **Aghanim's Scepter:** each hero hit by the initial wave echoes twice (6.78 adds illusions).

| Lvl | Mana | CD | AoE | Initial damage | Damage per echo |
|---|---|---|---|---|---|
| 1 | 145 | 150 | 500 | 160 | 40 |
| 2 | 205 | 130 | 500 | 210 | 55 |
| 3 | 265 | 110 | 500 | 270 | 70 |

Other snapshots: 6.6x had initial damage 200/275/350 and echo damage 35/45/65. Late 6.8x has AoE 550, and each unit caught by the initial blast echoes. The echo-search radius is UNVERIFIED beyond the "550" note above.

**Earthshaker: DotA 1 changelog (playdota changelogs)**
- 6.71: Enchant Totem no longer dispels on animation cancel.
- 6.72: Echo Slam initial damage 165/230/285→160/210/270.
- 6.75: Enchant Totem CD 7→6.
- 6.77: Aftershock stun 0.3/0.7/1.2/1.5→0.6/0.9/1.2/1.5 and damage 25/45/75/115→50/75/100/125. Enchant Totem CD 6→5.
- 6.78: turn rate 0.4→0.6, so it is **0.4 in 6.72**. Fissure range 1200→1300. Echo Slam counts illusions as heroes (affects Aghanim's Scepter).
- 6.79: Enchant Totem 75/150/225/300%→100/200/300/400%. Fissure range +100.
- 6.80: move speed 300→310. STR gain 2.5→2.9.

---

#### Pudge (Butcher), Scourge, Strength

Source pages: [6.6x](https://web.archive.org/web/20100102195103/http://www.playdota.com/heroes/butcher), [6.72](https://web.archive.org/web/20110613064805/http://www.playdota.com/heroes/butcher), [6.78](https://web.archive.org/web/20130601152139/http://www.playdota.com/heroes/butcher), [6.79–6.81](https://web.archive.org/web/20140621154804/http://www.playdota.com/heroes/butcher), [late 6.8x](https://web.archive.org/web/20160922000250/http://www.playdota.com/heroes/butcher)

| Stat | 6.72 | Changes in other snapshots |
|---|---|---|
| Primary | STR | |
| STR | 25 + 3.2 | same in all snapshots |
| AGI | 14 + 1.5 | |
| INT | 14 + 1.5 | |
| Damage (lvl 1) | 52–58 | |
| Armor (lvl 1) | 2 | 6.78 and later: 0.96 |
| Move speed | 285 | |
| Attack range | 128 (melee) | |
| Base attack time | 1.7 | |
| Attack anim | 0.5 / 1.17 | |
| Cast anim | 0.3 / 0.51 | |
| Projectile | Instant (melee) | |
| Vision day / night | 1800 / 800 | |

**Meat Hook** (Active, point-targeted, hotkey T). **Pure** damage. Hooks the first unit hit and drags it back to Pudge. Passes through cliffs, trees, structures and siege creeps; only the tip catches. Enemies take damage plus a ministun that interrupts. Allies can be pulled without damage. In 6.72 an invisible unit hooked takes damage but no ministun; by 6.78 invisible units are "hooked normally". Late 6.8x adds 5 s of free pathing if the target is pulled onto unpathable terrain. Hook speed is UNVERIFIED.

| Lvl | Mana | CD | Range | Hook width (AoE) | Damage |
|---|---|---|---|---|---|
| 1 | 110 | 14 | 600 | 100 | 90 |
| 2 | 120 | 13 | 800 | 100 | 180 |
| 3 | 130 | 12 | 1000 | 100 | 270 |
| 4 | 140 | 11 | 1200 | 100 | 360 |

Other snapshots: 6.6x had range 550/700/850/1000, width 125 and damage 100/200/300/400. 6.78 through late 6.8x match 6.72.

**Rot** (Toggle, hotkey R). Magical damage per second to enemies and to Pudge himself, and Pudge can deny himself with it. Mana 0, CD 0, AoE 250, lasts until toggled off, slows enemy move speed by 20%. Can be turned on during Dismember without interrupting it. Hits invisible units. Cannot be turned off while Pudge is silenced. Late 6.8x adds that the slow works on magic-immune units.

| Lvl | Damage/sec | Slow |
|---|---|---|
| 1 | 35 | 20% |
| 2 | 60 | 20% |
| 3 | 85 | 20% |
| 4 | 110 | 20% |

Other snapshots: 6.6x had 25/50/75/100 per second.

**Flesh Heap** (Passive, hotkey F). Gives spell resistance, which stacks with items, and permanent STR. STR is gained when an enemy hero dies within 450 of Pudge or when Pudge kills one. **Retroactive:** past kills recount at the new rate when the skill is levelled. `-st` shows the STR gained.

| Lvl | Spell resistance | STR per hero death |
|---|---|---|
| 1 | 4% | +0.9 |
| 2 | 8% | +1.2 |
| 3 | 12% | +1.5 |
| 4 | 16% | +1.8 |

Other snapshots: 6.6x matched 6.72, and so did 6.78. 6.79–6.81 and late 6.8x have 6/8/10/12% and +1/1.5/2/2.5 STR.

**Dismember** (Ultimate, channelled, unit-targeted, hotkey D). Magical damage. The disable works on magic-immune units, and the target stays disabled if it turns invisible.

| Lvl | Mana | CD | Range | Duration | Damage/sec | With Aghanim's Scepter |
|---|---|---|---|---|---|---|
| 1 | 100 | 30 | 160 | 3 s heroes / 6 s creeps | 75 | 75 + 0.75×STR per second, heals Pudge for the damage dealt |
| 2 | 130 | 30 | 160 | 3 s / 6 s | 125 | 125 + 0.75×STR |
| 3 | 170 | 30 | 160 | 3 s / 6 s | 175 | 175 + 0.75×STR |

Other snapshots: 6.6x and 6.78 match 6.72. 6.79–6.81 and late 6.8x change the Scepter bonus to 1.0×STR per second, still with the heal.

**Pudge: DotA 1 changelog (playdota changelogs)**
- 6.72: Rot 25/50/75/100→35/60/85/110 DPS. Fixed Meat Hook while ethereal, and Meat Hook wasting Linken's.
- 6.73: Flesh Heap overhead STR notification.
- 6.78: Flesh Heap STR per kill 0.9/1.2/1.5/1.8→1/1.5/2/2.5. The Jun 2013 playdota snapshot had not picked this up yet.
- 6.79: Pudge can use Blink Dagger. Hooked units dropped on unpathable ledges get 5 s of free pathing. Flesh Heap magic resistance 4/8/12/16%→6/8/10/12%. Dismember Scepter STR multiplier 0.75→1.0.

---

#### Juggernaut (Yurnero), Sentinel, Agility

Source pages: [6.6x](https://web.archive.org/web/20100102213755/http://www.playdota.com/heroes/juggernaut), [6.72](https://web.archive.org/web/20110613070057/http://www.playdota.com/heroes/juggernaut), [6.78](https://web.archive.org/web/20130604084711/http://www.playdota.com/heroes/juggernaut), [late 6.8x](https://web.archive.org/web/20160526033742/http://www.playdota.com/heroes/juggernaut)

| Stat | 6.72 | Changes in other snapshots |
|---|---|---|
| Primary | AGI | |
| STR | 20 + 1.9 | |
| AGI | 20 + 2.85 | late 6.8x: 20 + 2.4 |
| INT | 14 + 1.4 | |
| Damage (lvl 1) | 44–48 | |
| Armor (lvl 1) | 3.9 (3.8 in 6.78) | |
| Move speed | 305 | |
| Attack range | 128 (melee) | 6.6x page: 100 |
| Base attack time | 1.7 | 6.78: 1.6; late 6.8x: 1.4 |
| Attack anim | 0.33 / 0.84 | |
| Cast anim | 0.3 / 0.51 | |
| Projectile | Instant (melee) | |
| Vision day / night | 1800 / 800 | |

**Blade Fury** (Active, instant, hotkey F). Juggernaut becomes **magic immune** and deals magical damage per second in a 250 radius for 5 s. Does not affect mechanical units. Items, TP Scroll and Boots of Travel can be used during it. In 6.72 he can still attack units that Blade Fury does not affect. Late 6.8x adds that attacks during Blade Fury deal no damage, except against towers and mechanical units.

| Lvl | Mana | CD | AoE | Duration | Damage/sec |
|---|---|---|---|---|---|
| 1 | 110 | 30 | 250 | 5 s | 80 |
| 2 | 110 | 28 | 250 | 5 s | 100 |
| 3 | 110 | 26 | 250 | 5 s | 120 |
| 4 | 110 | 24 | 250 | 5 s | 140 |

Other snapshots: 6.6x had CD 30 at every level. Late 6.8x has mana 120/110/100/90 and CD 30/27/24/21.

**Healing Ward** (Active, point-targeted, hotkey G). Summons a mobile ward that heals allies by a percentage of their **max HP per second**. The ward can be ordered to move (movespeed 300 in 6.72, 450 in late 6.8x). Overlapping wards do not stack, and the ward does not heal mechanical units. The ward's own HP/armor is UNVERIFIED (not on playdota).

| Lvl | Mana | CD | Range | Heal AoE | Duration | Heal |
|---|---|---|---|---|---|---|
| 1 | 80 | 60 | 350 | 500 | 25 s | 2% max HP/s |
| 2 | 100 | 60 | 350 | 500 | 25 s | 3%/s |
| 3 | 120 | 60 | 350 | 500 | 25 s | 4%/s |
| 4 | 140 | 60 | 350 | 500 | 25 s | 5%/s |

Other snapshots: 6.6x had mana 140 at every level and 1/2/3/4% per second. Late 6.8x has mana 120/125/130/135.

**Blade Dance** (Passive, hotkey C). Chance to deal double damage (×2 crit).

| Lvl | Crit chance | Multiplier |
|---|---|---|
| 1 | 15% | 2× |
| 2 | 20% | 2× |
| 3 | 25% | 2× |
| 4 | 35% | 2× |

Other snapshots: 6.6x had 10/18/26/36%.

**Omnislash** (Ultimate, unit-targeted, hotkey E). Juggernaut is invulnerable while slashing. Physical damage, with 0.4 s between slashes. The search radius for the next target is 425 (6.72 description says 450; 6.6x table says 450). Interrupts the primary target's channelling. Slashes ignore evasion, but normal attacks made during the slash do not. With enough attack speed he also makes normal attacks, which can apply buffs and procs. Items can be used during it.

| Lvl | Mana | CD (Scepter) | Range | Jump AoE | Slashes (Scepter) | Damage per slash |
|---|---|---|---|---|---|---|
| 1 | 200 | 130 (110) | 450 | 425 | 3 (5) | 150–250 |
| 2 | 275 | 120 (100) | 450 | 425 | 5 (7) | 150–250 |
| 3 | 350 | 110 (90) | 450 | 425 | 8 (10) | 150–250 |

Other snapshots: 6.6x matched 6.72 except jump AoE 450. 6.78 has 3/6/9 slashes (6/9/12 with Scepter) at 175–250 per slash. It also adds 200 flying vision during the slash, kills lane and neutral creeps in one hit, and does not jump to Tombstone zombies. Late 6.8x has 200–225 per slash, a Scepter CD of 70 at every level, and allows Healing Ward to be cast during Omnislash.

**Juggernaut: DotA 1 changelog (playdota changelogs)**
- 6.70: Healing Ward move speed 150→300. Banished units are no longer immune to Blade Fury.
- 6.71: Healing Ward heal 1/2/4/5%→2/3/4/5% per second.
- 6.72: Blade Dance 10/20/25/35%→15/20/25/35%. Melee range set to 128 (Juggernaut was 100). New Omnislash cast animation.
- 6.73: Omnislash damage 150–250→175–250, hits 3/5/8 (5/7/10 with Scepter)→3/6/9 (6/9/12).
- 6.74: Omnislash gives 200 flying vision.
- 6.75: BAT 1.7→1.6. Omnislash kills lane creeps and neutrals in one hit (not player units or ancients).
- 6.78: Omnislash no longer jumps to Tombstone zombies.
- 6.79: Healing Ward move speed 300→450. Omnislash Scepter CD 110/100/90→70.
- 6.80: Blade Fury CD 30/28/26/24→30/27/24/21.
- Late values with no changelog found (6.81–6.83 changelog pages are not archived): AGI gain 2.4, BAT 1.4, Omnislash 200–225, Blade Fury mana 120/110/100/90, Healing Ward mana 120/125/130/135. UNVERIFIED as to version.

---

#### Drow Ranger (Traxex), Sentinel, Agility

Source pages: [6.6x](https://web.archive.org/web/20100102/http://www.playdota.com/heroes/drow-ranger), [6.72](https://web.archive.org/web/20110613/http://www.playdota.com/heroes/drow-ranger), [6.75/6.76](https://web.archive.org/web/20121101104044/http://www.playdota.com/heroes/drow-ranger), [6.79–6.81](https://web.archive.org/web/20140519070350/http://www.playdota.com/heroes/drow-ranger/), [late 6.8x](https://web.archive.org/web/20160801/http://www.playdota.com/heroes/drow-ranger)

| Stat | 6.72 | Changes in other snapshots |
|---|---|---|
| Primary | AGI | |
| STR | 17 + 1.9 | |
| AGI | 22 + 1.9 | 6.75/6.76 and later: 26 + 1.9 |
| INT | 15 + 1.4 | |
| Damage (lvl 1) | 40–51 | 6.75/6.76 and later: 44–55 |
| Armor (lvl 1) | 2.1 | 6.75/6.76: 2.64; 6.79–6.81 and later: 0.64 |
| Move speed | 300 | |
| Attack range | 625 | |
| Base attack time | 1.7 | |
| Attack anim | 0.7 / 0.3 | |
| Cast anim | 0.4 / 0.5 | |
| Projectile speed | 1250 | |
| Vision day / night | **1800 / 1700** (6.6x–6.76) | 6.79–6.81 and later: 1800 / 800 |

**Frost Arrows** (Active autocast **orb effect** and buff placer, unit-targeted, hotkey R). Slows last 1.5 s on heroes and 7 s on creeps. Repeated hits refresh the duration; they do not stack.

| Lvl | Mana per arrow | CD | Range | Attack-speed slow | Move-speed slow |
|---|---|---|---|---|---|
| 1 | 12 | 0 | 625 | 5% | 10% |
| 2 | 12 | 0 | 625 | 10% | 20% |
| 3 | 12 | 0 | 625 | 15% | 30% |
| 4 | 12 | 0 | 625 | 20% | 40% |

Other snapshots: 6.6x had range 600 with the same slows. The changelogs show 6.73 dropping the attack-speed slow and changing move-speed slow to 11/24/37/50%; 6.75 raised level 4 to 60%. The 6.75/6.76 snapshot shows 11/24/37/60%.

**Silence** (Active, area-targeted, hotkey E). Silences all enemies in the area. Does not block item use, and it disables permanent invisibility.

| Lvl | Mana | CD | Range | AoE | Silence |
|---|---|---|---|---|---|
| 1 | 90 | 15 | 900 | 275 | 3 s |
| 2 | 90 | 15 | 900 | 275 | 4 s |
| 3 | 90 | 15 | 900 | 275 | 5 s |
| 4 | 90 | 15 | 900 | 275 | 6 s |

Other snapshots: 6.75/6.76 has CD 13 and AoE 300. In 6.79–6.81 the skill is replaced by **Gust**: mana 90, CD 13, range 900, width 250, silence 3/4/5/6 s. Gust travels at 2000 speed and reaches up to 1150 units. It knocks back up to 350 units, with distance inversely proportional to the target's distance from Drow. Late 6.8x lists knockback durations of 0.6/0.7/0.8/0.9 s.

**Trueshot Aura** (Passive aura, hotkey T).

- **6.6x–6.72:** radius **900** (not global). Affects allied **ranged units only**, adding a percentage of base damage. That means base damage plus the primary-attribute bonus, not item damage.

| Lvl | Radius | Bonus to ranged allies |
|---|---|---|
| 1 | 900 | +7% base damage |
| 2 | 900 | +14% |
| 3 | 900 | +21% |
| 4 | 900 | +28% |

Other snapshots: 6.75/6.76 has radius 900 and +8/16/24/32%. In 6.79–6.81 and late 6.8x it becomes a **global** aura for allied ranged **heroes** that grants 18/24/30/36% of Drow's agility as damage. It can also be cast (mana 0, CD 120, lasts 30 s) to extend it to ranged creeps globally.

**Marksmanship** (Ultimate, passive, hotkey M). Adds agility to Drow.

| Lvl | Bonus AGI |
|---|---|
| 1 | +15 |
| 2 | +30 |
| 3 | +45 |

Other snapshots: 6.75/6.76 has +15/30/60. 6.79–6.81 and late 6.8x have +40/60/80, and the bonus is **lost while an enemy hero is within 400**. No other effect, such as the Dota 2 pierce/bonus-damage proc, appears on any playdota snapshot.

**Drow Ranger: DotA 1 changelog (playdota changelogs)**
- 6.73: Frost Arrows lose the attack-speed slow. Move-speed slow 10/20/30/40%→11/24/37/50%.
- 6.74: Silence AoE 275→300.
- 6.75: Frost Arrows level 4 50%→60%. Marksmanship level 3 45→60. Trueshot 7/14/21/28%→8/16/24/32%. Silence CD 15→13.
- 6.76: **Trueshot Aura reworked into a global aura**: "you and all allied ranged units" gain 14/18/22/26% of Drow's agility as damage, and it can be toggled to include or exclude creeps. **Marksmanship reworked** to +20/30/40 AGI, doubled when no enemy hero is within 375.
- 6.77: Trueshot affects non-hero units only within 900 of Drow. Drow base armor −2. Marksmanship focus AoE 375→400.
- 6.78: Trueshot 16/20/24/28%. Marksmanship bonus is removed (instead of halved) when enemy heroes are near; detection AoE 400→375.
- 6.79: night vision 1700→standard 800. Trueshot 18/24/30/36% and no longer affects creeps except through a manual cast (global, 30 s, CD 120). Marksmanship negation AoE 375→400.
- 6.80: Silence reworked into Gust.
- Late Marksmanship +40/60/80 is from the 2014/2016 snapshots; no changelog was found for it (UNVERIFIED as to version).

---

#### DotA 1 vs Dota 2 differences (the requested hot spots)

The DotA 1 side is from the playdota sources above. The Dota 2 side is general knowledge of modern Dota 2 and is *not* cited from these sources.

- **Juggernaut Healing Ward.** In DotA 1 it heals 2–5% of max HP per second (6.72), or 1–4% in 6.6x, in a 500 radius for 25 s. The ward is movable, with movespeed 300 in 6.72 and 450 late. Modern Dota 2 has changed the ward many times (HP-to-kill, values).
- **Drow Trueshot Aura.** From 6.6x through 6.75 it is a **900-radius** percentage-of-base-damage aura for ranged units. From 6.76 it is **global**, agility-based and applies to heroes (6.77: creeps only within 900). From 6.79 it is hero-only, with an optional cast for creeps. Dota 2 later replaced it with Precision Aura.
- **Drow Marksmanship.** In DotA 1 it is pure flat agility (+15/30/45 in 6.72), with the 400-radius enemy-hero disable added only in late 6.8x. Dota 2 Marksmanship is a proc that ignores armor and grants bonus damage, which is absent in DotA 1.
- **Drow Silence vs Gust.** It is AoE Silence up to the 6.7x snapshots; Gust (knockback and silence) appears by the 2014 snapshot.
- **Pudge Flesh Heap.** DotA 1 gives *spell resistance* (4/8/12/16% in 6.72) plus STR per hero death within 450. Dota 2 has moved to different mechanics (no magic-resistance component in recent patches).
- **Sven God's Strength.** In DotA 1 it adds a percentage of base damage (+100/125/150% in 6.72, +100/150/200% later), costs 100/150/200 mana and lasts 25 s with an 80 s CD. Warcry in DotA 1 adds armor and move speed; Dota 2 Warcry later added a barrier.
- **Juggernaut Omnislash.** DotA 1 slashes deal fixed damage (150–250 or later 175–250 / 200–225), not attack-damage-based. Blade Dance is a 2× crit.

#### Sources

- Playdota hero database (DotA-Allstars official site) via the Wayback Machine. The raw HTML was retrieved with `id_` URLs, and the values above were parsed from the per-level skill tables.
  - Sven: https://web.archive.org/web/20100102191948/http://www.playdota.com/heroes/rogue-knight · https://web.archive.org/web/20110613072647/http://www.playdota.com/heroes/rogue-knight · https://web.archive.org/web/20130604082518/http://www.playdota.com/heroes/rogue-knight · https://web.archive.org/web/20160913233506/http://www.playdota.com/heroes/rogue-knight
  - Earthshaker: https://web.archive.org/web/20100102202845/http://www.playdota.com/heroes/earthshaker · https://web.archive.org/web/20110613071932/http://www.playdota.com/heroes/earthshaker · https://web.archive.org/web/20130604085226/http://www.playdota.com/heroes/earthshaker · https://web.archive.org/web/20160921025908/http://www.playdota.com/heroes/earthshaker
  - Pudge: https://web.archive.org/web/20100102195103/http://www.playdota.com/heroes/butcher · https://web.archive.org/web/20110613064805/http://www.playdota.com/heroes/butcher · https://web.archive.org/web/20130601152139/http://www.playdota.com/heroes/butcher · https://web.archive.org/web/20140621154804/http://www.playdota.com/heroes/butcher · https://web.archive.org/web/20160922000250/http://www.playdota.com/heroes/butcher
  - Juggernaut: https://web.archive.org/web/20100102213755/http://www.playdota.com/heroes/juggernaut · https://web.archive.org/web/20110613070057/http://www.playdota.com/heroes/juggernaut · https://web.archive.org/web/20130604084711/http://www.playdota.com/heroes/juggernaut · https://web.archive.org/web/20160526033742/http://www.playdota.com/heroes/juggernaut
  - Drow Ranger: https://web.archive.org/web/20100102/http://www.playdota.com/heroes/drow-ranger · https://web.archive.org/web/20110613/http://www.playdota.com/heroes/drow-ranger · https://web.archive.org/web/20121101104044/http://www.playdota.com/heroes/drow-ranger · https://web.archive.org/web/20140519070350/http://www.playdota.com/heroes/drow-ranger/ · https://web.archive.org/web/20160801/http://www.playdota.com/heroes/drow-ranger
- Official DotA-Allstars changelogs on playdota (Wayback): https://web.archive.org/web/2016/http://www.playdota.com/changelogs/6.70 · https://web.archive.org/web/2016/http://www.playdota.com/changelogs/6.71 · https://web.archive.org/web/2013/http://www.playdota.com/changelogs/6.72 · https://web.archive.org/web/2013/http://www.playdota.com/changelogs/6.73 · …/6.74 · …/6.75 · …/6.76 · …/6.77 · …/6.78 · …/6.79 · https://web.archive.org/web/2013/http://www.playdota.com/changelogs/6.80 (6.81–6.83 pages not archived)
- Version release dates (search-result summaries; Liquipedia blocked direct fetch): https://www.liquipedia.net/dota2/Game_Versions/6.70_to_6.88f · https://games.softpedia.com/progChangelog/Warcraft-III-DotA-Allstars-Map-Changelog-12121.html
- 6.72c Warcry AoE 450→700 anchor (search-result summary): https://liquipedia.net/dota2/Archive:Version_6.72c
- 6.73 Great Cleave entry (also matches the playdota 6.73 changelog): https://dota2.fandom.com/wiki/6.73 · https://store.steampowered.com/news/7155

### Heroes B: Sniper, Crystal Maiden, Lina, Lion, Zeus (DotA 1)

**Baseline version: 6.73 / 6.73c (December 2011).** The tables come from playdota.com hero pages (the official DotA 1 site), as archived by the Wayback Machine in January 2012. I matched that snapshot against the official changelogs. It includes every 6.73 change for these heroes (for example Lina +3 base INT, Dragon Slave final AoE 150, Lion Mana Drain 120/s at 0.25 s ticks, Zeus Lightning Bolt cooldown 6, Static Field hits creeps) and none of the 6.74 changes (Voodoo is still on a 13 s cooldown). So it is 6.73-era data. Each hero has a "Changes 6.74 to 6.83" list taken from the official changelogs.

How to read the playdota fields:
- "Damage" and "Armor" are the **level-1** values with starting attributes included. Armor matches −1 + AGI×0.14 for every hero except Zeus, whose base is 0. For example Sniper: 21×0.14 − 1 = 1.94.
- "Attack Animation" and "Casting Animation" are given as *point / backswing*.
- "Sight Range" is day / night.

Shorthand for sources (full URLs at the end):
- [PD-x-2012]: the archived hero page from January 2012.
- [PD-x-2014]: the archived hero page from mid-2014.
- [CL-6.xx]: the playdota changelog for that version.

Caution: the 2014 playdota pages were **not fully up to date**. For example the 2014 Zeus page lacks the 6.81b Lightning Bolt changes. For 6.8x, trust the changelog entries over the 2014 pages.

---

#### Sniper (Kardel Sharpeye), Sentinel, Agility

| Stat | 6.73 value | Source |
|---|---|---|
| STR | 16 + 1.7 | [PD-sniper-2012] |
| AGI (primary) | 21 + 2.9 | [PD-sniper-2012] |
| INT | 15 + 2.6 | [PD-sniper-2012] |
| Damage (level 1) | 36–42 | [PD-sniper-2012] |
| Armor (level 1) | 1.94 | [PD-sniper-2012] |
| Move speed | 290 | [PD-sniper-2012] |
| Attack range | 550 (before Take Aim) | [PD-sniper-2012] |
| Base attack time | 1.7 | [PD-sniper-2012] |
| Attack animation (point/backswing) | 0.17 / 0.7 | [PD-sniper-2012] |
| Cast animation (point/backswing) | 0.3 / 0.51 | [PD-sniper-2012] |
| Projectile speed | 3000 | [PD-sniper-2012] |
| Sight day/night | 1800 / 1000 (night raised from 800 to 1000 in 6.68) | [PD-sniper-2012], [CL-6.68] |

**Shrapnel** (active, area target, hotkey R) [PD-sniper-2012]

| Lvl | Mana | CD | Range | AoE | Duration | Effect |
|---|---|---|---|---|---|---|
| 1 | 120 | 15 | 1200 | 350 | 8 s | 15% slow, 12 dps |
| 2 | 120 | 15 | 1200 | 350 | 8 s | 20% slow, 24 dps |
| 3 | 120 | 15 | 1200 | 350 | 8 s | 25% slow, 36 dps |
| 4 | 120 | 15 | 1200 | 350 | 8 s | 30% slow, 48 dps |

Notes:
- Magical damage, dealt 9 times: once when the effect lands, then every second.
- The effect is delayed 0.8 s.
- Deals 1/3 damage to buildings.
- In 6.73 it is a single cast with a cooldown, not charges.

**Headshot** (passive, hotkey O) [PD-sniper-2012]

| Lvl | Chance | Bonus dmg | Ministun |
|---|---|---|---|
| 1 | 25% | 30 | 0.01 s |
| 2 | 30% | 30 | 0.1 s |
| 3 | 35% | 40 | 0.2 s |
| 4 | 40% | 50 | 0.2 s |

Notes:
- Damage type is "hero physical".
- Works on allied creeps (when denying).
- Magic immunity blocks the stun.

**Take Aim** (passive, hotkey E) [PD-sniper-2012]

| Lvl | Bonus attack range |
|---|---|
| 1–4 | +65 / +130 / +195 / +260 |

Notes:
- Levels 3 and 4 let Sniper out-range towers.
- History: 55/110/165/220 before 6.61 [CL-6.61].

**Assassinate** (ultimate, unit target, hotkey T) [PD-sniper-2012]

| Lvl | Mana | CD | Range | Damage |
|---|---|---|---|---|
| 1 | 175 | 20 | 1500 | 355 |
| 2 | 275 | 15 | 2000 | 505 |
| 3 | 375 | 10 | 2500 | 655 |

Notes:
- Magical damage.
- Takes 1.7 s to cast.
- The crosshair is visible to allies only.
- Invisibility does not stop it.
- Interrupts channeling, including on magic-immune units.
- No Aghanim's upgrade is listed for 6.73.

**Sniper changes from 6.74 to 6.83:**
- **6.74:** Headshot damage 30/40/50/60 [CL-6.74].
- **6.75:**
  - Headshot damage 30/45/60/75.
  - Take Aim +75/150/225/300.
  - Shrapnel AoE 360, duration 9 s.

  [CL-6.75]
- **6.76:**
  - Headshot becomes 40% chance at all levels, 15/40/65/90 damage, 0.25 s ministun, and its procs cannot be evaded.
  - Shrapnel cast range 1800 and gives vision of the area.
  - Assassinate range 2000/2500/3000.

  [CL-6.76]
- **6.78:** Shrapnel AoE 400 [CL-6.78].
- **6.79:** Take Aim +80/160/240/320 [CL-6.79].
- **6.80:** Take Aim +100/200/300/400 [CL-6.80].
- **6.81b:** Fixed acquisition range not matching maximum attack range [CL-6.81b].

The mid-2014 page shows these 6.8x values, including 10 Shrapnel ticks and Shrapnel targeting "Point" [PD-sniper-2014].

**DotA 1 vs DotA 2:**
- In 6.73 Headshot is a variable-chance bash-style proc with a ministun. It is not a knockback/slow.
- Take Aim is a pure passive range bonus with no active component.
- Shrapnel is a single cast on a cooldown, not charge-based.

Shrapnel charges in later DotA 1 versions: UNVERIFIED. The 2014 page still lists a 15 s cooldown.

---

#### Crystal Maiden (Rylai Crestfall), Sentinel, Intelligence

| Stat | 6.73 value | Source |
|---|---|---|
| STR | 16 + 1.7 | [PD-cm-2012] |
| AGI | 16 + 1.6 | [PD-cm-2012] |
| INT (primary) | 21 + 2.9 | [PD-cm-2012] |
| Damage (level 1) | 38–44 | [PD-cm-2012] |
| Armor (level 1) | 1.24 | [PD-cm-2012] |
| Move speed | 280 | [PD-cm-2012] |
| Attack range | 600 | [PD-cm-2012] |
| Base attack time | 1.7 | [PD-cm-2012] |
| Attack animation (point/backswing) | 0.55 / 0 (as listed) | [PD-cm-2012] |
| Cast animation (point/backswing) | 0.3 / 2.4 | [PD-cm-2012] |
| Projectile speed | 900 | [PD-cm-2012] |
| Sight day/night | 1800 / 800 | [PD-cm-2012] |

**Crystal Nova** (active, area, hotkey V) [PD-cm-2012]

| Lvl | Mana | CD | Range | AoE | Slow duration | Damage |
|---|---|---|---|---|---|---|
| 1 | 100 | 15 | 700 | 400 | 3.5 s | 100 |
| 2 | 120 | 15 | 700 | 400 | 4 s | 150 |
| 3 | 140 | 15 | 700 | 400 | 4.5 s | 200 |
| 4 | 160 | 15 | 700 | 400 | 5 s | 250 |

Notes:
- Magical damage.
- Slows movement speed by 30% and attack speed by 20%.
- Damage and duration were set in 6.69 (previously 80/130/180/230 damage with a 5 s slow) [CL-6.69].

**Frostbite** (active, unit target, hotkey E) [PD-cm-2012]

| Lvl | Mana | CD | Range | Duration | Effect |
|---|---|---|---|---|---|
| 1 | 115 | 10 | 500 | 1.5 s | root + disarm, 70 dmg/s |
| 2 | 125 | 10 | 500 | 2 s | same |
| 3 | 140 | 10 | 500 | 2.5 s | same |
| 4 | 150 | 10 | 500 | 3 s | same |

Notes:
- Magical damage.
- Blocks blink and metamorphosis skills while active.
- Lasts 10 s on creeps of level 5 or lower.

**Brilliance Aura** (passive, **global**, hotkey R) [PD-cm-2012]

| Lvl | Mana regen to all allies on the map |
|---|---|
| 1–4 | +0.5 / +1.0 / +1.5 / +2.0 per second |

Notes:
- Stacks with Ring of Basilius's aura.
- Not counted by percentage mana-regen multipliers.
- History: 0.75/1.5/2.25/3 before 6.60 [CL-6.60b].

**Freezing Field** (ultimate, instant, channeled, hotkey F, Aghanim's upgrade) [PD-cm-2012]

| Lvl | Mana | CD | Total AoE | Explosion AoE | Duration | Dmg per explosion (Aghanim's) |
|---|---|---|---|---|---|---|
| 1 | 200 | 150 | 630 | 190 | 4 s | 105 (170) |
| 2 | 400 | 120 | 630 | 190 | 4 s | 170 (250) |
| 3 | 600 | 90 | 630 | 190 | 4 s | 250 (310) |

Notes:
- Magical damage.
- One explosion every 0.1 s, 40 in total.
- Slows movement speed by 30% and attack speed by 20% for every enemy within 630, lasting 1 s, even if no explosion hits them.
- Level-1 mana cost dropped from 300 to 200 in 6.72 [CL-6.72].
- The hotkey changed from Z to F in 6.72c [CL-6.72c].
- In 6.73 the Aghanim's upgrade raises damage only. The page lists no cooldown change.

**Crystal Maiden changes from 6.74 to 6.83:**
- **6.74:** Freezing Field AoE +100, with explosion AoE adjusted [CL-6.74].
- **6.75:** Freezing Field duration 7 s [CL-6.75].
- **6.76:** Brilliance Aura works twice as strongly on Crystal Maiden herself [CL-6.76].
- **6.78:**
  - Brilliance Aura 1/1.5/2/2.5.
  - Freezing Field explosion AoE goes from 215 to 230.
  - The Aghanim's attack-speed slow becomes −50.

  [CL-6.78]
- **6.79:** Freezing Field slow becomes −30 attack speed / −30% move speed. With Aghanim's it is −50/−50 [CL-6.79].
- **6.80:** Base intelligence reduced by 3 [CL-6.80].

The 2014 page shows:
- Freezing Field 7 s, 70 explosions, slow AoE 685, comet area 710, explosion AoE 230.
- Brilliance Aura 1/1.5/2/2.5, with 2/3/4/5 on CM herself.
- INT listed as "16 + 2.9".

[PD-cm-2014]

Crystal Maiden's base INT after 6.80: UNVERIFIED. 21 − 3 = 18, but the 2014 page says 16, and I found no other changelog line that would explain the difference.

**DotA 1 vs DotA 2:**
- Brilliance Aura is **global**, affecting all allied units on the whole map.
- Frostbite also deals damage over time.
- Freezing Field slows both movement and attack speed.

---

#### Lina (Lina Inverse, "Slayer"), Sentinel, Intelligence

| Stat | 6.73 value | Source |
|---|---|---|
| STR | 18 + 1.5 | [PD-lina-2012] |
| AGI | 16 + 1.5 | [PD-lina-2012] |
| INT (primary) | 27 + 3.2 | [PD-lina-2012] |
| Damage (level 1) | 40–58 | [PD-lina-2012] |
| Armor (level 1) | 1.24 | [PD-lina-2012] |
| Move speed | 295 | [PD-lina-2012] |
| Attack range | 625 (raised from 600 in 6.71) | [PD-lina-2012], [CL-6.71] |
| Base attack time | 1.7 | [PD-lina-2012] |
| Attack animation (point/backswing) | 0.75 / 0.78 | [PD-lina-2012] |
| Cast animation (point/backswing) | 0.45 / 1.08 | [PD-lina-2012] |
| Projectile speed | 900 | [PD-lina-2012] |
| Sight day/night | 1800 / 800 | [PD-lina-2012] |

**Dragon Slave** (active, point/line, hotkey D) [PD-lina-2012]

| Lvl | Mana | CD | Range | Width start/end, distance | Damage |
|---|---|---|---|---|---|
| 1 | 90 | 8.5 | 800 | 275 / 150, 800 | 100 |
| 2 | 105 | 8.5 | 800 | 275 / 150, 800 | 170 |
| 3 | 125 | 8.5 | 800 | 275 / 150, 800 | 230 |
| 4 | 140 | 8.5 | 800 | 275 / 150, 800 | 280 |

Notes:
- Magical damage.
- Can hit units up to 1225 away.
- 6.73 increased travel distance by 100 and final AoE from 125 to 150 [CL-6.73].

**Light Strike Array** (active, area, hotkey T) [PD-lina-2012]

| Lvl | Mana | CD | Range | AoE | Stun | Damage |
|---|---|---|---|---|---|---|
| 1 | 90 | 7 | 600 | 225 | 1.6 s | 90 |
| 2 | 100 | 7 | 600 | 225 | 1.6 s | 150 |
| 3 | 110 | 7 | 600 | 225 | 1.6 s | 210 |
| 4 | 125 | 7 | 600 | 225 | 1.6 s | 280 |

Notes:
- Magical damage.
- 0.5 s delay before it lands.
- Cooldown history: 10, then 8 in 6.67, then 7 in 6.68 [CL-6.67], [CL-6.68].

**Fiery Soul** (passive, hotkey F) [PD-lina-2012], [CL-6.72d]

| Lvl | Per stack | Duration | Max stacks |
|---|---|---|---|
| 1 | +30% attack speed, +3% move speed | 7 s | 3 |
| 2 | +40% attack speed, +4% move speed | 7 s | 3 |
| 3 | +50% attack speed, +5% move speed | 7 s | 3 |
| 4 | +60% attack speed, +6% move speed | 7 s | 3 |

Notes:
- Triggered by casting a spell. Items do not trigger it.
- The duration refreshes on each cast.
- Introduced in 6.60, replacing Lina's old ultimate [CL-6.60b].

**Laguna Blade** (ultimate, unit target, hotkey G, Aghanim's upgrade) [PD-lina-2012]

| Lvl | Mana | CD | Range (Aghanim's) | Damage (Aghanim's) |
|---|---|---|---|---|
| 1 | 280 | 120 | 600 (900) | 450 (600) |
| 2 | 420 | 90 | 600 (900) | 675 (925) |
| 3 | 680 | 55 | 600 (900) | 950 (1250) |

Notes:
- Magical damage.
- From 6.71, does not target magic-immune units [CL-6.71].

**Lina changes from 6.74 to 6.83:**
- **6.74:** Fiery Soul +40/50/60/70 attack speed, +4/5/6/7% move speed [CL-6.74].
- **6.75:**
  - Fiery Soul attack speed +40/55/70/85.
  - Laguna Blade cooldown 90/70/50.

  [CL-6.75]
- **6.76:** Laguna Blade cooldown 70/60/50 [CL-6.76].
- **6.78:** Attack range 635 [CL-6.78].
- **6.79:** Attack range 650 [CL-6.79].
- **6.80:**
  - Dragon Slave final AoE 200.
  - Fiery Soul duration 9 s, move speed +5/6/7/8%.

  [CL-6.80]
- **6.81b:**
  - Light Strike Array stun 1.6/1.7/1.8/1.9 s.
  - Aghanim's Laguna Blade goes through magic immunity.
  - Attack range 670.

  [CL-6.81b]

The 2014 page reflects these values [PD-lina-2014].

**DotA 1 vs DotA 2:** Lina's Fiery Soul stacks only 3 times.

---

#### Lion (Demon Witch), Scourge, Intelligence

| Stat | 6.73 value | Source |
|---|---|---|
| STR | 16 + 1.7 | [PD-lion-2012] |
| AGI | 15 + 1.5 | [PD-lion-2012] |
| INT (primary) | 22 + 3.0 | [PD-lion-2012] |
| Damage (level 1) | 42–48 | [PD-lion-2012] |
| Armor (level 1) | 1.1 | [PD-lion-2012] |
| Move speed | 290 | [PD-lion-2012] |
| Attack range | 600 | [PD-lion-2012] |
| Base attack time | 1.7 | [PD-lion-2012] |
| Attack animation (point/backswing) | 0.43 / 0.74 | [PD-lion-2012] |
| Cast animation (point/backswing) | 0.3 / 0.51 | [PD-lion-2012] |
| Projectile speed | 900 | [PD-lion-2012] |
| Sight day/night | 1800 / 800 | [PD-lion-2012] |

**Impale** (active, point/line, hotkey E) [PD-lion-2012]

| Lvl | Mana | CD | Range | Width | Stun | Damage |
|---|---|---|---|---|---|---|
| 1 | 100 | 12 | 500 | 125 | 0.5 s | 60 |
| 2 | 120 | 12 | 500 | 125 | 1 s | 130 |
| 3 | 145 | 12 | 500 | 125 | 1.5 s | 200 |
| 4 | 170 | 12 | 500 | 125 | 2 s | 260 |

Notes:
- Magical damage.
- Travels 700 at 1600 per second.
- Units are airborne for 0.52 s before the listed stun applies.
- The 6.67 and 6.79 "Impale" changelog lines belong to Nerubian Assassin, not Lion.

**Voodoo (Hex)** (active, unit target, hotkey D) [PD-lion-2012]

| Lvl | Mana | CD | Range | Duration |
|---|---|---|---|---|
| 1 | 110 | 13 | 500 | 1.25 s |
| 2 | 140 | 13 | 500 | 2 s |
| 3 | 170 | 13 | 500 | 2.75 s |
| 4 | 200 | 13 | 500 | 3.5 s |

Notes:
- Turns the target into a critter with base move speed 100.
- Disables abilities, damage block and evasion.
- Destroys illusions instantly.
- Durations were set in 6.60 (previously 1/2/3/4) [CL-6.60b].

**Mana Drain** (active, channeled, unit target, hotkey R) [PD-lion-2012], [CL-6.73]

| Lvl | Mana | CD | Range | Duration | Drain per second |
|---|---|---|---|---|---|
| 1 | 10 | 25 | 600 | 4 s | 20 |
| 2 | 10 | 20 | 650 | 4 s | 40 |
| 3 | 10 | 15 | 700 | 4 s | 60 |
| 4 | 10 | 10 | 750 | 4 s | 120 |

Notes:
- Drains in 0.25 s ticks (since 6.73).
- Breaks if the target goes beyond 800.
- Destroys illusions after 0.25 s.

**Finger of Death** (ultimate, unit target, hotkey F, Aghanim's upgrade) [PD-lion-2012]

| Lvl | Mana | CD (Aghanim's) | Range | Damage (Aghanim's) |
|---|---|---|---|---|
| 1 | 200 | 150 | 700 | 500 (600) |
| 2 | 420 | 90 | 700 | 650 (800) |
| 3 | 650 (625 with Aghanim's) | 55 (50) | 700 | 850 (1025) |

Notes:
- Magical damage.
- From 6.71, does not target magic-immune units [CL-6.71].
- No damage delay is listed on the page. The DotA 2-style delay is UNVERIFIED for DotA 1.

**Lion changes from 6.74 to 6.83:**
- **6.74:** Voodoo cooldown 15, mana 125/150/175/200, duration 1.75/2.5/3.25/4 [CL-6.74].
- **6.75:**
  - Finger of Death cooldown 170/105/40, damage 600/725/850. With Aghanim's: 725/875/1025.
  - Mana Drain cast range 750, break range 850, cooldown 20/15/10/5.

  [CL-6.75]
- **6.76:**
  - Mana Drain break range 1100.
  - Finger of Death cooldown 160/100/40, cast range 900.

  [CL-6.76]
- **6.78:**
  - Mana Drain cast range 850, break range 1200.
  - Aghanim's Finger of Death cooldown 100/60/20.

  [CL-6.78]
- **6.79:**
  - Mana Drain duration 5 s.
  - Voodoo cooldown 30/25/20/15, duration 2.5/3/3.5/4.

  [CL-6.79]
- **6.80:** Aghanim's Finger of Death also hits units within 200 of the primary target [CL-6.80].
- **6.81b:** Voodoo cooldown 30/24/18/12 [CL-6.81b].
- **6.83c:** Hexed unit move speed becomes 140, matching Scythe of Vyse [CL-6.83c].

The 2014 page reflects most of these [PD-lion-2014].

**DotA 1 vs DotA 2:** Lion's Finger of Death has a cooldown-reduction Aghanim's upgrade (from 6.78) and an AoE upgrade (from 6.80). It has no kill-stacking damage.

---

#### Zeus (Lord of Olympia), Sentinel, Intelligence

| Stat | 6.73 value | Source |
|---|---|---|
| STR | 19 + 1.8 | [PD-zeus-2012] |
| AGI | 11 + 1.2 (changed from 15 + 1.7 in 6.63) | [PD-zeus-2012], [CL-6.63] |
| INT (primary) | 20 + 2.7 | [PD-zeus-2012] |
| Damage (level 1) | 41–49 | [PD-zeus-2012] |
| Armor (level 1) | 1.54 | [PD-zeus-2012] |
| Move speed | 295 | [PD-zeus-2012] |
| Attack range | 350 | [PD-zeus-2012] |
| Base attack time | 1.7 | [PD-zeus-2012] |
| Attack animation (point/backswing) | 0.633 / 0.366 | [PD-zeus-2012] |
| Cast animation (point/backswing) | 0.4 / 0.5 | [PD-zeus-2012] |
| Projectile speed | 1100 | [PD-zeus-2012] |
| Sight day/night | 1800 / 800 | [PD-zeus-2012] |

**Arc Lightning** (active, unit target, hotkey C) [PD-zeus-2012]

| Lvl | Mana | CD | Range | Bounce radius | Max targets | Damage |
|---|---|---|---|---|---|---|
| 1 | 65 | 2 | 700 | 500 | 5 | 85 |
| 2 | 70 | 2 | 700 | 500 | 7 | 100 |
| 3 | 75 | 2 | 700 | 500 | 9 | 115 |
| 4 | 80 | 2 | 700 | 500 | 15 | 145 |

Notes:
- Magical damage.
- Damage does not fall off between bounces.
- Cooldown history: 2.5, then 2.25 in 6.68, then 2.0 in 6.72 [CL-6.68], [CL-6.72].
- Level-4 damage went from 130 to 145 in 6.73 [CL-6.73].

**Lightning Bolt** (active, unit target, hotkey G) [PD-zeus-2012]

| Lvl | Mana | CD | Range | Damage |
|---|---|---|---|---|
| 1 | 75 | 6 | 700 | 100 |
| 2 | 95 | 6 | 700 | 175 |
| 3 | 115 | 6 | 700 | 275 |
| 4 | 135 | 6 | 700 | 350 |

Notes:
- Magical damage with a ministun, which interrupts channeling.
- Gives true sight in 900 range and reveals 1000 range, day and night, for 3 s.
- Cooldown 6 since 6.73 [CL-6.73].

**Static Field** (passive, hotkey F) [PD-zeus-2012]

| Lvl | AoE | Damage |
|---|---|---|
| 1–4 | 1000 | 5% / 7% / 9% / 11% of **current** HP |

Notes:
- Triggers when Zeus casts a spell. Items do not trigger it.
- Its damage lands before the spell's damage.
- Does not need vision of the target.
- Hits creeps since 6.73 [CL-6.73].
- AoE history: 800, then 900 in 6.67, then 1000 in 6.71 [CL-6.67], [CL-6.71].

**Thundergod's Wrath** (ultimate, instant, global, hotkey W, Aghanim's upgrade) [PD-zeus-2012]

| Lvl | Mana | CD | Damage to every enemy hero (Aghanim's) |
|---|---|---|---|
| 1 | 225 | 120 | 210 (400) |
| 2 | 325 | 120 | 335 (500) |
| 3 | 450 | 120 | 460 (600) |

Notes:
- Magical damage.
- Gives 1000 vision and 900 true sight around each target for 3 s.
- Invisible targets are revealed but not damaged.
- Aghanim's damage was raised from 355/460/570 to 400/500/600 in 6.71 [CL-6.71].

**Zeus changes from 6.74 to 6.83:**
- **6.75:**
  - Arc Lightning cooldown 1.75.
  - Thundergod's Wrath cooldown 90.

  [CL-6.75]
- **6.76:** Thundergod's Wrath 225/350/475. With Aghanim's: 440/540/640 [CL-6.76].
- **6.77:** STR gain 2.3 [CL-6.77].
- **6.80:** Arc Lightning cast range 850 [CL-6.80].
- **6.81b:**
  - Arc Lightning cast point 0.2.
  - Lightning Bolt true sight 750, flying vision 750, lasting 4.5 s.
  - Lightning Bolt can be ground-targeted. It then hits the closest enemy hero within 325.

  [CL-6.81b]

**DotA 1 vs DotA 2:** Zeus's Static Field is a percentage of **current** HP, triggered by his own spell casts in a 1000 radius.

---

#### Sources
- [PD-sniper-2012] https://web.archive.org/web/20120121055043/http://www.playdota.com/heroes/dwarven-sniper
- [PD-sniper-2014] https://web.archive.org/web/20140815/http://www.playdota.com/heroes/dwarven-sniper
- [PD-cm-2012] https://web.archive.org/web/20120115/http://www.playdota.com/heroes/crystal-maiden
- [PD-cm-2014] https://web.archive.org/web/20140815/http://www.playdota.com/heroes/crystal-maiden
- [PD-lina-2012] https://web.archive.org/web/20120115/http://www.playdota.com/heroes/slayer
- [PD-lina-2014] https://web.archive.org/web/20140824074834/http://www.playdota.com/heroes/slayer
- [PD-lion-2012] https://web.archive.org/web/20120119032526/http://www.playdota.com/heroes/demon-witch
- [PD-lion-2014] https://web.archive.org/web/20140815/http://www.playdota.com/heroes/demon-witch
- [PD-zeus-2012] https://web.archive.org/web/20120115/http://www.playdota.com/heroes/lord-of-olympus
- [PD-zeus-2014] https://web.archive.org/web/20140729145205/http://www.playdota.com/heroes/lord-of-olympus
- [CL-6.xx] https://web.archive.org/web/2016/http://www.playdota.com/changelogs/6.xx. The versions used are 6.60b, 6.61, 6.63, 6.67, 6.68, 6.69, 6.71, 6.72, 6.72c, 6.72d, 6.73, 6.74, 6.75, 6.76, 6.77, 6.78, 6.79, 6.80, 6.81b and 6.83c. Replace "6.xx" with the version.
- Not archived or not retrievable: changelogs 6.81, 6.82, 6.82b, 6.82c, 6.83 and 6.83b. Changes in those versions are **not covered** (UNVERIFIED).

## 5. "Feel" details players said mattered

**Research note (read first).** Reddit was not reachable: both the search tool and direct fetches were blocked, so this file has no Reddit quotes. The Wayback Machine (archived playdota/getdota posts) timed out, and fandom.com / warcraft.wiki.gg sit behind Cloudflare. The quotes below come from Liquipedia wikitext (pulled through its API), TeamLiquid/LiquidDota threads from the Dota 2 beta in Oct–Dec 2011, Hive Workshop threads, Steam Community, DotA-Blog (2008) and D1Stats. "Verbatim" means I copied the quote from page text I downloaded myself. Quotes marked *(search snippet)* came from a search-engine summary of a page I could not open, so their wording is UNVERIFIED.

Liquipedia's Dota 2 wiki keeps DotA 1 version history: entries numbered 6.xx up to about 6.74 are WC3-only, and later ones were shared by both games. I use those entries as the DotA 1 rule wherever they exist.

---

### 1. Last-hitting

**Rule.** Only the killing blow pays gold. Experience goes to every enemy hero within range. Attack damage is a random range, not a fixed number. That range was inherited from WC3 and kept in Dota 2.

- "Enemies gain no gold and reduced experience from any denied unit." (Liquipedia, Denying) https://liquipedia.net/dota2/Denying
- Damage ranges as a deliberate last-hitting knob: "Chaos Knight, for instance, has a huge range of damage which makes it hard to last hit perfectly; In the same sense, certain heroes like Drow have a very small range". Barbiero, 18 Oct 2011, #59. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=3
- WC3 last-hitting felt harder: "in dota on wc3 it was much much harder to last hit creeps at least for me". Dr. E, 22 Oct 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- "the game feels a lot smoother so i do think its easier to last hit dota2 animations feel and look more natural than wc3 even though they're pretty much the same speeds". rabidch, LiquidDota staff, 16 Oct 2011, #6. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
- A small UI difference: "lasthitting and aiming units is very weird because the pointer doesn't change when you hit 'A"". Barbiero, #2, same thread.
- `-cs` showed your last hits and denies: "Shows how many creeps you have killed, as well as how many you have denied. Added in v6.28". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

### 2. Denying (version-dependent rules)

**Rules.**
- A creep can be denied once it is below 50% HP. Players attack their own unit with A-click.
- Early DotA punished attacking allies: "attempting to do so caused the attacking unit to get hexed for one second". The hex was later replaced with a stop command. After that, denying became allowed below 50% HP. https://liquipedia.net/dota2/Denying (Trivia)
- **Deny XP by version.** These are verbatim Liquipedia version-history lines. https://liquipedia.net/dota2/Denying and https://liquipedia.net/dota2/Archive:Experience

| Version | Rule |
|---|---|
| Before 6.36 | A denied creep gave the enemy **no** XP |
| 6.36 | "Denied units now give off minor experience instead of none" |
| 6.36b | "Lowered the recent XP deny change a little." |
| 6.38 | "Melee heroes get denied less XP than range heroes do." |
| 6.44 | Heroes under deny-able debuffs can only be denied below 25% HP: "You have to be below 25% health for an ally to be able to start denying you" |
| 6.78 | "Deny XP and Bonus XP/gold AoE is now the same as regular XP AoE (1000->1200)." |
| 6.79 | "Ranged heroes now get the same denied experience as melee heroes (instead of less)." |
| 6.82 | "Denied creeps now give less experience (50% experience, instead of a constant 36 XP per unit)." |

- **What this means for 6.48–6.81.** A denied creep gave each nearby enemy a small *flat* amount of XP, not a percentage. The Archive:Experience page quotes a 2011 wiki revision: 36 XP for melee heroes and 18 for ranged. The 6.82 line confirms "constant 36". The split between melee and ranged heroes lasted from 6.38 until 6.79. The exact ranged figure in WC3 is UNVERIFIED; 18 comes from a Dota 2-beta-era wiki revision.
- **A deny marker.** "Every time you, an ally or an opponent makes a deny you see an exclamation mark in the color of the player that denied the creep. Added in v6.39". `-showdeny` / `-hidedeny` toggled it. http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html
- **Towers.** "Towers may be denied once they drop to 10% health and below". This is current Dota 2. The DotA 1 tower deny threshold is UNVERIFIED but believed to be the same.

**How players felt about the later Dota 2 deny changes.** After 7.06, 7.07, 7.20 and 7.26b, the denier gained some XP and then some gold, and the share of XP the denied side still got rose from 25% up to 50%.
- "Now if all your creeps are denied and you stay solo in lane touching nothing you get as much exp as if you were in double lane and lasthitted every single creep." Steam, 22 May 2020. https://steamcommunity.com/app/570/discussions/0/2245553353021133678/
- The same poster on the motive: "they decided to reduce gap between expirienced players and noobs by reducing advantage winning team gets in lane." (same URL)

### 3. Orb walking

**Rule.** WC3 called unique attack modifiers "orb effects". Players manually cast an autocast attack such as Frost Arrows instead of right-clicking. This counts as a spell cast, so it does not draw creep or tower aggro, and the hero can be ordered to move during the cast without losing the attack.
- "In Warcraft III, unique attack modifiers are called orb effects … The term Orb Walking originates therefore from Warcraft III". https://liquipedia.net/dota2/Attack_Modifier
- "casting these attack modifiers manually, also known as Orb Walking, neither draws aggro from lane creeps nor towers." (same page)
- "orb walking and stutter stepping were very commonly used together, so commonly that it is just referred to as orb walking". (same page)
- From a DotA-forum-era guide (BLAOW, mirrored on Eurobattle), *(search snippet)*: "manually casting autocast abilities like Drow's frost arrows avoids aggro". https://forum.eurobattle.net/threads/69659-Line-and-map-control-guide-(written-by-BLAOW-from-dota-allstars-forum) (wording UNVERIFIED)
- "clinkz has great animation if you know how to animation cancel and orbwalk." brokor, 17 Oct 2011, #38. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2

### 4. Animation cancel, attack point and backswing (WC3 engine)

**Rule.** An attack has two phases:
- **Damage point (foreswing).** Cancelling here cancels the attack. For a ranged unit this is the moment the projectile launches.
- **Backswing.** Any new order can cancel it, and cancelling costs nothing. It does not let you attack faster.

Cast point and cast backswing work the same way for spells.
- "Backswing is an interruptible period, so a new attack will cut it short … if you re-issue an order they'll interrupt the backswing and move. This is where animation cancelling comes from in things like DotA." Hive Workshop, 19 Jan 2014. https://www.hiveworkshop.com/threads/dps-and-attack-animation-damage-point.247190/
- "For ranged it is the time between starting the animation and launching the projectile." (same thread)
- "Backswing point is the length of animation after an ability took effect. It can be canceled however via ordering another command." https://www.hiveworkshop.com/threads/cast-point-and-backswing-point.265781/
- "canceling your animation does not allow you to attack faster." https://liquipedia.net/dota2/Attack_Animation
- **Some WC3 heroes had bad animations on purpose, and players learned them as a skill:**
  - "zeus had to finish his attack animation before his projectile comes out … so you couldnt animation cancel like every other hero". Rhaegar99, #16. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
  - "I spent a lot of time learning specific heroes animation and attack points and it gave me a huge edge with certain weird hereos(razor, krob etc.)" Rainmaker5, #42, page 3 of the same thread.
  - "I think CM and other heroes should KEEP their shitty animation. Because the game was balanced around it". FacelessVoid, #31, page 2.
  - "attack points and animation times(front/backswings) are all the same, the animation just feels better." Barbiero, #48, page 3. This says Dota 2 kept the WC3 timings and only redrew the visuals.
  - "a huge portion of the balance is based around some heroes having bad animations. Lina, for example, would be absurd if her spells and autoattack didn't have a long animation." InvalidID, 16 Dec 2011. https://tl.net/blogs/295350-dota-2-interface-concerns
- Liquipedia 6.00: "Updated all of the attack animations so they more fully match the hero models." https://liquipedia.net/dota2/Attack_Animation

### 5. Creep aggro

**Dota 2 rule, as documented.** DotA 1 likely matches it, but that is UNVERIFIED.
- Lane creeps within 500 range of a hero who issues an attack order on an enemy hero switch to that hero. They stay aggroed for up to about 2.3 s, and the trigger has a roughly 3 s cooldown.
- Spells do not trigger it.
- Sources:
  - "Issuing an attack order on an enemy hero will draw the aggro of lane creeps within [500] range of the aggressor hero. An attack order alone is enough". https://liquipedia.net/dota2/Lane_Creeps
  - "The lane creeps' protective behavior only reacts on attacks, they completely ignore ability cast." (same page)

**DotA 1 evidence.**
- A DotA-forum-era guide *(search snippet)*: "directly attacking, i.e. right-clicking, an enemy hero near creeps will cause them to attack you, whereas manually casting spells will not". BLAOW guide, Eurobattle mirror (URL above). Wording UNVERIFIED.
- Community DotA 1 patch 6.88 (post-IceFrog): "Non-heroes will not draw creep aggro when attacking enemy hero anymore" and "Improved lane creeps aggro". https://d1stats.ru/88n-betas-changelog/?lang=en
  - This implies that up to 6.83, any unit (summons too) hitting an enemy hero drew creep aggro, as in the WC3 "call for help" behaviour. That reading is INFERENCE.
- Exact DotA 1 radius and duration: UNVERIFIED. The 500 / 2.3 s / 3 s figures are Dota 2 values.
- Players said creep behaviour felt a little different after the port:
  - "Creep AI is basically the same as in WC3". Barbiero, #2. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
  - "i feel like the creep ai has changed a good amount tho, they seem a lot less moody". Dead9, #52, page 3.

### 6. Tower aggro

**Rule (Dota 2, 2014; inherited from DotA, DotA 1 specifics UNVERIFIED).**
- Tower priority: "Closest enemy hero attacking a friendly hero with auto attack" comes first. Then an enemy creep attacking a friendly hero, then a unit attacking the tower, then a unit attacking any friendly unit, then the closest enemy, then siege.
- The tower keeps its target until a "Reseter": the target dies or becomes unattackable, or the tower's current target hero manually attacks a unit.
- "Spells ARE NOT AUTO-ATACK. This means you can spam spells under a tower and it won't switch."
- "The key to this technique is the difference between Tower Aggro Range (500) and Tower Attack Range (700)."
- Source: Steam guide, Jul/Aug 2014. https://steamcommunity.com/sharedfiles/filedetails/?id=288488800
- The guide calls tower and creep aggro "essentially the same" and "the most important step between low level of gameplay and intermediate level". (same URL)

### 7. Juking, trees and tree pathing

**Rules.**
- Trees block ground vision and pathing.
- DotA 1 trees regrew on a **single global 5-minute timer**, not one timer per tree. This is stated in the community 6.85 notes as the behaviour it replaced: "Trees now individually regrow 5 minutes after they were destroyed, instead of trees globally regrowing every 5 minutes." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/ (27 Apr 2017)
- In WC3, trees also changed projectile and ability flight height. Same changelog: "Trees no longer affect the flying height of projectiles and/or abilities (Elune's Arrow, Meat Hook, etc.)"

**What players said.**
- "DOTA 1 more makes us to jukes...play with fog and trees.. self timing etc.." ARTSZ*, 2 Aug 2021. https://steamcommunity.com/app/570/discussions/0/3053989078744434569/
- "DOTA 1 you can easily escape from enemy only by jukes on the tree for example." (same thread)
- A counterpoint in the same thread: "jukes are the same, people got better at reading them".
- Night gives "an easier of escaping by juking and utilizing the trees to obstruct your vision". https://tl.net/blogs/282827-dota-back-to-basics (Nov 2011; describes the Dota 2 beta port of 6.7x)

### 8. Fog of war and vision

**Rules.**
- Lower ground cannot see higher ground.
- Trees cast "shadows" in vision.
- Day and night change vision range.
- Units leaving vision cancel spells that are mid-cast, but not attacks.
- Sources:
  - "Units with ground vision cannot see areas that are on higher elevation than them." https://liquipedia.net/dota2/Vision
  - "If an enemy enters the fog of war during the cast time of an ability, the cast gets canceled. However, attacks do not cancel". (same page)
- Ranged attacks uphill had a miss chance: "for ranged heroes avoids the 25% miss chance of last hitting uphill." https://liquipedia.net/dota2/Creep_control_techniques
  - Whether the 25% figure is WC3's own number is UNVERIFIED. The WC3 gameplay constant was not confirmed.

**What players said.**
- "Fog bugs, much more." This appears in a list of DotA WC3 cons. Hive, 23 Sep 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- "Valve just farmed the original idea … some changes to hero abilities and fog mechanics." (same thread)

### 9. Unit collision and creep blocking

**Rule.** Hero bodies physically block creeps. Creep-blocking the first wave shifts where the lanes meet.
- "physically blocking your creeps' path with your hero so they take longer to reach the middle of the lane." https://liquipedia.net/dota2/Creep_control_techniques
- A WC3 habit: "In DotA a goal when blocking was often to let the ranged creep through so it would arrive first and die first". Yurie, 18 Oct 2011, #64. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=4
- "the range creep is supposed to be slightly harder to block". rabidch, #66 (same page)
- A WC3 pathing bug: Sentinel bottom-lane creeps sometimes wandered into the forest. Q: "do creep ever get bugged out on bot lane anymore and run though the sent lower forest?" A: "Nope". #54 and #60, page 3.
- "the bottom creep lane seems way less buggy in its pull behavior … it was a frustrating imbalance for the sentinel". InvalidID, #88, page 5.
- Some unit models were hard to click: "units like bounty hunter don't have that naturally "unclickable" unit model anymore". This is a translated iG report, #46, page 3. A WC3 quirk.

### 10. Item sharing, pooling and dropping

**Rules.**
- In WC3 DotA almost every item could be dropped on the ground and picked up by anyone.
- **6.52** brought in owner-locked ("muted") items for anyone who was not the owner, plus an optional pooling mode: "-poolingmode / -pm … This mode allows pooling (item sharing between players) … Added in v6.52". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html
- A Liquipedia archive page records 6.52: "Items now have muted form when carried by non-owner, and a pooling mode was added as an option" *(search snippet)*. https://liquipedia.net/dota2/Archive:Item_Sharing (wording UNVERIFIED)

**Engine quirks around items** (from the community 6.85 notes, which describe what they fixed). https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
- Items on cooldown could not be moved or dropped: "Using items no longer causes them to be stuck on the hero while they're on cooldown."
- The courier needed items dropped on the ground to combine recipes: "You no longer need to drop items on the ground to get proper item combination."
- Toggling items interrupted orders: "Toggling Radiance, Power Tread … no longer interrupts the hero."
- Inventory rearranging in DotA 1 worked as "right click the item once to select it and right click the new location again to move it". NowLookHere, #23. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2
- Power Treads key presses got "lost": "If you had pressed the key for your power treads a few times in a row, the engine would have lost it one time." Perscienter, #18, page 1.
  - Reply: "It wasn't lag. Items all have CDs." cyberspace, #19.
- Leavers: "All items from the leaver's hero are sold at half price, with the gold distributed evenly to all his teammates." (team-sell) https://warcraft.wiki.gg/wiki/Defense_of_the_Ancients:_Allstars (via fetch summary)
- `-itemswap x y` existed "for dealing with undroppable items or items in cooldown" (v6.55). DotA-Blog URL above.

### 11. Runes and the Bottle

**Rules (late-6.7x, as ported to the Dota 2 beta in 2011).**
- One rune spawns every 2 minutes in one of two river spots.
- Runes in the pool: Double Damage, Invisibility, Haste, Regeneration, Illusion.
- "At the beginning of every game and every second minute (00:00, 02:00, 04:00, etc.), there will spawn one rune in one of the two rune locations." https://tl.net/blogs/282827-dota-back-to-basics
- "Haste runes allow a hero to run at maximum movespeed (522) for 30 seconds". (same URL)

**Bottle quirks in 6.83** (community 6.85 notes, describing what they changed): "Runes are now placed into the bottle automatically on pickup. Using a stored rune no longer interrupts the hero." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
- The direct reading: in DotA 1, bottling took a deliberate action and using a stored rune interrupted the hero. The exact bottling input in WC3 is UNVERIFIED.
- The single-player test command `-powerup` spawns a rune, but "rune won't spawn if there already is one, or was one less than 5 seconds ago". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

### 12. Ganking and Smoke of Deceit

- **Smoke of Deceit is not in 6.48–6.69.** It was added in **6.70** (cost 100, stock 3) and changed in 6.71. https://liquipedia.net/dota2/Archive:Version_6.70 and https://liquipedia.net/dota2/Archive:Version_6.71 (*search snippet*; both pages were rate-limited, so wording is UNVERIFIED)
  - A faithful 6.48–6.69 remake should leave Smoke out. A 6.70+ remake needs it.
- Day and night shaped ganks: "Ganking in the different hours of day is very different because the enemy will be able to see you later in the gank". https://tl.net/blogs/282827-dota-back-to-basics

### 13. WC3 engine quirks: selection, hotkeys, orders, randomness

- **Selection.** Clicking another unit selects it. DotA 1 players worried about "accidentally deselecting my hero at horrible times. It makes the interface feel clunky." Ryhn, #27. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2
  - "failing to cast a skill because its on cooldown never deselected your hero in wc3". rabidch, #32 (same page)
- **Stop and hold.** Players spammed Stop (S) or Hold (H) to keep from auto-attacking while waiting to last-hit: "I don't enjoy spamming the shit out of S until I want to last hit." Zlasher, #50, page 3.
  - "usually people just walk their heroes around instead of spamming S or hitting H". Dead9, #52.
- **Auto-acquire** stayed in Dota 2: "default is wc3 behavior, you can turn off auto attacking if you want it to". rabidch, #36, page 2.
- **Hotkeys.** Items sat on the numpad by default, and many players used third-party tools to remap them. "By default they are bound to the Numpad". #84, page 5.
- **Information hiding.** To see an enemy's items or mana you had to click the enemy: "showing opponents items/mana/buffs will never be done because it wasnt in dota1 … you have to click on your opponent to know what items they have". Post #16. https://tl.net/blogs/295350-dota-2-interface-concerns
  - Always-on mana bars needed "dll injection and therefor were considered hacks in most competitive settings". InvalidID, #89. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=5
- **Order latency.** "The input latency thing is required for balance, and was changed slightly in DOTA2." InvalidID. https://tl.net/blogs/295350-dota-2-interface-concerns
  - The community 6.85 notes claim: "Every Warcraft 3 map has "Random delay" option, which randomly delays every given attack order by 0-0.25 seconds. Originally DotA didn't change it". https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - As an engine fact this is UNVERIFIED. Battle.net's roughly 250 ms default turn latency is widely reported but not sourced here.
- **Pseudo-random distribution.** Applies to Critical Strike, Bash, Pulverize, Hardened Skin, Orb of Slow and Demolish, but not to the damage roll. Playdota article (archived; not fetched), cited in thread #77, page 4: http://www.playdota.com/forums/7993/article-pseudo-random-distribution/
  - The damage roll is "dicerolls. Some heroes have more dices then others (creating a distribution rather than a linear range)". 5-s, #82, page 5.
- **Turn rate.** WC3 units already had turn rates, and Dota 2 kept them; Liquipedia gives the Dota 2 facing rule (11.5°). https://liquipedia.net/dota2/Turn_Rate
  - A 6.85 changelog line, "Increased turn rate of Black Drake and Black Dragon from 0.1 to 1.0", shows that some WC3 units had very slow turn rates. https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - DotA 1 hero turn-rate values: UNVERIFIED. They were not fetched.
- **"Paused" units.** The 6.85 notes say "All sources of "paused" unit state were removed, as this method is outdated and buggy." (same URL)
- **Neutral AI under observers.** Post: "Only when there are spectators watching (DotA uses player 11 and 12 for neutral creep AI when they are available)". Barbiero, #22, page 2.

### 14. Chat commands (DotA-Blog guide, 2008–2010; versions as stated)

URL for this section: http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

**Modes** are typed by the host as one string, for example "-dm -ar -id -sc". The main modes:
- `-ap` (All Pick)
- `-ar` (All Random)
- `-rd` (Random Draft)
- `-sd` (Single Draft)
- `-cm` (Captains Mode)
- `-lm` (League Mode)
- `-id` (Item Drop)
- `-sc` (Super Creeps)
- `-ns` (No Swap)
- `-nr` (No Repick; "Added in v6.48")
- `-pm` (Pooling, 6.52)
- `-oi` (Observer Info, 6.52)

**In-game commands:**
- `-ma`: "Displays which opponent controls which hero, and it's level. Also shows the time the player left the game".
- `-ms`: movement speed.
- `-cs` / `-cson`: creep stats.
- `-disablehelp`: blocks allied Chen, Furion and Harbinger teleports or sprouts.
- `-unstuck`: "Channels for 60 seconds, then moves your hero back to your base".
- `-roll #`: random number 0–#, "Added in v6.44". `-rollhero` was added in 6.60.
- `-random`: gives "250 bonus gold".
- `-repick`: costs 100 gold, or 400 in -ar/-tr; "only in the first minute".
- `-afk` and `-kickafk #`: 6.50.
- `-apm`: 6.50.
- `-courier`: 6.50.
- `-clear`: 6.50.
- `-invokelist`: 6.50.
- `-swap 1-5`, `-swapcancel`, `-swapall`: "only before 90 seconds mark". `-swap` was added in 6.46.
- `-ah` (Anti-MapHack): "Must be entered … between 15 and 60 seconds of game time. Added in v6.50".
- `-test`: 6.48.
- Single-player commands: `-lvlup`, `-refresh`, `-spawncreeps`, `-powerup`, `-neutrals`.
- `-ii`: not found in this list, UNVERIFIED. It may come from a later version.
- `-gg` (concede) did not exist in IceFrog's DotA. It was added in the community 6.85: "It can be used in order to fast finish the game … only available after 15 minutes." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/

### 15. Creep pulling and stacking (neutral respawn)

**Rules.**
- At each minute mark a camp respawns if its spawn box is empty. Pulling the creeps out of the box just before the minute (about xx:52–:55) stacks the camp.
- Standing in the box, having vision of it, or placing a ward there blocks the spawn.
- "If you're too close to the spawn or have vision of the actual spawn, you will block the spawn which means no creeps will spawn … you can actually block spawns from spawning with a ward." https://tl.net/blogs/282827-dota-back-to-basics
- That blog shows stacks timed "between :51-53" and says "you can stack a camp up three times". That is Dota 2-beta behaviour; the DotA 1 stack cap is UNVERIFIED.
- Allstars-era guide (Dota-Utilities, Jan 2009) *(search snippet)*: "creep respawning is based on an invisible rectangle around each camp. To make new creeps spawn, you have to get the original creeps to follow you out of the rectangle." http://www.dota-utilities.com/2009/01/pulling-and-stacking-neutral-creeps.html (wording UNVERIFIED)
- Pull timings were reported identical in the port: Q: "What are the jungle pull timings like? … Can you still stack creeps with pulls?" A: "Same as in dota." #85–86, page 5. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=5
  - A different poster: "the leftmost creep camp on scourge side can't be pulled" in HoN, implying it could be in DotA. #87.

### 16. Fountain, couriers, Roshan

- **Fountain.** The community 6.85 notes "fixed" both of these, so in 6.83 the fountain was attackable and healing started after a delay:
  - "Fountains are now invincible."
  - "Fountain zone now provides healing as soon as unit enters it, without any kind of delay."
  - URL: https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - What the delay was in 6.83: UNVERIFIED.
- **Couriers.** In DotA 1 couriers were shared units that anyone could control. When a courier dropped items outside base, no notice was given; the 6.85 notes added one: "If the courier drops any of your items on the ground outside of the fountain, you'll be notified about it." (same URL)
  - Late-6.83 era: "Can now unshare leaver so it's courier won't be used for transfering your items". https://d1stats.ru/88n-betas-changelog/?lang=en
- **Roshan (as ported, 2011).** Aegis lasts 10 minutes, Roshan respawns 10 minutes later, and cheese drops from the third kill on. https://tl.net/blogs/282827-dota-back-to-basics

### 17. No matchmaking, leavers, hosting

- No matchmaking. Games were hosted lobbies on Battle.net, Garena and so on. Leavers were managed with third-party banlists: "A free 3rd party program that keeps track of leavers and attempts to prevent them from joining games." https://warcraft.wiki.gg/wiki/Defense_of_the_Ancients:_Allstars (via fetch summary)
  - "Third party programs also are against the battle.net terms of use." (same URL)
- "when a person leaves the game, it creates a favorable position for the opposing team." (same URL)
- Player view: "I also hated the idea of banlists in the original dota even though I never got banned, thanks god that was changed in DotA 2 to a less biased system." Hive. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
  - Reply: "DotA 2 really needs a surrender option." (same thread)
- The leaver's gold is split among the team that remains (from the v3.0a changelog, an early pre-IceFrog version, per search). https://dota2.fandom.com/wiki/Version_3.0a (UNVERIFIED wording)

### 18. Day and night

- Day and night switch vision ranges. Night Stalker's passive turns on at night. https://tl.net/blogs/282827-dota-back-to-basics
- The cycle length in DotA 1 (WC3 day length) is UNVERIFIED. It was not fetched.

### 19. DotA 1 vs Dota 2: other differences players reported

- Camera: "The camera movement feels a little odd on the first try". #2. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
- Projectiles: "Razor's lightning actually damages now when it lands". 5-s, #33, page 2.
- Visual clarity, cutting both ways:
  - "heroes such as CM, lina, zeus etc even though they have the animation timing carryied over, the graphic has been made much more clear and noob friendly". NB, #69, page 4.
  - "in dota 1 there were bright teamcolors atleast so it was easy to" spot heroes. Hive, 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- Ward pricing in the port: "wards you buy them in bunches of two for 200 gold". #60, page 3.
- Wanting things kept as they were: "I guess those DotA purists have got to have everything EXACTLY THE SAME." PSIDefenseUp, #58, page 3.
- The original thread opener's framing: "There were a lot of subtle things that the WC3 engine caused, and whether the developers meant for them to happen or they just came about, they became part of the game." Eeeegor, 16 Oct 2011, #1.

---

### Summary lists

**Things players say must not be simplified (they carry skill or balance)**
1. **Last-hit gold only, plus denying below 50% HP.**
   - Denied creeps should give the enemy only a little XP. Use the version-correct rule from the table in section 2.
   - Keep the deny "!" marker.
   - Sources: Liquipedia Denying; Hive post "The lack of creep denial was a letdown for me" (about LoL).
2. **Per-hero attack point and backswing, plus animation cancelling.**
   - Bad animations were balance tools (Lina, CM, Zeus, Razor), and players liked learning them.
   - Sources: TL #31, #42; InvalidID on the TL blog.
3. **Damage ranges per hero.** These set how hard a hero is to last-hit (CK vs Drow). TL #59, #73.
4. **Orb walking and the attack-vs-spell split for creep and tower aggro.**
   - Manual orb casts and spells must not draw aggro; right-clicking a hero must.
   - Sources: Liquipedia Attack Modifier; Steam tower guide.
5. **Deterministic tower targeting**, with the "A-click your own creep" reset. Steam tower guide.
6. **Trees as juke terrain and vision blockers**, with fog and night making escapes possible. Steam 2021 thread; TL blog.
7. **Body-blocking creeps**, including the ranged creep that slips past. TL #64, #66.
8. **Pull and stack timing at the minute mark**, and blocking a camp with a ward or by standing in it. TL blog; TL #85–86.
9. **High-ground vision asymmetry.** Liquipedia Vision.
10. **PRD for bash and crit procs.** "Some things are really affected by it, so it would be a big change." SKC, #77.

**Engine quirks people don't miss (OK to smooth, or offer as an option)**
1. Accidentally deselecting your hero, "clunky" selection. TL #27.
2. Spamming S/H to keep from auto-attacking. TL #50. An auto-attack toggle is fine.
3. Visuals that do not match the damage moment (Zeus's hammer, Razor's lightning hitting before the visual). TL #48, #33, #68. Keep the *timing* and fix the *visual*.
4. Global tree regrowth, trees changing projectile flight height, unclickable models. Hive 6.85 notes; TL #46.
5. Items stuck while on cooldown, courier recipe combining needing items dropped on the ground, toggling items interrupting orders, lost key presses. Hive 6.85; TL #18.
6. Fog and spell bugs, "paused"-state bugs, observers changing neutral AI, Sentinel bottom-lane creeps wandering into the forest. Hive 2014; TL #22, #54, #88.
7. Random order delay or input latency, if the "Random delay" claim holds. UNVERIFIED. Hive 6.85 notes.
8. Banlists and having no concede. Hive 2014. People wanted `-gg` and matchmaking.
9. Hidden mana bars and hidden enemy items. This is disputed: some treated clicking to inspect as skill (TL blog #16), while others called third-party mana bars "hacks" (TL #89).

## 6. What to simplify for an MVP, and what never to simplify

Each line below is a judgement drawn from the sources in §1–§5. The reason it rests on is given with it.

### Never simplify (identity of the game; players single these out)
1. **Last-hit-only gold, and denying allies below 50% HP.** A denied creep gives no gold and only the flat deny XP: 36/n to melee heroes, 18/n to ranged, until 6.79 [PD-XP], [PD-Lane]. Players name this as the core skill (§5.1–5.2).
2. **Per-unit attack damage point and backswing, cancellable with any order, and cast point/backswing.** The Lina, CM and Zeus "bad animations" were balance levers [PD-Anim], [PD-Units], with TL quotes in §5.4. Store the point and backswing on every unit and hero; the hero tables have them.
3. **Damage as a dice roll within a range** (e.g. 19-23 for creeps, built from XdY dice so the middle is likelier) [PD-Units]. Last-hit difficulty depends on it (§5.1).
4. **The attack vs spell aggro split.** An attack order on an enemy hero draws creep aggro and tower aggro. Spells and manually cast orbs do not [PD-Lane], [D2W13-Aggro], §5.3–5.6. Tower targeting must be deterministic and resettable by attacking your own creep.
5. **The DotA attack/armor table, the armor formula, and the 25% hero spell resistance (Spells → Hero 75%).** Use DotA's table in §2.10, **not** stock WC3's [PD-DA], [BNet]. Siege towers, pierce-weak buildings and hero-armor mitigation shape the whole laning and pushing economy.
6. **The economy formulas.** These are: 603 starting gold; periodic gold; the reliable/unreliable split; death loss of 30 × level of unreliable gold; the kill bounty of 200 + 5L + streak; assist AoE gold; buyback cost and cooldown; and 4 × level respawn [PD-Gold], [CL-6.70], [CL-6.73]. They are cheap to implement and drive comeback and snowball dynamics.
7. **XP sharing by radius and the level curve** [PD-XP]. Solo vs dual lanes exist because XP is shared.
8. **Neutral spawn-box rule at each minute mark (stacking and pulling) and the 30-second wave cadence with 7.5-minute upgrades** [PD-Neutrals], [PD-Units], §5.15.
9. **Trees that block vision and pathing (juking), high-ground vision asymmetry, and day/night vision** [PD-Terrain], [PD-Time], §5.7–5.8.
10. **Unit collision (body-blocking creeps) and turn rate.** Collision values are UNVERIFIED and must be measured from the .w3x (§2.11). Creep-blocking and juking depend on both.
11. **Rune cadence and the Bottle storing runes** [PD-Runes], [PD-RuneGuide]. Rune control decides lanes, according to the official guide.
12. **Tower invulnerability chain, backdoor protection and Glyph** [PD-Buildings], [CL-6.60], [CL-6.55]. Without them, pushing and split-pushing balance breaks.
13. **Orb effects that do not stack, and unique attack modifiers** (section 3 notes) [PD Orb Effects page]. Item choices depend on these rules.
14. **PRD for bash and crit** (§5.13). One player: "Some things are really affected by it, so it would be a big change." Implementation detail: UNVERIFIED (constants not fetched).

### Safe to simplify or defer for an MVP
1. **Game modes.** Ship one mode, an -ap-like all pick with 10 heroes. Draft modes, -em, -dm and the many toggles in §2.15 are lobby options [PD-Cmd].
2. **Map scale.** Use the official topology: 3 lanes, river, 2 rune spots, a Roshan pit, 6 camps per side, 2 secret shops and 2 side shops. Exact WC3 coordinates can wait, since map dimensions are UNVERIFIED anyway.
3. **Neutral camp variety.** Start with one group per tier using the §2.4 stats, then add random selection. Ancient camps and Thunder Lizards can come later.
4. **Mega and super creeps.** Keep Mega creeps, because they are part of the win condition after barracks fall. Skip -sc Super Creeps.
5. **Courier details.** One shared walking courier is enough. Flying upgrade, models, "transfer items" and "return home" can wait [CL-6.57], [CL-6.60]. Keep the rule that items bought with a full inventory drop at the Circle of Power (there is no stash) [CL-6.55].
6. **Item roster.** About 40 key items plus all basic components from §3 are enough. Defer Dagon levels 2–5, Aghanim's upgrades for heroes outside the MVP roster, and exotic orb conflicts beyond "one orb wins".
7. **Item sharing restrictions.** Implement owner-locking, the partially/fully shareable lists and -pm later [PD-Share]. For the MVP, simply forbid using items you do not own, except consumables.
8. **Roshan's growth over time, Slam and spell block, and Cheese.** A static Roshan with Bash, 75% spell resistance, a fixed 10-minute respawn and Aegis is enough at first [PD-Roshan].
9. **Version churn.** Pick one baseline (recommended: **6.72/6.73**, the date of the mechanics snapshot) and ignore the deltas. Keep the changelog lines in this document for later.
10. **Cosmetic and engine annoyances players don't miss** (§5 summary): accidental deselection, spamming S/H, random order delay, lost key presses, items stuck on cooldown, global tree regrowth, fog bugs, banlists and no concede. Keep the *timings* while dropping the *bugs*.
11. **Smoke of Deceit.** It exists only from 6.70. Leave it out if the baseline is 6.6x; at 6.72 it is a 100-gold consumable [CL-6.70].

### Open items to resolve before locking data files
- Lane creep armor type: "Unarmored" per playdota or "Medium" per Dota 2 (§2.3). It changes pierce damage by 2x.
- Periodic gold: 0.875 or 0.8 s per gold before 6.79.
- Creep aggro radius and duration in DotA 1, collision sizes, per-hero turn rates, map size, and the tower deny threshold. Read these from the 6.83d .w3x with a World Editor or an MPQ/SLK extractor (the object editor holds collision, turn rate, damage point and acquisition range directly).

## 7. Round 7: two added heroes, Troll Warlord (Jah'rakal) and Goblin Shredder (Rizzrak, Dota 2's Timbersaw)

Compiled 19 October 2026 by a research agent, with the same method as the sections above: playdota.com hero pages and IceFrog changelogs through the Wayback Machine (raw captures, compared against each other), the Dota 2 Wiki (Gamepedia) 2013–2015 snapshots and its DotA-era hero changelogs, and 2011–2012 player guides on playdota. fandom.com and Liquipedia returned 429 to direct fetches, so those were reached only through archived Gamepedia copies. Every number carries a source key; the keys are at the end of this section.

### 7.0 Version findings

1. **Goblin Shredder was added in 6.73, not 6.74.** The 6.73 changelog opens his entry with "New Strength Hero: Rizzrak, Goblin Shredder" [CL-6.73]; 6.73b and 6.73c fixed his growth values, attack range and Chakram drain [CL-6.73b], [CL-6.73c]. The first playdota hero-page capture is 2012-01-07 [PD-Shred-6.73]; the Dota 2 Wiki: "6.73: Created." [D2W-TimbCL]. **He is inside this game's 6.72/6.73 pin.**
2. **Troll Warlord's 6.72/6.73 kit is Berserker Rage / Whirling Axes (ranged and melee) / Fervor / Battle Trance.** 6.60 replaced Rampage with Battle Trance and made Fervor a stacking on-attack buff [D2W-TrollCL]; **6.72 replaced Blind with Whirling Axes** [CL-6.72]; 6.72c gave the ranged axes their 75 damage and rescaled Fervor [CL-6.72c]; 6.73 let Fervor build on towers [CL-6.73].
3. **Pin used:** both heroes as of **6.73** (Shredder with the 6.73b growth and attack-range fixes). Open choice: Chakram's drain, 20/30/40 a second in 6.73 or 20/25/30 in 6.73c; playdota still showed 20/30/40 on 2012-01-07 [PD-Shred-6.73].

### 7.1 Troll Warlord (Jah'rakal), Sentinel, Agility

| Stat | 6.72/6.73 | Notes |
|---|---|---|
| Primary | AGI | [PD-Troll-6.72] |
| STR | 17 + 2.2 | same in every snapshot 2010–2013 |
| AGI | 21 + 2.75 | same |
| INT | 13 + 1 | same |
| Damage (level 1) | 38–56 | raw base 17–35 (6.35: "total damage is now 38-56") [D2W-TrollCL] |
| Armor (level 1) | 1.94 | base armor −1 |
| HP / mana (level 1) | 473 / 169; 573 HP in melee form | +100 HP in Berserker Rage |
| Move speed | 300 | 6.50 [D2W-TrollCL] |
| Attack range | 500 ranged; 128 melee | 128 from [D2W15-Troll], DotA melee range otherwise UNVERIFIED |
| Missile speed | 1200 | [PD-Troll-6.72] |
| Base attack time | 1.7 ranged / 1.55 melee | [PD-Troll-6.72] |
| Attack point / backswing | 0.3 / 0.3 | [PD-Troll-6.72] |
| Cast point / backswing | 0.2 / 0.3 | 6.72 "cast point reduced by 0.3" [CL-6.72], [D2W-TrollCL] |
| Acquisition range | 600 | UNVERIFIED (inferred from Dota 2 6.82) |
| Turn rate, collision | 0.5, 24 | Dota 2 port values, UNVERIFIED for DotA |
| Vision day / night | 1800 / 800 | [PD-Troll-6.72] |

Hotkeys (playdota): Berserker Rage G, Whirling Axes E (both forms), Fervor F, Battle Trance R.

**Berserker Rage** (toggle, instant). *"Jah'rakal goes berserk, using his throwing axes as melee weapons. He gains bonus damage, movement speed, attack speed, hit points, armor, and the Bash ability, but can only attack at melee range."*

| Level | Mana | CD | Damage | Armor | HP | MS | BAT | Bash chance | Bash damage (magical) | Bash stun |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 0 | 0 | +15 | +3 | +100 | +20 | 1.55 | 10% | 20 | 0.8 s |
| 2 | 0 | 0 | +15 | +3 | +100 | +20 | 1.55 | 10% | 30 | 1.2 s |
| 3 | 0 | 0 | +15 | +3 | +100 | +20 | 1.55 | 10% | 40 | 1.6 s |
| 4 | 0 | 0 | +15 | +3 | +100 | +30 | 1.55 | 10% | 50 | 2.0 s |

Armor +3 from [D2W-TrollCL] (6.35), [G-Axe], [D2W13-Troll]; the rest [PD-Troll-6.72]. Rules: no mana, no cooldown, and since 6.72 no transformation time [CL-6.72]. Bash only in melee form; its stun pierces magic immunity; it does not stack with Cranium Basher; plain random in DotA (pseudo-random only in the Dota 2 port) [D2W15-Troll]. Switching keeps the health percentage (the guide's worked example: 120/744 in melee becomes 103/644 in ranged) [G-Axe]. Illusions made by Manta die when Troll changes form [G-Basic]. Reverting to ranged form on death: Dota 2 behaviour, DotA UNVERIFIED but likely the same.

**Whirling Axes** (learned once; two spells, one per form, each with its own cooldown and mana: "you can cast one, transform into your other form, and cast the second ability immediately") [PD-Troll-6.72], [CL-6.72].

*Ranged* (point, E). *"Troll hurls a fistful of five axes in a cone shape. Axes travel up to 900 range, slowing and damaging enemy units."*

| Level | Mana | CD | Range | Axe radius | Slow | Duration | Damage (magical) |
|---|---|---|---|---|---|---|---|
| 1 | 50 | 20 | 900 | 100 | 30% MS | 3 s | 75 |
| 2 | 50 | 20 | 900 | 100 | 30% | 3.75 s | 75 |
| 3 | 50 | 20 | 900 | 100 | 30% | 4.5 s | 75 |
| 4 | 50 | 20 | 900 | 100 | 30% | 5.25 s | 75 |

Five axes spread forward in a cone; the 2011 guide measured "100 (triangle tip) / 900 (distance) / 600 (triangle base)" [G-Axe]; the Dota 2 port uses a 25° spread and 1500 speed (DotA speed UNVERIFIED). Hit once per cast (UNVERIFIED for DotA; Dota 2). Vision where cast [G-Axe].

*Melee* (no target, E). *"Troll unleashes a set of magical axes which whirl around him in a circle. Nearby enemies take damage and have a chance to miss on their attacks."*

| Level | Mana | CD | Area | Damage (magical) | Miss chance | Duration |
|---|---|---|---|---|---|---|
| 1 | 50 | 12 | 450 | 125 | 60% | 4 s |
| 2 | 50 | 12 | 450 | 175 | 60% | 5 s |
| 3 | 50 | 12 | 450 | 225 | 60% | 6 s |
| 4 | 50 | 12 | 450 | 275 | 60% | 7 s |

Two axes spin out from Troll to 450 and back, centred on him; out and back 1.5 s each [D2W-TrollCL] or 2 s each [G-Axe] (UNVERIFIED). Only units an axe touches are hit, once a cast (UNVERIFIED). True Strike ignores the miss chance [G-Axe].

**Fervor** (passive). *"... With each continuous blow on the same target, Jah'rakal ... increasing his attack speed by 20 per attack."* +20 attack speed a consecutive attack on the same target, at most 40/60/80/100 (2/3/4/5 stacks) [PD-Troll-6.72], [CL-6.72c]. On a change of target the count starts again: on each attack, stacks + 1 (capped) if the target is the last one, else 1. When the stack is added is UNVERIFIED (Dota 2: at the attack point). From 6.73 it builds on towers too [CL-6.73].

**Battle Trance** (ultimate, no target). *"... Greatly increases attack speed for a short duration, and lasts twice the duration on Jah'rakal."*

| Level | Mana | CD | Area | Attack speed | Duration allies / self |
|---|---|---|---|---|---|
| 1 | 75 | 20 | the whole map | +60 | 5 s / 10 s |
| 2 | 75 | 20 | the whole map | +100 | 5 s / 10 s |
| 3 | 75 | 20 | the whole map | +140 | 5 s / 10 s |

Allied heroes and Troll; not illusions [PD-Troll-6.72].

How players described him (2011): Whirling Axes gave him burst and a chase for the first time; the chase is a form-switch combo (axes in one form, switch, axes in the other, ult, attack); max Whirling Axes first, Berserker Rage at rank 1; a late-game carry that reaches the attack-speed cap [G-Axe], [G-Basic].

### 7.2 Goblin Shredder (Rizzrak), Sentinel, Strength

| Stat | 6.73 | Notes |
|---|---|---|
| Primary | STR | [CL-6.73] |
| STR | 22 + 2.4 | 6.75: gain 2.1 |
| AGI | 16 + 1.3 | 6.73b fixed growth to these values |
| INT | 21 + 1.8 | 6.78: gain 2.4 |
| Damage (level 1) | 48–52 | raw base 26–30 |
| Armor (level 1) | 0.24 | base armor −2; the 6.73 changelog's "Starting Armor: 0" disagrees with playdota and the Dota 2 port |
| HP / mana (level 1) | 568 / 273 | |
| Move speed | 290 | |
| Attack range | 128 (melee) | 6.73b fix [CL-6.73b] |
| Base attack time | 1.7 | |
| Attack point / backswing | 0.36 / 0.64 | [PD-Shred-6.73] |
| Cast point / backswing | 0.3 / 0.6 | [PD-Shred-6.73] |
| Turn rate, collision | 0.5, 24 | Dota 2 port, UNVERIFIED |
| Vision day / night | 1800 / 800 | |

Hotkeys: Whirling Death D, Timber Chain T, Reactive Armor (passive), Chakram C; Return Chakram takes Chakram's button while the blade is out.

**Whirling Death** (no target, D). *"... whirls extremely sharp edges dealing damage and reducing primary attribute of any unfortunate enemy hero who comes in a 300 AOE around him. Deals Pure damage if it cuts down trees in the process. Lowers primary attribute by 15% for 7 seconds."*

| Level | Mana | CD | Area | Damage | Primary attribute (heroes) | Duration |
|---|---|---|---|---|---|---|
| 1 | 70 | 8 | 300 | 100 | −15% | 7 s |
| 2 | 80 | 8 | 300 | 150 | −15% | 7 s |
| 3 | 90 | 8 | 300 | 200 | −15% | 7 s |
| 4 | 100 | 8 | 300 | 250 | −15% | 7 s |

Damage 100/150/200/250 per [CL-6.73], [PD-Shred-6.73] and the port; 6.73 in-game tooltip copies show 50/100/150/200 and 6.73b "fixed Whirling Death damage tooltip" [CL-6.73b], [G-DotaBlog]. Destroys trees within 300; damage is magical, or pure if this cast cuts at least one tree. Against magic immunity the damage is blocked and the debuff is not [D2W13-Timb]. A STR loss lowers max HP, keeping the percentage. 15% of base or total attribute UNVERIFIED (Dota 2: total).

**Timber Chain** (point, T). *"Fires a chain that embeds itself in the first tree it hits, pulling you to it and cutting any enemy in your path for pure damage."*

| Level | Mana | CD | Range | Damage radius | Tree latch radius | Damage (pure) |
|---|---|---|---|---|---|---|
| 1 | 60 | 4 | 800 | 225 | 90 | 80 |
| 2 | 70 | 4 | 1000 | 225 | 90 | 120 |
| 3 | 80 | 4 | 1200 | 225 | 90 | 160 |
| 4 | 90 | 4 | 1400 | 225 | 90 | 200 |

6.74 raised the damage to 100/140/180/220 [CL-6.74]. The chain latches on the first tree within 90 of its path; Shredder is pulled to it and the tree is destroyed; enemies within 225 of his path take pure damage once. No tree: the chain retracts. **6.73: "You will not be pulled in if you are stunned when the chain latches onto a tree"** [PD-Shred-6.73] (the 2014 rule, stopping the pull on any disable, is later). He crosses cliffs. Chain and pull speed UNVERIFIED (port: 1600/2000/2400/2800, twice the range a second).

**Reactive Armor** (passive). *"What doesn't kill ya makes ya stronger! Every physical attack made on Rizzrak increases his armor and gives him bonus hp regen. Each stack lasts for 16 seconds."* +1 armor and +1 HP a second a stack; at most 4/8/12/16 stacks; 16 s each. A stack comes when an attack on him begins (the WC3 "unit is attacked" event; the port changed it to landing in 6.84) [D2W-TimbCL]; any attacker counts [D2W15-Timb]; each stack its own timer (UNVERIFIED for DotA; the tooltip suggests it).

**Chakram** (ultimate, point, C; Return Chakram no target). *"Fires your main saw blade at the target location, and once there the blade will spin, dealing damage in an AoE around it and slowing enemies by 5% for every 5% of health they are missing. The blade deals pure damage and cuts down trees in its path when fired and retracted. You lose the ability to attack while the blade is out."*

| Level | Mana | Mana a second while spinning | CD | Range | Radius | Pass damage (pure) | Damage a second (pure) | Slow |
|---|---|---|---|---|---|---|---|---|
| 1 | 125 | 20 | 8 | 1200 | 200 | 100 | 50 | 5% per 5% HP missing |
| 2 | 150 | 30 (6.73c: 25) | 8 | 1200 | 200 | 140 | 75 | same |
| 3 | 175 | 40 (6.73c: 30) | 8 | 1200 | 200 | 180 | 100 | same |

Blade speed 1000 in 6.73 (6.75 −20%, 6.78 900) [CL-6.75], [CL-6.78c]. Disarmed while the blade is out; it cuts trees on its path out and back and in its area; returns by itself if Shredder is more than 2000 away [PD-Shred-6.73]. Pass damage once out and once back [D2W13-Timb]. Spin damage and drain in 0.5 s ticks from 0.5 s after arrival (port; DotA UNVERIFIED). Slow: 5% for each 5% of max HP missing, an aura lingering 0.5 s (port). Out of mana: UNVERIFIED (port: the blade returns). The cooldown starts at the cast [G-SHR]. Vision 800 while flying, 3.34 s after (6.73) [CL-6.76].

How players described him (6.73–6.74): a chaser and tank, Timber Chain "like a blink but can be used in combat, as long as you're not stunned", "almost all his damage is pure", heavy mana use; max Timber Chain first, one early point of Reactive Armor, Chakram whenever possible [G-SHR], [G-DotaBlog].

### 7.3 Hard to reproduce on a fixed tick, in integers

- Troll's form toggle swaps base attack time, range, missile or instant hit, and the castable Whirling Axes, and rescales current HP (cur × newMax / oldMax, fixed rounding).
- Fervor needs a last-attack target per hero and a fixed moment for the stack.
- Melee Whirling Axes: two point hitboxes (r 100) on an orbit stepped each tick, a hit-once set; ranged: five hitboxes on fanned headings.
- Trees as gameplay objects: Whirling Death counts trees cut; Timber Chain sweeps a circle of 90 along a segment against trees, forced movement across terrain, a stun check at latch time.
- Whirling Death's percentage attribute loss changes max HP and mana and restores them on expiry, with fixed rounding.
- Reactive Armor keeps up to 16 independent timers.
- Chakram is a state machine: out at 1000, spin with 0.5 s ticks and drain, back to the moving caster, hit-once sets each way, a 2000 leash, disarm, Return in its slot, a slow from missing HP.

### 7.4 Source keys for this section

| Key | Title | URL |
|---|---|---|
| [PD-Troll-6.6x] | playdota Troll Warlord, Jan 2010 | https://web.archive.org/web/20100127135845/http://www.playdota.com:80/heroes/troll-warlord |
| [PD-Troll-6.72] | playdota Troll Warlord, Dec 2011 (6.72/6.73) | https://web.archive.org/web/20111207045014/http://www.playdota.com:80/heroes/troll-warlord |
| [PD-Troll-2012-01] | playdota Troll Warlord, Jan 2012 | https://web.archive.org/web/20120106214915/http://www.playdota.com:80/heroes/troll-warlord |
| [PD-Shred-6.73] | playdota Goblin Shredder, Jan 2012 (6.73) | https://web.archive.org/web/20120107195913/http://www.playdota.com:80/heroes/goblin-shredder |
| [PD-Shred-6.74] | playdota Goblin Shredder, Mar 2012 | https://web.archive.org/web/20120309043425/http://www.playdota.com:80/heroes/goblin-shredder |
| [PD-Shred-2014] | playdota Goblin Shredder, Jul 2014 | https://web.archive.org/web/20140703195549/http://www.playdota.com/heroes/goblin-shredder |
| [CL-6.72] | changelog 6.72 | https://web.archive.org/web/20110502144633/http://www.playdota.com/changelogs/6.72 |
| [CL-6.72b] | 6.72b | https://web.archive.org/web/20110511135337/http://www.playdota.com/changelogs/6.72b |
| [CL-6.72c] | 6.72c | https://web.archive.org/web/20110606060843/http://www.playdota.com/changelogs/6.72c |
| [CL-6.72d] | 6.72d | https://web.archive.org/web/20110731144954/http://www.playdota.com/changelogs/6.72d |
| [CL-6.73] | 6.73 | https://web.archive.org/web/20120104134752/http://www.playdota.com/changelogs/6.73 |
| [CL-6.73b] | 6.73b | https://web.archive.org/web/20120102055914/http://www.playdota.com/changelogs/6.73b |
| [CL-6.73c] | 6.73c | https://web.archive.org/web/20120113004153/http://www.playdota.com/changelogs/6.73c |
| [CL-6.74] | 6.74 | https://web.archive.org/web/20120313011356/http://www.playdota.com/changelogs/6.74 |
| [CL-6.75] | 6.75 | https://web.archive.org/web/20121006104138/http://www.playdota.com/changelogs/6.75 |
| [CL-6.76] | 6.76 | https://web.archive.org/web/20121022184640/http://www.playdota.com/changelogs/6.76 |
| [CL-6.78c] | 6.78c | https://web.archive.org/web/20130703094839/http://www.playdota.com/changelogs/6.78c |
| [CL-6.79c] | 6.79c | https://web.archive.org/web/20131215181329/http://www.playdota.com/changelogs/6.79c |
| [D2W13-Troll] | Dota 2 Wiki Troll Warlord, Jun 2013 | https://web.archive.org/web/20130630214538/http://dota2.gamepedia.com/Troll_Warlord |
| [D2W15-Troll] | Dota 2 Wiki Troll Warlord, Feb 2015 | https://web.archive.org/web/20150207075530/http://dota2.gamepedia.com/Troll_Warlord |
| [D2W-TrollCL] | Dota 2 Wiki Troll Warlord/Changelogs | https://web.archive.org/web/20200806200025/http://dota2.gamepedia.com/Troll_Warlord/Changelogs |
| [D2W13-Timb] | Dota 2 Wiki Timbersaw, 2013 and 2014 | https://web.archive.org/web/20130629214450/http://dota2.gamepedia.com/Timbersaw |
| [D2W15-Timb] | Dota 2 Wiki Timbersaw, Feb 2015 | https://web.archive.org/web/20150207202452/http://dota2.gamepedia.com/Timbersaw |
| [D2W-TimbCL] | Dota 2 Wiki Timbersaw/Changelogs | https://web.archive.org/web/20200922212528/http://dota2.gamepedia.com/Timbersaw/Changelogs |
| [G-Axe] | playdota guide "Master of the Axe", Jul 2011 (6.72c) | https://web.archive.org/web/20110906042009/http://www.playdota.com/guides/master-of-the-axe |
| [G-Basic] | playdota guide "Basic Guide to Troll Warlord", 2011 | https://web.archive.org/web/20110923151404/http://www.playdota.com/guides/i-kill-for-you |
| [G-SHR] | playdota guide "SHRredderR!", Jan 2012 | https://web.archive.org/web/20120306085344/http://www.playdota.com:80/guides/shrredderr |
| [G-DotaBlog] | DotA-Blog "Rizzrak Guide", Dec 2011 | http://www.dota-blog.com/2011/12/rizzrak-guide-goblin-shredder-build.html |
