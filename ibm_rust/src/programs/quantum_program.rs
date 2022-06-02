use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuantumProgram {
  pub id: String,
  pub control_instrument: String,
  pub initial_value: f64,
  pub operations: Vec<Operation>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Operation {
  #[serde(rename="type")]
  pub op_type: String,
  pub value: f64,
}