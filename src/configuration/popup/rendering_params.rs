use crate::utils::serde::{no, yes};
use serde::{Deserialize, Serialize};

const DEFAULT_MAX_CONTENT_WIDTH: f32 = 800.0;
const DEFAULT_MAX_CONTENT_HEIGHT: f32 = 350.0;

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
    pub show_images_tab: bool,

    #[serde(default = "default_blacklisted_tabs")]
    pub blacklisted_tabs: Vec<String>,

    #[serde(default = "yes")]
    pub show_tag_bar: bool,

    #[serde(default = "yes")]
    pub auto_pin_on_tab_hover: bool,

    #[serde(default = "no")]
    pub allow_popup_collapsing: bool,

    #[serde(default = "default_max_content_width")]
    pub max_content_width: f32,

    #[serde(default = "default_max_content_height")]
    pub max_content_height: f32,
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
            blacklisted_tabs: default_blacklisted_tabs(),
            show_images_tab: yes(),
            show_tag_bar: yes(),
            auto_pin_on_tab_hover: yes(),
            allow_popup_collapsing: no(),
            max_content_width: default_max_content_width(),
            max_content_height: default_max_content_height(),
        }
    }
}

fn default_blacklisted_tabs() -> Vec<String> {
    vec![
        "external links".to_string(),
        "references".to_string(),
        "trivia".to_string(),
        "dialogue".to_string(),
        "quotes".to_string(),
        "version history".to_string(),
        "gallery".to_string(),
        "gem store history".to_string(),
        "text".to_string(),
    ]
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

pub fn default_max_content_height() -> f32 {
    DEFAULT_MAX_CONTENT_HEIGHT
}
