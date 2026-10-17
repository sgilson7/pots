# Measurements

Stage 4, recorded by the agent before building and again at handoff. `B2 Require step-by-step justification`.

The state measured rather than recalled, so later changes can be compared with it. A probe prints numbers and asserts loose bounds at most; record its output here with the command that produced it.

| What | Command | Result | When |
| --- | --- | --- | --- |
| Toolchain | `rustc --version; wasm-bindgen --version` | rustc 1.99.0, wasm-bindgen 0.2.127 | 16 Oct 2026 |
| Core tests | `./scripts/count-tests.sh` | 93 passed, 0 failed, 4 ignored (the ignored are diagnostics) | 16 Oct 2026 |
| Tick cost, ten bots | `cargo test -p pots_core --test bots -- --nocapture` | 200–320 µs a tick natively over a 20-minute match (machine shared with other runs) | 16 Oct 2026 |
| Frame rate | a 5v5 arcade match in Chromium on this Mac (Metal), measured 5, 60 and 120 s in | 60 frames a second each time | 16 Oct 2026 |
| WebAssembly size | printed by `./scripts/build.sh` | about 1 MB | 16 Oct 2026 |
| Bot match length | `cargo test -p pots_core --test diag_bots how_long -- --ignored --nocapture` | seed 11: Moonward won at minute 41; seed 3: no winner by minute 60, two towers left | 16 Oct 2026 |
| Bot last hits | `cargo test -p pots_core --test bots -- --nocapture` | 295–301 last hits over ten normal bots in 20 minutes (was 90 before the farm rework) | 16 Oct 2026 |
| Online, in one process | `cargo test -p pots_core --lib online` | five copies, messages late by up to 6 frames and a tenth of fast ones lost: identical checksums; rollbacks happen; ticks keep to the clock | 16 Oct 2026 |
| Online, two browsers | `.venv/bin/python tests/online/smoke.py` | met through the public relays in about a second; 1200 ticks with identical checksums | 16 Oct 2026 |
| Core tests, round 2 | `./scripts/count-tests.sh` | 110 passed, 0 failed, 4 ignored | 16 Oct 2026 |
| Model parts, round 2 | `python3 tools/modelport.py` | 2448 parts over 50 models (546 in round 1); worst rebuild error of a part's transform 1.5 % | 16 Oct 2026 |
| Frame rate, round 2 | `.venv/bin/python tests/browser/fps.py 100` | a custom 5v5 on this Mac (Metal): 60 frames a second at 5 s and at 100 s, camera on the middle of the map (the browser's cap) | 16 Oct 2026 |
| WebAssembly size, round 2 | printed by `./scripts/build.sh` | 1336 KB (1289 KB before the ability marks) | 16 Oct 2026 |
| Creeps inside each other, round 3 | a probe like `crates/core/tests/collision.rs`, seed 5, the first 70 s after the horn, three looks a second | before solid units: up to 17 pairs of creeps at once with centres closer than their two radii (1,779 pair-looks in all; 35 closer than three quarters, 2 closer than half); after: none | 16 Oct 2026 |
| Attack shot size, round 3 | `data/presentation.json` before and after | radius before: every attack shot 0.15 scene units in one colour, a spell's 1.0; after: 0.6 to 1.4 by kind (towers largest), spells 1.0 to 2.2 | 16 Oct 2026 |
| Frame rate, round 3 | `.venv/bin/python tests/browser/fps.py` | a custom 5v5 on this Mac (Metal): 60 frames a second at 5 s and at 90 s with solid units | 16 Oct 2026 |
| WebAssembly size, round 3 | printed by `scripts/wasm_imports.py` | 1421 KB | 16 Oct 2026 |
| Online, round 3 | `.venv/bin/python tests/online/smoke.py --rejoin` | two browsers through the public relays: the dropped one rejoined at its seat, and the checksums at ticks 1350 to 1800 agreed | 16 Oct 2026 |

## Not measured

Two players on different networks (every online run here is on one machine, over the public relays); two strict NATs (no TURN server, so they cannot connect); phones and tablets; Firefox and Safari in an online match; a full match played by a person.
