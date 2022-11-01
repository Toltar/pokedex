#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use client::PokemonApiClient;
mod client;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn lookup_pokemon(name: &str) -> String {
    let client = PokemonApiClient::new();
    let pokemon = client.get_pokemon(name).await;
    let pokemon_types_vector = match pokemon.types {
        None => vec![],
        Some(types) => types,
    };

    let pokemon_types = pokemon_types_vector
        .iter()
        .map(|t| {
            t.type_.as_ref().map_or(String::from("Unknown Type"), |y| {
                y.clone().name.unwrap_or(String::from("Unknown Type"))
            })
        })
        .reduce(|curr, next| format!("{} {}", curr, next))
        .unwrap();

    let pokemon_name_string = pokemon.name.unwrap();
    format!("{}! you are a {} type", pokemon_name_string, pokemon_types)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![lookup_pokemon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
