use crate::configuration::popup::rendering_params::RenderingParams;
use crate::render::ui::HIGHLIGHT_COLOR;
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::style::Style;
use nexus::imgui::Ui;

impl Context {
    pub fn render_text(
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

    pub fn render_words<F>(
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

    fn handle_line_wrap(
        ui: &Ui,
        current_indent: i32,
        word_width: f32,
        rendering_params: &RenderingParams,
    ) {
        let cursor_pos = ui.cursor_pos();
        if cursor_pos[0] + word_width > rendering_params.max_content_width {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
    }
}
