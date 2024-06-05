#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

struct IdState {
    free_id: Mutex<u64>,
}

struct PersonsState {
    names: Mutex<HashSet<String>>,
}

#[derive(Debug)]
struct LibraryState {
    medias: Mutex<Vec<Media>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    role: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Media {
    id: u64,
    title: String,
    people: Vec<Person>,
    notes: String,
}

impl Media {
    fn new(id: u64, title: String, people: Vec<Person>, notes: String) -> Self {
        Self {
            id,
            title,
            people,
            notes,
        }
    }
}

#[tauri::command]
fn add_media(
    title: String,
    people: Vec<Person>,
    notes: String,
    library: State<LibraryState>,
    id_state: State<IdState>,
    all_persons: State<PersonsState>,
) -> Result<&'static str, &'static str> {
    library.medias.lock().unwrap().push(Media::new(
        *id_state.free_id.lock().unwrap(),
        title,
        people,
        notes,
    ));

    *id_state.free_id.lock().unwrap() += 1;

    println!("{:?}", library);

    Ok("Addition complete")
}

#[tauri::command]
fn load_library_file(
    library: State<'_, LibraryState>,
    id_state: State<'_, IdState>,
    all_persons: State<'_, PersonsState>,
) {
    let lib = match FileDialogBuilder::new().pick_file() {
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
        .unwrap_or(&Media::new(0, String::new(), Vec::new(), String::new()))
        .id
        + 1;

    for media in &*library.medias.lock().unwrap() {
        for ppl in &media.people {
            all_persons.names.lock().unwrap().insert(ppl.name.clone());
        }
    }
}

fn read_lib_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Media>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let l = serde_json::from_reader(reader)?;

    Ok(l)
}

#[tauri::command]
fn get_all_medias(library: State<LibraryState>) -> Vec<Media> {
    return (*library.medias.lock().unwrap()).clone();
}

#[tauri::command]
fn get_empty() -> Vec<Media> {
    Vec::new()
}

pub fn main() {
    tauri::Builder::default()
        .manage(LibraryState {
            medias: Mutex::new(Vec::new()),
        })
        .manage(IdState {
            free_id: Mutex::new(0),
        })
        .manage(PersonsState {
            names: Mutex::new(HashSet::new()),
        })
        .invoke_handler(tauri::generate_handler![
            load_library_file,
            get_empty,
            add_media
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
