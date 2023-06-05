use axum::{
    extract::State,
    routing::post,
    http::StatusCode,
    Json,
    Router
};
use std::net::SocketAddr;

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use azeventhubs::producer::{
    EventHubProducerClient,
    EventHubProducerClientOptions,
    SendEventOptions
};
use std::fs;

mod resource;
pub use crate::resource::r#struct::model;
pub use crate::resource::util::calculator::*;

#[tokio::main]
async fn main() {
    let settings = {
        let text = fs::read_to_string("src/config/settings.json").expect("Não foi possível ler arquivo de configuração.");
        serde_json::from_str::<model::Config>(&text).expect("Não foi possível interpretar arquivo de configuração.")
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/Simulacao", post(post_emprestimo))
        .with_state(settings);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_emprestimo(
    State(settings): State<model::Config>,
    Json(payload): Json<RequisicaoSimulacao>) -> (StatusCode, Json<RetornoSimulacao>) {
    let mut config = Config::new();

    config.host(settings.db.host);
    config.port(settings.db.port);
    config.database(settings.db.database);
    config.authentication(AuthMethod::sql_server(settings.db.user, settings.db.pass));

    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let mut client = Client::connect(config, tcp.compat_write()).await.unwrap();

    let rows = client.query("SELECT CO_PRODUTO, NO_PRODUTO, STR(PC_TAXA_JUROS, 25, 5) AS PC_TAXA_JUROS FROM PRODUTO WHERE \
        NU_MINIMO_MESES <= @P1 \
        AND NU_MAXIMO_MESES >= @P2 \
        AND VR_MINIMO <= @P3 \
        AND VR_MAXIMO >= @P4", &[&payload.prazo, &payload.prazo, &payload.valor_desejado, &payload.valor_desejado]).await.unwrap()
        .into_first_result()
        .await.unwrap();

    let rows = rows
        .iter()
        .map(Produto::from_row)
        .collect::<Result<Vec<_>, _>>().unwrap();

    let mut result = RetornoSimulacao {
        codigo_produto: 0,
        descricao_produto: "".to_string(),
        taxa_juros: 0.0,
        resultado_simulacao: vec![]
    };

    for row in rows {
        result.codigo_produto = row.CO_PRODUTO;
        result.descricao_produto = row.NO_PRODUTO;
        result.taxa_juros = match row.PC_TAXA_JUROS.trim().parse::<f64>() {
            Ok(x) => {x}
            Err(_) => {0.0}
        };
        result.resultado_simulacao.push(calcula_sac(payload.valor_desejado, result.taxa_juros, payload.prazo));
        result.resultado_simulacao.push(calcula_price(payload.valor_desejado, result.taxa_juros, payload.prazo));
        break;
    }

    let mut producer_client =
        EventHubProducerClient::from_connection_string(
            settings.hub.connection_string,
            settings.hub.hub_name,
            EventHubProducerClientOptions::default()
        ).await.unwrap();

    let event = serde_json::to_string(&result).unwrap();
    let options = SendEventOptions::new();
    producer_client.send_event(event, options).await.unwrap();

    producer_client.close().await.unwrap();

    (StatusCode::CREATED, Json(result))
}