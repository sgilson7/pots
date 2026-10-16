//! The view model: every screen and the in-match HUD as a tree of nodes the
//! page builds without knowing what any of them means.
//!
//! A node names a tag, a class, its text, an icon, a hover tip, and an
//! action (`name`, `value`) the page sends back through `act`. Numbers a
//! rule decides are written into the text here; the page adds none.

use serde::Serialize;

use crate::game::{Aim, Game, Outcome};
use crate::rules::{Kind, Target, TPS};
use crate::stats;
use crate::text::{self, clock, hundredths};
use crate::world::{Uid, World, M, NO_UNIT};

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct Tip {
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sub: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct Node {
    /// A stable key, so the page replaces only what changed.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub k: String,
    pub t: &'static str,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub c: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub x: String,
    /// An image's address.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub i: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub a: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub v: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<Tip>,
    /// CSS custom properties: name and value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub s: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub n: Vec<Node>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub off: bool,
    /// A glossary term the page explains on hover.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub term: String,
}

fn node(t: &'static str, c: &str) -> Node {
    Node {
        t,
        c: c.into(),
        ..Node::default()
    }
}

fn text(t: &'static str, c: &str, x: impl Into<String>) -> Node {
    Node {
        t,
        c: c.into(),
        x: x.into(),
        ..Node::default()
    }
}

fn button(c: &str, x: impl Into<String>, a: &str, v: impl Into<String>) -> Node {
    Node {
        t: "button",
        c: c.into(),
        x: x.into(),
        a: a.into(),
        v: v.into(),
        ..Node::default()
    }
}

fn keyed(mut n: Node, k: impl Into<String>) -> Node {
    n.k = k.into();
    n
}

fn with(mut n: Node, kids: Vec<Node>) -> Node {
    n.n = kids;
    n
}

fn pct(part: i64, whole: i64) -> String {
    if whole <= 0 {
        return "0%".into();
    }
    format!("{}%", (part * 100 / whole).clamp(0, 100))
}

pub fn art(path: &str) -> String {
    if path.is_empty() {
        String::new()
    } else {
        format!("art/{path}.png")
    }
}

/// The whole view: title, screen and its nodes.
#[derive(Debug, Clone, Serialize)]
pub struct View {
    pub screen: String,
    pub title: String,
    pub root: Node,
    /// The cursor the 3D view should show: "", "aim" or "attack".
    pub cursor: &'static str,
    pub notice: String,
}

pub fn view(g: &Game) -> View {
    let root = match g.screen.as_str() {
        "title" => title(g),
        "arcade" => arcade(g),
        "pick" => pick(g, true),
        "heroes" => pick(g, false),
        "help" => help(g),
        "match" => hud(g),
        s if s.starts_with("online") => crate::online::view(g),
        _ => title(g),
    };
    View {
        screen: g.screen.clone(),
        title: g.strings.ui("title"),
        root,
        cursor: g.cursor(),
        notice: g.notice.clone(),
    }
}

fn title(g: &Game) -> Node {
    let s = &g.strings;
    with(
        node("div", "menu title-screen"),
        vec![
            text("h1", "game-title", s.ui("title")),
            text("p", "tagline", s.ui("title.tagline")),
            with(
                node("div", "menu-buttons"),
                vec![
                    button("primary", s.ui("title.arcade"), "screen", "arcade"),
                    button("primary", s.ui("title.online"), "screen", "online"),
                    button("", s.ui("title.heroes"), "screen", "heroes"),
                    button("", s.ui("title.help"), "screen", "help"),
                ],
            ),
            text("p", "fine", s.ui("title.fine")),
        ],
    )
}

fn difficulty_name(g: &Game, d: u8) -> String {
    let ai = g.rules.ai.as_ref().expect("loaded");
    let id = &ai.difficulty[(d as usize).min(ai.difficulty.len() - 1)].id;
    g.strings
        .difficulty
        .get(id)
        .cloned()
        .unwrap_or_else(|| id.clone())
}

fn arcade(g: &Game) -> Node {
    let s = &g.strings;
    let mut missions = Vec::new();
    for (k, m) in g.arcade.missions.iter().enumerate() {
        let named = s.missions.get(&m.id).cloned().unwrap_or_default();
        let goal = text::fill(&named.title, 0, &|key| match key {
            "target" => Some(vec![m.target]),
            "minutes" => Some(vec![m.minutes]),
            _ => None,
        });
        let line = s
            .ui("arcade.sides")
            .replace("{allies}", &m.allies.to_string())
            .replace("{enemies}", &m.enemies.to_string())
            .replace("{ally}", &difficulty_name(g, m.ally))
            .replace("{enemy}", &difficulty_name(g, m.enemy));
        let chosen = g.setup.mission == Some(k);
        let mut card = with(
            node("button", if chosen { "mission chosen" } else { "mission" }),
            vec![
                text("span", "mission-no", (k + 1).to_string()),
                text("strong", "", named.name.clone()),
                text("span", "goal", goal),
                text("span", "sides", line),
            ],
        );
        card.a = "mission".into();
        card.v = k.to_string();
        card.tip = Some(Tip {
            title: named.name,
            text: named.text,
            ..Tip::default()
        });
        missions.push(keyed(card, format!("m{k}")));
    }
    let st = g.setup;
    let stepper = |label: String, value: String, act: &str| {
        with(
            node("div", "stepper"),
            vec![
                text("span", "label", label),
                button("step", "−", act, "-"),
                text("span", "value", value),
                button("step", "+", act, "+"),
            ],
        )
    };
    let custom = with(
        node("section", "panel custom"),
        vec![
            text("h2", "", s.ui("arcade.custom")),
            text("p", "fine", s.ui("arcade.custom.text")),
            stepper(s.ui("arcade.allies"), st.allies.to_string(), "setup.allies"),
            stepper(s.ui("arcade.enemies"), st.enemies.to_string(), "setup.enemies"),
            stepper(
                s.ui("arcade.ally_skill"),
                difficulty_name(g, st.ally),
                "setup.ally",
            ),
            stepper(
                s.ui("arcade.enemy_skill"),
                difficulty_name(g, st.enemy),
                "setup.enemy",
            ),
            button("primary", s.ui("arcade.custom.start"), "custom", ""),
        ],
    );
    with(
        node("div", "menu arcade"),
        vec![
            with(
                node("header", "menu-head"),
                vec![
                    text("h1", "", s.ui("arcade.title")),
                    button("back", s.ui("back"), "screen", "title"),
                ],
            ),
            with(
                node("div", "arcade-body"),
                vec![
                    with(node("section", "panel missions"), {
                        let mut v = vec![text("h2", "", s.ui("arcade.ladder"))];
                        v.extend(missions);
                        v
                    }),
                    custom,
                ],
            ),
        ],
    )
}

/// A hero's tip: its abilities, each with its numbers.
fn hero_tip(g: &Game, roster: usize) -> Tip {
    let def = &g.rules.units[g.rules.heroes[roster]];
    let named = g.strings.hero(&def.id);
    let mut lines = Vec::new();
    for &k in &g.rules.unit_abilities[g.rules.heroes[roster]] {
        let a = &g.rules.abilities[k];
        lines.push(g.strings.ability(&a.id).name);
    }
    let attrs = def.attrs.as_ref();
    let sub = match attrs.map(|a| a.primary) {
        Some(crate::rules::Attr::Str) => g.strings.ui("attr.str"),
        Some(crate::rules::Attr::Agi) => g.strings.ui("attr.agi"),
        _ => g.strings.ui("attr.int"),
    };
    Tip {
        title: named.name,
        sub,
        text: named.text,
        lines,
        icon: art(&format!("heroes/{}", def.id)),
    }
}

/// An ability's tip, as the HUD shows it (`level` 0 marks no level).
pub fn ability_tip(g: &Game, k: usize, level: i32) -> Tip {
    let a = &g.rules.abilities[k];
    let named = g.strings.ability(&a.id);
    let look = |key: &str| -> Option<Vec<i32>> {
        match key {
            "mana" => Some(a.mana.clone()),
            "cooldown" => Some(a.cooldown.clone()),
            "range" => Some(a.range.clone()),
            _ => a.values.get(key).cloned(),
        }
    };
    let mut lines = Vec::new();
    if !a.mana.is_empty() {
        lines.push(text::fill(&g.strings.ui("tip.mana"), level, &look));
    }
    if !a.cooldown.is_empty() {
        lines.push(text::fill(&g.strings.ui("tip.cooldown"), level, &look));
    }
    let sub = match a.target {
        Target::Passive => g.strings.ui("tip.passive"),
        Target::Toggle => g.strings.ui("tip.toggle"),
        _ if a.ultimate => g.strings.ui("tip.ultimate"),
        _ => String::new(),
    };
    Tip {
        title: named.name,
        sub,
        text: text::fill(&named.text, level, &look),
        lines,
        icon: art(&a.icon),
    }
}

fn item_tip(g: &Game, k: usize, price: Option<i32>) -> Tip {
    let it = &g.rules.items[k];
    let named = g.strings.item(&it.id);
    let look = |key: &str| -> Option<Vec<i32>> {
        match key {
            "cooldown" => Some(vec![it.cooldown]),
            "mana" => Some(vec![it.mana]),
            "range" => Some(vec![it.range]),
            "charges" => Some(vec![it.charges]),
            _ => it.values.get(key).map(|v| vec![*v]),
        }
    };
    let mut lines = Vec::new();
    let b = &it.bonus;
    let mut bonus = |v: i32, key: &str| {
        if v != 0 {
            lines.push(g.strings.ui(key).replace("{v}", &v.to_string()));
        }
    };
    bonus(b.str, "bonus.str");
    bonus(b.agi, "bonus.agi");
    bonus(b.int, "bonus.int");
    bonus(b.damage, "bonus.damage");
    bonus(b.attack_speed, "bonus.attack_speed");
    bonus(b.move_speed, "bonus.move_speed");
    bonus(b.move_pct, "bonus.move_pct");
    bonus(b.hp, "bonus.hp");
    bonus(b.mana, "bonus.mana");
    bonus(b.evasion, "bonus.evasion");
    bonus(b.lifesteal, "bonus.lifesteal");
    bonus(b.mana_regen_pct, "bonus.mana_regen_pct");
    if b.armor != 0 {
        lines.push(g.strings.ui("bonus.armor").replace("{v}", &hundredths(b.armor)));
    }
    if b.hp_regen != 0 {
        lines.push(
            g.strings
                .ui("bonus.hp_regen")
                .replace("{v}", &hundredths(b.hp_regen)),
        );
    }
    if !it.parts.is_empty() {
        let names: Vec<String> = it.parts.iter().map(|p| g.strings.item(p).name).collect();
        let mut recipe = names.join(" + ");
        if it.recipe {
            recipe.push_str(&g.strings.ui("tip.recipe").replace("{v}", &it.cost.to_string()));
        }
        lines.push(g.strings.ui("tip.built_from").replace("{v}", &recipe));
    }
    let sub = match price {
        Some(p) => g.strings.ui("tip.price").replace("{v}", &p.to_string()),
        None => String::new(),
    };
    Tip {
        title: named.name,
        sub,
        text: text::fill(&named.text, 0, &look),
        lines,
        icon: art(&it.icon),
    }
}

fn pick(g: &Game, choosing: bool) -> Node {
    let s = &g.strings;
    let mut cards = Vec::new();
    for (k, &u) in g.rules.heroes.iter().enumerate() {
        let def = &g.rules.units[u];
        let named = s.hero(&def.id);
        let chosen = choosing && g.setup.hero == k as i32;
        let mut card = with(
            node("button", if chosen { "hero-card chosen" } else { "hero-card" }),
            vec![
                Node {
                    t: "img",
                    c: "portrait".into(),
                    i: art(&format!("heroes/{}", def.id)),
                    ..Node::default()
                },
                text("strong", "", named.name.clone()),
                text("span", "fine", named.title.clone()),
            ],
        );
        card.tip = Some(hero_tip(g, k));
        if choosing {
            card.a = "pick".into();
            card.v = k.to_string();
        }
        cards.push(keyed(card, format!("h{k}")));
    }
    let detail = if g.setup.hero >= 0 || !choosing {
        let k = g.setup.hero.max(0) as usize;
        let u = g.rules.heroes[k];
        let mut abil = Vec::new();
        for (slot, &a) in g.rules.unit_abilities[u].iter().enumerate() {
            let tip = ability_tip(g, a, 0);
            let mut b = with(
                node("div", "ability-row"),
                vec![
                    Node {
                        t: "img",
                        c: "icon".into(),
                        i: tip.icon.clone(),
                        ..Node::default()
                    },
                    text("strong", "", tip.title.clone()),
                ],
            );
            b.tip = Some(tip);
            abil.push(keyed(b, format!("a{slot}")));
        }
        let named = s.hero(&g.rules.units[u].id);
        with(node("section", "panel hero-detail"), {
            let mut v = vec![text("h2", "", named.name), text("p", "", named.text)];
            v.extend(abil);
            v
        })
    } else {
        text("section", "panel hero-detail fine", s.ui("pick.hint"))
    };
    let mut head = vec![
        text(
            "h1",
            "",
            s.ui(if choosing { "pick.title" } else { "heroes.title" }),
        ),
        button(
            "back",
            s.ui("back"),
            "screen",
            if choosing { "arcade" } else { "title" },
        ),
    ];
    if choosing {
        let mut go = button("primary", s.ui("pick.start"), "start", "");
        go.off = g.setup.hero < 0;
        head.push(go);
    }
    with(
        node("div", "menu pick"),
        vec![
            with(node("header", "menu-head"), head),
            with(
                node("div", "pick-body"),
                vec![with(node("div", "hero-grid"), cards), detail],
            ),
        ],
    )
}

fn help(g: &Game) -> Node {
    let s = &g.strings;
    let mut rows = Vec::new();
    for k in 1..=12 {
        let key = format!("help.{k}");
        if s.ui.contains_key(&key) {
            rows.push(text("li", "", s.ui(&key)));
        }
    }
    with(
        node("div", "menu help"),
        vec![
            with(
                node("header", "menu-head"),
                vec![
                    text("h1", "", s.ui("help.title")),
                    button("back", s.ui("back"), "screen", "title"),
                ],
            ),
            with(
                node("section", "panel"),
                vec![with(node("ol", "help-list"), rows)],
            ),
        ],
    )
}

fn hero_name(g: &Game, w: &World, p: usize) -> String {
    let pick = w.players[p].pick;
    if pick < 0 {
        return String::new();
    }
    g.strings
        .hero(&g.rules.units[g.rules.heroes[pick as usize]].id)
        .name
}

/// The in-match HUD.
fn hud(g: &Game) -> Node {
    let Some(m) = g.m.as_ref() else { return title(g) };
    let w = &m.world;
    let s = &g.strings;
    let ctx = g.ctx();
    let me = m.me;
    let p = &w.players[me];
    let hero = p.hero;
    let mut kids = Vec::new();
    // top bar: clock, day or night, the kills of each side, my score
    let since = w.tick as i32 - w.horn_at as i32;
    let clock_text = if since < 0 {
        format!("-{}", clock(-since / TPS))
    } else {
        clock(since / TPS)
    };
    let kills = |team: u8| -> i32 { w.players.iter().filter(|x| x.team == team).map(|x| x.kills).sum() };
    let day = w.is_day(&ctx);
    kids.push(keyed(
        with(
            node("div", "topbar"),
            vec![
                text("span", "score sun", kills(0).to_string()),
                text("span", "clock", clock_text),
                text(
                    "span",
                    if day { "daynight day" } else { "daynight night" },
                    s.ui(if day { "hud.day" } else { "hud.night" }),
                ),
                text("span", "score moon", kills(1).to_string()),
                text(
                    "span",
                    "kda",
                    s.ui("hud.kda")
                        .replace("{k}", &p.kills.to_string())
                        .replace("{d}", &p.deaths.to_string())
                        .replace("{a}", &p.assists.to_string())
                        .replace("{lh}", &p.last_hits.to_string())
                        .replace("{dn}", &p.denies.to_string()),
                ),
            ],
        ),
        "top",
    ));
    // the feed
    let feed: Vec<Node> = m
        .feed
        .iter()
        .map(|(_, line)| text("li", "", line.clone()))
        .collect();
    kids.push(keyed(with(node("ul", "feed"), feed), "feed"));
    if let Some(ms) = m.mission {
        let mi = &g.arcade.missions[ms];
        let named = s.missions.get(&mi.id).cloned().unwrap_or_default();
        let goal = text::fill(&named.title, 0, &|key| match key {
            "target" => Some(vec![mi.target]),
            "minutes" => Some(vec![mi.minutes]),
            _ => None,
        });
        let progress = match mi.goal.as_str() {
            "last_hits" => p.last_hits,
            "tower" => m.towers_taken,
            "kills" => p.kills,
            "boss" => m.bosses,
            _ => 0,
        };
        let line = if mi.goal == "sphere" {
            goal
        } else {
            format!("{goal} · {progress}/{}", mi.target)
        };
        kids.push(keyed(text("div", "goal-line", line), "goal"));
    }
    if m.online && !g.online.status.is_empty() && g.online.status != "online.playing" {
        kids.push(keyed(text("div", "net-line", s.ui(&g.online.status)), "net"));
    }
    if p.bot.is_some() && hero != NO_UNIT {
        kids.push(keyed(text("div", "net-line", s.ui("hud.autopilot")), "autopilot"));
    }
    if let Some(aim) = g.aim {
        let key = match aim {
            Aim::AttackMove => "hud.aim_attack",
            _ => "hud.aim",
        };
        kids.push(keyed(text("div", "aim-hint", s.ui(key)), "aim"));
    }
    if g.learn {
        kids.push(keyed(text("div", "aim-hint", s.ui("hud.learn")), "aim"));
    }
    // gold earned, floating over the spot where it was earned
    let cam = g.camera();
    let aspect = g.aspect();
    let mut pops = Vec::new();
    for (k, (tick, at, amount)) in m.popups.iter().enumerate() {
        let h = crate::scene::ground_at(&g.terrain, *at) + crate::fx::ONE * 4;
        if let Some((x, y)) = cam.screen(aspect, crate::scene::to_scene(*at, h)) {
            let mut n = text("span", "gold-pop", format!("+{amount}"));
            n.s.push(("--x".into(), format!("{}%", x / crate::fx::ONE)));
            n.s.push(("--y".into(), format!("{}%", y / crate::fx::ONE)));
            pops.push(keyed(n, format!("g{tick}-{k}")));
        }
    }
    kids.push(keyed(with(node("div", "pops"), pops), "pops"));
    kids.push(keyed(bottom(g, w, me, hero), "bottom"));
    if g.shop {
        kids.push(keyed(shop(g, w, me), "shop"));
    }
    if g.score {
        kids.push(keyed(scoreboard(g, w), "scoreboard"));
    }
    if let Some(u) = w.get(hero) {
        if !u.alive {
            let left = u
                .hero
                .as_ref()
                .map(|h| h.respawn_at.saturating_sub(w.tick) as i32 / TPS)
                .unwrap_or(0);
            let cost = crate::lanes::buyback_cost(w, &ctx, me);
            let mut bb = button(
                "primary",
                s.ui("hud.buyback").replace("{v}", &cost.to_string()),
                "buyback",
                "",
            );
            bb.off = p.gold < cost || p.buyback_ready > w.tick;
            kids.push(keyed(
                with(
                    node("div", "dead"),
                    vec![
                        text(
                            "strong",
                            "",
                            s.ui("hud.respawn").replace("{v}", &left.to_string()),
                        ),
                        bb,
                    ],
                ),
                "dead",
            ));
        }
    }
    if let Some(o) = m.over {
        let key = if o == Outcome::Won {
            "result.won"
        } else {
            "result.lost"
        };
        let mut buttons = vec![
            button("primary", s.ui("result.again"), "again", ""),
            button("", s.ui("result.arcade"), "screen", "arcade"),
        ];
        if m.online {
            buttons = vec![button("primary", s.ui("result.title"), "screen", "title")];
        }
        kids.push(keyed(
            with(
                node("div", "overlay result"),
                vec![
                    text("h1", "", s.ui(key)),
                    scoreboard(g, w),
                    with(node("div", "menu-buttons"), buttons),
                ],
            ),
            "result",
        ));
    } else if g.menu || m.paused {
        kids.push(keyed(
            with(
                node("div", "overlay pause"),
                vec![
                    text("h1", "", s.ui("menu.title")),
                    with(
                        node("div", "menu-buttons"),
                        vec![
                            button("primary", s.ui("menu.resume"), "resume", ""),
                            button(
                                "",
                                s.ui(if p.bot.is_some() {
                                    "menu.autopilot_off"
                                } else {
                                    "menu.autopilot_on"
                                }),
                                "autopilot",
                                "",
                            ),
                            button("", s.ui("menu.quit"), "quit", ""),
                        ],
                    ),
                ],
            ),
            "menu",
        ));
    }
    with(node("div", "hud"), kids)
}

fn bottom(g: &Game, w: &World, me: usize, hero: Uid) -> Node {
    let s = &g.strings;
    let ctx = g.ctx();
    let p = &w.players[me];
    let shown = if g.selected != NO_UNIT && w.live(g.selected).is_some() {
        g.selected
    } else {
        hero
    };
    let mut parts = Vec::new();
    if let Some(u) = w.get(shown) {
        let d = &g.rules.units[u.def as usize];
        let st = stats::of(&ctx, w, shown);
        let name = if d.kind == Kind::Hero {
            s.hero(&d.id).name
        } else {
            s.unit(&d.id)
        };
        let portrait = if d.kind == Kind::Hero {
            art(&format!("heroes/{}", d.id))
        } else {
            String::new()
        };
        let mut card = vec![
            Node {
                t: "img",
                c: "portrait".into(),
                i: portrait,
                ..Node::default()
            },
            text("strong", "name", name),
        ];
        if let Some(h) = &u.hero {
            let levels = &g.rules.t.xp_levels;
            let next = levels.get((h.level - 1) as usize).copied().unwrap_or(h.xp);
            let prev = if h.level >= 2 {
                levels[(h.level - 2) as usize]
            } else {
                0
            };
            card.push(text(
                "span",
                "level",
                s.ui("hud.level").replace("{v}", &h.level.to_string()),
            ));
            let mut xp = node("div", "bar xp");
            xp.s.push((
                "--fill".into(),
                pct((h.xp - prev) as i64, (next - prev).max(1) as i64),
            ));
            card.push(xp);
        }
        let mut hp = text("div", "bar hp", format!("{} / {}", u.hp / 1000, st.max_hp));
        hp.s.push(("--fill".into(), pct(u.hp as i64, st.max_hp as i64 * 1000)));
        card.push(hp);
        if st.max_mana > 0 {
            let mut mp = text("div", "bar mana", format!("{} / {}", u.mana / 1000, st.max_mana));
            mp.s.push(("--fill".into(), pct(u.mana as i64, st.max_mana as i64 * 1000)));
            card.push(mp);
        }
        let mut st_rows = vec![
            stat(
                g,
                "stat.damage",
                format!("{}–{} +{}", st.damage[0], st.damage[1], st.bonus),
                "Damage",
            ),
            stat(g, "stat.armor", hundredths(st.armor - st.armor % 10), "Armor"),
            stat(g, "stat.speed", st.move_speed.to_string(), "Movement speed"),
        ];
        if d.kind == Kind::Hero {
            st_rows.push(stat(g, "stat.str", st.attrs[0].to_string(), "Strength"));
            st_rows.push(stat(g, "stat.agi", st.attrs[1].to_string(), "Agility"));
            st_rows.push(stat(g, "stat.int", st.attrs[2].to_string(), "Intelligence"));
        }
        card.push(with(node("div", "stats"), st_rows));
        parts.push(keyed(with(node("div", "unit-card"), card), "card"));
    }
    // abilities of my hero
    if let Some(u) = w.get(hero) {
        let points = u.hero.as_ref().map(|h| h.points).unwrap_or(0);
        let mut row = Vec::new();
        for (slot, &k) in g.rules.unit_abilities[u.def as usize].iter().enumerate() {
            let a = &g.rules.abilities[k];
            let st = u.abil[slot];
            let tip = ability_tip(g, k, st.level);
            let mut btn = node("button", "slot ability");
            btn.a = "ability".into();
            btn.v = slot.to_string();
            btn.i = tip.icon.clone();
            let mut kids = vec![text(
                "span",
                "hotkey",
                a.key.trim_start_matches("Key").to_string(),
            )];
            let pips: Vec<Node> = (0..a.max_level())
                .map(|i| node("i", if i < st.level { "pip on" } else { "pip" }))
                .collect();
            kids.push(with(node("span", "pips"), pips));
            if st.ready_at > w.tick {
                let left = (st.ready_at - w.tick) as i32;
                let total = (a.cooldown_at(st.level) * TPS / 100).max(1);
                let mut cd = text("span", "cooldown", ((left + TPS - 1) / TPS).to_string());
                cd.s.push(("--cd".into(), pct(left as i64, total as i64)));
                kids.push(cd);
            }
            if st.on {
                btn.c.push_str(" toggled");
            }
            if st.level == 0 || a.target == Target::Passive {
                btn.c.push_str(" passive");
            }
            if st.level > 0 && u.mana < a.mana_at(st.level) * 1000 && a.target != Target::Passive {
                btn.c.push_str(" low-mana");
            }
            if g.aim == Some(Aim::Ability(slot as u8)) {
                btn.c.push_str(" aiming");
            }
            btn.n = kids;
            btn.tip = Some(tip);
            let mut cell = vec![btn];
            if points > 0 {
                cell.push(button("learn", "+", "learn", slot.to_string()));
            }
            row.push(keyed(with(node("div", "slot-cell"), cell), format!("ab{slot}")));
        }
        if points > 0 {
            let mut stats_btn = button("learn stats", s.ui("hud.stats_bonus"), "learn", "4");
            stats_btn.tip = Some(Tip {
                title: s.ui("hud.stats_bonus.title"),
                text: s.ui("hud.stats_bonus.text"),
                ..Tip::default()
            });
            row.push(keyed(stats_btn, "stats"));
        }
        parts.push(keyed(with(node("div", "abilities"), row), "abilities"));
        // inventory
        let mut inv = Vec::new();
        for (slot, it) in u.items.iter().enumerate() {
            let mut b = node("button", "slot item");
            if it.item > 0 {
                let k = it.item as usize - 1;
                let def = &g.rules.items[k];
                b.i = art(&def.icon);
                b.a = "item".into();
                b.v = slot.to_string();
                let mut kids = vec![text("span", "hotkey", (slot + 1).to_string())];
                if it.charges > 0 || def.stacks {
                    kids.push(text("span", "charges", it.charges.to_string()));
                }
                if it.ready_at > w.tick {
                    let left = (it.ready_at - w.tick) as i32;
                    let mut cd = text("span", "cooldown", ((left + TPS - 1) / TPS).to_string());
                    cd.s.push((
                        "--cd".into(),
                        pct(left as i64, (def.cooldown * TPS / 100).max(1) as i64),
                    ));
                    kids.push(cd);
                }
                b.n = kids;
                let mut tip = item_tip(g, k, None);
                if crate::item::near_shop(w, &ctx, me, "home")
                    || crate::item::near_shop(w, &ctx, me, "side")
                    || crate::item::near_shop(w, &ctx, me, "secret")
                {
                    tip.lines.push(
                        s.ui("tip.sell")
                            .replace("{v}", &(g.rules.item_total(k) / 2).to_string()),
                    );
                }
                b.tip = Some(tip);
            } else {
                b.c.push_str(" empty");
            }
            inv.push(keyed(b, format!("it{slot}")));
        }
        parts.push(keyed(with(node("div", "inventory"), inv), "inventory"));
    }
    // gold and the side buttons
    let mut gold = text("div", "gold", p.gold.to_string());
    gold.term = "Gold".into();
    let mut side = vec![
        gold,
        button("", s.ui("hud.shop"), "shop.toggle", ""),
        button("", s.ui("hud.score"), "score.toggle", ""),
    ];
    let k = (p.team as usize).min(1);
    let mut glyph = button("", s.ui("hud.glyph"), "glyph", "");
    glyph.off = w.glyph_ready[k] > w.tick;
    glyph.term = "Glyph".into();
    side.push(glyph);
    if !p.pending.is_empty() {
        side.push(text(
            "span",
            "fine",
            s.ui("hud.courier").replace("{v}", &p.pending.len().to_string()),
        ));
    }
    parts.push(keyed(with(node("div", "side-buttons"), side), "side"));
    with(node("div", "bottombar"), parts)
}

fn stat(g: &Game, key: &str, v: String, term: &str) -> Node {
    let mut n = with(
        node("span", "stat"),
        vec![text("b", "", g.strings.ui(key)), text("span", "", v)],
    );
    n.term = term.into();
    n
}

fn shop(g: &Game, w: &World, me: usize) -> Node {
    let s = &g.strings;
    let ctx = g.ctx();
    let mut tabs_order: Vec<String> = Vec::new();
    for it in &g.rules.items {
        if it.shop != "none" && !tabs_order.contains(&it.tab) {
            tabs_order.push(it.tab.clone());
        }
    }
    let tabs: Vec<Node> = tabs_order
        .iter()
        .map(|t| {
            let label = s.tabs.get(t).cloned().unwrap_or_else(|| t.clone());
            let c = if *t == g.shop_tab { "tab on" } else { "tab" };
            button(c, label, "shop.tab", t.clone())
        })
        .collect();
    let mut items = Vec::new();
    let near_home = crate::item::near_shop(w, &ctx, me, "home");
    let courier = crate::courier::at_base(w, &ctx, w.players[me].team);
    for (k, it) in g.rules.items.iter().enumerate() {
        if it.tab != g.shop_tab || it.shop == "none" {
            continue;
        }
        let price = crate::item::price(w, &ctx, me, k);
        let here = crate::item::near_shop(w, &ctx, me, &it.shop) || (it.shop == "home" && courier);
        let mut b = with(
            node("button", "shop-item"),
            vec![
                Node {
                    t: "img",
                    c: "icon".into(),
                    i: art(&it.icon),
                    ..Node::default()
                },
                text("span", "cost", price.to_string()),
            ],
        );
        b.a = "buy".into();
        b.v = k.to_string();
        b.off = w.players[me].gold < price || !here;
        let mut tip = item_tip(g, k, Some(price));
        if it.shop == "secret" {
            tip.lines.push(s.ui("tip.secret"));
        }
        if !near_home && it.shop == "home" && courier {
            tip.lines.push(s.ui("tip.courier"));
        }
        b.tip = Some(tip);
        items.push(keyed(b, format!("si{k}")));
    }
    with(
        node("div", "shop panel"),
        vec![
            with(
                node("header", "shop-head"),
                vec![
                    text("h2", "", s.ui("shop.title")),
                    button("back", s.ui("close"), "shop.toggle", ""),
                ],
            ),
            with(node("div", "tabs"), tabs),
            with(node("div", "shop-grid"), items),
        ],
    )
}

fn scoreboard(g: &Game, w: &World) -> Node {
    let s = &g.strings;
    let mut rows = vec![with(
        node("div", "row head"),
        ["score.hero", "score.level", "score.kda", "score.lh", "score.gold"]
            .iter()
            .map(|k| text("span", "", s.ui(k)))
            .collect(),
    )];
    for (p, pl) in w.players.iter().enumerate() {
        if !pl.present || pl.pick < 0 {
            continue;
        }
        let level = w.get(pl.hero).map(|u| u.level()).unwrap_or(1);
        let side = if pl.team == 0 { "row sun" } else { "row moon" };
        let mut name = hero_name(g, w, p);
        if pl.bot.is_some() {
            name = s.ui("score.bot").replace("{v}", &name);
        }
        rows.push(keyed(
            with(
                node("div", side),
                vec![
                    text("span", "", name),
                    text("span", "", level.to_string()),
                    text(
                        "span",
                        "",
                        format!("{} / {} / {}", pl.kills, pl.deaths, pl.assists),
                    ),
                    text("span", "", format!("{} / {}", pl.last_hits, pl.denies)),
                    text("span", "", pl.gold.to_string()),
                ],
            ),
            format!("p{p}"),
        ));
    }
    with(node("div", "scoreboard panel"), rows)
}

/// For a unit on screen, whether it is invisible to the viewer: used by the HUD.
pub fn hidden(w: &World, id: Uid) -> bool {
    w.get(id).map(|u| u.has(M::Invisible)).unwrap_or(true)
}

/// The online screens: the form to host or join, and the lobby.
pub fn online_view(g: &Game) -> Node {
    let s = &g.strings;
    let o = &g.online;
    let status = if o.status.is_empty() {
        String::new()
    } else {
        s.ui(&o.status)
    };
    if g.screen == "online" {
        let mut name = node("input", "text-input");
        name.a = "online.name".into();
        name.v = o.name.clone();
        name.x = s.ui("online.name_hint");
        name.k = "name".into();
        let mut code = node("input", "text-input code");
        code.a = "online.code".into();
        code.v = o.typed.clone();
        code.x = s.ui("online.code_hint");
        code.k = "code".into();
        return with(
            node("div", "menu online"),
            vec![
                with(
                    node("header", "menu-head"),
                    vec![
                        text("h1", "", s.ui("online.title")),
                        button("back", s.ui("back"), "screen", "title"),
                    ],
                ),
                with(
                    node("div", "online-body"),
                    vec![
                        with(
                            node("section", "panel"),
                            vec![
                                text("h2", "", s.ui("online.you")),
                                name,
                                text("p", "fine", s.ui("online.relays")),
                            ],
                        ),
                        with(
                            node("section", "panel"),
                            vec![
                                text("h2", "", s.ui("online.host_title")),
                                text("p", "", s.ui("online.host_text")),
                                button("primary", s.ui("online.host"), "online.host", ""),
                            ],
                        ),
                        with(
                            node("section", "panel"),
                            vec![
                                text("h2", "", s.ui("online.join_title")),
                                code,
                                button("primary", s.ui("online.join"), "online.join", ""),
                            ],
                        ),
                    ],
                ),
                text("p", "status-line", status),
            ],
        );
    }
    // the lobby
    let host = o.role == crate::online::Role::Host;
    let bots = difficulty_name(g, o.bots);
    let mut sides = Vec::new();
    for (team, range, label) in [(0usize, 0..5, "online.sunward"), (1, 5..10, "online.moonward")] {
        let mut rows = vec![text(
            "h2",
            if team == 0 {
                "side-title sun"
            } else {
                "side-title moon"
            },
            s.ui(label),
        )];
        for k in range {
            let seat = o.seats.get(k).cloned().unwrap_or_default();
            let mine = o.my_seat == Some(k);
            let mut kids = Vec::new();
            if seat.pick >= 0 {
                let id = &g.rules.units[g.rules.heroes[seat.pick as usize]].id;
                kids.push(Node {
                    t: "img",
                    c: "portrait".into(),
                    i: art(&format!("heroes/{id}")),
                    ..Node::default()
                });
            } else {
                kids.push(node("span", "portrait empty"));
            }
            let who = if seat.human {
                let mut n = seat.name.clone();
                if seat.away {
                    n = s.ui("online.away").replace("{v}", &n);
                }
                n
            } else {
                s.ui("online.open_seat").replace("{v}", &bots)
            };
            kids.push(text("strong", "", who));
            if seat.pick >= 0 {
                kids.push(text(
                    "span",
                    "fine",
                    s.hero(&g.rules.units[g.rules.heroes[seat.pick as usize]].id).name,
                ));
            }
            if !seat.human && !o.started {
                kids.push(button("sit", s.ui("online.sit"), "online.sit", k.to_string()));
            }
            let c = if mine {
                "seat mine"
            } else if seat.human {
                "seat taken"
            } else {
                "seat"
            };
            rows.push(keyed(with(node("div", c), kids), format!("seat{k}")));
        }
        sides.push(with(node("section", "panel seats"), rows));
    }
    // the hero choice, as in the arcade
    let mut cards = Vec::new();
    let mine = o
        .my_seat
        .and_then(|k| o.seats.get(k))
        .map(|s| s.pick)
        .unwrap_or(-1);
    for (k, &u) in g.rules.heroes.iter().enumerate() {
        let id = &g.rules.units[u].id;
        let taken = o.seats.iter().any(|s| s.pick == k as i32) && mine != k as i32;
        let mut card = with(
            node(
                "button",
                if mine == k as i32 {
                    "hero-mini chosen"
                } else {
                    "hero-mini"
                },
            ),
            vec![Node {
                t: "img",
                c: "portrait".into(),
                i: art(&format!("heroes/{id}")),
                ..Node::default()
            }],
        );
        card.a = "pick".into();
        card.v = k.to_string();
        card.off = taken || o.started;
        card.tip = Some(hero_tip(g, k));
        cards.push(keyed(card, format!("hm{k}")));
    }
    let mut head = vec![
        text("h1", "", s.ui("online.lobby")),
        with(
            node("div", "room-code"),
            vec![
                text("span", "label", s.ui("online.room")),
                text("strong", "code", o.room.clone()),
                button("", s.ui("online.copy_code"), "copy", o.room.clone()),
                button(
                    "",
                    s.ui("online.copy_link"),
                    "copy.link",
                    format!("room={}", o.room),
                ),
            ],
        ),
        button("back", s.ui("online.leave"), "online.leave", ""),
    ];
    if host {
        let mut start = button("primary", s.ui("online.start"), "online.start", "");
        start.off = o.started;
        head.insert(2, start);
    }
    let mut setup = vec![
        text("h2", "", s.ui("online.your_hero")),
        with(node("div", "hero-mini-grid"), cards),
    ];
    if host {
        setup.push(with(
            node("div", "stepper"),
            vec![
                text("span", "label", s.ui("online.bot_skill")),
                button("step", "−", "online.bots", "-"),
                text("span", "value", bots.clone()),
                button("step", "+", "online.bots", "+"),
            ],
        ));
    }
    setup.push(text("p", "status-line", status));
    with(
        node("div", "menu lobby"),
        vec![
            with(node("header", "menu-head"), head),
            with(
                node("div", "lobby-body"),
                vec![
                    sides.remove(0),
                    with(node("section", "panel"), setup),
                    sides.remove(0),
                ],
            ),
        ],
    )
}
