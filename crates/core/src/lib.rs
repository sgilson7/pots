#![allow(clippy::too_many_arguments, clippy::type_complexity)]
//! pots_core: every rule of Protection of the Spheres.
//!
//! Builder requirement: this crate performs no input or output. It takes
//! values and returns values, so `cargo test` reaches every rule without a
//! browser. `tests/boundary.rs` enforces that, along with the dependency
//! allowlist and the reference ban on floats, hash maps and clocks.

pub mod ability;
pub mod ai;
pub mod api;
pub mod combat;
pub mod courier;
pub mod draw;
pub mod fx;
pub mod game;
pub mod geom;
pub mod glossary;
pub mod item;
pub mod lanes;
pub mod map;
pub mod mesh;
pub mod models;
pub mod online;
pub mod orders;
pub mod path;
pub mod render;
pub mod rng;
pub mod rules;
pub mod scene;
pub mod stats;
pub mod text;
pub mod ui;
pub mod vision;
pub mod world;
