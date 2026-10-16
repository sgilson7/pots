//! Reader-visible words: every string comes from `data/strings.json`, and
//! every number in a string is a `{placeholder}` the core fills from the
//! rules, so a description cannot disagree with the ability it describes.
//!
//! A placeholder names a value of the ability or item (`{damage}`), with an
//! optional form after a colon: `{stun:s}` turns hundredths of a second
//! into seconds, `{pct:%}` adds a percent sign. A value that changes with
//! level is written for every level, the current one marked: `100 / [175] /
//! 250 / 325`.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Named {
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Strings {
    pub ui: BTreeMap<String, String>,
    #[serde(default)]
    pub heroes: BTreeMap<String, Named>,
    #[serde(default)]
    pub abilities: BTreeMap<String, Named>,
    #[serde(default)]
    pub items: BTreeMap<String, Named>,
    #[serde(default)]
    pub units: BTreeMap<String, String>,
    #[serde(default)]
    pub missions: BTreeMap<String, Named>,
    #[serde(default)]
    pub difficulty: BTreeMap<String, String>,
    #[serde(default)]
    pub tabs: BTreeMap<String, String>,
}

impl Strings {
    pub fn load(json: &str) -> Result<Strings, String> {
        serde_json::from_str(json).map_err(|e| format!("strings.json could not be read: {e}"))
    }

    /// A UI string by key; a missing key shows the key, so a gap is visible.
    pub fn ui(&self, key: &str) -> String {
        self.ui.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    pub fn hero(&self, id: &str) -> Named {
        self.heroes.get(id).cloned().unwrap_or(Named {
            name: id.to_string(),
            ..Named::default()
        })
    }

    pub fn ability(&self, id: &str) -> Named {
        self.abilities.get(id).cloned().unwrap_or(Named {
            name: id.to_string(),
            ..Named::default()
        })
    }

    pub fn item(&self, id: &str) -> Named {
        self.items.get(id).cloned().unwrap_or(Named {
            name: id.to_string(),
            ..Named::default()
        })
    }

    pub fn unit(&self, id: &str) -> String {
        self.units.get(id).cloned().unwrap_or_else(|| id.to_string())
    }
}

/// Hundredths as a short decimal: 125 → "1.25", 300 → "3", 50 → "0.5".
pub fn hundredths(v: i32) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let v = v.abs();
    let (whole, frac) = (v / 100, v % 100);
    if frac == 0 {
        format!("{sign}{whole}")
    } else if frac % 10 == 0 {
        format!("{sign}{whole}.{}", frac / 10)
    } else {
        format!("{sign}{whole}.{frac:02}")
    }
}

/// Seconds of game time as m:ss.
pub fn clock(seconds: i32) -> String {
    let s = seconds.max(0);
    format!("{}:{:02}", s / 60, s % 60)
}

/// Fills `{key}` and `{key:form}` from `look`, which returns the per-level
/// list for a key. `level` marks the current level (0 marks none).
pub fn fill(template: &str, level: i32, look: &dyn Fn(&str) -> Option<Vec<i32>>) -> String {
    let mut out = String::new();
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('}') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let inner = &rest[open + 1..open + close];
        let (key, form) = match inner.split_once(':') {
            Some((k, f)) => (k, f),
            None => (inner, ""),
        };
        match look(key) {
            Some(list) if !list.is_empty() => {
                let shown: Vec<String> = list
                    .iter()
                    .map(|&v| match form {
                        "s" => hundredths(v),
                        "%" => format!("{v}%"),
                        "x" => format!("{}x", hundredths(v)),
                        _ => v.to_string(),
                    })
                    .collect();
                let all_same = shown.iter().all(|s| *s == shown[0]);
                if all_same {
                    out.push_str(&shown[0]);
                } else {
                    let parts: Vec<String> = shown
                        .iter()
                        .enumerate()
                        .map(|(i, s)| {
                            if level == i as i32 + 1 {
                                format!("[{s}]")
                            } else {
                                s.clone()
                            }
                        })
                        .collect();
                    out.push_str(&parts.join(" / "));
                }
            }
            _ => {
                out.push('{');
                out.push_str(inner);
                out.push('}');
            }
        }
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hundredths_read_as_short_decimals() {
        assert_eq!(hundredths(125), "1.25");
        assert_eq!(hundredths(300), "3");
        assert_eq!(hundredths(50), "0.5");
        assert_eq!(hundredths(-150), "-1.5");
    }

    #[test]
    fn placeholders_fill_every_level_and_mark_the_current_one() {
        let look = |k: &str| match k {
            "damage" => Some(vec![100, 175, 250, 325]),
            "stun" => Some(vec![125, 150]),
            "radius" => Some(vec![255]),
            _ => None,
        };
        assert_eq!(
            fill("Deals {damage} and stuns {stun:s} s in {radius}.", 2, &look),
            "Deals 100 / [175] / 250 / 325 and stuns 1.25 / [1.5] s in 255."
        );
        // an unknown key stays visible so a gap is found
        assert_eq!(fill("{nope}", 1, &look), "{nope}");
    }

    #[test]
    fn the_clock_reads_minutes_and_seconds() {
        assert_eq!(clock(0), "0:00");
        assert_eq!(clock(605), "10:05");
    }
}
