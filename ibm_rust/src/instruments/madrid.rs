use crate::errors::invalid_operation_error::InvalidOperationError;
use crate::programs::quantum_program::Operation;
use crate::instruments::acts_as_control::ActsAsControl;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize)]
pub struct Madrid {
  pub program_code: Vec<String>,
}

impl Madrid {
  pub fn new(initial_value: f64, operations: Vec<Operation>) -> Box<dyn ActsAsControl> {
    let mut program_code = Self::initialize_program_code(initial_value);
    Self::parse_operations(&mut program_code, operations);
    Box::new(Self { program_code })
  }

  fn initialize_program_code(initial_value: f64) -> Vec<String> {
    let mut initial_code: Vec<String> = Vec::new();
    initial_code.push(initial_value.to_string());
    initial_code.push(String::from("Madrid_initial_state_pulse"));
    initial_code
  }

  fn parse_operations(program_code: &mut Vec<String>, operations: Vec<Operation>) -> Result<Vec<String>, InvalidOperationError> {
    for operation in operations {
      match operation.op_type.to_uppercase().as_str() {
        "SUM" => {
          program_code.push(operation.value.to_string());
          program_code.push(String::from("Madrid_pulse_1"));
        },
        "MUL" => {
          program_code.push(operation.value.to_string());
          program_code.push(String::from("Madrid_pulse_2"));
          program_code.push(String::from("Madrid_pulse_2"));
        },
        "DIV" => {
          program_code.push(operation.value.to_string());
          program_code.push(String::from("Madrid_pulse_2"));
          program_code.push(String::from("Madrid_pulse_1"));
        },
        _ => { return Err(InvalidOperationError); },
      };
    }

    Ok((&program_code).to_vec())
  }
}

#[typetag::serde]
impl ActsAsControl for Madrid {
  fn port_number(&self) -> String {
    (8000).to_string()
  }

  fn post_url(&self) -> String {
    String::from("program/load")
  }

  fn get_url(&self) -> String {
    String::from("program/run")
  }
}