#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(clippy::too_many_arguments)]

mod library;
mod saving;

use library::{GenresState, IdState, LibraryState, LocationsState, NamesState};
use saving::UnsavedState;
use std::{collections::HashSet, sync::Mutex};

#[tauri::command]
fn get_false() -> bool {
    false
}

pub fn main() {
    tauri::Builder::default()
        .manage(LibraryState {
            medias: Mutex::new(Vec::new()),
        })
        .manage(IdState {
            free_id: Mutex::new(0),
        })
        .manage(UnsavedState {
            unsaved: Mutex::new(false),
        })
        .manage(NamesState {
            names: Mutex::new(HashSet::new()),
        })
        .manage(GenresState {
            genres: Mutex::new(HashSet::new()),
        })
        .manage(LocationsState {
            locations: Mutex::new(HashSet::new()),
        })
        .invoke_handler(tauri::generate_handler![
            saving::load_library_file,
            saving::save_lib_to_file,
            library::get_empty,
            get_false,
            library::get_all_medias,
            library::get_filtered_medias,
            saving::get_unsaved_status,
            library::get_filtered_names,
            library::get_filtered_genres,
            library::get_filtered_locations,
            library::get_sorted_medias,
            library::add_media,
            library::edit_media,
            library::remove_media,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
