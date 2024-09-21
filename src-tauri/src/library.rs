use std::{collections::HashSet, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::saving::UnsavedState;

pub struct IdState {
    pub free_id: Mutex<u64>,
}

#[derive(Debug)]
pub struct NamesState {
    pub names: Mutex<HashSet<String>>,
}
#[derive(Debug)]
pub struct GenresState {
    pub genres: Mutex<HashSet<String>>,
}
#[derive(Debug)]
pub struct LocationsState {
    pub locations: Mutex<HashSet<String>>,
}

#[derive(Debug)]
pub struct LibraryState {
    pub medias: Mutex<Vec<Media>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: u64,
    title: String,
    pub author: String,
    pub genre: String,
    copies: u64,
    year: i64,
    pub location: String,
    notes: String,
}

impl Media {
    pub fn new(
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
pub fn add_media(
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
pub fn edit_media(
    id: u64,
    changed_title: String,
    changed_author: String,
    changed_genre: String,
    changed_copies: u64,
    changed_year: i64,
    changed_location: String,
    changed_notes: String,
    library: State<LibraryState>,
    unsaved_state: State<UnsavedState>,
) -> Result<(), String> {
    let mut medias = library.medias.lock().unwrap();
    if let Some(media_idx) = medias.iter().rposition(|media| media.id == id) {
        medias[media_idx] = Media::new(
            id,
            changed_title,
            changed_author,
            changed_genre,
            changed_copies,
            changed_year,
            changed_location,
            changed_notes,
        );
        *unsaved_state.unsaved.lock().unwrap() = true;
        Ok(())
    } else {
        Err("Could not find this id.".to_string())
    }
}

#[tauri::command]
pub fn remove_media(
    id: u64,
    library: State<LibraryState>,
    unsaved_state: State<UnsavedState>,
) -> Result<(), String> {
    let mut medias = library.medias.lock().unwrap();
    if let Some(media_idx) = medias.iter().rposition(|media| media.id == id) {
        medias.remove(media_idx);
        *unsaved_state.unsaved.lock().unwrap() = true;
        Ok(())
    } else {
        Err("Could not find this id.".to_string())
    }
}

#[tauri::command]
pub fn get_all_medias(library: State<LibraryState>) -> Vec<Media> {
    return (*library.medias.lock().unwrap()).clone();
}

#[tauri::command]
pub fn get_sorted_medias(by: &str, library: State<LibraryState>) -> Vec<Media> {
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
pub fn get_filtered_medias(query: String, library: State<LibraryState>) -> Vec<Media> {
    let normalized_query = query.to_lowercase();
    Vec::from_iter(
        (*library.medias.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|media| {
                media.title.to_lowercase().contains(&normalized_query)
                    | media.author.to_lowercase().contains(&normalized_query)
                    | media.genre.to_lowercase().contains(&normalized_query)
                    | media.location.to_lowercase().contains(&normalized_query)
                    | media.notes.to_lowercase().contains(&normalized_query)
            }),
    )
}

#[tauri::command]
pub fn get_filtered_names(substring: String, all_names: State<NamesState>) -> Vec<String> {
    Vec::from_iter(
        (*all_names.names.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&substring.to_lowercase())),
    )
}

#[tauri::command]
pub fn get_filtered_genres(substring: String, all_genres: State<GenresState>) -> Vec<String> {
    Vec::from_iter(
        (*all_genres.genres.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&substring.to_lowercase())),
    )
}

#[tauri::command]
pub fn get_filtered_locations(
    substring: String,
    all_locations: State<LocationsState>,
) -> Vec<String> {
    Vec::from_iter(
        (*all_locations.locations.lock().unwrap())
            .clone()
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&substring.to_lowercase())),
    )
}

#[tauri::command]
pub fn get_empty() -> Vec<Media> {
    Vec::new()
}
