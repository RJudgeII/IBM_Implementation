use ibm_rust::instruments::acts_as_control::ControlMaker;
use ibm_rust::programs::quantum_program::QuantumProgram;
use reqwest;
use std::env;
use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main() {
  //  This block just gets the input and creates a list of programs to run from
  //    the given inputs.
  let args = parse_args();
  let input_file = parse_input(args);
  let program_list = get_program_list(input_file);

  let client = reqwest::Client::new();

  //  The meat of it all. For each program, make an instrument to feed the 
  //    program into, load the program, run the program, and output the 
  //    response to stdout.
  for program in program_list {
    let instrument = ControlMaker::new_instrument(program.clone()).unwrap();

    let post_response = instrument.load_program(client.clone());
    let get_response = instrument.run_program(post_response.await, client.clone());

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
  let q_program: serde_json::Value = serde_json::from_reader(input_file).expect("Data from file not parsable.");
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