use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct QuantumProgram {
  pub id: String,
  pub control_instrument: String,
  pub initial_value: f64,
  pub operations: Vec<Operation>,
}

impl QuantumProgram {
  pub fn new(
    id: String,
    control_instrument: String,
    initial_value: f64,
    operations: Vec<Operation>,
  ) -> Self {
    Self {
      id,
      control_instrument,
      initial_value,
      operations,
    }
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Operation {
  #[serde(rename = "type")]
  pub op_type: String,
  pub value: f64,
}

impl Operation {
  pub fn new(op_type: String, value: f64) -> Self {
    Self { op_type, value }
  }
}
