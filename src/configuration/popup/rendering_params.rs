use serde::{Deserialize, Serialize};
use crate::core::utils::serde::yes;

const DEFAULT_MAX_CONTENT_WIDTH: f32 = 700.0;
const DEFAULT_CONTENT_MARGIN_RIGHT: f32 = 20.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingParams {
    #[serde(default = "default_link_color")]
    pub link_color: [f32; 4],

    #[serde(default = "default_gold_coin_color")]
    pub gold_coin_color: [f32; 4],

    #[serde(default = "default_silver_coin_color")]
    pub silver_coin_color: [f32; 4],

    #[serde(default = "default_copper_coin_color")]
    pub copper_coin_color: [f32; 4],

    #[serde(default = "yes")]
    pub use_bullet_list_punctuation: bool,

    #[serde(default = "yes")]
    pub show_general_tab: bool,

    #[serde(default = "yes")]
    pub show_acquisition_tab: bool,

    #[serde(default = "yes")]
    pub show_teaches_recipe_tab: bool,

    #[serde(default = "yes")]
    pub show_getting_there_tab: bool,

    #[serde(default = "yes")]
    pub show_walkthrough_tab: bool,

    #[serde(default = "yes")]
    pub show_location_tab: bool,

    #[serde(default = "yes")]
    pub show_rewards_tab: bool,

    #[serde(default = "yes")]
    pub show_related_achievements_tab: bool,

    #[serde(default = "yes")]
    pub show_contents_tab: bool,

    #[serde(default = "yes")]
    pub show_notes_tab: bool,

    #[serde(default = "yes")]
    pub show_images_tab: bool,

    #[serde(default = "yes")]
    pub show_tag_bar: bool,

    #[serde(default = "yes")]
    pub auto_pin_on_tab_hover: bool,

    #[serde(default = "default_max_content_width")]
    pub max_content_width: f32,

    #[serde(default = "default_content_margin_right")]
    pub content_margin_right: f32,
}

impl Default for RenderingParams {
    fn default() -> Self {
        Self {
            link_color: default_link_color(),
            gold_coin_color: default_gold_coin_color(),
            silver_coin_color: default_silver_coin_color(),
            copper_coin_color: default_copper_coin_color(),
            use_bullet_list_punctuation: yes(),
            show_general_tab: yes(),
            show_acquisition_tab: yes(),
            show_teaches_recipe_tab: yes(),
            show_getting_there_tab: yes(),
            show_contents_tab: yes(),
            show_notes_tab: yes(),
            show_location_tab: yes(),
            show_walkthrough_tab: yes(),
            show_rewards_tab: yes(),
            show_related_achievements_tab: yes(),
            show_images_tab: yes(),
            show_tag_bar: yes(),
            auto_pin_on_tab_hover: yes(),
            max_content_width: default_max_content_width(),
            content_margin_right: default_content_margin_right(),
        }
    }
}

fn default_link_color() -> [f32; 4] {
    [0.2, 0.4, 0.8, 1.0]
}

fn default_gold_coin_color() -> [f32; 4] {
    [1.0, 0.843, 0.0, 1.0]
}

fn default_silver_coin_color() -> [f32; 4] {
    [0.75, 0.75, 0.75, 1.0]
}

fn default_copper_coin_color() -> [f32; 4] {
    [0.72, 0.45, 0.20, 1.0]
}

pub fn default_max_content_width() -> f32 {
    DEFAULT_MAX_CONTENT_WIDTH
}
pub fn default_content_margin_right() -> f32 {
    DEFAULT_CONTENT_MARGIN_RIGHT
}
