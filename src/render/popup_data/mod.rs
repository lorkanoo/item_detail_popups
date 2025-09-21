use crate::api::gw2_wiki::href_to_wiki_url;
use crate::state::cache::cache::{Cache, StoreInCache};
use crate::state::cache::caching_status::CachingStatus;
use crate::configuration::popup::rendering_params::RenderingParams;
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::popup_state::PopupState;
use crate::state::popup::token::Token;
use crate::state::popup::Popup;
use nexus::imgui::sys;
use nexus::imgui::MenuItem;
use nexus::imgui::{ChildWindow, MouseButton, Ui};
use std::ptr;
use std::thread;
use crate::configuration::config::read_config;
use crate::state::context::write_context;
use crate::core::threads::lock_threads;
use crate::core::utils::ui::{UiAction, UiExtended, UiLink};

pub mod price;

const NON_CHILD_WINDOW_TEXT_WRAP_LIMIT: usize = 25;

impl Context {
    pub fn render_popup_data(
        ui: &Ui,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
    ) {
        let rendering_params = read_config().rendering_params.clone();
        Self::render_title_bar(ui, popup, cache, bold_font);
        if !popup.state.collapsed {
            Self::render_content(
                ui,
                pinned_popup_vec_index,
                popup,
                ui_actions,
                cache,
                bold_font,
                &rendering_params,
            );
            render_close_button(
                ui,
                pinned_popup_vec_index,
                &mut popup.state,
                rendering_params,
            );
        }
        let window_width = ui.window_size()[0];
        popup.state.width = Some(window_width);
    }

    fn render_title_bar(ui: &Ui, popup: &mut Popup, cache: &mut Cache, bold_font: &Option<Font>) {
        let dimensions = match &popup.data.item_icon {
            Some(Token::Image(href, dimensions)) => Self::render_image(ui, href, dimensions, cache),
            _ => None,
        };
        ui.same_line();
        if let Some(bold_font) = bold_font {
            let _token = bold_font.push();
            if let Some(dimensions) = dimensions {
                ui.text_vert_centered(
                    &popup.data.title,
                    &dimensions.height,
                    &popup.state.collapsed,
                );
            } else {
                ui.text_or_disabled(&popup.data.title, &popup.state.collapsed);
                ui.spacing();
            }
        } else if let Some(dimensions) = dimensions {
            ui.text_vert_centered(
                &popup.data.title,
                &dimensions.height,
                &popup.state.collapsed,
            );
        } else {
            ui.text_or_disabled(&popup.data.title, &popup.state.collapsed);
            ui.spacing();
        }

        if ui.is_item_hovered() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                popup.state.title_dragging = true;
            }
            if ui.is_mouse_released(MouseButton::Left) {
                if popup.state.title_dragging {
                    popup.state.title_dragging = false;
                } else {
                    popup.state.collapsed = !popup.state.collapsed;
                }
            }
        } else {
            popup.state.title_dragging = false;
        }
    }

    fn render_content(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
    ) {
        if rendering_params.show_tag_bar {
            Self::render_tag_bar(ui, popup, ui_actions, rendering_params);
        }
        if popup.data.is_not_empty() {
            if let Some(_token) = ui.tab_bar(format!("tabs##rps{}", popup.state.id)) {
                Self::render_tab(
                    "General",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &popup.data.item_ids,
                    &popup.data.description,
                    rendering_params.show_general_tab,
                    &mut popup.state,
                    rendering_params,
                    true,
                );
                Self::render_tab(
                    "Acquisition",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.acquisition,
                    rendering_params.show_acquisition_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Teaches recipe",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.teaches_recipe,
                    rendering_params.show_teaches_recipe_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Getting there",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.getting_there,
                    rendering_params.show_getting_there_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Location",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.location,
                    rendering_params.show_location_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Walkthrough",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.walkthrough,
                    rendering_params.show_walkthrough_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Rewards",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.rewards,
                    rendering_params.show_rewards_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Related achievements",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.related_achievements,
                    rendering_params.show_related_achievements_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Contents",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.contents,
                    rendering_params.show_contents_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_tab(
                    "Notes",
                    ui,
                    pinned_popup_vec_index,
                    ui_actions,
                    cache,
                    bold_font,
                    &None,
                    &popup.data.notes,
                    rendering_params.show_notes_tab,
                    &mut popup.state,
                    rendering_params,
                    false,
                );
                Self::render_images_tab(
                    ui,
                    pinned_popup_vec_index,
                    popup,
                    ui_actions,
                    cache,
                    rendering_params,
                );
            }
        }
        Self::render_button_ribbon(ui, pinned_popup_vec_index, popup, ui_actions);
    }

    fn render_tab(
        tab_name: &str,
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        item_ids: &Option<Vec<u32>>,
        tokens: &Vec<Token>,
        should_render: bool,
        popup_state: &mut PopupState,
        rendering_params: &RenderingParams,
        general_tab: bool,
    ) {
        if should_render && (!tokens.is_empty() || (general_tab && item_ids.is_some())) {
            let token = ui.tab_item(format!("{tab_name}##idp{}", popup_state.id));
            if ui.is_item_hovered()
                && pinned_popup_vec_index.is_none()
                && rendering_params.auto_pin_on_tab_hover
            {
                Self::pin_popup(ui, popup_state, ui_actions);
            }
            if token.is_some() {
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        &mut popup_state.pinned,
                        tokens,
                        ui_actions,
                        cache,
                        bold_font,
                        rendering_params,
                    );
                };
                if tokens.len() > NON_CHILD_WINDOW_TEXT_WRAP_LIMIT && !general_tab {
                    let screen_height = ui.io().display_size[1];
                    let cursor_pos_x = ui.cursor_pos()[0];
                    Self::next_window_size_constraints(
                        [
                            rendering_params.max_content_width - cursor_pos_x,
                            screen_height * 0.15,
                        ],
                        [
                            rendering_params.max_content_width - cursor_pos_x,
                            screen_height * 0.15,
                        ],
                    );
                    ChildWindow::new(format!("{tab_name}_scroll##idp{}", popup_state.id).as_str())
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, render_func);
                    return;
                }
                render_func();
                if !tokens.is_empty() {
                    ui.new_line();
                }
                if item_ids.is_some() {
                    Self::render_prices(ui, item_ids, cache, rendering_params);
                }
            }
        }
    }

    fn render_images_tab(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        rendering_params: &RenderingParams,
    ) {
        if !rendering_params.show_images_tab || popup.data.images.is_empty() {
            return;
        }
        let token = ui.tab_item(format!("Images##idp{}", popup.state.id));
        if ui.is_item_hovered()
            && pinned_popup_vec_index.is_none()
            && rendering_params.auto_pin_on_tab_hover
        {
            Self::pin_popup(ui, &mut popup.state, ui_actions);
        }
        if token.is_some() {
            let mut render_func = || {
                for token in &popup.data.images {
                    match token {
                        Token::Image(href, _) => {
                            let cached_data_opt = cache.textures.retrieve(href.to_string());
                            if let Some(cached_data) = cached_data_opt {
                                if let CachingStatus::Cached = cached_data.caching_status {
                                    if let Some(texture) = cached_data.value() {
                                        let window_width = ui.window_size()[0];
                                        let start_offset =
                                            window_width / 2.0 - texture.width as f32 / 2.0;
                                        ui.set_cursor_pos([start_offset, ui.cursor_pos()[1]]);
                                        ui.invisible_button(
                                            href,
                                            [texture.width as f32, texture.height as f32],
                                        );
                                        if !ui.not_in_view(&(texture.height as f32)) {
                                            ui.get_window_draw_list()
                                                .add_image(
                                                    texture.id(),
                                                    ui.item_rect_min(),
                                                    ui.item_rect_max(),
                                                )
                                                .build();
                                        }
                                        ui.spacing();
                                    }
                                }
                            }
                        }
                        Token::Text(text, _) => {
                            let words: Vec<&str> = text.split_whitespace().collect();
                            for chunk in words.chunks(4) {
                                let text = chunk.join(" ");
                                let text_size = ui.calc_text_size(&text)[0];
                                let window_width = ui.window_size()[0];
                                ui.set_cursor_pos([
                                    window_width / 2.0 - text_size / 2.0,
                                    ui.cursor_pos()[1],
                                ]);
                                ui.text(text);
                            }
                            ui.spacing();
                        }
                        _ => {}
                    }
                }
                ui.new_line();
            };

            let count = popup
                .data
                .images
                .iter()
                .filter(|item| matches!(item, Token::Image(..)))
                .count();
            if count <= 1 {
                render_func();
                return;
            }

            let screen_height = ui.io().display_size[1];
            Self::next_window_size_constraints(
                [100.0, screen_height * 0.05],
                [400.0, screen_height * 0.30],
            );
            ChildWindow::new(format!("notes_scroll##rps{}", popup.state.id).as_str())
                .border(true)
                .scroll_bar(true)
                .build(ui, render_func);
        }
    }

    pub fn next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2]) {
        unsafe {
            sys::igSetNextWindowSizeConstraints(
                size_min.into(),
                size_max.into(),
                None,
                ptr::null_mut(),
            )
        }
    }

    fn render_button_ribbon(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
    ) {
        if let Some(index) = pinned_popup_vec_index {
            ui.spacing();

            if ui.button(format!("Open wiki##idp{}", popup.state.id)) {
                let href = popup
                    .data
                    .redirection_href
                    .as_ref()
                    .unwrap_or(&popup.data.href);
                if let Err(err) = open::that_detached(href_to_wiki_url(href)) {
                    log::error!("Failed to open wiki url: {err}");
                }
            }

            ui.same_line();

            if ui.button(format!("More..##idp{}", popup.state.id)) {
                ui.open_popup(format!("##Popup_idp{}", popup.state.id));
            }
            ui.popup(format!("##Popup_idp{}", popup.state.id), || {
                if MenuItem::new(format!("Refresh##idp{}", popup.state.id)).build(ui) {
                    popup.state.pos = Some(ui.window_pos());
                    ui_actions.push(UiAction::Refresh(index));
                }
                if ui.is_item_hovered() {
                    ui.tooltip(|| {
                        let formatted_date = popup.data.cached_date.format("%d-%m-%Y %H:%M");
                        ui.text(format!("Last refreshed on {}", formatted_date));
                    });
                }
                if MenuItem::new(format!("Copy name##idp{}", popup.state.id)).build(ui) {
                    let name = popup.data.title.clone();
                    lock_threads().push(thread::spawn(move || {
                        let _ = write_context().clipboard.set_text(name.as_str());
                    }));
                }
            });
        }
    }

    fn render_tag_bar(
        ui: &Ui<'_>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        rendering_params: &RenderingParams,
    ) {
        let tag_iterator = popup.data.tags.iter_mut().enumerate().peekable();
        if tag_iterator.len() > 0 {
            for (_, tag) in tag_iterator {
                let cursor_pos: [f32; 2] = ui.cursor_pos();
                let text_width = ui.calc_text_size(&tag.1)[0];
                if *cursor_pos.first().unwrap() + text_width + rendering_params.content_margin_right
                    > rendering_params.max_content_width
                {
                    ui.new_line();
                }
                ui.text_colored(rendering_params.link_color, format!("[{}]", tag.1));
                if ui.is_item_hovered()
                    && ui.is_mouse_released(MouseButton::Left)
                    && popup.state.pinned
                {
                    ui_actions.push(UiAction::Open(UiLink {
                        href: tag.0.clone(),
                        title: tag.1.clone(),
                    }));
                }
                ui.same_line();
            }
            ui.new_line();
        }
    }
}

fn render_close_button(
    ui: &Ui<'_>,
    pinned_popup_vec_index: Option<usize>,
    popup_state: &mut PopupState,
    rendering_params: RenderingParams,
) {
    let window_width = ui.window_size()[0];
    let is_resizing = window_width != popup_state.width.unwrap_or(window_width);
    if pinned_popup_vec_index.is_some()
        && !is_resizing
        && ui.close_button(
            format!("##idp_close{}", popup_state.id),
            &rendering_params.max_content_width,
        )
    {
        popup_state.opened = false
    }
}
