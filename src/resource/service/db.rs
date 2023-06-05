use tiberius::{AuthMethod, Client, Config, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use crate::model::{Db, RequisicaoSimulacao};

/// Busca produto relevante no banco de dados com base no envelope recebido
/// OBS: Idealmente esta chamada consumiria um pool de conexão, e validaria o payload com mais
/// regras. As configurações do SQL Server na Azure utilizado para o desafio no entanto
/// impossibilita o uso de ORM mais elaborado full-rust (SQLx|SEA-ORM).
pub async fn busca_produto(payload: &RequisicaoSimulacao, settings: &Db) -> Vec<Row> {
    // Configuração de conexão
    let mut config = Config::new();
    config.host(settings.host.to_owned());
    config.port(settings.port.to_owned());
    config.database(settings.database.to_owned());
    config.authentication(AuthMethod::sql_server(settings.user.to_owned(), settings.pass.to_owned()));
    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    // Instancia cliente
    let mut client = Client::connect(config, tcp.compat_write()).await.unwrap();

    // Consulta tabela PRODUTO
    let rows = client.query("SELECT CO_PRODUTO, NO_PRODUTO, STR(PC_TAXA_JUROS, 25, 5) AS PC_TAXA_JUROS FROM PRODUTO WHERE \
        NU_MINIMO_MESES <= @P1 \
        AND NU_MAXIMO_MESES >= @P2 \
        AND VR_MINIMO <= @P3 \
        AND VR_MAXIMO >= @P4",
        &[&payload.prazo, &payload.prazo, &payload.valor_desejado, &payload.valor_desejado])
        .await.unwrap()
        .into_first_result()
        .await.unwrap();

    client.close().await.expect("Não foi possível fechar conexão com o banco.");

    rows
}