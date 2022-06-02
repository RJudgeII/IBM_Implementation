use reqwest;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::path::Path;
use ibm_rust::instruments::{acme::Acme, madrid::Madrid, acts_as_control::ActsAsControl};
use ibm_rust::programs::quantum_program::{QuantumProgram, Operation};
use reqwest::Response;

#[derive(Debug, Deserialize)]
struct PostResponse {
    program_id: String,
}

#[derive(Debug, Deserialize)]
struct GetResponse {
    result: f64,
}

#[derive(Debug, Deserialize)]
struct GeneratedCode {
    #[serde(rename="type")]
    c_type: String,
    value: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    let input_file = parse_input(args);
    let program_list = get_program_list(input_file);

    let client = reqwest::Client::new();

    for program in program_list {
        let instrument;

        match program.control_instrument.to_uppercase().as_str() {
            "ACME" => {
                instrument = Acme::new(program.initial_value, program.operations.clone());                
            },
            "MADRID" => {
                instrument = Madrid::new(program.initial_value, program.operations.clone());
            },
            _ => instrument = Acme::new(0.0, Vec::new()),
        };

        let post_response = load_program(&*instrument, client.clone());
        let get_response = run_program(&*instrument, post_response.await, client.clone());


        println!("{:?}:\t\t{:?}", program.id, get_response.await.result);
    }



    
}

fn parse_args() -> Vec<String> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      panic!("\n\nAn input file must be passed as argument to the executable. Try 'cargo run <filename>'.\n\n");
  }
  args
}

fn parse_input(args: Vec<String>) -> File {
  let file_path = Path::new(&args[1]);
  let file = File::open(file_path).expect("File not found");
  file
}

fn get_program_list(input_file: File) -> Vec<QuantumProgram> {
  let q_program: serde_json::Value = serde_json::from_reader(input_file).expect("bad data");
  let cast = serde_json::from_value(q_program.clone());

  if !cast.is_err() {
    return cast.unwrap();
  } else {
    let single: QuantumProgram = serde_json::from_value(q_program).unwrap();
    let mut output: Vec<QuantumProgram> = Vec::new();
    output.push(single);
    output
  }
}

async fn load_program(instrument: &dyn ActsAsControl, client: reqwest::Client) -> PostResponse {
  let json = serde_json::to_string(&instrument).unwrap();
  let val: GeneratedCode = serde_json::from_str(&json).unwrap();

  let url = String::from("http://127.0.0.1:") + &instrument.port_number() +  "/" + &instrument.post_url();
  let post = val.value.to_string();

  let response: PostResponse = client
    .post(url)
    .body(post)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

  response
}

async fn run_program(instrument: &dyn ActsAsControl, post_response: PostResponse, client: reqwest::Client) -> GetResponse {
  let json = serde_json::to_string(&instrument).unwrap();
  let val: GeneratedCode = serde_json::from_str(&json).unwrap();

  let url = String::from("http://127.0.0.1:") + &instrument.port_number() +  "/" + &instrument.get_url() + "/" + &post_response.program_id;

  let response: GetResponse = client
    .get(url)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

  response
}