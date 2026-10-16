//! Models: every hero, creep, building and neutral is a set of posed shapes
//! read from `data/models.json` (after Gatecrasher's `person()` and Peter's
//! part lists, but held in data so a look changes without a code change).
//!
//! A part is one mesh, placed by its centre, size and rotation in hundredths
//! of a scene unit and whole degrees, with a colour role and a joint. The
//! joint decides how the part moves: legs swing when walking, the weapon arm
//! strikes on an attack, both arms rise on a cast, and the body tips over in
//! death. Each joint turns about its pivot. A model's forward is +x.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::fx::{self, ONE};
use crate::geom::{v3, Affine, V3};
use crate::mesh::Mesh;
use crate::render::Rgb;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct PartFile {
    mesh: String,
    at: [i32; 3],
    size: [i32; 3],
    #[serde(default)]
    rot: [i32; 3],
    color: String,
    #[serde(default = "body")]
    joint: String,
    #[serde(default)]
    style: i32,
    /// Drawn without an outline (glows, flat marks).
    #[serde(default)]
    plain: bool,
}

fn body() -> String {
    "body".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelFile {
    /// Height of the top of the model, hundredths of a scene unit: where the
    /// health bar sits.
    height: i32,
    parts: Vec<PartFile>,
    #[serde(default)]
    pivots: BTreeMap<String, [i32; 3]>,
    /// Spin the whole model slowly (runes, the Spheres' cores).
    #[serde(default)]
    spin: i32,
    /// Free text for the person.
    #[serde(default, rename = "_note")]
    note: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelsFile {
    palette: BTreeMap<String, [i32; 3]>,
    /// Team colour roles: "team" in a part becomes the side's colour.
    teams: [BTreeMap<String, [i32; 3]>; 3],
    models: BTreeMap<String, ModelFile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joint {
    Body,
    Head,
    ArmL,
    ArmR,
    LegL,
    LegR,
    /// Never moves with the pose (bases, shadows).
    Fixed,
    /// Turns slowly about the vertical axis (a Sphere's core, a rune).
    Spin,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub mesh: Mesh,
    pub at: V3,
    pub size: V3,
    pub rot: Affine,
    /// Palette index; negative is a team role: -1 - role index.
    pub color: i32,
    pub joint: Joint,
    pub style: i32,
    pub plain: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub height: i32,
    pub parts: Vec<Part>,
    pub pivots: [V3; 8],
    pub spin: i32,
}

#[derive(Debug, Clone)]
pub struct Models {
    pub list: Vec<Model>,
    pub palette: Vec<Rgb>,
    pub palette_names: Vec<String>,
    /// For each team (sun, moon, neutral): the colour of each team role.
    pub teams: [Vec<Rgb>; 3],
    pub team_roles: Vec<String>,
}

fn hundredths(v: [i32; 3]) -> V3 {
    v3(v[0] * ONE / 100, v[1] * ONE / 100, v[2] * ONE / 100)
}

fn rgb(c: [i32; 3]) -> Rgb {
    [c[0] * ONE / 255, c[1] * ONE / 255, c[2] * ONE / 255]
}

fn mesh_named(s: &str) -> Option<Mesh> {
    Some(match s {
        "cube" => Mesh::Cube,
        "wedge" => Mesh::Wedge,
        "octa" => Mesh::Octa,
        "ground" => Mesh::Ground,
        "sphere" => Mesh::Sphere,
        "cyl" => Mesh::Cyl,
        "cone" => Mesh::Cone,
        "ring" => Mesh::Ring,
        "blob" => Mesh::Blob,
        "card" => Mesh::Card,
        "tri" => Mesh::Tri,
        "taper" => Mesh::Taper,
        _ => return None,
    })
}

fn joint_named(s: &str) -> Option<Joint> {
    Some(match s {
        "body" => Joint::Body,
        "head" => Joint::Head,
        "arm_l" => Joint::ArmL,
        "arm_r" => Joint::ArmR,
        "leg_l" => Joint::LegL,
        "leg_r" => Joint::LegR,
        "fixed" => Joint::Fixed,
        "spin" => Joint::Spin,
        _ => return None,
    })
}

impl Models {
    pub fn load(json: &str) -> Result<Models, String> {
        let f: ModelsFile =
            serde_json::from_str(json).map_err(|e| format!("models.json could not be read: {e}"))?;
        let palette_names: Vec<String> = f.palette.keys().cloned().collect();
        let palette: Vec<Rgb> = f.palette.values().map(|c| rgb(*c)).collect();
        let team_roles: Vec<String> = f.teams[0].keys().cloned().collect();
        let mut teams: [Vec<Rgb>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (k, t) in f.teams.iter().enumerate() {
            for role in &team_roles {
                let c = t
                    .get(role)
                    .ok_or_else(|| format!("Team {k} has no colour \"{role}\"."))?;
                teams[k].push(rgb(*c));
            }
        }
        let mut list = Vec::new();
        for (name, m) in &f.models {
            let mut parts = Vec::new();
            for p in &m.parts {
                let mesh = mesh_named(&p.mesh)
                    .ok_or_else(|| format!("Model \"{name}\": no mesh \"{}\".", p.mesh))?;
                let joint = joint_named(&p.joint)
                    .ok_or_else(|| format!("Model \"{name}\": no joint \"{}\".", p.joint))?;
                let color = if let Some(k) = palette_names.iter().position(|n| *n == p.color) {
                    k as i32
                } else if let Some(k) = team_roles.iter().position(|n| *n == p.color) {
                    -1 - k as i32
                } else {
                    return Err(format!("Model \"{name}\": no colour \"{}\".", p.color));
                };
                let rot = Affine::rot_y(fx::deg(p.rot[1]))
                    .then(&Affine::rot_x(fx::deg(p.rot[0])))
                    .then(&Affine::rot_z(fx::deg(p.rot[2])));
                parts.push(Part {
                    mesh,
                    at: hundredths(p.at),
                    size: hundredths(p.size),
                    rot,
                    color,
                    joint,
                    style: p.style,
                    plain: p.plain,
                });
            }
            let mut pivots = [V3::ZERO; 8];
            for (j, at) in &m.pivots {
                let k =
                    joint_named(j).ok_or_else(|| format!("Model \"{name}\": no joint \"{j}\" to pivot."))?;
                pivots[k as usize] = hundredths(*at);
            }
            let _ = &m.note;
            list.push(Model {
                name: name.clone(),
                height: m.height * ONE / 100,
                parts,
                pivots,
                spin: m.spin,
            });
        }
        Ok(Models {
            list,
            palette,
            palette_names,
            teams,
            team_roles,
        })
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.list.iter().position(|m| m.name == name)
    }

    pub fn color(&self, team: u8, c: i32) -> Rgb {
        if c >= 0 {
            self.palette[c as usize]
        } else {
            self.teams[(team as usize).min(2)][(-1 - c) as usize]
        }
    }

    pub fn named(&self, name: &str) -> Rgb {
        self.palette_names
            .iter()
            .position(|n| n == name)
            .map(|k| self.palette[k])
            .unwrap_or([ONE, 0, ONE])
    }

    pub fn team_named(&self, team: u8, name: &str) -> Rgb {
        self.team_roles
            .iter()
            .position(|n| n == name)
            .map(|k| self.teams[(team as usize).min(2)][k])
            .unwrap_or([ONE, 0, ONE])
    }
}

/// How a model is posed this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pose {
    /// Walk cycle phase in angle units (0 when standing).
    pub walk: i32,
    /// Attack swing progress, Q16 0..ONE (0 when not attacking).
    pub swing: i32,
    /// Cast: arms raised share, Q16.
    pub cast: i32,
    /// Death: share of the fall, Q16.
    pub death: i32,
    /// Spin angle for spinning parts.
    pub spin: i32,
    /// A spinning attack (Blade Fury): the whole body turns.
    pub whirl: i32,
}

impl Pose {
    pub const STILL: Pose = Pose {
        walk: 0,
        swing: 0,
        cast: 0,
        death: 0,
        spin: 0,
        whirl: 0,
    };
}

fn about(pivot: V3, rot: &Affine) -> Affine {
    Affine::translate(pivot)
        .then(rot)
        .then(&Affine::translate(pivot.neg()))
}

/// The transform of each joint for a pose, in model space.
pub fn joints(m: &Model, pose: &Pose) -> [Affine; 8] {
    let swing_leg = fx::mul(fx::sin(pose.walk), fx::deg(28));
    let swing_arm = fx::mul(fx::sin(pose.walk), fx::deg(22));
    let mut j = [Affine::IDENTITY; 8];
    let piv = |k: Joint| m.pivots[k as usize];
    j[Joint::LegL as usize] = about(piv(Joint::LegL), &Affine::rot_z(swing_leg));
    j[Joint::LegR as usize] = about(piv(Joint::LegR), &Affine::rot_z(-swing_leg));
    // arms: opposite the legs when walking; raised when casting
    let raise = fx::mul(pose.cast, fx::deg(115));
    j[Joint::ArmL as usize] = about(piv(Joint::ArmL), &Affine::rot_z(-swing_arm + raise));
    // the weapon arm: drawn back then struck through on an attack
    let strike = if pose.swing > 0 {
        // back to +70° by 40 %, then down to −50° by the hit
        let t = pose.swing;
        if t < ONE * 2 / 5 {
            fx::mul(fx::div(t, ONE * 2 / 5), fx::deg(70))
        } else {
            fx::deg(70) - fx::mul(fx::div(t - ONE * 2 / 5, ONE * 3 / 5), fx::deg(120))
        }
    } else {
        0
    };
    j[Joint::ArmR as usize] = about(piv(Joint::ArmR), &Affine::rot_z(swing_arm + raise + strike));
    // head nods slightly with the step
    let nod = fx::mul(fx::sin(pose.walk * 2), fx::deg(3));
    j[Joint::Head as usize] = about(piv(Joint::Head), &Affine::rot_z(nod));
    j[Joint::Spin as usize] = Affine::rot_y(pose.spin);
    j
}

/// Every part of a model placed in the world: `place` puts the model's
/// origin (its feet) and turns it to its facing.
pub fn pose_parts(m: &Model, pose: &Pose, place: &Affine, mut emit: impl FnMut(&Part, Affine)) {
    let j = joints(m, pose);
    // the body bobs with the step and falls over in death
    let bob = fx::mul(fx::sin(pose.walk * 2).abs(), ONE / 20);
    let fall = Affine::rot_x(fx::mul(pose.death, fx::deg(88)));
    let sink = Affine::translate(v3(0, bob - fx::mul(pose.death, ONE / 3), 0));
    let whirl = Affine::rot_y(pose.whirl);
    let body = sink.then(&fall).then(&whirl);
    for p in &m.parts {
        let local = Affine::translate(p.at).then(&p.rot).then(&Affine::scale(p.size));
        let joint = match p.joint {
            Joint::Body => Affine::IDENTITY,
            Joint::Fixed => {
                emit(p, place.then(&local));
                continue;
            }
            k => j[k as usize],
        };
        let a = place.then(&body).then(&joint).then(&local);
        emit(p, a);
    }
}
