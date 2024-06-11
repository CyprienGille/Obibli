#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    error::Error,
    fs::{write, File},
    io::BufReader,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{api::dialog::blocking::FileDialogBuilder, State};

struct IdState {
    free_id: Mutex<u64>,
}

struct UnsavedState {
    unsaved: Mutex<bool>,
}

#[derive(Debug)]
struct NamesState {
    names: Mutex<HashSet<String>>,
}
#[derive(Debug)]
struct GenresState {
    genres: Mutex<HashSet<String>>,
}
#[derive(Debug)]
struct LocationsState {
    locations: Mutex<HashSet<String>>,
}

#[derive(Debug)]
struct LibraryState {
    medias: Mutex<Vec<Media>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Media {
    id: u64,
    title: String,
    author: String,
    genre: String,
    copies: u64,
    year: i64,
    location: String,
    notes: String,
}

impl Media {
    fn new(
        id: u64,
        title: String,
        author: String,
        genre: String,
        copies: u64,
        year: i64,
        location: String,
        notes: String,
    ) -> Self {
        Self {
            id,
            title,
            author,
            genre,
            copies,
            year,
            location,
            notes,
        }
    }
}

#[tauri::command]
fn add_media(
    title: String,
    author: String,
    genre: String,
    copies: u64,
    year: i64,
    location: String,
    notes: String,
    library: State<LibraryState>,
    id_state: State<IdState>,
    unsaved_state: State<UnsavedState>,
    all_names: State<NamesState>,
    all_genres: State<GenresState>,
    all_locations: State<LocationsState>,
) -> Result<&'static str, &'static str> {
    library.medias.lock().unwrap().push(Media::new(
        *id_state.free_id.lock().unwrap(),
        title,
        author.clone(),
        genre.clone(),
        copies,
        year,
        location.clone(),
        notes,
    ));

    *id_state.free_id.lock().unwrap() += 1;

    all_names.names.lock().unwrap().insert(author);
    all_genres.genres.lock().unwrap().insert(genre);
    all_locations.locations.lock().unwrap().insert(location);

    println!("{:?}", library);

    *unsaved_state.unsaved.lock().unwrap() = true;

    Ok("Addition complete")
}

#[tauri::command]
fn load_library_file(
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
fn save_lib_to_file(library: State<LibraryState>, unsaved_state: State<UnsavedState>) {
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
fn get_all_medias(library: State<LibraryState>) -> Vec<Media> {
    return (*library.medias.lock().unwrap()).clone();
}

#[tauri::command]
fn get_sorted_medias(by: &str, library: State<LibraryState>) -> Vec<Media> {
    match by {
        "title" => {
            (*library.medias.lock().unwrap()).sort_by(|med_1, med_2| med_1.title.cmp(&med_2.title))
        }
        "year" => {
            (*library.medias.lock().unwrap()).sort_by(|med_1, med_2| med_1.year.cmp(&med_2.year))
        }
        "author" => (*library.medias.lock().unwrap())
            .sort_by(|med_1, med_2| med_1.author.cmp(&med_2.author)),
        &_ => (),
    }
    return (*library.medias.lock().unwrap()).clone();
}

#[tauri::command]
fn get_unsaved_status(unsaved_state: State<UnsavedState>) -> bool {
    return *unsaved_state.unsaved.lock().unwrap();
}

#[tauri::command]
fn get_filtered_medias(query: String, library: State<LibraryState>) -> Vec<Media> {
    Vec::from_iter(
        (*library.medias.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|media| {
                media.title.contains(&query)
                    | media.author.contains(&query)
                    | media.genre.contains(&query)
                    | media.location.contains(&query)
                    | media.notes.contains(&query)
            }),
    )
}

#[tauri::command]
fn get_filtered_names(substring: String, all_names: State<NamesState>) -> Vec<String> {
    Vec::from_iter(
        (*all_names.names.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.contains(&substring)),
    )
}

#[tauri::command]
fn get_filtered_genres(substring: String, all_genres: State<GenresState>) -> Vec<String> {
    Vec::from_iter(
        (*all_genres.genres.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.contains(&substring)),
    )
}

#[tauri::command]
fn get_filtered_locations(substring: String, all_locations: State<LocationsState>) -> Vec<String> {
    Vec::from_iter(
        (*all_locations.locations.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.contains(&substring)),
    )
}

#[tauri::command]
fn get_empty() -> Vec<Media> {
    Vec::new()
}

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
            load_library_file,
            save_lib_to_file,
            get_empty,
            get_false,
            get_all_medias,
            get_filtered_medias,
            get_unsaved_status,
            get_filtered_names,
            get_filtered_genres,
            get_filtered_locations,
            get_sorted_medias,
            add_media,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
