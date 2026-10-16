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
        self.wire.push(Flight {
            at: self.frame + delay,
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
        room.games[1].m.as_ref().unwrap().world.tick > 800,
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
