use crate::sdk::*;
use crate::sdk::CharacterStance;
use crate::config::{Config};
use log::*;
use memlib::math::Vector2;
use serde::{Serialize, Deserialize};
use window_overlay::imgui::overlay::ImguiOverlay;
use window_overlay::types::*;
use window_overlay::color::Color;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClosestPlayerConfig {
    pub enabled: bool,
    pub ignore_downed: bool,
    pub ignore_teammates: bool,
}

impl ClosestPlayerConfig {
    pub fn default() -> Self {
        Self {
            enabled: true,
            ignore_downed: true,
            ignore_teammates: true,
        }
    }
}

pub fn closest_player(game_info: &GameInfo, global_config: &Config, overlay: &ImguiOverlay) {
    let config = &global_config.closest_player_config;
    if !config.enabled {
        return;
    }

    let players = &game_info.players;
    let local_player = game_info.get_local_player();

    let closest_player = players
        .iter()
        .filter(|player| {
            if player.id == local_player.id {
                return false;
            }
            if config.ignore_teammates && player.is_teammate(&game_info, &global_config.friends) {
                return false;
            }
            if config.ignore_downed && player.stance == CharacterStance::Downed {
                return false;
            }

            true
        })
        .map(|player| (player, units_to_m((player.origin - local_player.origin).length())))
        .min_by(|&(_, dist1), &(_, dist2)| {
            dist1.partial_cmp(&dist2).unwrap()
        });

    // If there aren't any players that fit the criteria, return
    if closest_player.is_none() {
        return
    }
    let closest_player = closest_player.unwrap();

    let player = closest_player.0;
    let distance = closest_player.1;
    let angle = memlib::math::calculate_relative_angles(&local_player.origin, &player.origin, &game_info.local_view_angles);

    overlay.draw_text(
        Vector2{x: 7.0, y: 20.0},
        &format!("Closest player: {}\t({}m),\t({})", player.name, distance.round(), -angle.yaw.round()),
        TextOptions::default().color(if distance < 50.0 { Color::from_rgb(255, 0, 0) } else { Color::from_rgb(255, 255, 255) })
    );
}