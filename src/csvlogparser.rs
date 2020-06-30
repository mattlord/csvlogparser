use std::io;
use std::process;
use clap::clap_app;

fn parse_input() -> Result<(), csv::Error> {
  let mut reader = csv::Reader::from_reader(io::stdin());

  for result in reader.records() {
    let record = result?;
    println!("record: {:?}", record);
  }

  Ok(())
}

fn parse_file(file_path: &str) -> Result<(), csv::Error> {
  let mut reader = csv::Reader::from_path(file_path)?;

  for result in reader.records() {
    let record = result?;
    println!("record: {:?}", record);
  }

  Ok(())
}

fn main() {
  let matches = clap_app!(csvlogstreamparser =>
      (version: "0.1")
      (author: "Matt Lord <mattalord@gmail.com>")
      (about: "Parses HTTP Logs in CSV Format")
      (@arg INPUT_FILE: -f --file +takes_value "Sets a file to process [default is STDIN]")
      (@arg RECORD_NUMBER: -s --frequency +takes_value "How many records to examine per stat update [default is 10]")
  ).get_matches();

  let file_path = matches.value_of("INPUT_FILE").unwrap_or("");

  if file_path.is_empty() {
    if let Err(err) = parse_input() {
      println!("Error processing CSV stream: {}", err);
      process::exit(1);
    }
  } else {
    if let Err(err) = parse_file(file_path) {
      println!("Error processing CSV file: {}", err);
      process::exit(1);
    }
  }
}
