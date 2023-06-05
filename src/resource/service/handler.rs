use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::model;
use crate::resource::service::db::busca_produto;
use crate::resource::service::hub::envia_evento_hub;
use crate::resource::util::calculator;

/// Handler da requisição POST /Simulacao.
/// Se encarrega de processar o payload enviado, consultar o produto relevante no banco,
/// enviar resposta em event hub, e retornar resultado ao usuário.
pub async fn post_simulacao (
    State(settings): State<model::Config>,
    Json(payload): Json<model::RequisicaoSimulacao>
    ) -> (StatusCode, Json<model::RetornoSimulacao>) {

    // Consulta produto
    let rows = busca_produto(&payload, &settings.db)
        .await
        .iter()
        .map(model::Produto::from_row)
        .collect::<Result<Vec<_>, _>>().unwrap();

    // Gere resposta
    let mut result = model::RetornoSimulacao {
        codigo_produto: 0,
        descricao_produto: "".to_string(),
        taxa_juros: 0.0,
        resultado_simulacao: vec![]
    };

    // Calcula prestações e preenche resposta
    for row in rows {
        result.codigo_produto = row.CO_PRODUTO;
        result.descricao_produto = row.NO_PRODUTO;
        result.taxa_juros = match row.PC_TAXA_JUROS.trim().parse::<f64>() {
            Ok(x) => {x}
            Err(_) => {0.0}
        };
        result.resultado_simulacao.push(calculator::calcula_parcelas(payload.valor_desejado, result.taxa_juros, payload.prazo, "SAC"));
        result.resultado_simulacao.push(calculator::calcula_parcelas(payload.valor_desejado, result.taxa_juros, payload.prazo, "PRICE"));
        break;
    }

    // Envia resposta ao hub
    envia_evento_hub(&result, &settings.hub).await;

    // Retorna resultado ao cliente
    (StatusCode::OK, Json(result))
}