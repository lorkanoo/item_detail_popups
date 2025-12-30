use crate::api::gw2_wiki::href_to_wiki_url;
use crate::configuration::popup::rendering_params::RenderingParams;
use crate::configuration::read_config;
use crate::render::ui::{UiAction, UiExtended, UiLink};
use crate::service::popup::{close_all_popups, copy_popup_title, process_text};
use crate::state::cache::caching_status::CachingStatus;
use crate::state::cache::{Cache, StoreInCache};
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::popup_state::PopupState;
use crate::state::popup::token::Token;
use crate::state::popup::Popup;
use log::{debug, error};
use nexus::imgui::MenuItem;
use nexus::imgui::{sys, TabBarFlags};
use nexus::imgui::{ChildWindow, MouseButton, Ui};
use std::{f32, ptr};

pub mod price;

const NON_CHILD_WINDOW_TEXT_WRAP_LIMIT: usize = 25;
const ADDITIONAL_SCROLLABLE_MARGIN_RIGHT: f32 = 45.0;

impl Context {
    pub fn render_popup_data(
        ui: &Ui,
        pinned_popup_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
    ) {
        debug!("[render_popup_data]");
        let rendering_params = read_config().rendering_params.clone();
        Self::render_title_bar(ui, popup, cache, bold_font, &rendering_params);
        if !popup.state.collapsed {
            Self::render_popup_content(
                ui,
                pinned_popup_index,
                popup,
                ui_actions,
                cache,
                bold_font,
                &rendering_params,
            );
            render_close_button(ui, pinned_popup_index, &mut popup.state);
        }
        let window_width = ui.window_size()[0];
        popup.state.width = Some(window_width);
    }

    fn crop_title_to_ui_width(ui: &Ui, title: &str, max_ui_width: f32) -> String {
        let title_size_width = ui.calc_text_size(title)[0];

        if title_size_width <= max_ui_width {
            return title.to_string();
        }

        let window_is_smaller_by_percentage = max_ui_width / title_size_width;

        let title_char_count = title.chars().count();
        let cut_off_character_count = ((title_char_count as f32 * window_is_smaller_by_percentage)
            .floor() as usize)
            .clamp(0, title_char_count);
        format!(
            "{}{}",
            title
                .chars()
                .take(cut_off_character_count)
                .collect::<String>(),
            ".."
        )
    }

    fn render_title_bar(
        ui: &Ui,
        popup: &mut Popup,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
    ) {
        let dimensions = match &popup.data.item_icon {
            Some(Token::Image(href, dimensions)) => Self::render_image(ui, href, dimensions, cache),
            _ => None,
        };
        ui.same_line();
        let mut _token = None;
        let processed_title = process_text(
            Self::crop_title_to_ui_width(ui, &popup.data.title, rendering_params.max_content_width)
                .as_str(),
        );
        if let Some(bold_font) = bold_font {
            _token = bold_font.push();
        }

        if let Some(dimensions) = dimensions {
            ui.text_vert_centered(&processed_title, &dimensions.height, &popup.state.collapsed);
        } else {
            ui.text_or_disabled(&processed_title, &popup.state.collapsed);
            ui.new_line();
        }
        if ui.is_item_clicked_with_button(MouseButton::Right) {
            copy_popup_title(popup);
        }

        if ui.is_item_hovered() {
            if ui.is_mouse_dragging(MouseButton::Left) {
                popup.state.title_dragging = true;
            }
            if ui.is_mouse_released(MouseButton::Left) {
                if popup.state.title_dragging {
                    popup.state.title_dragging = false;
                } else if rendering_params.allow_popup_collapsing {
                    popup.state.collapsed = !popup.state.collapsed;
                }
            }
        } else {
            popup.state.title_dragging = false;
        }
    }

    fn render_popup_content(
        ui: &Ui<'_>,
        pinned_popup_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache,
        bold_font: &Option<Font>,
        rendering_params: &RenderingParams,
    ) {
        debug!("[render_popup_content]");
        if rendering_params.show_tag_bar {
            Self::render_tag_bar(ui, popup, ui_actions, rendering_params);
        }
        if popup.data.is_not_empty() {
            if let Some(_token) = ui.tab_bar_with_flags(
                format!("tabs##idp{}", popup.state.id),
                TabBarFlags::FITTING_POLICY_RESIZE_DOWN,
            ) {
                Self::render_tab(
                    "General",
                    ui,
                    pinned_popup_index,
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

                for (section_name, tokens) in &popup.data.sections {
                    if rendering_params
                        .blacklisted_tabs
                        .contains(&section_name.to_lowercase())
                    {
                        continue;
                    }
                    Self::render_tab(
                        section_name,
                        ui,
                        pinned_popup_index,
                        ui_actions,
                        cache,
                        bold_font,
                        &None,
                        tokens,
                        !rendering_params.blacklisted_tabs.contains(section_name),
                        &mut popup.state,
                        rendering_params,
                        false,
                    );
                }

                Self::render_images_tab(
                    ui,
                    pinned_popup_index,
                    popup,
                    ui_actions,
                    cache,
                    rendering_params,
                );
            }
        }
        Self::render_ribbon(
            ui,
            pinned_popup_index,
            popup,
            ui_actions,
            rendering_params,
            cache,
            popup.data.item_ids.clone(),
            popup.state.item_quantity,
        );
    }

    fn render_tab(
        tab_name: &str,
        ui: &Ui<'_>,
        pinned_popup_index: Option<usize>,
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
        debug!("[render_tab] render {tab_name} tab");
        if should_render && (!tokens.is_empty() || (general_tab && item_ids.is_some())) {
            let token = ui.tab_item(format!("{tab_name}##idp{}", popup_state.id));
            if ui.is_item_hovered()
                && pinned_popup_index.is_none()
                && rendering_params.auto_pin_on_tab_hover
            {
                Self::pin_popup(ui, popup_state, ui_actions);
            }
            if token.is_some() {
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        &mut popup_state.pinned,
                        popup_state.id,
                        tab_name,
                        tokens,
                        ui_actions,
                        cache,
                        bold_font,
                        rendering_params,
                        true,
                    );
                };
                let contains_table = tokens.iter().any(|t| matches!(t, Token::Table(..)));
                if (tokens.len() > NON_CHILD_WINDOW_TEXT_WRAP_LIMIT || contains_table)
                    && !general_tab
                {
                    let cursor_pos_x = ui.cursor_pos()[0];
                    Self::next_window_size_constraints(
                        [
                            rendering_params.max_content_width - cursor_pos_x
                                + ADDITIONAL_SCROLLABLE_MARGIN_RIGHT,
                            rendering_params.max_content_height,
                        ],
                        [
                            rendering_params.max_content_width - cursor_pos_x
                                + ADDITIONAL_SCROLLABLE_MARGIN_RIGHT,
                            rendering_params.max_content_height,
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
            }
        }
    }

    fn render_images_tab(
        ui: &Ui<'_>,
        pinned_popup_index: Option<usize>,
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
            && pinned_popup_index.is_none()
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
                                            // if ui.is_item_clicked() {
                                            //     log::info!("Clicked on image: {}", href);
                                            // }
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

            let cursor_pos_x = ui.cursor_pos()[0];
            Self::next_window_size_constraints(
                [
                    rendering_params.max_content_width - cursor_pos_x,
                    rendering_params.max_content_height,
                ],
                [
                    rendering_params.max_content_width - cursor_pos_x,
                    rendering_params.max_content_height,
                ],
            );
            ChildWindow::new(format!("images_scroll##idp{}", popup.state.id).as_str())
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

    fn render_ribbon(
        ui: &Ui<'_>,
        pinned_popup_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        rendering_params: &RenderingParams,
        cache: &mut Cache,
        item_ids: Option<Vec<u32>>,
        item_quantity: usize,
    ) {
        ui.separator();
        ui.text_colored(rendering_params.link_color, "Open wiki");
        if ui.is_item_clicked() {
            let href = popup
                .data
                .redirection_href
                .as_ref()
                .unwrap_or(&popup.data.href);
            if let Err(err) = open::that_detached(href_to_wiki_url(href)) {
                error!("Failed to open wiki url: {err}");
            }
        }
        ui.same_line();
        ui.text_disabled(" | ");
        ui.same_line();
        ui.text_colored(rendering_params.link_color, "More..");
        if ui.is_item_clicked() {
            ui.open_popup(format!("##Popup_idp{}", popup.state.id));
        }
        ui.popup(format!("##Popup_idp{}", popup.state.id), || {
            if MenuItem::new(format!("Refresh##idp{}", popup.state.id)).build(ui) {
                popup.state.pos = Some(ui.window_pos());
                if let Some(index) = pinned_popup_index {
                    ui_actions.push(UiAction::Refresh(index));
                }
            }
            if ui.is_item_hovered() {
                ui.tooltip(|| {
                    let formatted_date = popup.data.cached_date.format("%d-%m-%Y %H:%M");
                    ui.text(format!("Last refreshed on {}", formatted_date));
                });
            }
            if MenuItem::new(format!("Copy name##idp{}", popup.state.id)).build(ui) {
                copy_popup_title(popup)
            }
            if MenuItem::new(format!("Close all##idp{}", popup.state.id)).build(ui) {
                close_all_popups();
            }
        });
        ui.same_line();
        Self::render_prices(ui, &item_ids, cache, rendering_params, item_quantity);
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
                if *cursor_pos.first().unwrap() + text_width > rendering_params.max_content_width {
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
    pinned_popup_index: Option<usize>,
    popup_state: &mut PopupState,
) {
    let window_width = ui.window_size()[0];
    let is_resizing = window_width != popup_state.width.unwrap_or(window_width);
    if pinned_popup_index.is_some()
        && !is_resizing
        && ui.close_button(format!("##idp_close{}", popup_state.id), &window_width)
    {
        popup_state.opened = false
    }
    if ui.is_item_clicked_with_button(MouseButton::Right) {
        close_all_popups();
    }
}
