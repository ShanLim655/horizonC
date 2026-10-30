//! Local offline profiles. Names and selections are stored in the launcher's data directory;
//! game-file ownership and Microsoft/Xbox authentication are not part of this module.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Accounts {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offline: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_selected: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub looks: BTreeMap<String, crate::skins::Look>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Player {
    Offline(String),
    GameName,
}

pub fn valid_offline_name(name: &str) -> bool {
    (3..=16).contains(&name.chars().count())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

impl Accounts {
    fn path() -> PathBuf {
        crate::backend::data_dir().join("accounts.json")
    }

    pub fn load() -> Self {
        std::fs::read_to_string(Self::path())
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
        }
        let text = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;
        std::fs::write(path, text + "\n").map_err(|error| error.to_string())
    }

    pub fn player(&self) -> Player {
        self.offline_selected
            .as_ref()
            .map_or(Player::GameName, |name| Player::Offline(name.clone()))
    }

    pub fn play_as(&mut self, player: Player) -> Result<(), String> {
        match player {
            Player::Offline(name) => {
                let name = name.trim();
                if !valid_offline_name(name) {
                    return Err(format!(
                        "{name}: an offline name must be 3 to 16 ASCII letters, digits, or underscores"
                    ));
                }
                let kept = self
                    .offline
                    .iter()
                    .find(|saved| saved.eq_ignore_ascii_case(name))
                    .cloned()
                    .unwrap_or_else(|| {
                        self.offline.push(name.to_string());
                        name.to_string()
                    });
                self.offline_selected = Some(kept);
            }
            Player::GameName => self.offline_selected = None,
        }
        Ok(())
    }

    pub fn remove_offline(&mut self, name: &str, skins_dir: &Path) -> Result<(), String> {
        let before = self.offline.len();
        self.offline.retain(|saved| !saved.eq_ignore_ascii_case(name));
        if before == self.offline.len() {
            return Err(format!("No offline profile named {name}."));
        }
        self.looks.remove(&name.to_ascii_lowercase());
        crate::skins::forget(skins_dir, name);
        if self
            .offline_selected
            .as_ref()
            .is_some_and(|selected| selected.eq_ignore_ascii_case(name))
        {
            self.offline_selected = None;
        }
        Ok(())
    }

    pub fn look(&self, name: &str) -> crate::skins::Look {
        self.looks
            .get(&name.to_ascii_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    pub fn look_env(&self) -> Vec<(&'static str, String)> {
        self.name_arg().map_or_else(Vec::new, |name| {
            crate::skins::env(&crate::skins::dir(), name, &self.look(name))
        })
    }

    pub fn name_arg(&self) -> Option<&str> {
        self.offline_selected.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_names_follow_minecraft_username_rules() {
        for name in ["abc", "Builder_1", "Player1234567890"] {
            assert!(valid_offline_name(name));
        }
        for name in ["ab", "this_name_is_too_long", "has space", "has-dash", "ééé"] {
            assert!(!valid_offline_name(name));
        }
    }

    #[test]
    fn selecting_profiles_persists_the_selected_name_in_serialized_data() {
        let mut profiles = Accounts::default();
        profiles.play_as(Player::Offline("Builder_1".into())).unwrap();
        assert_eq!(profiles.player(), Player::Offline("Builder_1".into()));
        assert_eq!(profiles.name_arg(), Some("Builder_1"));
        let saved = serde_json::to_string(&profiles).unwrap();
        let loaded: Accounts = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded, profiles);
    }

    #[test]
    fn the_game_name_is_selected_when_no_offline_name_is_set() {
        let mut profiles = Accounts::default();
        profiles.play_as(Player::GameName).unwrap();
        assert_eq!(profiles.player(), Player::GameName);
        assert_eq!(profiles.name_arg(), None);
    }
}
