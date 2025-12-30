use crate::configuration::{read_config, write_config};
use crate::render::ui::UiExtended;
use crate::service::search::search_wiki;
use crate::state::context::{read_context, write_context, Context};
use crate::state::search::search_result::SearchResult::{MultipleMatches, SingleMatch};
use crate::state::threads::link::open_link_thread;
use crate::threads::lock_threads;
use nexus::imgui::{Key, Ui};
use std::thread;

const MAX_SEARCH_RESULTS: usize = 5;
const SEARCH_POS_OFFSET_X: f32 = 40.0;

impl Context {
    pub fn render_search_prompt(&mut self, ui: &Ui) {
        let mut should_focus_input = false;
        if self.ui.should_open_search_prompt {
            let mut search_position = ui.io().mouse_pos;
            search_position[0] += SEARCH_POS_OFFSET_X;
            self.ui.search_position = search_position;
            ui.set_next_window_pos(search_position);
            ui.open_popup("##Search_prompt_idp");
            self.ui.should_open_search_prompt = false;
            self.ui.search_popup_input = "".to_string();
            should_focus_input = true;
        }
        ui.popup("##Search_prompt_idp", || {
            if should_focus_input {
                ui.set_keyboard_focus_here();
            }
            ui.input_text("##search_input_idp", &mut self.ui.search_popup_input)
                .build();
            ui.same_line();
            let mut should_search = ui.button("Search") && !self.ui.search_popup_input.is_empty();

            if !self.ui.search_popup_input.is_empty() {
                let needle = self.ui.search_popup_input.as_str();
                let link_color = read_config().rendering_params.link_color;
                for entry in read_config()
                    .search_params
                    .search_history
                    .find_containing(needle, MAX_SEARCH_RESULTS)
                {
                    ui.text_colored(link_color, entry);
                    if ui.is_item_clicked() {
                        self.ui.search_popup_input = entry.clone();
                        should_search = true;
                    }
                }
            }

            if ui.is_key_released(Key::Enter) || should_search {
                write_config()
                    .search_params
                    .search_history
                    .push(self.ui.search_popup_input.clone());
                ui.close_current_popup();
                lock_threads().push(thread::spawn(move || {
                    write_context().ui.loading_progress = Some(1);
                    let item_name = read_context().ui.search_popup_input.clone();
                    write_context().ui.search_popup_input = "".to_string();
                    write_context().ui.search_result = Some(search_wiki(item_name.as_str()));
                    write_context().ui.should_open_search_result = true;
                    write_context().ui.loading_progress = None;
                }));
            }
        });
    }

    pub fn render_search_result(&mut self, ui: &Ui) {
        let Some(search_result) = &self.ui.search_result else {
            return;
        };

        match search_result {
            SingleMatch(popup) => {
                self.ui.hovered_popup = Some(popup.clone());
                self.ui.search_result = None;
            }
            MultipleMatches(matches) => {
                if self.ui.should_open_search_result {
                    ui.set_next_window_pos(self.ui.search_position);
                    ui.open_popup("##Search_result_idp");
                    self.ui.should_open_search_result = false;
                }
                ui.popup("##Search_result_idp", || {
                    if matches.is_empty() {
                        ui.text("No results found.");
                        return;
                    }
                    ui.text_disabled("Did you mean:");
                    ui.spacing();
                    let link_color = read_config().rendering_params.link_color;
                    for entry in matches {
                        ui.text_colored(link_color, &entry.text);
                        if ui.is_item_clicked() {
                            ui.close_current_popup();
                            open_link_thread(entry.href.clone(), entry.text.clone());
                            write_config()
                                .search_params
                                .search_history
                                .push(entry.text.clone());
                        }
                    }
                });
            }
        }
    }
}
