// Online play, as the page sees it: opens the transport (web/net/rtc.js)
// when the core asks for it, hands the core every peer, departure and
// message, and sends what the core leaves in its outbox. The core decides
// everything; this file only carries. It is the one page file that imports
// the transport (online_fence.rs), and it imports it only once a player
// hosts or joins, so loading and idling the page stay on its own origin.
const encoder = new TextEncoder();
const decoder = new TextDecoder();

let game = null;
let act = null;
let link = null;
let linkKey = "";
let opening = false;
const peerIds = new Map();

async function open(role, room) {
  opening = true;
  const rtc = await import("./net/rtc.js?v=10edb21dc95e");
  opening = false;
  link = rtc.open({
    isHost: role === "host",
    mode: "room",
    room,
    build: "",
    onPeer: (id) => {
      peerIds.set(String(id), id);
      game.net_event("peer", String(id));
    },
    onLeft: (id) => game.net_event("left", String(id)),
    onMessage: (id, bytes) => game.net_in(String(id), decoder.decode(bytes)),
    onError: (key) => game.net_event("error", key),
  });
}

function close() {
  if (link) link.close();
  link = null;
  linkKey = "";
  peerIds.clear();
}

// A room link (#room=CODE or #room=CODE.SEAT) joins on opening, and after a
// reload rejoins the seat; a link pasted into an open page joins too.
function joinFromLink() {
  const room = new URLSearchParams(location.hash.slice(1)).get("room");
  if (!room || linkKey !== "") return;
  act("screen", "online");
  act("online.join", room);
}

export function setup(g, a) {
  game = g;
  act = a;
  joinFromLink();
  window.addEventListener("hashchange", joinFromLink);
}

export function pump(g) {
  if (!game) return;
  const net = JSON.parse(g.net_out());
  const key = net.want ? `${net.want.role} ${net.want.room}` : "";
  if (key !== linkKey && !opening) {
    close();
    if (net.want) {
      linkKey = key;
      open(net.want.role, net.want.room);
    }
  }
  if (net.hash && location.hash.slice(1) !== net.hash) history.replaceState(null, "", `#${net.hash}`);
  if (!link) return;
  for (const [to, text, fast] of net.out) {
    const bytes = encoder.encode(text);
    const targets = to === "*" ? Array.from(peerIds.values()) : [peerIds.get(to)];
    for (const id of targets) if (id !== undefined) link.send(id, bytes, fast);
  }
}

// Copying a room code or link is the page's own: the core only names what to copy.
export function handles(name, value) {
  if (name !== "copy" && name !== "copy.link") return false;
  const text = name === "copy" ? value : `${location.origin}${location.pathname}#${value}`;
  // older browsers, and pages not served over https, have no clipboard API
  const fallback = () => {
    const box = document.createElement("textarea");
    box.value = text;
    box.setAttribute("readonly", "");
    box.className = "offscreen";
    document.body.append(box);
    box.select();
    document.execCommand("copy");
    box.remove();
  };
  if (navigator.clipboard) navigator.clipboard.writeText(text).catch(fallback);
  else fallback();
  return true;
}

export function after(g, name) {
  if (name === "online.leave") {
    close();
    history.replaceState(null, "", location.pathname + location.search);
  }
  return g;
}
