use async_trait::async_trait;
use serde::Deserialize;

use crate::programs::quantum_program::{Operation, QuantumProgram};

use super::{acme::Acme, madrid::Madrid};

#[async_trait]
#[typetag::serde(tag = "type", content = "value")]
pub trait ActsAsControl: Send + Sync {
  fn port_number(&self) -> String;
  fn post_url(&self) -> String;
  fn get_url(&self) -> String;

  fn set_initial(&self, program_code: &mut Vec<String>, value: f64);
  fn set_sum(&self, program_code: &mut Vec<String>, value: f64);
  fn set_mul(&self, program_code: &mut Vec<String>, value: f64);
  fn set_div(&self, program_code: &mut Vec<String>, value: f64);

  fn parse_operations(&self, program_code: &mut Vec<String>, operations: Vec<Operation>) {
    for operation in operations {
      match operation.op_type.to_uppercase().as_str() {
        "SUM" => {
          self.set_sum(program_code, operation.value);
        }
        "MUL" => {
          self.set_mul(program_code, operation.value);
        }
        "DIV" => {
          self.set_div(program_code, operation.value);
        }
        _ => {
          panic!("An invalid operation was passed when parsing the input JSON data.");
        }
      };
    }
  }

  fn initialize_program_code(&self, initial_value: f64) -> Vec<String> {
    let mut initial_code: Vec<String> = Vec::new();
    self.set_initial(&mut initial_code, initial_value);
    initial_code
  }

  //  These two are here because the exercise said to not assume all control
  //    instruments are REST services. As long as the output of these methods
  //    are conformed to the listed types, we can have any kind of load/run.
  async fn load_program(&self, client: reqwest::Client) -> PostResponse;
  async fn run_program(&self, post_response: PostResponse, client: reqwest::Client) -> GetResponse;
}

pub struct ControlMaker {}

impl ControlMaker {
  //  When adding any new control instruments, a new type can be made and added
  //    to this list.
  pub fn new_instrument(program: QuantumProgram) -> Option<Box<dyn ActsAsControl>> {
    match program.control_instrument.to_uppercase().as_str() {
      "ACME" => {
        let mut instrument = Acme::new();
        let mut program_code = instrument.initialize_program_code(program.initial_value);
        instrument.parse_operations(&mut program_code, program.operations);
        instrument.program_code = program_code;

        return Some(Box::new(instrument));
      }
      "MADRID" => {
        let mut instrument = Madrid::new();
        let mut program_code = instrument.initialize_program_code(program.initial_value);
        instrument.parse_operations(&mut program_code, program.operations);
        instrument.program_code = program_code;

        return Some(Box::new(instrument));
      }
      _ => return None,
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct PostResponse {
  pub program_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetResponse {
  pub result: f64,
}

#[cfg(test)]
mod program_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_empty_input() {
    let q_program = QuantumProgram::new("".to_string(), "".to_string(), 0.0, Vec::new());
    if let None = ControlMaker::new_instrument(q_program) {
      panic!("Got 'None' from create new instrument");
    }
  }

  #[test]
  #[should_panic]
  fn divide_by_zero() {
    let mut ops_list: Vec<Operation> = Vec::new();
    let op = Operation::new("Div".to_string(), 0.0);
    ops_list.push(op);

    let q_program = QuantumProgram::new(
      "div_zero_test".to_string(),
      "Acme".to_string(),
      10.0,
      ops_list,
    );
    ControlMaker::new_instrument(q_program);
  }
}
