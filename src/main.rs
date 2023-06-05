use axum::{ routing::post, Router };
use std::net::SocketAddr;
use std::fs;
pub use crate::resource::r#struct::model;
pub use crate::resource::service::handler::*;
mod resource;

#[tokio::main]
async fn main() {
    // Parse das configurações de execução (credenciais e afins).
    // Descritas em arquivo .json para facilitar manipulação via pipelines em DevOps.
    // OBS: Para a localização padrão, o working directory da execução deve ser a raiz do projeto.
    let settings = {
        let text = fs::read_to_string("src/config/settings.json").expect("Não foi possível ler arquivo de configuração.");
        serde_json::from_str::<model::Config>(&text).expect("Não foi possível interpretar arquivo de configuração.")
    };

    // Definição de rotas disponíveis na API.
    let app = Router::new()
        .route("/api/Simulacao", post(post_simulacao))
        .with_state(settings);

    // Inicia o servidor web (localhost, porta 80)
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Não foi possível iniciar servidor web.");
}