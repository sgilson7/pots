// The page draws what the core returns and decides nothing.
//
// The core describes every screen and the HUD as a tree of nodes (tag,
// class, text, icon, tip, action); this file builds the DOM from that tree,
// keyed so only what changed is replaced. Every button sends its action
// name and value back through `act`. Raw input (key codes, mouse buttons,
// pointer pixels, the clock) goes to the core as it comes; the core decides
// what it means. The 3D view is a WebGL replay of the core's scene list.
import init, { Game, numbers } from "./pkg/tool_shim.js";
import { makeRenderer } from "./gl.js";
import { createRenderer } from "./render2d.js";
import * as online from "./online.js";

const el = (id) => document.getElementById(id);
const FILES = ["strings", "rules", "units", "abilities", "items", "map", "ai", "models", "presentation", "keys", "arcade"];

let game = null;
let N = null;
let renderer = null;
let hints = new Map();
let lastUi = "";
let lastUiTime = -Infinity;
let groundVersion = -Infinity;
let tiles = null;
let mini = null;

async function text(path) {
  const res = await fetch(path);
  if (!res.ok) throw new Error(`${path}: ${res.status}`);
  return res.text();
}

// ---------------------------------------------------------------- the view model

const tipBox = () => el("tip");

function showTip(tip, e) {
  const box = tipBox();
  box.replaceChildren();
  const head = document.createElement("div");
  head.className = "tip-head";
  if (tip.icon) {
    const img = document.createElement("img");
    img.src = tip.icon;
    img.alt = "";
    head.append(img);
  }
  const titles = document.createElement("div");
  const t = document.createElement("strong");
  t.textContent = tip.title;
  titles.append(t);
  if (tip.sub) {
    const s = document.createElement("span");
    s.textContent = tip.sub;
    titles.append(s);
  }
  head.append(titles);
  box.append(head);
  if (tip.text) {
    const p = document.createElement("p");
    p.textContent = tip.text;
    box.append(p);
  }
  for (const line of tip.lines || []) {
    const p = document.createElement("p");
    p.className = "line";
    p.textContent = line;
    box.append(p);
  }
  box.hidden = false;
  moveTip(e);
}

function moveTip(e) {
  const box = tipBox();
  // in the upper half the tip opens below the pointer, in the right half to its left
  box.dataset.below = String(e.clientY < window.innerHeight - e.clientY);
  box.dataset.left = String(e.clientX > window.innerWidth - e.clientX);
  box.style.setProperty("--x", `${e.clientX}px`);
  box.style.setProperty("--y", `${e.clientY}px`);
}

function hideTip() {
  tipBox().hidden = true;
}

// Builds one node; its JSON is kept on the element so an unchanged node is reused.
function build(n) {
  const node = document.createElement(n.t);
  if (n.c) node.className = n.c;
  if (n.t === "img") {
    node.src = n.i;
    node.alt = "";
  }
  // an icon or a text beside children becomes a child of its own, so the
  // keyed children below keep them
  const kids = n.n || [];
  const extra = [];
  if (n.i && n.t !== "img") extra.push({ t: "img", c: "slot-icon", i: n.i, k: "icon" });
  if (n.x && kids.length) extra.push({ t: "span", c: "label", x: n.x, k: "label" });
  if (n.x && !kids.length) node.textContent = n.x;
  for (const [k, v] of n.s || []) node.style.setProperty(k, v);
  if (n.a) {
    node.dataset.a = n.a;
    node.dataset.v = n.v || "";
  }
  if (n.off) {
    node.disabled = true;
    node.classList.add("off");
  }
  if (n.tip) node.tipData = n.tip;
  if (n.term && hints.has(n.term)) {
    node.dataset.term = n.term;
    node.title = hints.get(n.term);
  }
  if (extra.length || kids.length) sync(node, extra.concat(kids));
  return node;
}

// Makes `parent`'s children match `kids`, reusing any child whose JSON did not change.
function sync(parent, kids) {
  const old = Array.from(parent.children).filter((c) => c.jsonKey !== undefined);
  const byKey = new Map(old.map((c) => [c.jsonKey, c]));
  const next = kids.map((k, i) => {
    const key = k.k || `${k.t}.${i}`;
    const json = JSON.stringify(k);
    const prev = byKey.get(key);
    if (prev && prev.json === json) return prev;
    const made = build(k);
    made.jsonKey = key;
    made.json = json;
    return made;
  });
  const same = next.length === parent.children.length && next.every((n, i) => parent.children[i] === n);
  if (!same) parent.replaceChildren(...next);
}

function drawUi(force) {
  const json = game.ui();
  if (!force && json === lastUi) return;
  lastUi = json;
  const v = JSON.parse(json);
  document.title = v.title;
  document.body.dataset.screen = v.screen;
  document.body.dataset.cursor = v.cursor;
  el("notice").textContent = v.notice;
  sync(el("ui"), [v.root]);
}

function act(name, value) {
  const r = JSON.parse(game.act(name, value));
  if (r.error) el("notice").textContent = r.error;
  online.after(game, name);
  drawUi(true);
}

// ---------------------------------------------------------------- input

function wireInput(canvas) {
  el("ui").addEventListener("click", (e) => {
    const target = e.target.closest("[data-a]");
    if (!target || target.disabled) return;
    act(target.dataset.a, target.dataset.v);
  });
  el("ui").addEventListener("mouseover", (e) => {
    const t = e.target.closest("*");
    let n = t;
    while (n && !n.tipData && n !== el("ui")) n = n.parentElement;
    if (n && n.tipData) showTip(n.tipData, e);
    else hideTip();
  });
  el("ui").addEventListener("mousemove", (e) => {
    if (!tipBox().hidden) moveTip(e);
  });
  el("ui").addEventListener("mouseleave", hideTip);
  const local = (e) => {
    const r = canvas.getBoundingClientRect();
    return [e.clientX - r.left, e.clientY - r.top];
  };
  canvas.addEventListener("mousemove", (e) => {
    const [x, y] = local(e);
    game.pointer(x, y, true);
  });
  canvas.addEventListener("mouseleave", () => game.pointer(0, 0, false));
  canvas.addEventListener("mousedown", (e) => {
    const [x, y] = local(e);
    game.click(e.button, x, y);
    drawUi(true);
  });
  canvas.addEventListener("contextmenu", (e) => e.preventDefault());
  canvas.addEventListener("wheel", (e) => {
    e.preventDefault();
    game.wheel(e.deltaY);
  }, { passive: false });
  const miniCanvas = el("minimap-top");
  miniCanvas.addEventListener("mousedown", (e) => {
    const r = miniCanvas.getBoundingClientRect();
    game.minimap_click(e.button, e.clientX - r.left, e.clientY - r.top);
  });
  miniCanvas.addEventListener("contextmenu", (e) => e.preventDefault());
  const typing = (e) => e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement;
  window.addEventListener("keydown", (e) => {
    if (typing(e)) return;
    if (document.body.dataset.screen === "match") e.preventDefault();
    game.key(e.code, true);
    drawUi(true);
  });
  window.addEventListener("keyup", (e) => {
    if (typing(e)) return;
    game.key(e.code, false);
  });
  el("ui").addEventListener("input", (e) => {
    const t = e.target;
    if (t.dataset && t.dataset.a) act(t.dataset.a, t.value);
  });
}

function resize(canvas) {
  const [w, h] = game.resize(canvas.clientWidth, canvas.clientHeight, window.devicePixelRatio || 1);
  renderer.resize(w, h);
}

// ---------------------------------------------------------------- the loop

function loop(now) {
  game.frame(now);
  online.pump(game);
  game.render();
  renderer.setUniforms(game.uniforms(), game.clear_color());
  renderer.frame(game.mats(), game.draws(), game.scene());
  if (document.body.dataset.screen === "match") {
    const v = game.ground_version();
    if (v !== groundVersion) {
      groundVersion = v;
      mini.ground.draw(game.minimap_ground());
    }
    mini.top.draw(game.minimap_units());
  }
  if (now - lastUiTime >= N.ui_every_ms) {
    lastUiTime = now;
    drawUi(false);
    document.body.dataset.probe = game.probe();
  }
  if (tiles && document.body.dataset.tile !== "drawn") {
    document.body.dataset.tile = "drawn";
  }
  requestAnimationFrame(loop);
}

async function main() {
  await init();
  N = JSON.parse(numbers());
  const texts = await Promise.all(FILES.map((f) => text(`data/${f}.json`)));
  const data = Object.fromEntries(FILES.map((f, i) => [f, texts[i]]));
  const glossary = await text("data/glossary.md");
  const vertex = await text("data/shaders/scene.vert");
  const fragment = await text("data/shaders/scene.frag");
  const seed = new Uint32Array(1);
  crypto.getRandomValues(seed);
  game = new Game(data.strings, glossary, data.rules, data.units, data.abilities, data.items, data.map, data.ai,
    data.models, data.presentation, data.keys, data.arcade, seed[0]);
  hints = new Map(JSON.parse(game.hints()));
  const canvas = el("view");
  renderer = makeRenderer(canvas, N, { vertex, fragment }, (id) => game.mesh(id));
  if (!renderer) {
    el("notice").textContent = el("notice").dataset.nogl;
    return;
  }
  const assets = JSON.parse(N.minimap);
  mini = { ground: createRenderer(el("minimap-ground"), assets), top: createRenderer(el("minimap-top"), assets) };
  resize(canvas);
  new ResizeObserver(() => resize(canvas)).observe(canvas);
  wireInput(canvas);
  online.setup(game, act);
  const params = new URLSearchParams(location.search);
  tiles = params.get("tile");
  if (tiles) {
    document.body.dataset.stage = "still";
    const r = JSON.parse(game.stage(tiles));
    if (r.error) el("notice").textContent = r.error;
  }
  drawUi(true);
  document.body.dataset.ready = "true";
  requestAnimationFrame(loop);
}

main().catch((e) => {
  el("notice").textContent = String(e);
});
