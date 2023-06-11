use validator::ValidationError;
use crate::model::RequisicaoSimulacao;

pub fn valida_envelope(payload: &RequisicaoSimulacao) -> Result<(), ValidationError> {
    return Ok(());
}