pub use crate::resource::r#struct::model::*;

/// Simula parcelas de um empréstimo com base no valor total, juros, número de parcelas e tipo (SAC|PRICE) 
pub fn calcula_parcelas(valor_desejado: f64, taxa_juros: f64, prazo: i32, tipo: &str) -> ResultadoSimulacao {
    let mut parcelas = ResultadoSimulacao { tipo: "SAC".to_string(), parcelas: vec![] };
    let mut valor_restante = valor_desejado;
    let prestacao = (valor_desejado * taxa_juros) / (f64::from(1) - (f64::from(1) + taxa_juros).powf(-prazo as f64));

    for nu_parcela in 1..prazo+1 {
        let juros = valor_restante * taxa_juros;
        let amortizacao: f64;
        let valor: f64;

        if tipo == "SAC" {
            amortizacao = valor_restante / f64::from(prazo-(nu_parcela-1));
            valor_restante = valor_restante - amortizacao;
            valor = amortizacao + juros;
        }
        else {
            amortizacao = prestacao - juros;
            valor_restante = valor_restante - (prestacao - juros);
            valor = prestacao;
        }

        let parcela = Parcela {
            numero: nu_parcela,
            valor_amortizacao: (amortizacao * 100.0).round() / 100.0,
            valor_juros: (juros * 100.0).round() / 100.0,
            valor_prestacao: (valor * 100.0).round() / 100.0,
        };

        parcelas.parcelas.push(parcela);
    }
    parcelas
}