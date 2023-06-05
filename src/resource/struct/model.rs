use serde::{Deserialize, Serialize};
use tiberius_derive::*;

/// Estrutura de configuração de execução.
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub db: Db,
    pub hub: Hub,
}

/// Estrutura de configuração do banco de dados.
#[derive(Clone, Serialize, Deserialize)]
pub struct Db {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub pass: String,
}

/// Estrutura de configuração do event hub.
#[derive(Clone, Serialize, Deserialize)]
pub struct Hub {
    pub connection_string: String,
    pub hub_name: String,
}

/// Envelope da requisição de simulação de empréstimo.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequisicaoSimulacao {
    pub(crate) valor_desejado: f64,
    pub(crate) prazo: i32,
}

/// Envelope do retorno da simulação de empréstimo
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetornoSimulacao {
    pub(crate) codigo_produto: i32,
    pub(crate) descricao_produto: String,
    pub(crate) taxa_juros: f64,
    pub(crate) resultado_simulacao: Vec<ResultadoSimulacao>,
}

/// Estrutura do resultado da simulação de empréstimo.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoSimulacao {
    pub(crate) tipo: String,
    pub(crate) parcelas: Vec<Parcela>,
}

/// Estrutura de parcela de simulação de empréstimo.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcela {
    pub(crate) numero: i32,
    pub(crate) valor_amortizacao: f64,
    pub(crate) valor_juros: f64,
    pub(crate) valor_prestacao: f64,
}

/// Modelo da tabela PRODUTO para consultas de produto.
#[allow(non_snake_case)]
#[derive(FromRow, Serialize, Deserialize)]
#[tiberius_derive(auto)]
pub struct Produto {
    pub(crate) CO_PRODUTO: i32,
    pub(crate) NO_PRODUTO: String,
    pub(crate) PC_TAXA_JUROS: String
}