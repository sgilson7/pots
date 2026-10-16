// Online play: the transport is loaded only when a player opens online
// play. Until then this module does nothing.
let ready = false;

export function setup(game, act) {
  ready = Boolean(game && act);
}

export function pump(game) {
  if (!ready) return;
  game.net_out();
}

export function after(game, name) {
  return [game, name];
}
