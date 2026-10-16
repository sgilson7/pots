//! tool_shim: moves values between the page and `pots_core::game`.
//!
//! Builder requirement: the shim decides nothing. Each function is one call
//! into the core and a conversion of its arguments or result. The core
//! speaks Q16.16 integers; the few values WebGL wants as floats (the
//! matrices, the uniforms, the clear colour) are divided by the core's own
//! `ONE` here. `crates/core/tests/boundary.rs` fails if a branch, a loop or
//! the `?` operator appears in this file.

use pots_core::{fx, game};
use wasm_bindgen::prelude::*;

const ONE: f32 = fx::ONE as f32;

fn floats(v: Vec<i32>) -> Vec<f32> {
    v.into_iter().map(|x| x as f32 / ONE).collect()
}

#[wasm_bindgen]
pub struct Game {
    inner: game::Game,
}

#[wasm_bindgen]
impl Game {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        strings: &str,
        glossary: &str,
        rules: &str,
        units: &str,
        abilities: &str,
        items: &str,
        map: &str,
        ai: &str,
        models: &str,
        presentation: &str,
        keys: &str,
        arcade: &str,
        seed: u32,
    ) -> Result<Game, JsValue> {
        let files = game::Files {
            strings,
            glossary,
            rules,
            units,
            abilities,
            items,
            map,
            ai,
            models,
            presentation,
            keys,
            arcade,
        };
        game::Game::new(&files, seed as u64)
            .map(|inner| Game { inner })
            .map_err(|e| JsValue::from_str(&e))
    }

    /// CSS size and device pixel ratio in; drawing-buffer size out.
    pub fn resize(&mut self, css_w: f64, css_h: f64, dpr: f64) -> Vec<i32> {
        self.inner
            .resize(css_w as i32, css_h as i32, (dpr * 100.0) as i32)
            .to_vec()
    }

    /// The page's clock in milliseconds.
    pub fn frame(&mut self, now_ms: f64) {
        self.inner.frame((now_ms * 1000.0) as i64)
    }

    pub fn pointer(&mut self, x: f64, y: f64, inside: bool) {
        self.inner.pointer_move(x as i32, y as i32, inside)
    }

    pub fn click(&mut self, button: i32, x: f64, y: f64) {
        self.inner.click(button, x as i32, y as i32)
    }

    pub fn minimap_click(&mut self, button: i32, x: f64, y: f64) {
        self.inner.minimap_click(button, x as i32, y as i32)
    }

    pub fn wheel(&mut self, dy: f64) {
        self.inner.wheel(dy as i32)
    }

    pub fn key(&mut self, code: &str, down: bool) {
        self.inner.key(code, down)
    }

    /// A button or setting: `{"ok":true}` or `{"error":"..."}`.
    pub fn act(&mut self, name: &str, value: &str) -> String {
        game::answer(self.inner.act(name, value))
    }

    pub fn ui(&self) -> String {
        self.inner.ui_json()
    }

    pub fn hints(&self) -> String {
        self.inner.hints_json()
    }

    /// Builds the frame's scene; `scene` and `draws` then read it.
    pub fn render(&mut self) {
        self.inner.render()
    }

    pub fn scene(&self) -> Vec<i32> {
        game::values(self.inner.last.clone())
    }

    pub fn draws(&self) -> Vec<u32> {
        game::draw_list(&self.inner.last)
    }

    pub fn mats(&self) -> Vec<f32> {
        floats(self.inner.view_mats())
    }

    pub fn uniforms(&self) -> Vec<f32> {
        floats(self.inner.uniforms())
    }

    pub fn clear_color(&self) -> Vec<f32> {
        floats(self.inner.clear_color())
    }

    pub fn mesh(&self, id: u32) -> Vec<i32> {
        game::mesh(id)
    }

    pub fn minimap_ground(&self) -> Vec<i32> {
        self.inner.minimap_ground()
    }

    pub fn minimap_units(&self) -> Vec<i32> {
        self.inner.minimap_units()
    }

    pub fn ground_version(&self) -> u32 {
        self.inner.ground_version()
    }

    /// For screenshots (`?tile=<spec>`): a still scene.
    pub fn stage(&mut self, spec: &str) -> String {
        game::answer(self.inner.set_stage(spec))
    }

    /// Online: messages out as JSON, and one message in from a peer.
    pub fn net_out(&mut self) -> String {
        self.inner.net_out()
    }

    pub fn net_in(&mut self, peer: &str, text: &str) {
        self.inner.net_in(peer, text)
    }

    pub fn net_event(&mut self, kind: &str, peer: &str) {
        self.inner.net_event(kind, peer)
    }

    /// The match state the browser checks and runners read.
    pub fn probe(&self) -> String {
        self.inner.probe()
    }
}

#[wasm_bindgen]
pub fn numbers() -> String {
    game::numbers()
}
