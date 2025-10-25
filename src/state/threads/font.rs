use crate::configuration::config::fonts_dir;

use crate::configuration::config::read_config;
use crate::state::context::write_context;
use crate::state::font::Font;
use log::{debug, error, info};
use nexus::font::{add_font_from_file, RawFontReceive};
use nexus::font_receive;
use nexus::imgui::sys::{self, ImFontConfig};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn load_fonts() {
    let bold_font_bytes = include_bytes!("../../../fonts/default_bold.ttf");
    let mut bold_font_path = fonts_dir();
    let _ = fs::create_dir(&bold_font_path);
    bold_font_path.push("default_bold.ttf");
    if !bold_font_path.exists() {
        let mut bold_font_file =
            File::create(&bold_font_path).expect("Couldn't create a bold font file.");
        bold_font_file
            .write_all(bold_font_bytes)
            .expect("Couldn't write to bold font file.");
    }
    let entries = fs::read_dir(fonts_dir());
    if entries.is_err() {
        error!("[load_fonts] Couldn't load fonts");
        return;
    }

    let mut loaded_count = 0;
    let font_size = unsafe { sys::igGetFontSize() };

    for entry in entries.unwrap() {
        if entry.is_err() {
            error!("[load_fonts] Couldn't process entry");
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if let (Some(extension), Some(filestem)) = (path.extension(), path.file_stem()) {
            if extension == "ttf" {
                // #[cfg(not(debug_assertions))] {
                let font_receive: RawFontReceive = font_receive!(|id, _font| {
                    debug!("Font loaded: {id}");
                });
                let config: &mut ImFontConfig = unsafe { &mut *std::ptr::null_mut() };
                let filename = filestem.to_string_lossy();
                add_font_from_file(
                    format!("IDP_{filename}"),
                    path,
                    font_size,
                    config,
                    font_receive,
                )
                .revert_on_unload();
                // }

                loaded_count += 1;
            }
        }
    }
    info!("[load_fonts] Loaded {} fonts", loaded_count);
}

pub(crate) fn preselect_fonts() {
    for font in unsafe { Font::get_all() } {
        unsafe {
            if let (Ok(font_name), Some(selected_bold_font_name)) = (
                font.name_raw().to_str(),
                read_config().selected_bold_font_name.clone(),
            ) {
                if font_name == selected_bold_font_name {
                    write_context().bold_font = Some(font);
                }
            }
        }
    }
}
