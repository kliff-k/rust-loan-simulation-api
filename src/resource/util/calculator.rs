pub use crate::resource::r#struct::model::*;

pub fn calcula_sac(valor_desejado: f64, taxa_juros: f64, prazo: i32) -> ResultadoSimulacao {
    let mut sac = ResultadoSimulacao { tipo: "SAC".to_string(), parcelas: vec![] };
    let mut valor_restante = valor_desejado;
    for nu_parcela in 1..prazo+1 {
        let amortizacao = valor_restante / f64::from(prazo-(nu_parcela-1));
        let juros = valor_restante * taxa_juros;
        valor_restante = valor_restante - amortizacao;
        let parcela = Parcela {
            numero: nu_parcela,
            valor_amortizacao: (amortizacao * 100.0).round() / 100.0,
            valor_juros: (juros * 100.0).round() / 100.0,
            valor_prestacao: ((amortizacao + juros) * 100.0).round() / 100.0,
        };

        sac.parcelas.push(parcela);
    }
    sac
}

pub fn calcula_price(valor_desejado: f64, taxa_juros: f64, prazo: i32) -> ResultadoSimulacao {
    let mut price = ResultadoSimulacao { tipo: "PRICE".to_string(), parcelas: vec![] };

    let mut valor_restante = valor_desejado;
    let prestacao = (valor_desejado * taxa_juros) / (f64::from(1) - (f64::from(1) + taxa_juros).powf(-prazo as f64));
    for nu_parcela in 1..prazo+1 {
        let juros = valor_restante * taxa_juros;
        valor_restante = valor_restante - (prestacao - juros);
        let parcela = Parcela {
            numero: nu_parcela,
            valor_amortizacao: ((prestacao - juros) * 100.0).round() / 100.0,
            valor_juros: (juros * 100.0).round() / 100.0,
            valor_prestacao: (prestacao * 100.0).round() / 100.0,
        };

        price.parcelas.push(parcela);
    }
    price
}