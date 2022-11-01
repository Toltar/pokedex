use rustemon;

pub struct PokemonApiClient {
    client: rustemon::client::RustemonClient,
}

impl PokemonApiClient {
    pub fn new() -> Self {
        Self {
            client: rustemon::client::RustemonClient::default(),
        }
    }

    pub async fn get_pokemon(&self, name: &str) -> rustemon::model::pokemon::Pokemon {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(name, &self.client)
            .await
            .expect("Should have been able to read the pokemon name");
        pokemon
    }
}
