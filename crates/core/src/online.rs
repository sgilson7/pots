//! Online play: placeholder until the rollback session is written.

#[derive(Debug, Default)]
pub struct Online {}

pub fn frame(_g: &mut crate::game::Game, _dt: i64) {}

pub fn view(g: &crate::game::Game) -> crate::ui::Node {
    let _ = g;
    crate::ui::Node::default()
}

pub fn enter(_g: &mut crate::game::Game, _screen: &str) {}
pub fn leave(_g: &mut crate::game::Game) {}
pub fn pick(_g: &mut crate::game::Game, _k: u8) {}
pub fn act(_g: &mut crate::game::Game, _name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

pub fn take_out(_g: &mut crate::game::Game) -> String {
    "[]".into()
}
pub fn receive(_g: &mut crate::game::Game, _peer: &str, _text: &str) {}
pub fn event(_g: &mut crate::game::Game, _kind: &str, _peer: &str) {}
pub fn probe(_g: &crate::game::Game) -> serde_json::Value {
    serde_json::Value::Null
}
