//! Online play between several copies of the game in one test, over a
//! simulated wire that delays and drops messages.

use super::*;
use crate::game::{Files, Game};
use crate::rng::Rng;

fn files() -> Files<'static> {
    Files {
        strings: include_str!("../../../../data/strings.json"),
        glossary: include_str!("../../../../GLOSSARY.md"),
        rules: include_str!("../../../../data/rules.json"),
        units: include_str!("../../../../data/units.json"),
        abilities: include_str!("../../../../data/abilities.json"),
        items: include_str!("../../../../data/items.json"),
        map: include_str!("../../../../data/map.json"),
        ai: include_str!("../../../../data/ai.json"),
        models: include_str!("../../../../data/models.json"),
        presentation: include_str!("../../../../data/presentation.json"),
        keys: include_str!("../../../../data/keys.json"),
        arcade: include_str!("../../../../data/arcade.json"),
    }
}

/// A message in flight: when it lands, from whom, to whom, what.
struct Flight {
    at: u32,
    fast: bool,
    from: usize,
    to: usize,
    text: String,
}

/// Browsers in one room: browser 0 hosts. The wire delays each message by
/// up to `lag` frames and loses `loss` percent of fast ones.
struct Room {
    games: Vec<Game>,
    connected: Vec<bool>,
    wire: Vec<Flight>,
    rng: Rng,
    frame: u32,
    lag: u32,
    loss: u32,
}

const FRAME_US: i64 = 16_667;

fn peer_name(k: usize) -> String {
    if k == 0 {
        "host".into()
    } else {
        format!("j{k}")
    }
}

impl Room {
    fn new(n: usize, lag: u32, loss: u32, seed: u64) -> Room {
        let mut games: Vec<Game> = (0..n)
            .map(|k| Game::new(&files(), seed + k as u64).expect("game loads"))
            .collect();
        games[0].act("online.name", "host").unwrap();
        games[0].act("online.host", "").unwrap();
        let code = games[0].online.room.clone();
        for (k, g) in games.iter_mut().enumerate().skip(1) {
            g.act("online.name", &format!("p{k}")).unwrap();
            g.act("online.join", &code).unwrap();
        }
        let mut r = Room {
            connected: vec![true; n],
            games,
            wire: Vec::new(),
            rng: Rng::new(seed),
            frame: 0,
            lag,
            loss,
        };
        for k in 1..n {
            r.connect(k);
        }
        r
    }

    fn connect(&mut self, k: usize) {
        self.connected[k] = true;
        net_event(&mut self.games[k], "peer", "host");
    }

    fn post(&mut self, from: usize, to: usize, text: String, fast: bool) {
        if fast && self.loss > 0 && self.rng.below(100) < self.loss {
            return;
        }
        let delay = if self.lag > 0 {
            self.rng.between(0, self.lag as i32) as u32
        } else {
            0
        };
        let mut at = self.frame + delay;
        // the reliable channel keeps order between two browsers; only fast messages may overtake
        if !fast {
            let last = self
                .wire
                .iter()
                .filter(|f| f.from == from && f.to == to && !f.fast)
                .map(|f| f.at)
                .max()
                .unwrap_or(0);
            at = at.max(last);
        }
        self.wire.push(Flight {
            at,
            fast,
            from,
            to,
            text,
        });
    }

    fn step(&mut self, orders: bool) {
        self.frame += 1;
        let now = self.frame as i64 * FRAME_US;
        let n = self.games.len();
        for k in 0..n {
            if !self.connected[k] {
                continue;
            }
            if orders && self.games[k].m.is_some() && self.rng.below(30) == 0 {
                // a player's click somewhere near the middle of the map
                let x = self.rng.between(3000, 13000);
                let y = self.rng.between(3000, 13000);
                self.games[k].command(Cmd::Move([x, y]));
            }
            self.games[k].frame(now);
            let out: serde_json::Value = serde_json::from_str(&take_out(&mut self.games[k])).unwrap();
            for item in out["out"].as_array().unwrap() {
                let to = item[0].as_str().unwrap().to_string();
                let text = item[1].as_str().unwrap().to_string();
                let fast = item[2].as_bool().unwrap();
                if k == 0 {
                    let targets: Vec<usize> = if to == "*" {
                        (1..n).filter(|&j| self.connected[j]).collect()
                    } else {
                        (1..n).filter(|&j| peer_name(j) == to).collect()
                    };
                    for j in targets {
                        self.post(0, j, text.clone(), fast);
                    }
                } else {
                    self.post(k, 0, text, fast);
                }
            }
        }
        let due: Vec<Flight> = {
            let (now_due, later): (Vec<Flight>, Vec<Flight>) = std::mem::take(&mut self.wire)
                .into_iter()
                .partition(|f| f.at <= self.frame);
            self.wire = later;
            now_due
        };
        for f in due {
            if !self.connected[f.to] || !self.connected[f.from] {
                continue;
            }
            let from = if f.from == 0 {
                "host".to_string()
            } else {
                peer_name(f.from)
            };
            receive(&mut self.games[f.to], &from, &f.text);
        }
    }

    fn checksums_agree(&self) -> (usize, bool) {
        let host = self.games[0].online.s.as_ref().expect("host session");
        let mut compared = 0;
        let mut agree = true;
        for g in &self.games[1..] {
            let Some(s) = g.online.s.as_ref() else { continue };
            for (tick, sum) in &s.sums {
                if let Some(h) = host.sums.get(tick) {
                    compared += 1;
                    agree &= h == sum;
                }
            }
        }
        (compared, agree)
    }
}

fn net_event(g: &mut Game, kind: &str, peer: &str) {
    event(g, kind, peer);
}

fn pick_and_start(room: &mut Room) {
    // every browser picks its own hero
    for _ in 0..30 {
        room.step(false);
    }
    let n = room.games.len();
    for k in 0..n {
        room.games[k].act("pick", &k.to_string()).unwrap();
    }
    for _ in 0..30 {
        room.step(false);
    }
    let seats = room.games[0].online.seats.clone();
    room.games[0].act("online.start", "").unwrap_or_else(|e| {
        panic!(
            "everyone picked: {e} {seats:?} peers {:?}",
            room.games[0].online.peers
        )
    });
}

#[test]
fn every_message_round_trips() {
    let msgs = vec![
        Msg::Hello {
            proto: PROTOCOL,
            name: "a".into(),
            token: "T".into(),
        },
        Msg::Sit { seat: 3 },
        Msg::Pick { hero: 2 },
        Msg::Reroll,
        Msg::Lobby {
            seats: vec![Seat {
                rerolled: true,
                ..Seat::default()
            }],
            bots: 2,
            started: false,
            aram: true,
            arena: false,
            fill: false,
        },
        Msg::In {
            seats: vec![(4, 120, vec![[118, 2, 100, 200, 0, 0]])],
        },
        Msg::Sum {
            tick: 90,
            sum: "123".into(),
        },
        Msg::Bye {
            reason: "online.full".into(),
        },
    ];
    for m in msgs {
        assert_eq!(Msg::parse(&m.text()), Some(m));
    }
    assert_eq!(Msg::parse("not json"), None);
}

#[test]
fn the_room_name_carries_the_protocol_and_not_the_build() {
    assert_eq!(room_name(" abcde "), format!("{PROTOCOL}-ABCDE"));
}

#[test]
fn five_browsers_with_lag_and_loss_play_the_same_match() {
    // five players, messages late by up to six frames, a tenth of fast ones lost
    let mut room = Room::new(5, 6, 10, 41);
    pick_and_start(&mut room);
    for _ in 0..20 {
        room.step(false);
    }
    for k in 0..5 {
        assert!(room.games[k].m.is_some(), "browser {k} is in the match");
    }
    for _ in 0..(60 * 40) {
        room.step(true);
    }
    let (compared, agree) = room.checksums_agree();
    assert!(compared >= 4 * 5, "checksums were compared: {compared}");
    assert!(
        agree,
        "every browser's world agrees with the host's at every checksum"
    );
    assert!(
        !room.games.iter().any(|g| g.online.desync),
        "no browser saw a desync"
    );
    let rollbacks: u32 = room
        .games
        .iter()
        .filter_map(|g| g.online.s.as_ref())
        .map(|s| s.rollbacks)
        .sum();
    assert!(rollbacks > 0, "late orders were rolled back into place");
    let ticks: Vec<u32> = room
        .games
        .iter()
        .map(|g| g.m.as_ref().unwrap().world.tick)
        .collect();
    let (lo, hi) = (*ticks.iter().min().unwrap(), *ticks.iter().max().unwrap());
    assert!(
        hi - lo <= MAX_AHEAD + DELAY + 6,
        "browsers stay within the limit of each other: {ticks:?}"
    );
    assert!(lo > 30 * 30, "the match ran: {ticks:?}");
    // the clock: 2400 frames at 60 a second are 40 seconds, 1200 ticks
    assert!(
        hi <= 1200 + 2 * (MAX_AHEAD + DELAY),
        "the match keeps to the clock, not faster: {ticks:?}"
    );
}

#[test]
fn a_player_who_drops_is_played_by_the_computer_and_rejoins() {
    let mut room = Room::new(3, 3, 0, 77);
    pick_and_start(&mut room);
    for _ in 0..600 {
        room.step(true);
    }
    // browser 2 drops
    room.connected[2] = false;
    net_event(&mut room.games[0], "left", "j2");
    let seat2 = room.games[2].online.my_seat.unwrap();
    for _ in 0..600 {
        room.step(true);
    }
    let host_world = &room.games[0].m.as_ref().unwrap().world;
    assert!(
        host_world.players[seat2].bot.is_some(),
        "the computer plays the dropped seat"
    );
    let info: Vec<_> = room
        .games
        .iter()
        .map(|g| {
            (
                g.m.as_ref().map(|m| m.world.tick),
                g.online.s.as_ref().map(|s| (confirmed(s), s.known, s.stalls)),
            )
        })
        .collect();
    assert!(
        room.games[1].m.as_ref().unwrap().world.tick > 550,
        "the match did not wait for the dropped browser: {info:?}"
    );
    // the same player returns in a fresh page with its room link
    let token = room.games[2].online.token.clone();
    let code = room.games[0].online.room.clone();
    let mut fresh = Game::new(&files(), 999).unwrap();
    fresh.act("online.join", &format!("{code}.{token}")).unwrap();
    room.games[2] = fresh;
    room.connect(2);
    for _ in 0..1200 {
        room.step(true);
    }
    let back = &room.games[2];
    assert_eq!(
        back.online.my_seat,
        Some(seat2),
        "the returning player has the same seat"
    );
    assert!(!back.online.catching, "the replay of the match so far finished");
    assert!(
        back.m.as_ref().unwrap().world.players[seat2].bot.is_none(),
        "the player has the seat back"
    );
    let (compared, agree) = room.checksums_agree();
    assert!(
        compared > 0 && agree,
        "the returning browser's world agrees with the host's ({compared})"
    );
}

#[test]
fn a_browser_that_falls_silent_is_handed_to_the_computer_without_a_long_wait() {
    let mut room = Room::new(3, 2, 0, 5);
    pick_and_start(&mut room);
    for _ in 0..300 {
        room.step(false);
    }
    // browser 2 stops answering, and the transport never says it left
    room.connected[2] = false;
    let seat2 = room.games[2].online.my_seat.unwrap();
    let before = room.games[0].m.as_ref().unwrap().world.tick;
    // five seconds of frames: the host hears nothing for two and a half,
    // then gives the seat to the computer and the match goes on
    for _ in 0..300 {
        room.step(false);
    }
    let host = room.games[0].m.as_ref().unwrap();
    assert!(
        host.world.players[seat2].bot.is_some(),
        "the silent seat is the computer's"
    );
    let ran = host.world.tick - before;
    // 300 frames are 150 ticks; at most the silence (75 ticks) and the limit are lost
    assert!(
        ran >= 150 - 75 - MAX_AHEAD - DELAY - 10,
        "the match waited too long: {ran} ticks in five seconds"
    );
}

#[test]
fn browsers_on_a_fast_wire_keep_to_the_clock() {
    // no lag at all, as between browsers on one machine: each sees the
    // others' ordinary lead at once, which must not set them chasing
    let mut room = Room::new(3, 0, 0, 13);
    pick_and_start(&mut room);
    // and one browser drops, so the host speaks for its seat, as in the
    // run where the browsers once raced at twice the clock
    room.connected[2] = false;
    net_event(&mut room.games[0], "left", "j2");
    let start = room.games[0].m.as_ref().unwrap().world.tick;
    for _ in 0..1200 {
        room.step(false);
    }
    let ran = room.games[1].m.as_ref().unwrap().world.tick - start;
    // 1200 frames at 60 a second are 600 ticks
    assert!(ran <= 600 + 10, "the match ran {ran} ticks in 600 ticks of time");
    assert!(ran >= 600 - 10, "the match ran {ran} ticks in 600 ticks of time");
}

#[test]
fn a_player_back_before_the_host_noticed_the_drop_gets_the_seat() {
    let mut room = Room::new(2, 1, 0, 23);
    pick_and_start(&mut room);
    for _ in 0..300 {
        room.step(false);
    }
    let seat = room.games[1].online.my_seat.unwrap();
    // the page reloads at once: no leave, no silence long enough to notice
    let token = room.games[1].online.token.clone();
    let code = room.games[0].online.room.clone();
    let mut fresh = Game::new(&files(), 4242).unwrap();
    fresh.act("online.join", &format!("{code}.{token}")).unwrap();
    room.games[1] = fresh;
    room.connect(1);
    for _ in 0..900 {
        room.step(false);
    }
    let back = &room.games[1];
    assert_eq!(back.online.my_seat, Some(seat), "the same seat");
    assert!(back.online.status != "online.full", "not turned away");
    assert!(!back.online.catching && back.m.is_some(), "in the match");
    let (compared, agree) = room.checksums_agree();
    assert!(compared > 0 && agree, "and agreeing ({compared})");
}

#[test]
fn three_browsers_in_all_random_all_mid_are_dealt_alike_and_play_one_match() {
    // round 4: the host switches the mode on; a joiner rerolls; everyone
    // sees the same dealt heroes and the match agrees in every browser
    let mut room = Room::new(3, 4, 5, 57);
    for _ in 0..30 {
        room.step(false);
    }
    room.games[0].act("online.mode", "").unwrap();
    for _ in 0..30 {
        room.step(false);
    }
    assert!(
        room.games.iter().all(|g| g.online.aram),
        "every browser sees the mode"
    );
    let before: Vec<i32> = room.games[0].online.seats.iter().map(|s| s.pick).collect();
    let mine = room.games[1].online.my_seat.expect("seated");
    room.games[1].act("online.reroll", "").unwrap();
    for _ in 0..30 {
        room.step(false);
    }
    let after: Vec<i32> = room.games[0].online.seats.iter().map(|s| s.pick).collect();
    assert_ne!(
        after[mine], before[mine],
        "the reroll dealt that seat another hero"
    );
    for (k, (a, b)) in before.iter().zip(&after).enumerate() {
        if k != mine {
            assert_eq!(a, b, "nobody else's hero changed");
        }
    }
    for g in &room.games[1..] {
        let seen: Vec<i32> = g.online.seats.iter().map(|s| s.pick).collect();
        assert_eq!(seen, after, "every lobby shows the same deal");
    }
    assert!(room.games[1].act("online.reroll", "").is_err(), "one reroll only");
    assert!(
        room.games[2].act("pick", "0").is_err(),
        "no choosing in this mode"
    );
    room.games[0].act("online.start", "").unwrap();
    for _ in 0..(60 * 20) {
        room.step(true);
    }
    for g in &room.games {
        let w = &g.m.as_ref().expect("in the match").world;
        assert!(w.aram, "the match is in the mode");
        for (k, p) in w.players.iter().enumerate() {
            let def = w.get(p.hero).map(|u| u.def as usize);
            assert!(def.is_some(), "seat {k} has a hero");
            if after[k] >= 0 {
                assert_eq!(
                    def,
                    Some(g.rules.heroes[after[k] as usize]),
                    "seat {k} plays the dealt hero"
                );
            }
        }
    }
    let (compared, agree) = room.checksums_agree();
    assert!(compared >= 4, "checksums were compared: {compared}");
    assert!(
        agree && !room.games.iter().any(|g| g.online.desync),
        "one match in every browser"
    );
}

#[test]
fn with_computer_players_off_only_the_people_in_the_room_play() {
    // T15: the host switches the computer players off; the open seats stay
    // empty in every browser, and the match still agrees between them
    for aram in [false, true] {
        let mut room = Room::new(2, 3, 5, 61);
        for _ in 0..30 {
            room.step(false);
        }
        if aram {
            room.games[0].act("online.mode", "").unwrap();
        }
        assert!(
            room.games.iter().all(|g| g.online.fill),
            "on until the host switches it off"
        );
        room.games[0].act("online.fill", "").unwrap();
        for _ in 0..30 {
            room.step(false);
        }
        assert!(
            room.games.iter().all(|g| !g.online.fill),
            "every lobby sees it off"
        );
        if !aram {
            room.games[0].act("pick", "0").unwrap();
            room.games[1].act("pick", "1").unwrap();
            for _ in 0..30 {
                room.step(false);
            }
        }
        room.games[0].act("online.start", "").unwrap();
        for _ in 0..(60 * 20) {
            room.step(true);
        }
        for g in &room.games {
            let w = &g.m.as_ref().expect("in the match").world;
            let present = w.players.iter().filter(|p| p.present).count();
            let heroes = w
                .players
                .iter()
                .filter(|p| p.hero != crate::world::NO_UNIT)
                .count();
            let bots = w.players.iter().filter(|p| p.bot.is_some()).count();
            assert_eq!(
                (present, heroes, bots),
                (2, 2, 0),
                "two players and no computer (aram {aram})"
            );
        }
        let (compared, agree) = room.checksums_agree();
        assert!(compared >= 4 && agree, "one match in both browsers ({compared})");
    }
}

#[test]
fn a_name_changed_in_the_lobby_reaches_every_browser() {
    // round 7: names can change in the lobby, the host's and a joiner's,
    // and every browser in the room sees them before the match starts
    let mut room = Room::new(3, 3, 0, 71);
    for _ in 0..30 {
        room.step(false);
    }
    let names = |room: &Room, k: usize| -> Vec<String> {
        room.games[k]
            .online
            .seats
            .iter()
            .filter(|s| s.human)
            .map(|s| s.name.clone())
            .collect()
    };
    for k in 0..3 {
        let mut n = names(&room, k);
        n.sort();
        assert_eq!(n, ["host", "p1", "p2"], "browser {k} before");
    }
    room.games[2].act("online.name", "Kestrel fan").unwrap();
    room.games[0].act("online.name", "The host").unwrap();
    for _ in 0..30 {
        room.step(false);
    }
    for k in 0..3 {
        let mut n = names(&room, k);
        n.sort();
        assert_eq!(n, ["Kestrel fan", "The host", "p1"], "browser {k} after");
    }
    // a name loses control characters, is cut at 16 characters ("avery long
    // name ") and shown without the trailing space
    room.games[1]
        .act("online.name", "a\tvery long name indeed")
        .unwrap();
    for _ in 0..30 {
        room.step(false);
    }
    assert!(names(&room, 0).contains(&"avery long name".to_string()));
}

#[test]
fn three_browsers_on_the_arena_choose_any_hero_and_play_one_match() {
    // round 8: the host's mode switch goes Normal, All Random All Mid, the
    // arena; each browser picks its own hero; the match is on the arena's
    // map in every browser and agrees tick for tick
    let mut room = Room::new(3, 4, 5, 81);
    for _ in 0..30 {
        room.step(false);
    }
    room.games[0].act("online.mode", "").unwrap();
    room.games[0].act("online.mode", "").unwrap();
    for _ in 0..30 {
        room.step(false);
    }
    assert!(
        room.games.iter().all(|g| g.online.arena && !g.online.aram),
        "every browser sees the arena"
    );
    pick_and_start(&mut room);
    for _ in 0..(60 * 30) {
        room.step(true);
    }
    for (k, g) in room.games.iter().enumerate() {
        let w = &g.m.as_ref().expect("in the match").world;
        assert!(w.arena && g.on_arena, "browser {k} plays on the arena");
        assert_eq!(g.rules.map.size, 8192, "the arena's map is in place");
        let def = w
            .get(w.players[g.online.my_seat.unwrap()].hero)
            .map(|u| u.def as usize);
        assert_eq!(
            def,
            Some(g.rules.heroes[k]),
            "browser {k} plays the hero it chose"
        );
        assert!(w.tickets[0] > 0 && w.tickets[0] <= 500);
    }
    let (compared, agree) = room.checksums_agree();
    assert!(compared >= 4, "checksums were compared: {compared}");
    assert!(
        agree && !room.games.iter().any(|g| g.online.desync),
        "one match in every browser"
    );
}
