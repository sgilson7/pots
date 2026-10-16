# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | "copy the builder repo in here" meant the empty `OGDota` folder. | The person made `OGDota` minutes before; Builder's procedure copies `template/`, not the whole reference repository. | The template's tracked files were copied into `OGDota` (no build output). The folder keeps its name; the repository and the game are `pots`. | | done |
| R1.2 | worklist | Every hero, item, creep and building needed an original name and look ("I DO NOT want to copy characters"). | The names and looks are the agent's: Ironvow, Rime, Cinder, Thistle, Gristle, Tectus, Kestrel, Hexwright, Longwick, Tempest; the Sunward and Moonward; the Spheres; the Warden. Item names are in `data/strings.json`. | The person reads the list and decides whether each is far enough from its DotA source. Ability names keep DotA's mechanics under new names. | | person |
| R1.3 | divergence | The tenth hero would follow Axe. | The literature review sourced Zeus fully (6.73 tables) and not Axe. | The tenth hero, Tempest, follows Zeus; Axe's model became the storm caller. | | done |
| R1.4 | divergence | DotA used Warcraft III's stock attack/armor table. | DotA 6.72 edited it: Normal→Hero 75 %, Siege→Hero 75 %, Chaos→Fortified 40 %, and spells deal 75 % to heroes (the 25 % innate resistance). | `data/rules.json` carries DotA's table; heroes resist 25 % of spell damage through `hero_magic_resist`. A reversion to the stock table is the risk this row names. | `data/rules.json`; a test to come | done |
| R1.5 | divergence | Creep and hero damage rolled evenly within its range. | DotA rolls XdY dice, so the middle comes up more often; players name it as part of last-hitting. | `combat::roll` adds two dice. | | done |
| R1.6 | divergence | Bash and crit chances were plain rolls. | Warcraft III uses pseudo-random chance; players said long streaks would change the game. | `combat::prd` keeps a counter per unit and proc, with Warcraft III's constants. | | done |
| R1.7 | divergence | A hero's attack on an enemy hero drew creep aggro when it landed. | DotA draws aggro when the attack *order* is issued, and spells never do. | The aggro moved to `orders::apply` for `Cmd::Attack` on an enemy hero. | | done |
| R1.8 | divergence | Kill rewards used my own formulas (assist gold for damagers, XP as base + per level). | DotA 6.72: hero-kill XP by victim level plus a bonus by the number sharing; assist gold to the other enemy heroes within 1000 by count; first blood +200; streak values 50 to 400; denies give 36/n XP to melee and 18/n to ranged heroes. | `combat::kill_hero` and `combat::kill` follow the review's tables in `data/rules.json`. | | done |
| R1.9 | finding | The map's real size and coordinates. | The review could not verify DotA's World Editor size. | The map is laid out to the review's topology at 16 384 units square, point-symmetric through the centre (`tools/mapgen.py`), with three lanes, a diagonal river, two rune spots, the boss pit by the lower rune, six camps a side (easy, two moderate, two hard, ancient), two side and two secret shops. | | done |
| R1.10 | divergence | A per-side courier had to be bought, as in DotA (200 gold). | Without one, buying away from the base is impossible, which a first-time player meets at once. | Each side gets one courier at the start, invulnerable; a purchase away from the shop goes through it at full price. | | person |
| R1.11 | divergence | Items bought with a full inventory drop at the fountain in DotA (no stash). | Ground items are not in this build. | A purchase at the shop with a full inventory is refused; through the courier it waits on the courier until a slot is free. | | open |
| R1.12 | finding | Creep and lane-creep armor type. | Playdota prints "Unarmored"; the Dota 2 port says Medium. Pierce damage differs 2×. | The data follows the primary source (Unarmored) and the review flags it. | | person |
| R1.13 | divergence | Backdoor protection: life lost without the regen armor can never regenerate. | That rule needs a second health number per building. | Protected buildings regenerate 90 a second after 15 s with no enemy creeps within 900; the lost-life rule is left out. | | open |
| R1.14 | finding | The first full 20-minute match between ten bots. | It ran at about 200 µs a tick natively; 27 kills, 135 last hits, 7 towers down. | Rollback replays up to several ticks a frame, so the tick cost is the budget to watch. | `MEASUREMENTS.md` | done |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read. Listed when `data/strings.json` is written.

| Key | String |
| --- | --- |
