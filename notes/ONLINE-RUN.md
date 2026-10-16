# Online run "first": live notes

Five Chromium runners, one 5v5 online match (tests/online/runners.py, runs/first). Notes appended per read.

## 01:04 EDT
Room RFCPF created by runner 1 (t=1792127051). Waiting for joiners.

## 01:06-01:08 EDT: first attempt failed in setup
`joined` event logged `peers: 0` after 122.5 s; the script then timed out waiting for `.hero-mini[data-v='2']` on runner 2 (Playwright TimeoutError in runs-first.out) and exited. No match ran. Cause (per the lead): joiners reused the already-open page and `goto(BASE#room=CODE)` only changed the hash, so web/online.js `setup()` (which reads `#room=` once at load) never ran and no join happened. Fixed in the script by opening a fresh page; runs/first deleted and the run restarted.

# Second attempt (same folder)

## 01:08 EDT (match t+45 s)
Room HNVPG; `joined` peers=4 after 4.0 s; `started` at 10.0 s. All five on screen=match, status online.playing, ticks 1236-1239 at the same poll, each advancing 29.9-30.1 ticks/s. Tick spread within a poll round <= 3. tick - confirmed in [-2, +2] (confirmed can exceed tick by DELAY=2 because a browser's own orders are scheduled 2 ticks ahead). Rollbacks 0, deepest 0, stalls 0 on all five. 13 checksum ticks seen (90..1170), all identical across runners; the live per-tick `sum` also agrees on all 40 ticks two runners happened to report. Runner 2 errors=1 since its first probe (text only available at the end). Joiners report peers=0 (only the host counts peers).

## 01:09 EDT (match t+110 s)
Ticks 3311-3312 at one poll, all 30.0/s, spread <= 4 ever. Rollbacks/deepest/stalls still 0 everywhere; tick - confirmed in [-2, +2]. 36 checksum ticks, all agree; 82 shared live-sum ticks agree. No status other than online.playing, desync false.
Screenshots at 71 s: r1 (Ironvow, level 1, 309 gold) shows the HUD, minimap with both sides' buildings, banner "The computer is playing your hero.", camera on a plain with a red item/box; clock 0:01. r4 (Kestrel, 80 gold, Q on cooldown 14, mana 108/208 so it has cast) shows a cluster of heroes in front of a purple (Moonward) tower; same banner and clock 0:01. The game is visibly running under autopilot.
Observation: the rollback path is not being exercised. All five browsers are on one machine, so orders arrive within the 2-tick DELAY and no tick is ever run on a guess (confirmed >= tick most polls).

## 01:10 EDT (match t+176 s)
Ticks 5262-5263 at one poll, 30/s, spread <= 4. Rollbacks/deepest/stalls 0; 58 checksum ticks (to 5220) all agree; 130 shared live sums agree. All online.playing, no desync.
New: runner 4's error count went 0 -> 52 between t+141 s and t+157 s (5, 10, 14, 22, 28, 36, 42, 49, 52 at 2 s polls), then stopped. At ~3/s this is console errors from the page, not the script's pokes (a poke happens on ~25% of 2 s loops). Text not available until the summary.
Screenshots at 193 s: r2 (Cinder) and r4 (Kestrel) show the same kill feed ("First blood / Tempest killed Thistle / Cinder killed Ironvow / Cinder killed Longwick / Tempest killed Gristle / Ironvow killed Cinder"), the same score 1-4 and clock 2:03, and the same fight at a river bank (Sunward orange and Moonward purple heroes). Kestrel 140/587 hp, level 2. Two browsers draw the same match state.

## 01:12-01:13 EDT (match t+240..313 s): drop, freeze, double speed, failed rejoin
**Drop** logged at t+240.5 (runner 4, url `#room=HNVPG.GDGRD`).
**Freeze, ~11 s.** The four remaining browsers stopped at tick 7238 = confirmed 7223 + MAX_AHEAD 15 from t+242.8 to t+251.2 (polls: 7238/7223 four times in a row), stalls rising 0 -> 103 -> 223 -> 361 -> 484 -> 603 -> 664 (one per display frame). At t+253.2 the host shows peers 4 -> 3 and away=1, and everyone resumes. So the host learned of the drop about 11-12 s after the browser closed, presumably from the transport (connectionstatechange/Trystero onPeerLeave); SILENT_TICKS=300 counts ticks and ticks were frozen, so the silence check could not fire during the freeze. Joiners report away=0 throughout: only the host's session sets `s.away` (event "left"), so away=1 on the host alone is what the code produces.
**Rollbacks appear at resumption:** rollbacks 1-2, deepest 2-6, replayed 2-8; unchanged since.
**Double speed after the drop.** From t+255 on every runner advances ~60 ticks/s (rate 60.0/60.0/60.3/60.0 over 20 s) and has not slowed: tick 11145 at t+312.5, whereas 30/s since start gives ~9375. It is not a catch-up of the freeze (that would end ~330 ticks later). Likely cause in crates/core/src/online.rs: `speak_for_away` sets `s.known[k] = max(known, now + DELAY + 1)` for the away seat, and `frame` computes `ahead_of_me = lead - (tick + DELAY)` over all other human seats including the away one. On the host that is always >= 1, and with `ahead_of_me > 0` the loop runs a tick without waiting for `acc_us` (and clamps acc to -1 tick, so the credit is not repaid). Joiners receive that relayed known and run unpaced too. Pace becomes "as fast as the confirmed window allows", here one tick per 60 Hz frame. Checksums still agree (118 checksum ticks, 0 disagreements), so the speed-up is consistent across browsers but wrong.
**Rejoin failed.** `rejoin` logged at t+271.5. Runner 4 loads with screen=online.lobby, role join, seat None, tick 0, status online.connecting, and stays that way. At t+296 (24.5 s later) runner 4 and runners 1, 2, 3 all switch status to `online.no_path`; runner 5 stays online.playing. In web/net/rtc.js `no_path` comes from Trystero's `onJoinError` with a peerId, i.e. a handshake with a particular browser failed, and core `event("error", key)` writes it into `o.status` even mid-match, so the playing browsers now carry an error status while still playing. Runner 4 never reached catching=true; `autopilot_again` cannot fire because its screen is not match.

## 01:14 EDT (match t+392 s)
Runners 1, 2, 3, 5 at ticks 15900-15902, still ~60/s (59.9-60.2). Runner 4 still lobby, tick 0, online.no_path. 176 checksum ticks seen, 0 disagreements (among the four still playing; runner 4 has none). No desync. Rollbacks unchanged (1-2, deepest 2-6), stalls frozen at 664.
Runner 3 errors climbing since t+338.9: 8, 22, 42, 61 ... 345 at t+403.9 (~4-10/s).
Screenshots at 377 s: r4 shows the online LOBBY for HNVPG with all ten seats "Open: computer (Passive)" (it never received the host's lobby/seats) and the text "The two browsers could not reach each other directly. A network on either side may block it." r3 is in the match: clock 7:00 NIGHT, score 1-9, kill feed (e.g. "Kestrel killed Ironvow", "One of your towers fell"); its HUD shows the selected Sunward Base Tower with a broken-image portrait (core `ui.rs` gives non-hero units an empty portrait src) and the no_path toast drawn on top of "The computer is playing your hero." (two banners overlapping). The game clock confirms the speed-up: 0:01 at t+71 and 2:03 at t+193 (clock ~= t - 70 s), but 7:00 at t+377 where real time would give ~5:07.
Hypothesis (unconfirmed until the summary lists error texts): runner 3's error stream starts when it has a non-hero unit selected (broken portrait), as runner 4's 52 errors at t+141-157 may have.

## 01:15 EDT (match t+479 s)
Four playing runners at ticks 21129-21134, still ~60/s; runner 4 unchanged (lobby, tick 0, no_path). 234 checksum ticks, 0 disagreements; no desync. Rollbacks/stalls unchanged. Runner 3 errors 608 and still rising. Match not over (over=false); hero levels 2-6.

## 01:16 EDT (match t+547 s)
Ticks 25153-25155 on the four, ~60/s; 279 checksum ticks, 0 disagreements. Runner 3 errors steady at 608 (stopped rising).
Screenshots at 499 s: r1 (Ironvow, lvl 2) and r5 (Longwick, lvl 4) show the same fight at the same river crossing, the same eight-line kill feed and score 1-10, clocks 11:04 / 11:05 (one poll apart). 11:05 at ~490 s of real match time again shows the ~2x speed after the drop. r1 still shows the no_path banner overlapping the autopilot banner; r5 (status online.playing) does not.

## 01:17 EDT: end (summary.json written at t+600.9)
Final ticks [28455, 28453, 28450, 0, 28452]; desync all false; rollbacks [2,1,2,0,2]; deepest [6,2,6,0,6]; stalls [664,664,664,0,664]. Errors: r2 one `WebSocket connection to 'wss://relay.damus.io/' failed ... 503`; r3 (608) and r4 (52) are all `Failed to load resource: ... 404 (File not found)`; r1, r5 none.
The summary's `checksums_agree: true` is vacuous: `common_checksum_ticks` is [] because runner 4's sumlog is empty, so the intersection over five runners is empty. My own comparison over every poll: 314 checksum ticks (90 .. 28260) seen; 235 reported by 4 runners and 79 by all 5, none differing. Live per-tick `sum` agreed on all 303 ticks reported by two or more runners at the same tick.

# Verdict
**Was a 5v5 match played by 5 runners over the online implementation?** Partly. All five joined room HNVPG within 4 s, the match started at 10 s, and for the first 240 s all five ran the same match at 30.0 ticks/s, at most 4 ticks apart, with computer-played heroes fighting (screenshots at 71 s and 193 s show matching kill feeds, scores and clocks across browsers). After runner 4 was closed it never returned, so the last 360 s were played by four browsers, with the host speaking for seat 4.
**Did checksums agree?** Yes, wherever compared: 0 disagreements across 314 checksum ticks, host desync flag never set. The script's own end check compared nothing (see above), so this rests on the per-poll sumlogs.
**Rollback.** On one machine, rollback was barely exercised: 0 rollbacks for the first 240 s (orders always arrived within DELAY=2, tick - confirmed stayed in [-2, +2]). The only rollbacks (1-2 per browser, deepest 2-6, replayed 2-8) came at the resumption after the drop. Stalls: 664 per browser, all during one ~11 s freeze.
**Did the dropped runner rejoin and agree?** No. It reloaded `#room=HNVPG.GDGRD` at t+271.5, sat in the lobby (all seats shown "Open: computer (Passive)") with status online.connecting, and at t+296 got `online.no_path`; catching was never true, its tick stayed 0, `autopilot_again` never logged.

## Problems, with evidence and likely cause
1. **Match runs at double speed after a drop (most serious).** Rate 30.0/s before t+240, 59.9-60.3/s from t+255 to the end on all four; game clock 2:03 at t+193 but 7:00 at t+377 and 11:05 at ~t+490. Likely cause (crates/core/src/online.rs): `speak_for_away` sets `known[away] = now + DELAY + 1` every host frame; `frame` takes `lead` over all other human seats including the away one, so `ahead_of_me = lead - (tick + DELAY) >= 1` permanently, and a positive `ahead_of_me` bypasses the `acc_us < tick_us` pacing check (acc is clamped at -1 tick, so nothing is paid back). Joiners receive the relayed known and do the same. The away seat should not count as a lead (or the host should only speak for it up to its own paced tick).
2. **~11 s freeze for everyone when a browser vanishes.** All four held at tick 7238 = confirmed 7223 + MAX_AHEAD from t+242.8 to t+251.2; host peers 4 -> 3 and away=1 only at t+253.2. The host waits for the transport to report the leave; the SILENT_TICKS=300 fallback counts world ticks, which do not advance while stalled, so it cannot fire during the freeze.
3. **Rejoin fails.** Evidence above. rtc.js maps a Trystero `onJoinError` with a peerId to `online.no_path` (a handshake with a specific browser failed). The handshake failure itself is not explained by the logs (runner 2's 503 from relay.damus.io shows relay flakiness, but initial joining worked in 4 s). Needs a reproduction with the rejoiner's console.
4. **A transport error overwrites the status of browsers mid-match.** Runners 1, 2, 3 switched to `online.no_path` at t+296 while still playing (core `event("error", key)` sets `o.status` unconditionally); the screenshots show the "could not reach each other" banner drawn over "The computer is playing your hero." for the rest of the match.
5. **404 console errors in bursts** (r4: 52 at t+141-157; r3: 608 from t+339 to ~t+480). The file name is not logged. Runner 3's screenshot at t+377 showed a selected Sunward Base Tower with a broken-image portrait, so a missing image for a selected unit is a candidate, not confirmed.
6. **Test-script verdict is vacuous** when any runner has no sumlog (`common_checksum_ticks: []` yields `checksums_agree: true`).
