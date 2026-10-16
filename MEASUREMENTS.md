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

## Not measured

Two players on different networks (every online run here is on one machine, over the public relays); two strict NATs (no TURN server, so they cannot connect); phones and tablets; Firefox and Safari in an online match; a full match played by a person.
