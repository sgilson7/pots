//! Online play: a room of up to ten browsers, rollback netcode, and
//! rejoining a match in progress.
//!
//! **Shape.** One browser hosts; the others join it by room code (the
//! transport in `web/net/` introduces them over public relays and then
//! carries text). The host's browser keeps the lobby, relays every seat's
//! orders to every other browser, and speaks for seats whose player has
//! dropped. This is the star Raiders of the Lost Mart used for four seats,
//! with Galactic Survivors' rollback session grown to ten.
//!
//! **Rollback.** Every browser runs the whole match. Its own orders are
//! scheduled `DELAY` ticks ahead and sent at once. Another seat's orders for
//! a tick may arrive after this browser has run that tick; until then it
//! guesses "no new order", which is right on most ticks of a game played by
//! clicking. When a late order arrives, the world is restored from the
//! snapshot taken before that tick and the ticks since are run again with
//! the order in place. A browser never runs more than `MAX_AHEAD` ticks past
//! the last tick for which it holds every seat's orders; past that it waits.
//!
//! **Rejoining.** Each seat has a token, kept in the page's address so a
//! reload keeps it. When a player drops, the host hands the seat to a
//! computer player at once, by an `Autopilot` order in that seat's stream,
//! so every browser does the same. When the player returns with the token,
//! the host sends the start and every order so far; the returning browser
//! replays the match at speed, and at a tick the host names it takes the
//! seat back with its own `Autopilot(255)` order.
//!
//! **Checks.** Every `SUM_EVERY` ticks, once a tick is confirmed, each
//! browser sends the checksum of its world at that tick; the host compares
//! and reports `online.desync` when they differ.

use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::game::{Game, Match};
use crate::orders::{Cmd, CMD_VALUES};
use crate::rules::TPS;
use crate::ui::Node;
use crate::world::{Ctx, World, PLAYERS};

/// Changes when a message changes shape; `web/net/config.js` carries it too.
pub const PROTOCOL: u32 = 1;
/// Ticks between an order and the tick it acts on.
pub const DELAY: u32 = 2;
/// Ticks a browser may run past its confirmed tick before it waits.
pub const MAX_AHEAD: u32 = 15;
/// Ticks between checksums.
pub const SUM_EVERY: u32 = 90;
/// Ticks of a seat's recent orders repeated in every message, so a lost
/// message is covered by the next.
pub const RESEND: u32 = 45;
/// Ticks a returning player's seat stays with the computer after the host
/// sends the log, before the player takes it back.
pub const TAKE_OVER_AFTER: u32 = 60;
/// Most ticks run in one display frame while catching up.
pub const CATCH_UP_PER_FRAME: u32 = 240;
/// A seat whose browser has said nothing for this long (microseconds of
/// real time, not ticks: ticks stop while the match waits for it) is away.
pub const SILENT_US: i64 = 2_500_000;

/// One seat of the room.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Seat {
    pub name: String,
    /// A browser sits here.
    pub human: bool,
    /// The seat's hero, or -1.
    pub pick: i32,
    /// The browser dropped; the computer plays the seat until it returns.
    #[serde(default)]
    pub away: bool,
}

/// What goes over the wire, as JSON text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "k")]
pub enum Msg {
    Hello {
        proto: u32,
        name: String,
        token: String,
    },
    /// Host to one joiner: its seat and the token that brings it back.
    Welcome {
        seat: i32,
        token: String,
    },
    Lobby {
        seats: Vec<Seat>,
        bots: u8,
        started: bool,
    },
    Sit {
        seat: usize,
    },
    Pick {
        hero: u8,
    },
    Start {
        seed: u64,
        horn: u32,
        bots: u8,
        seats: Vec<Seat>,
        seat: usize,
        log: Vec<[i32; 7]>,
        upto: u32,
        take_over: u32,
    },
    /// Orders: for each seat, the tick below which its orders are all known,
    /// and its orders for the recent ticks as [tick, five values].
    In {
        seats: Vec<(usize, u32, Vec<[i32; 6]>)>,
    },
    Sum {
        tick: u32,
        sum: String,
    },
    Bye {
        reason: String,
    },
    /// Part of the order log for a returning player, sent before its Start:
    /// a long match's log is larger than one data-channel message may be.
    Log {
        entries: Vec<[i32; 7]>,
    },
}

/// Log entries in one message.
pub const LOG_CHUNK: usize = 1500;

impl Msg {
    pub fn text(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn parse(text: &str) -> Option<Msg> {
        serde_json::from_str(text).ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    #[default]
    None,
    Host,
    Joiner,
}

/// A browser connected to the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    pub id: String,
    pub seat: Option<usize>,
    pub token: String,
    pub name: String,
}

/// The rollback session of a match in progress.
#[derive(Debug, Clone)]
pub struct Session {
    pub me: usize,
    pub humans: [bool; PLAYERS],
    /// Every order of seat s for ticks below `known[s]` is held.
    pub known: [u32; PLAYERS],
    /// Orders by tick, then (seat, values).
    pub inputs: BTreeMap<u32, Vec<(u8, [i32; CMD_VALUES])>>,
    /// The world at the start of recent ticks, oldest first.
    pub snaps: VecDeque<World>,
    pub rollback_to: Option<u32>,
    /// The first tick this browser plays its own seat (after a rejoin).
    pub take_over: u32,
    /// The host speaks for these seats (players away).
    pub away: [bool; PLAYERS],
    /// When each seat was last heard from: the tick, and this browser's
    /// microseconds (host).
    pub heard: [u32; PLAYERS],
    pub heard_us: [i64; PLAYERS],
    /// This browser's clock since the match began, microseconds.
    pub clock_us: i64,
    pub seed: u64,
    pub horn: u32,
    pub bots: u8,
    pub seats: Vec<Seat>,
    /// Checksums this browser computed, by tick.
    pub sums: BTreeMap<u32, u64>,
    /// Checksums received from other browsers, by tick (host).
    pub their_sums: BTreeMap<u32, Vec<(String, u64)>>,
    pub acc_us: i64,
    /// Counters for the runners and the HUD.
    pub rollbacks: u32,
    pub deepest: u32,
    pub stalls: u32,
    pub replayed: u64,
    /// Seats coming back, and the tick each takes its seat again (host).
    pub returning: Vec<(usize, u32)>,
    /// After a rejoin, the host repeats every order since the log it sent,
    /// until this tick, so nothing falls between the log and the live stream.
    pub resend_from: Option<(u32, u32)>,
}

#[derive(Debug, Default)]
pub struct Online {
    pub role: Role,
    /// What the page's transport should be: (role, room) or none.
    pub want: Option<(String, String)>,
    pub room: String,
    pub name: String,
    pub token: String,
    pub seats: Vec<Seat>,
    pub bots: u8,
    pub started: bool,
    pub peers: Vec<Peer>,
    pub host_peer: String,
    pub my_seat: Option<usize>,
    /// A status key for strings.json (online.waiting and the rest).
    pub status: String,
    pub out: Vec<(String, String, bool)>,
    pub s: Option<Session>,
    pub desync: bool,
    pub catching: bool,
    /// A seed from the page for the next match.
    pub seed: u64,
    /// The code typed into the join box.
    pub typed: String,
    /// The host: each seat's token, kept for the whole match so a player
    /// who drops can come back to the same seat.
    pub tokens: Vec<String>,
    /// A returning joiner: log entries received before the Start.
    pub early_log: Vec<[i32; 7]>,
}

fn empty_seats() -> Vec<Seat> {
    (0..PLAYERS)
        .map(|_| Seat {
            name: String::new(),
            human: false,
            pick: -1,
            away: false,
        })
        .collect()
}

/// The room name the transport joins: the protocol and the typed code,
/// never the build, so a deploy does not split rooms (Ribbit King's lesson).
pub fn room_name(code: &str) -> String {
    format!("{PROTOCOL}-{}", code.trim().to_uppercase())
}

/// A room code from a seed: five letters with no look-alikes.
pub fn code_from(seed: u64) -> String {
    const LETTERS: &[u8] = b"ACDEFGHJKMNPQRTUVWXY";
    let mut r = crate::rng::Rng::new(seed);
    (0..5)
        .map(|_| LETTERS[r.below(LETTERS.len() as u32) as usize] as char)
        .collect()
}

fn send(o: &mut Online, to: &str, m: &Msg, fast: bool) {
    o.out.push((to.to_string(), m.text(), fast));
}

fn broadcast_lobby(o: &mut Online) {
    let m = Msg::Lobby {
        seats: o.seats.clone(),
        bots: o.bots,
        started: o.started,
    };
    send(o, "*", &m, false);
}

/// Called by `act`: online.* actions from the lobby screens.
pub fn act(g: &mut Game, name: &str, value: &str) -> Result<(), String> {
    let o = &mut g.online;
    match name {
        "online.name" => o.name = value.chars().filter(|c| !c.is_control()).take(16).collect(),
        "online.code" => {
            o.typed = value
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .take(8)
                .collect()
        }
        "online.host" => {
            o.role = Role::Host;
            o.room = code_from(g.seed ^ 0x5eed);
            o.token = code_from(g.seed ^ 0x70c3);
            o.seats = empty_seats();
            o.seats[0] = Seat {
                name: display_name(&o.name),
                human: true,
                pick: -1,
                away: false,
            };
            o.my_seat = Some(0);
            o.peers.clear();
            o.started = false;
            o.bots = 2;
            o.status = "online.waiting".into();
            o.want = Some(("host".into(), room_name(&o.room)));
            g.screen = "online.lobby".into();
        }
        "online.join" => {
            let code = if value.is_empty() {
                o.typed.clone()
            } else {
                value.to_string()
            };
            if code.trim().len() < 4 {
                return Err(g.strings.ui("online.bad_code"));
            }
            let (code, token) = match code.split_once('.') {
                Some((c, t)) => (c.to_string(), t.to_string()),
                None => (code, code_from(g.seed ^ 0x70c3)),
            };
            o.role = Role::Joiner;
            o.room = code.trim().to_uppercase();
            o.token = token;
            o.seats = empty_seats();
            o.my_seat = None;
            o.started = false;
            o.status = "online.connecting".into();
            o.want = Some(("join".into(), room_name(&o.room)));
            g.screen = "online.lobby".into();
        }
        "online.sit" => {
            let seat: usize = value.parse().map_err(|_| "No such seat.".to_string())?;
            if seat >= PLAYERS {
                return Err("No such seat.".into());
            }
            match o.role {
                Role::Host => {
                    if !o.seats[seat].human {
                        if let Some(old) = o.my_seat {
                            let me = o.seats[old].clone();
                            o.seats[old] = Seat::default();
                            o.seats[old].pick = -1;
                            o.seats[seat] = me;
                            o.my_seat = Some(seat);
                        }
                        broadcast_lobby(o);
                    }
                }
                Role::Joiner => {
                    let host = o.host_peer.clone();
                    send(o, &host, &Msg::Sit { seat }, false);
                }
                Role::None => {}
            }
        }
        "online.bots" => {
            let levels = g.rules.ai.as_ref().map(|a| a.difficulty.len()).unwrap_or(1) as i32;
            if o.role == Role::Host {
                let v = o.bots as i32 + if value == "+" { 1 } else { -1 };
                o.bots = v.clamp(0, levels - 1) as u8;
                broadcast_lobby(o);
            }
        }
        "online.start" => {
            if o.role != Role::Host {
                return Ok(());
            }
            if o.seats.iter().any(|s| s.human && s.pick < 0) {
                return Err(g.strings.ui("online.all_pick"));
            }
            start_as_host(g);
        }
        "online.leave" => leave(g),
        other => return Err(format!("Unknown action \"{other}\".")),
    }
    Ok(())
}

fn display_name(name: &str) -> String {
    if name.trim().is_empty() {
        "Player".into()
    } else {
        name.trim().to_string()
    }
}

/// The hero chosen in the lobby.
pub fn pick(g: &mut Game, hero: u8) {
    let o = &mut g.online;
    match o.role {
        Role::Host => {
            if let Some(me) = o.my_seat {
                if !o
                    .seats
                    .iter()
                    .enumerate()
                    .any(|(k, s)| k != me && s.pick == hero as i32)
                {
                    o.seats[me].pick = hero as i32;
                    broadcast_lobby(o);
                }
            }
        }
        Role::Joiner => {
            let host = o.host_peer.clone();
            send(o, &host, &Msg::Pick { hero }, false);
        }
        Role::None => {}
    }
}

pub fn enter(g: &mut Game, screen: &str) {
    if screen == "online" && g.online.role == Role::None {
        g.online.status = String::new();
    }
}

pub fn leave(g: &mut Game) {
    let o = &mut g.online;
    if o.role != Role::None {
        send(
            o,
            "*",
            &Msg::Bye {
                reason: "online.left".into(),
            },
            false,
        );
    }
    *o = Online {
        name: o.name.clone(),
        out: std::mem::take(&mut o.out),
        ..Online::default()
    };
    o.want = None;
    if g.m.as_ref().map(|m| m.online).unwrap_or(false) {
        g.m = None;
    }
    g.screen = "online".into();
}

/// The transport's events: "peer" when a browser connects, "left" when one goes.
pub fn event(g: &mut Game, kind: &str, peer: &str) {
    let o = &mut g.online;
    match (kind, o.role) {
        ("peer", Role::Joiner) => {
            o.host_peer = peer.to_string();
            o.status = "online.waiting".into();
            let hello = Msg::Hello {
                proto: PROTOCOL,
                name: display_name(&o.name),
                token: o.token.clone(),
            };
            send(o, peer, &hello, false);
        }
        ("peer", Role::Host) => {}
        ("left", Role::Host) => {
            let Some(k) = o.peers.iter().position(|p| p.id == peer) else {
                return;
            };
            let p = o.peers.remove(k);
            if let Some(seat) = p.seat {
                if o.started {
                    // the computer takes the seat until the player returns
                    o.seats[seat].away = true;
                    if let Some(s) = o.s.as_mut() {
                        s.away[seat] = true;
                    }
                    hand_to_bot(g, seat);
                } else {
                    o.seats[seat] = Seat {
                        pick: -1,
                        ..Seat::default()
                    };
                }
                broadcast_lobby(&mut g.online);
            }
        }
        ("left", Role::Joiner) => {
            if peer == o.host_peer {
                o.status = "online.host_left".into();
            }
        }
        // a transport error in a running match (a relay failing, a retry under a
        // working link) does not replace the match's status
        ("error", _) if !o.started => o.status = peer.to_string(),
        _ => {}
    }
}

/// Every message from a peer.
pub fn receive(g: &mut Game, peer: &str, text: &str) {
    let Some(msg) = Msg::parse(text) else {
        g.online.status = "online.garbled".into();
        return;
    };
    match g.online.role {
        Role::Host => host_receives(g, peer, msg),
        Role::Joiner => joiner_receives(g, peer, msg),
        Role::None => {}
    }
}

fn host_receives(g: &mut Game, peer: &str, msg: Msg) {
    match msg {
        Msg::Hello { proto, name, token } => {
            let o = &mut g.online;
            if proto != PROTOCOL {
                send(
                    o,
                    peer,
                    &Msg::Bye {
                        reason: "online.reload".into(),
                    },
                    false,
                );
                return;
            }
            // a returning player: the same token as a seat's
            let back = o.peers.iter().position(|p| p.token == token);
            if let Some(k) = back {
                o.peers[k].id = peer.to_string();
            }
            if o.tokens.len() < PLAYERS {
                o.tokens = vec![String::new(); PLAYERS];
            }
            let seat = (0..PLAYERS).find(|&k| {
                o.seats[k].human
                    && o.seats[k].away
                    && o.peers.iter().all(|p| p.seat != Some(k))
                    && o.tokens[k] == token
            });
            let seat = match seat {
                Some(s) => Some(s),
                None if !o.started => {
                    // the first open seat, alternating sides
                    [5, 1, 6, 2, 7, 3, 8, 4, 9, 0]
                        .into_iter()
                        .find(|&k| !o.seats[k].human)
                }
                None => None,
            };
            let Some(seat) = seat else {
                send(
                    o,
                    peer,
                    &Msg::Bye {
                        reason: "online.full".into(),
                    },
                    false,
                );
                return;
            };
            o.peers.retain(|p| p.id != peer);
            o.tokens[seat] = token.clone();
            o.peers.push(Peer {
                id: peer.to_string(),
                seat: Some(seat),
                token: token.clone(),
                name: name.clone(),
            });
            let returning = o.seats[seat].away;
            if !returning {
                o.seats[seat] = Seat {
                    name: display_name(&name),
                    human: true,
                    pick: -1,
                    away: false,
                };
            }
            send(
                o,
                peer,
                &Msg::Welcome {
                    seat: seat as i32,
                    token,
                },
                false,
            );
            broadcast_lobby(o);
            if returning && o.started {
                welcome_back(g, peer, seat);
            }
        }
        Msg::Sit { seat } => {
            let o = &mut g.online;
            if o.started || seat >= PLAYERS || o.seats[seat].human {
                return;
            }
            let Some(p) = o.peers.iter_mut().find(|p| p.id == peer) else {
                return;
            };
            if let Some(old) = p.seat {
                let mine = o.seats[old].clone();
                o.seats[old] = Seat {
                    pick: -1,
                    ..Seat::default()
                };
                o.seats[seat] = mine;
            }
            p.seat = Some(seat);
            broadcast_lobby(o);
        }
        Msg::Pick { hero } => {
            let o = &mut g.online;
            let Some(seat) = o.peers.iter().find(|p| p.id == peer).and_then(|p| p.seat) else {
                return;
            };
            if o.started
                || o.seats
                    .iter()
                    .enumerate()
                    .any(|(k, s)| k != seat && s.pick == hero as i32)
            {
                return;
            }
            o.seats[seat].pick = hero as i32;
            broadcast_lobby(o);
        }
        Msg::In { seats } => {
            let seat = g.online.peers.iter().find(|p| p.id == peer).and_then(|p| p.seat);
            let Some(seat) = seat else { return };
            // a joiner speaks only for its own seat
            let mine: Vec<_> = seats.into_iter().filter(|(s, _, _)| *s == seat).collect();
            if g.online.s.as_ref().map(|s| s.away[seat]).unwrap_or(true) {
                return;
            }
            take_in(g, mine);
        }
        Msg::Sum { tick, sum } => {
            let Ok(sum) = sum.parse::<u64>() else { return };
            if let Some(s) = g.online.s.as_mut() {
                s.their_sums
                    .entry(tick)
                    .or_default()
                    .push((peer.to_string(), sum));
            }
            compare_sums(g);
        }
        Msg::Bye { .. } => event(g, "left", peer),
        _ => g.online.status = "online.garbled".into(),
    }
}

fn joiner_receives(g: &mut Game, peer: &str, msg: Msg) {
    if peer != g.online.host_peer {
        return;
    }
    match msg {
        Msg::Welcome { seat, token } => {
            let o = &mut g.online;
            o.my_seat = usize::try_from(seat).ok();
            o.token = token;
        }
        Msg::Lobby { seats, bots, started } => {
            let o = &mut g.online;
            o.seats = seats;
            o.bots = bots;
            o.started = started;
        }
        Msg::Start {
            seed,
            horn,
            bots,
            seats,
            seat,
            log,
            upto,
            take_over,
        } => {
            g.online.seats = seats.clone();
            g.online.my_seat = Some(seat);
            g.online.started = true;
            begin(g, seed, horn, bots, seats, seat, take_over);
            let early = std::mem::take(&mut g.online.early_log);
            let s = g.online.s.as_mut().expect("just begun");
            for e in early.into_iter().chain(log) {
                let (tick, seat) = (e[0] as u32, e[1] as u8);
                let v = [e[2], e[3], e[4], e[5], e[6]];
                s.inputs.entry(tick).or_default().push((seat, v));
            }
            for k in 0..PLAYERS {
                s.known[k] = s.known[k].max(upto);
            }
            g.online.catching = upto > 0;
            g.online.status = if upto > 0 {
                "online.catching".into()
            } else {
                "online.playing".into()
            };
        }
        Msg::In { seats } => take_in(g, seats),
        Msg::Log { entries } => g.online.early_log.extend(entries),
        Msg::Bye { reason } => {
            g.online.status = reason;
            g.online.want = None;
        }
        _ => g.online.status = "online.garbled".into(),
    }
}

/// Orders arrive: store what is new, and mark a rollback if a tick already
/// run has changed.
fn take_in(g: &mut Game, seats: Vec<(usize, u32, Vec<[i32; 6]>)>) {
    let Some(m) = g.m.as_ref() else { return };
    let now = m.world.tick;
    let Some(s) = g.online.s.as_mut() else { return };
    for (seat, upto, cmds) in seats {
        if seat >= PLAYERS || seat == s.me && now >= s.take_over {
            continue;
        }
        let start = s.known[seat];
        if upto <= start {
            continue;
        }
        for c in cmds {
            let tick = c[0] as u32;
            if tick < start || tick >= upto {
                continue;
            }
            let v = [c[1], c[2], c[3], c[4], c[5]];
            s.inputs.entry(tick).or_default().push((seat as u8, v));
            if tick < now {
                s.rollback_to = Some(s.rollback_to.map(|r| r.min(tick)).unwrap_or(tick));
            }
        }
        s.known[seat] = upto;
        s.heard[seat] = now;
        s.heard_us[seat] = s.clock_us;
    }
}

/// Builds the world every browser starts from: the seed, the seats, the picks.
fn begin(g: &mut Game, seed: u64, horn: u32, bots: u8, seats: Vec<Seat>, me: usize, take_over: u32) {
    let mut w = g.new_world(seed);
    w.horn_at = horn;
    let ctx = Ctx {
        rules: &g.rules,
        terrain: &g.terrain,
    };
    let mut humans = [false; PLAYERS];
    for (k, seat) in seats.iter().enumerate() {
        w.players[k].present = true;
        if seat.human {
            humans[k] = true;
            if seat.pick >= 0 {
                crate::orders::apply(&mut w, &ctx, k, Cmd::Pick(seat.pick as u8));
            }
        } else {
            w.players[k].bot = Some(bots);
        }
    }
    g.begin(w, me, None, true);
    g.online.started = true;
    g.online.s = Some(Session {
        me,
        humans,
        known: [0; PLAYERS],
        inputs: BTreeMap::new(),
        snaps: VecDeque::new(),
        rollback_to: None,
        take_over,
        away: [false; PLAYERS],
        heard: [0; PLAYERS],
        heard_us: [0; PLAYERS],
        clock_us: 0,
        seed,
        horn,
        bots,
        seats,
        sums: BTreeMap::new(),
        their_sums: BTreeMap::new(),
        acc_us: 0,
        rollbacks: 0,
        deepest: 0,
        stalls: 0,
        replayed: 0,
        returning: Vec::new(),
        resend_from: None,
    });
}

fn start_as_host(g: &mut Game) {
    let seed = g.seed ^ 0x9e37_79b9;
    let horn = (g.rules.t.horn * TPS) as u32;
    let o = &mut g.online;
    o.started = true;
    let seats = o.seats.clone();
    let bots = o.bots;
    let peers = o.peers.clone();
    for p in &peers {
        if let Some(seat) = p.seat {
            let m = Msg::Start {
                seed,
                horn,
                bots,
                seats: seats.clone(),
                seat,
                log: Vec::new(),
                upto: 0,
                take_over: 0,
            };
            send(o, &p.id, &m, false);
        }
    }
    broadcast_lobby(o);
    let me = o.my_seat.unwrap_or(0);
    begin(g, seed, horn, bots, seats, me, 0);
    g.online.status = "online.playing".into();
}

/// The host hands a dropped seat to the computer, by an order in the
/// seat's own stream, so every browser does the same.
fn hand_to_bot(g: &mut Game, seat: usize) {
    let now = g.m.as_ref().map(|m| m.world.tick).unwrap_or(0);
    let bots = g.online.bots;
    let Some(s) = g.online.s.as_mut() else { return };
    let at = s.known[seat].max(now + DELAY);
    s.inputs
        .entry(at)
        .or_default()
        .push((seat as u8, Cmd::Autopilot(bots).encode()));
    if at < now {
        s.rollback_to = Some(s.rollback_to.map(|r| r.min(at)).unwrap_or(at));
    }
    s.known[seat] = at + 1;
}

/// A returning player: the start, and every order so far.
fn welcome_back(g: &mut Game, peer: &str, seat: usize) {
    let now = g.m.as_ref().map(|m| m.world.tick).unwrap_or(0);
    let o = &mut g.online;
    let Some(s) = o.s.as_mut() else { return };
    let upto = confirmed(s);
    let take_over = now + TAKE_OVER_AFTER;
    let mut log = Vec::new();
    for (&tick, list) in s.inputs.range(..upto) {
        for (st, v) in list {
            log.push([tick as i32, *st as i32, v[0], v[1], v[2], v[3], v[4]]);
        }
    }
    s.away[seat] = false;
    s.take_over_for(seat, take_over);
    s.resend_from = Some((upto, now + 10 * TPS as u32));
    let m = Msg::Start {
        seed: s.seed,
        horn: s.horn,
        bots: s.bots,
        seats: s.seats.clone(),
        seat,
        log: Vec::new(),
        upto,
        take_over,
    };
    o.seats[seat].away = false;
    for part in log.chunks(LOG_CHUNK) {
        send(
            o,
            peer,
            &Msg::Log {
                entries: part.to_vec(),
            },
            false,
        );
    }
    send(o, peer, &m, false);
}

impl Session {
    /// The host keeps speaking for a returning seat until it takes over.
    fn take_over_for(&mut self, seat: usize, at: u32) {
        self.heard[seat] = at;
        self.heard_us[seat] = self.clock_us + SILENT_US;
        self.returning.push((seat, at));
    }
}

/// One display frame of an online match: catch up, roll back if a late
/// order arrived, run the ticks the clock allows within the limit, send.
pub fn frame(g: &mut Game, dt: i64) {
    let tick_us = 1_000_000 / TPS as i64;
    let Some(s) = g.online.s.as_mut() else { return };
    s.acc_us += dt;
    s.clock_us += dt;
    let rollback = s.rollback_to.take();
    if let Some(r) = rollback {
        roll_back(g, r);
    }
    let catching = g.online.catching;
    let mut ran = 0u32;
    loop {
        let (now, confirmed, ahead_of_me) = {
            let m = g.m.as_ref().expect("online match");
            let s = g.online.s.as_ref().expect("session");
            let c = confirmed(s);
            // the furthest any other seat's browser has reported
            let lead = (0..PLAYERS)
                .filter(|&k| s.humans[k] && k != s.me && !s.away[k])
                .map(|k| s.known[k])
                .max()
                .unwrap_or(0);
            // a browser that is behind by more than the usual lead catches up;
            // one tick of lead is ordinary and must not set the browsers
            // chasing each other faster than the clock
            (m.world.tick, c, lead.saturating_sub(m.world.tick + DELAY + 3))
        };
        let limit = if catching {
            CATCH_UP_PER_FRAME
        } else {
            4 + ahead_of_me.min(20)
        };
        if ran >= limit {
            break;
        }
        if now >= confirmed + MAX_AHEAD {
            g.online.s.as_mut().expect("session").stalls += 1;
            break;
        }
        if catching {
            // replay the log at speed while orders for the ticks are held
            if now >= confirmed {
                g.online.catching = false;
                g.online.status = "online.playing".into();
                g.online.s.as_mut().expect("session").acc_us = 0;
                break;
            }
        } else {
            let s = g.online.s.as_mut().expect("session");
            if s.acc_us < tick_us && ahead_of_me == 0 {
                break;
            }
            s.acc_us = (s.acc_us - tick_us).max(-tick_us);
        }
        forward(g);
        ran += 1;
    }
    send_orders(g);
    sums(g);
    if g.online.role == Role::Host {
        speak_for_away(g);
    }
}

/// The tick below which every seat's orders are held.
pub fn confirmed(s: &Session) -> u32 {
    (0..PLAYERS)
        .filter(|&k| s.humans[k])
        .map(|k| s.known[k])
        .min()
        .unwrap_or(u32::MAX)
}

/// Runs the next tick for the first time: schedules this browser's orders,
/// snapshots, steps.
fn forward(g: &mut Game) {
    g.remember();
    let m = g.m.as_mut().expect("online match");
    let s = g.online.s.as_mut().expect("session");
    let t = m.world.tick;
    let me = s.me;
    // this browser's own orders, DELAY ticks ahead, once it holds its seat
    let at = t + DELAY;
    if at >= s.take_over {
        let mut mine: Vec<(u8, [i32; CMD_VALUES])> =
            m.pending.drain(..).map(|c| (me as u8, c.encode())).collect();
        if at == s.take_over && s.take_over > 0 {
            mine.insert(0, (me as u8, Cmd::Autopilot(255).encode()));
        }
        if !mine.is_empty() {
            s.inputs.entry(at).or_default().extend(mine);
        }
        s.known[me] = s.known[me].max(at + 1);
    } else {
        m.pending.clear();
    }
    let keep = (MAX_AHEAD + DELAY + SUM_EVERY) as usize;
    s.snaps.push_back(m.world.clone());
    while s.snaps.len() > keep {
        s.snaps.pop_front();
    }
    step_with_inputs(m, s, &g.rules, &g.terrain);
    let ctx = Ctx {
        rules: &g.rules,
        terrain: &g.terrain,
    };
    let m = g.m.as_mut().expect("online match");
    Game::after_tick(m, &ctx, &g.strings, &g.arcade);
}

fn step_with_inputs(m: &mut Match, s: &Session, rules: &crate::rules::Rules, terrain: &crate::map::Terrain) {
    let t = m.world.tick;
    let orders: Vec<(u8, Cmd)> = s
        .inputs
        .get(&t)
        .map(|l| {
            l.iter()
                .filter_map(|(seat, v)| Cmd::decode(*v).map(|c| (*seat, c)))
                .collect()
        })
        .unwrap_or_default();
    let ctx = Ctx { rules, terrain };
    m.world.step(&ctx, &orders);
}

/// Restores the snapshot taken before tick `r` and runs again to the tick
/// this browser had reached.
fn roll_back(g: &mut Game, r: u32) {
    let m = g.m.as_mut().expect("online match");
    let s = g.online.s.as_mut().expect("session");
    let reached = m.world.tick;
    if r >= reached {
        return;
    }
    let Some(k) = s.snaps.iter().position(|w| w.tick == r) else {
        // older than any snapshot: cannot happen inside MAX_AHEAD; report it
        g.online.status = "online.desync".into();
        g.online.desync = true;
        return;
    };
    s.snaps.truncate(k + 1);
    m.world = s.snaps.pop_back().expect("found");
    s.rollbacks += 1;
    s.deepest = s.deepest.max(reached - r);
    while m.world.tick < reached {
        s.snaps.push_back(m.world.clone());
        step_with_inputs(m, s, &g.rules, &g.terrain);
        s.replayed += 1;
    }
}

/// Sends this browser's recent orders (the host: every seat's).
fn send_orders(g: &mut Game) {
    let o = &mut g.online;
    let Some(s) = o.s.as_ref() else { return };
    let seats: Vec<usize> = if o.role == Role::Host {
        (0..PLAYERS).filter(|&k| s.humans[k]).collect()
    } else {
        vec![s.me]
    };
    let mut list = Vec::new();
    for k in seats {
        let upto = s.known[k];
        let mut from = upto.saturating_sub(RESEND);
        if let Some((since, until)) = s.resend_from {
            if s.known[s.me] < until {
                from = from.min(since);
            }
        }
        let mut cmds = Vec::new();
        for (&tick, l) in s.inputs.range(from..upto) {
            for (seat, v) in l {
                if *seat as usize == k {
                    cmds.push([tick as i32, v[0], v[1], v[2], v[3], v[4]]);
                }
            }
        }
        list.push((k, upto, cmds));
    }
    let m = Msg::In { seats: list };
    let to = if o.role == Role::Host {
        "*".to_string()
    } else {
        o.host_peer.clone()
    };
    send(o, &to, &m, true);
}

/// The host speaks for seats whose player is away, so the match never
/// waits on a browser that is gone, and notices a browser that has gone
/// silent without the transport saying so.
fn speak_for_away(g: &mut Game) {
    let now = g.m.as_ref().map(|m| m.world.tick).unwrap_or(0);
    let mut newly_silent = Vec::new();
    if let Some(s) = g.online.s.as_mut() {
        for k in 0..PLAYERS {
            if !s.humans[k] || k == s.me {
                continue;
            }
            let returning = s.returning.iter().find(|(seat, _)| *seat == k).map(|(_, at)| *at);
            let speak = s.away[k] || returning.map(|at| now + DELAY < at).unwrap_or(false);
            if speak {
                s.known[k] = s.known[k].max(now + DELAY + 1);
            } else if s.clock_us > s.heard_us[k] + SILENT_US && s.clock_us > SILENT_US {
                newly_silent.push(k);
            }
        }
        s.returning.retain(|(_, at)| now + DELAY < *at);
    }
    for k in newly_silent {
        let peer = g
            .online
            .peers
            .iter()
            .find(|p| p.seat == Some(k))
            .map(|p| p.id.clone());
        match peer {
            Some(id) => event(g, "left", &id),
            None => {
                g.online.seats[k].away = true;
                if let Some(s) = g.online.s.as_mut() {
                    s.away[k] = true;
                }
                hand_to_bot(g, k);
            }
        }
    }
}

/// Checksums of confirmed ticks: sent to the host, compared there.
fn sums(g: &mut Game) {
    let o = &mut g.online;
    let Some(s) = o.s.as_mut() else { return };
    let c = confirmed(s);
    let mut fresh = Vec::new();
    for w in s.snaps.iter() {
        if w.tick < c && w.tick > 0 && w.tick % SUM_EVERY == 0 && !s.sums.contains_key(&w.tick) {
            fresh.push((w.tick, w.checksum()));
        }
    }
    for (t, sum) in &fresh {
        s.sums.insert(*t, *sum);
    }
    if o.role == Role::Joiner {
        let host = o.host_peer.clone();
        for (tick, sum) in fresh {
            send(
                o,
                &host,
                &Msg::Sum {
                    tick,
                    sum: sum.to_string(),
                },
                false,
            );
        }
    } else {
        compare_sums(g);
    }
}

fn compare_sums(g: &mut Game) {
    let o = &mut g.online;
    let Some(s) = o.s.as_mut() else { return };
    let mut bad = false;
    for (tick, theirs) in &s.their_sums {
        if let Some(mine) = s.sums.get(tick) {
            if theirs.iter().any(|(_, x)| x != mine) {
                bad = true;
            }
        }
    }
    if bad {
        o.desync = true;
        o.status = "online.desync".into();
    }
}

/// What the page's transport should do, and the messages to send.
pub fn take_out(g: &mut Game) -> String {
    let o = &mut g.online;
    let out: Vec<(String, String, bool)> = std::mem::take(&mut o.out);
    serde_json::json!({
        "want": o.want.as_ref().map(|(role, room)| serde_json::json!({"role": role, "room": room})),
        "out": out,
        "hash": if o.started && !o.room.is_empty() { format!("room={}.{}", o.room, o.token) } else { String::new() },
    })
    .to_string()
}

pub fn probe(g: &Game) -> serde_json::Value {
    let o = &g.online;
    let s = o.s.as_ref();
    serde_json::json!({
        "role": match o.role { Role::Host => "host", Role::Joiner => "join", Role::None => "" },
        "room": o.room,
        "seat": o.my_seat,
        "status": o.status,
        "peers": o.peers.len(),
        "started": o.started,
        "confirmed": s.map(confirmed).unwrap_or(0),
        "rollbacks": s.map(|s| s.rollbacks).unwrap_or(0),
        "deepest": s.map(|s| s.deepest).unwrap_or(0),
        "stalls": s.map(|s| s.stalls).unwrap_or(0),
        "replayed": s.map(|s| s.replayed).unwrap_or(0),
        "sums": s.map(|s| s.sums.len()).unwrap_or(0),
        "desync": o.desync,
        "catching": o.catching,
        "away": s.map(|s| s.away.iter().filter(|a| **a).count()).unwrap_or(0),
        "seats": o.seats.iter().map(|x| format!("{}{}", if x.human { "h" } else { "-" }, if x.away { "a" } else { "" })).collect::<Vec<_>>(),
        // the last few checksums, so runners can be compared tick for tick
        "sumlog": s.map(|s| s.sums.iter().rev().take(6).map(|(t, v)| (*t, v.to_string())).collect::<Vec<_>>()).unwrap_or_default(),
    })
}

/// The online screens: the name and code form, and the lobby.
pub fn view(g: &Game) -> Node {
    crate::ui::online_view(g)
}

#[cfg(test)]
mod tests;
