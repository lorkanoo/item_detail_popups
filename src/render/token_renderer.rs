use crate::addon::Addon;
use crate::cache::Cache;
use crate::cache::Cacheable;
use crate::cache::CachingStatus;
use crate::config::rendering_params::RenderingParams;
use crate::context::font::Font;
use crate::context::ui::popup::dimensions::Dimensions;
use crate::context::ui::popup::style::Style;
use crate::context::ui::popup::tag_params::TagParams;
use crate::context::ui::popup::token::Token;
use crate::context::Context;
use crate::render::util::ui::{UiAction, HIGHLIGHT_COLOR};
use nexus::imgui::{MouseButton, Ui};
use util::ui::UiLink;

use super::util;
use super::util::ui::extended::UiExtended;

impl Context {
    pub fn get_rendering_params() -> RenderingParams {
        Addon::read_config().rendering_params.clone()
    }

    pub fn render_tokens(
        ui: &Ui,
        pinned: &mut bool,
        tokens: &Vec<Token>,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
    ) {
        let item_spacing_style = ui.push_style_var(nexus::imgui::StyleVar::ItemSpacing([0.0, 5.0]));
        ui.spacing();
        let mut starts_with_list = tokens
            .first()
            .is_some_and(|t| matches!(t, Token::ListElement) || matches!(t, Token::Indent(_)));
        let mut current_indent = -1;
        let mut last_token: Option<&Token> = None;
        for token in tokens {
            if !matches!(last_token, Some(Token::Spacing)) {
                ui.same_line();
            }
            match token {
                Token::Indent(depth) => current_indent = *depth,
                Token::Spacing => {
                    ui.spacing();
                }
                Token::Text(text, style) => {
                    Self::render_text(ui, text, style, current_indent, rendering_params, bold_font)
                }
                Token::Tag(tag_params) => {
                    Self::render_tag(
                        ui,
                        tag_params,
                        pinned,
                        ui_actions,
                        current_indent,
                        rendering_params,
                    );
                }
                Token::ListElement => Self::render_list_element(
                    ui,
                    &mut starts_with_list,
                    current_indent,
                    rendering_params.use_bullet_list_punctuation,
                ),
                Token::Image(href, dimensions) => {
                    let _ = Self::render_image(ui, href, dimensions, cache);
                }
            }
            last_token = Some(token);
        }
        item_spacing_style.pop();
    }

    fn render_words<F>(
        ui: &Ui,
        text: &str,
        current_indent: i32,
        rendering_params: &RenderingParams,
        mut render_word: F,
    ) where
        F: FnMut(&Ui, &str),
    {
        let mut first_word = true;
        for word in text.split(" ") {
            if word.is_empty() {
                continue;
            }
            let word_width = ui.calc_text_size(word)[0];
            let final_word = if [".", ",", ":"].iter().any(|s| word.starts_with(s)) {
                if first_word {
                    first_word = false;
                } else {
                    ui.same_line();
                }
                word.to_string()
            } else {
                Self::handle_line_wrap(ui, current_indent, word_width, rendering_params);
                format!(" {}", word)
            };
            render_word(ui, final_word.as_str());
            ui.same_line();
        }
    }

    fn render_text(
        ui: &Ui,
        text: &str,
        style: &Style,
        current_indent: i32,
        rendering_params: &RenderingParams,
        bold_font: &Option<Font>,
    ) {
        Self::render_words(
            ui,
            text,
            current_indent,
            rendering_params,
            |ui, word| match style {
                Style::Normal => ui.text(word),
                Style::Bold => {
                    if let Some(bold_font) = bold_font {
                        let token = bold_font.push();
                        ui.text(word);
                        drop(token);
                    } else {
                        ui.text_colored(HIGHLIGHT_COLOR, word);
                    }
                }
                Style::Disabled => ui.text_disabled(word),
            },
        );
    }

    pub fn render_image(
        ui: &Ui,
        href: &str,
        dimensions: &Option<Dimensions>,
        cache: &mut Cache,
    ) -> Option<Dimensions> {
        if let Some(output) = dimensions
            .as_ref()
            .filter(|d| ui.not_in_view(&d.height))
            .map(|d| Self::render_dummy(ui, d, href))
        {
            return output;
        }

        let cached_data_opt = cache.textures.retrieve(href.to_string());
        if let Some(cached_data) = cached_data_opt {
            match cached_data.caching_status {
                CachingStatus::Cached => {
                    if let Some(texture) = cached_data.value() {
                        let (width, height) = match dimensions {
                            Some(d) => d.tuple(),
                            None => (texture.width as f32, texture.height as f32),
                        };
                        ui.invisible_button(href, [width, height]);
                        ui.get_window_draw_list()
                            .add_image(texture.id(), ui.item_rect_min(), ui.item_rect_max())
                            .build();
                        return Some(Dimensions::new(width, height));
                    }
                }
                _ => {
                    return dimensions
                        .as_ref()
                        .and_then(|d| Self::render_dummy(ui, d, href))
                }
            }
        }
        None
    }

    fn render_dummy(ui: &Ui, dimensions: &Dimensions, href: &str) -> Option<Dimensions> {
        let (width, height) = dimensions.tuple();
        ui.invisible_button(href, [width, height]);
        Some(dimensions.clone())
    }

    fn render_tag(
        ui: &Ui,
        tag_params: &TagParams,
        pinned: &mut bool,
        ui_actions: &mut Vec<UiAction>,
        current_indent: i32,
        rendering_params: &RenderingParams,
    ) {
        let href = tag_params.href.to_string();
        let title = tag_params.title.to_string();
        Self::render_words(
            ui,
            &tag_params.text,
            current_indent,
            rendering_params,
            |ui: &Ui<'_>, word| {
                if ui.not_in_view(&30.0) {
                    return;
                }
                ui.text_colored(rendering_params.link_color, word);
                if ui.is_item_hovered() && ui.is_mouse_released(MouseButton::Left) && *pinned {
                    ui_actions.push(UiAction::Open(UiLink {
                        title: title.clone(),
                        href: href.clone(),
                    }));
                }
            },
        );
    }

    fn render_list_element(
        ui: &Ui,
        starts_with_list: &mut bool,
        current_indent: i32,
        use_bullet_list_punctuation: bool,
    ) {
        if !*starts_with_list {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
        *starts_with_list = false;
        if use_bullet_list_punctuation {
            ui.text(if current_indent.eq(&0) { "• " } else { "- " });
        } else {
            ui.text("- ");
        }
    }

    fn handle_line_wrap(
        ui: &Ui,
        current_indent: i32,
        word_width: f32,
        rendering_params: &RenderingParams,
    ) {
        let cursor_pos = ui.cursor_pos();
        if cursor_pos[0] + word_width + rendering_params.content_margin_right
            > rendering_params.max_content_width
        {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
    }

    fn add_indent(ui: &Ui, current_indent: i32) {
        if current_indent >= 0 {
            ui.text("    ".repeat(current_indent as usize));
            ui.same_line();
        }
    }
}
