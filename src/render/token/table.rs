use crate::configuration::popup::rendering_params::RenderingParams;
use crate::render::ui::UiAction;
use crate::state::cache::Cache;
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::table_params::TableParams;
use log::debug;
use nexus::imgui::{TableFlags, Ui};

impl Context {
    pub fn render_table(
        ui: &Ui,
        pinned: &mut bool,
        popup_id: u64,
        section_label: &str,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
        table_params: &TableParams,
    ) {
        debug!("[render_table] {table_params:?}");
        if let Some(_t) = ui.begin_table_with_flags(
            format!(
                "table_{}_{}_{}##idp",
                table_params.uuid, popup_id, section_label
            ),
            table_params.headers.len(),
            TableFlags::RESIZABLE | TableFlags::NO_SAVED_SETTINGS,
        ) {
            debug!("[render_table setup headers]");
            for header in &table_params.headers {
                ui.table_setup_column(header);
            }
            ui.table_headers_row();
            Self::render_table_rows(
                ui,
                pinned,
                popup_id,
                section_label,
                ui_actions,
                cache,
                bold_font,
                rendering_params,
                table_params,
            );
        }
    }

    fn render_table_rows(
        ui: &Ui,
        pinned: &mut bool,
        popup_id: u64,
        section_label: &str,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
        table_params: &TableParams,
    ) {
        debug!("[render_table_rows]");
        for row in &table_params.rows {
            ui.table_next_row();
            for cell in &row.cells {
                ui.table_next_column();
                debug!("[render_table_rows] recursion");
                Self::render_tokens(
                    ui,
                    pinned,
                    popup_id,
                    section_label,
                    &cell.tokens,
                    ui_actions,
                    cache,
                    bold_font,
                    rendering_params,
                    false,
                );
                debug!("[render_table_rows] recursion end");
            }
        }
    }
}
