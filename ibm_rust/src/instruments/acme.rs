use crate::instruments::acts_as_control::ActsAsControl;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::acts_as_control::{PostResponse, GetResponse};

#[derive(Deserialize, Serialize)]
pub struct Acme {
  pub program_code: Vec<String>,
}

impl Acme {
  pub fn new() -> Self {
    Self {
      program_code: Vec::new(),
    }
  }
}

#[async_trait]
#[typetag::serde]
impl ActsAsControl for Acme {
  fn port_number(&self) -> String {
    (8001).to_string()
  }

  fn post_url(&self) -> String {
    String::from("load_program")
  }

  fn get_url(&self) -> String {
    String::from("run_program")
  }

  fn set_initial(&self, program_code: &mut Vec<String>, value: f64) {
    program_code.push(String::from("Acme_initial_state_pulse"));
    program_code.push(value.to_string());
  }

  fn set_sum(&self, program_code: &mut Vec<String>, value: f64) {
    program_code.push(String::from("Acme_pulse_1"));
    program_code.push(String::from("Acme_pulse_2"));
    program_code.push(value.to_string());
  }

  fn set_mul(&self, program_code: &mut Vec<String>, value: f64) {
    program_code.push(String::from("Acme_pulse_2"));
    program_code.push(String::from("Acme_pulse_1"));
    program_code.push(String::from("Acme_pulse_1"));
    program_code.push(value.to_string());
  }

  fn set_div(&self, program_code: &mut Vec<String>, value: f64) {
    if value == 0.0 { panic!("Tried to divide by zero."); }
    program_code.push(String::from("Acme_pulse_2"));
    program_code.push(String::from("Acme_pulse_2"));
    program_code.push(value.to_string());
  }

  async fn load_program(&self, client: reqwest::Client) -> PostResponse {
    let json = serde_json::to_string(&self).unwrap();
    let url = String::from("http://127.0.0.1:") + &self.port_number() + "/" + &self.post_url();
    
    let response: PostResponse = client
      .post(url)
      .body(json)
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();
    response
  }

  async fn run_program(&self, 
    post_response: PostResponse,
    client: reqwest::Client,
  ) -> GetResponse {
    let url = String::from("http://127.0.0.1:")
      + &self.port_number()
      + "/"
      + &self.get_url()
      + "/"
      + &post_response.program_id;
  
    let response: GetResponse = client.get(url).send().await.unwrap().json().await.unwrap();
  
    response
  }
}