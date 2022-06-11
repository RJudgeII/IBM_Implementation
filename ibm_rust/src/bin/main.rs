use futures::{stream, StreamExt};
use ibm_rust::instruments::acts_as_control::ControlMaker;
use ibm_rust::programs::quantum_program::QuantumProgram;
use reqwest;
use std::env;
use std::fs::File;
use std::path::Path;
use tokio;

#[tokio::main]
async fn main() {
  //  This block just gets the input and creates a list of programs to run from
  //    the given inputs.
  let args = parse_args();
  let input_file = parse_input(args);
  let program_list = get_program_list(input_file);

  let results: Vec<ProgramOutput> = execute_all_programs_in_list(program_list).await;

  sort_and_print_results(results);
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
  let q_program: serde_json::Value =
    serde_json::from_reader(input_file).expect("Data from file not parsable.");
  let cast = serde_json::from_value(q_program.clone());

  //  This handles the cases for the two files. The small file has a single
  //    object, where the large file has an array of objects. This tries to
  //    read the JSON in as an array, but if it can't it assumes it has a
  //    single object which it then creates an array around.
  if !cast.is_err() {
    return cast.unwrap();
  } else {
    let single: QuantumProgram = serde_json::from_value(q_program).unwrap();
    let mut output: Vec<QuantumProgram> = Vec::new();
    output.push(single);
    output
  }
}

async fn execute_program(program: QuantumProgram) -> ProgramOutput {
  let client = reqwest::Client::new();
  let instrument = ControlMaker::new_instrument(program.clone()).unwrap();

  let post_response = instrument.load_program(client.clone());
  let get_response = instrument.run_program(post_response.await, client);

  ProgramOutput {
    id: program.id,
    value: get_response.await.result,
  }
}

async fn execute_all_programs_in_list(program_list: Vec<QuantumProgram>) -> Vec<ProgramOutput> {
  stream::iter(
    program_list
      .clone()
      .into_iter()
      .map(|program| execute_program(program)),
  )
  .buffer_unordered(program_list.len())
  .collect::<Vec<ProgramOutput>>()
  .await
}

fn sort_and_print_results(mut results: Vec<ProgramOutput>) {
  results.sort_by(|a, b| a.id.cmp(&b.id));

  for result in results {
    println!("{:?}:\t\t{:?}", result.id, result.value);
  }
}

struct ProgramOutput {
  id: String,
  value: f64,
}
