use super::yes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingParams {
    #[serde(default = "default_link_color")]
    pub link_color: [f32; 4],
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
}

impl Default for RenderingParams {
    fn default() -> Self {
        Self {
            link_color: default_link_color(),
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
        }
    }
}

fn default_link_color() -> [f32; 4] {
    [0.2, 0.4, 0.8, 1.0]
}


