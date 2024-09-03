use std::{
    error::Error,
    fs::{write, File},
    io::BufReader,
    path::{Path, PathBuf},
    sync::Mutex,
};

use tauri::{api::dialog::blocking::FileDialogBuilder, State};

use crate::library::{GenresState, IdState, LibraryState, LocationsState, Media, NamesState};

pub struct UnsavedState {
    pub unsaved: Mutex<bool>,
}

#[tauri::command]
pub fn load_library_file(
    library: State<'_, LibraryState>,
    id_state: State<'_, IdState>,
    unsaved_state: State<'_, UnsavedState>,
    all_names: State<'_, NamesState>,
    all_genres: State<'_, GenresState>,
    all_locations: State<'_, LocationsState>,
) {
    let lib = match FileDialogBuilder::new()
        .add_filter("JSON files", &["json"])
        .pick_file()
    {
        Some(fp) => read_lib_from_file(fp).unwrap_or_default(),
        None => Vec::<Media>::default(),
    };
    *library.medias.lock().unwrap() = lib;

    *id_state.free_id.lock().unwrap() = library
        .medias
        .lock()
        .unwrap()
        .iter()
        .max_by_key(|m| m.id)
        .unwrap_or(&Media::new(
            0,
            String::new(),
            String::new(),
            String::new(),
            0,
            0,
            String::new(),
            String::new(),
        ))
        .id
        + 1;

    *unsaved_state.unsaved.lock().unwrap() = false;

    for media in &*library.medias.lock().unwrap() {
        all_names.names.lock().unwrap().insert(media.author.clone());
        all_genres
            .genres
            .lock()
            .unwrap()
            .insert(media.genre.clone());
        all_locations
            .locations
            .lock()
            .unwrap()
            .insert(media.location.clone());
    }
}

fn read_lib_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Media>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let l = serde_json::from_reader(reader)?;

    Ok(l)
}

#[tauri::command]
pub fn save_lib_to_file(library: State<LibraryState>, unsaved_state: State<UnsavedState>) {
    let path = match FileDialogBuilder::new()
        .set_file_name("library.json")
        .save_file()
    {
        Some(path) => path,
        None => PathBuf::from(""), // this ensures failure at the next step if the user closed the file dialog
    };
    println!(
        "Saving status : {:?}",
        write(
            path,
            serde_json::to_string_pretty(&*library.medias.lock().unwrap()).unwrap()
        )
    );

    *unsaved_state.unsaved.lock().unwrap() = false;
}

#[tauri::command]
pub fn get_unsaved_status(unsaved_state: State<UnsavedState>) -> bool {
    return *unsaved_state.unsaved.lock().unwrap();
}
