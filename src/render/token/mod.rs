mod text;
mod tag;
mod list;
mod image;
mod table;

use crate::configuration::popup::rendering_params::RenderingParams;
use crate::render::ui::UiAction;
use crate::state::cache::Cache;
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::token::Token;
use log::debug;
use nexus::imgui::Ui;
use nexus::imgui::StyleVar::ItemSpacing;

const INDENT: &'static str = "    ";

impl Context {
    pub fn render_tokens(
        ui: &Ui,
        pinned: &mut bool,
        popup_id: u64,
        section_label: &str,
        tokens: &Vec<Token>,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
        render_tables: bool,
    ) {
        debug!("[render_tokens]");
        let item_spacing_style = ui.push_style_var(ItemSpacing([0.0, 5.0]));
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
                    ui.text(" ");
                    ui.same_line();
                    let _ = Self::render_image(ui, href, dimensions, cache);
                }
                Token::Table(table_params) => {
                    if render_tables {
                        Self::render_table(
                            ui,
                            pinned,
                            popup_id,
                            section_label,
                            ui_actions,
                            cache,
                            bold_font,
                            rendering_params,
                            table_params,
                        )
                    }
                }
            }
            last_token = Some(token);
        }
        item_spacing_style.pop();
    }

    fn add_indent(ui: &Ui, current_indent: i32) {
        if current_indent >= 0 {
            ui.text(INDENT.repeat(current_indent as usize));
            ui.same_line();
        }
    }
}
