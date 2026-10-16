# Handoff

Stage 9, `C5 Consolidate in your own words`. A fresh session starts here, not from memory.

## Where things stand (16 October 2026, round 1)

**Protection of the Spheres** is built and published: a browser game that plays like DotA Allstars 6.72/6.73 with ten original heroes, an arcade mission ladder against computer players, and 5v5 online play with rollback netcode and rejoining. Live at <https://sgilson7.github.io/pots/>; repository <https://github.com/sgilson7/pots>. The design journal (a private claude.ai artifact) is <https://claude.ai/artifact/54ByhyyFPtmixHLiECsRXq>, one tab per round.

What exists, by part:

- **The match** (`crates/core`): map generated from a seed, A* pathing around trees, DotA's combat (attack points and backswing, dice damage, the 6.72 damage table, armor, magic resistance, pseudo-random procs), ten heroes with forty abilities, 92 items with recipes and actives, creep waves with upgrades and mega creeps, towers in order with backdoor protection and the glyph, neutral camps refilling on the minute, runes, the Warden and its shield, gold, experience, deaths, respawn, buyback, vision with fog, day and night, a courier per side.
- **Computer players** (`ai.rs`, `data/ai.json`): behavior trees after Vagrancy's pilots; five difficulty levels set apart from the trees; ally and enemy skill chosen separately in the arcade.
- **Arcade**: ten missions and a custom match (0–4 allies, 1–5 enemies, two skills).
- **Online** (`online.rs`, `web/online.js`, `web/net/`): rooms by code through public Nostr relays, a host-centred star of up to ten browsers, rollback over orders, the computer taking a dropped seat, rejoining by the room link with a seat token.
- **3D**: every model in `data/models.json` (from `tools/modelgen.py`), drawn by the WebGL renderer after Gatecrasher's; icons for every item, ability and hero drawn in TikZ with no text, all with hover tips.

## What passed

Recorded at the end of this round; the counts come from the commands, not from memory.

- `./scripts/check.sh`: see the last line of this section.
- `python3 scripts/break_tests.py`: see below.
- Online: in-process tests (five browsers over a lagged, lossy wire; a drop and rejoin; a silent browser) pass; the two-browser smoke test over the real relays passes, including a rejoin; the five-runner test is described in `notes/ONLINE-RUN.md`.

## Open for the person

Each row of `SECOND-ORDER.md` with status *person*, by number:

- **R1.2**: the names and looks of heroes, items and creeps. Are they far enough from their DotA sources?
- **R1.10**: one free courier per side from the start (DotA sold it). Keep?
- **R1.12**: lane creep armor type, Unarmored (playdota) or Medium (Dota 2 port). It changes ranged-creep damage 2×.
- **R1.16**: if the host's browser leaves, the online match ends. A host hand-over is not built.
- **R1.18**: the 143 icons; a few are plain at 48 px.
- **R1.21**: online depends on public relays and STUN, with no TURN server; two strict NATs cannot connect. Try two real networks by hand.

## Try by hand first

1. Arcade, mission 1 ("First steps"): walk to the middle lane, wait for creeps, and land last hits as their health drops. Count: the goal line says how many you have.
2. A custom match, 4 allies and 5 enemies on Normal: buy boots at the base shop (B), learn abilities (the + under each), and play to a tower.
3. Online with a second computer on another network: host, send the link, both pick, start. Close one browser mid-match and reopen its link.

## What the next session must not assume

- That two players on different networks can connect: every online test so far ran on one machine.
- That the bots play well: they last-hit, deny, use abilities and push, but a match between them lasts 40–60 minutes.
- That the art is final: the models are simple shapes; the icons are first drafts.
