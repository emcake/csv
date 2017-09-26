extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::path::PathBuf;

struct Query (String);

enum FileSource {
    ReadFromFile(PathBuf)
}

struct Args {
    source : FileSource,
    query : Query
}

fn read_args() -> Result<Args, Box<Error>> {
    match env::args_os().count() {
        2 => {
            let q = env::args_os().nth(1).unwrap();
            let p = env::args_os().nth(2).unwrap();
            Ok(Args {
                source : FileSource::ReadFromFile(From::from(p)), 
                query : Query(q.into_string().unwrap())
                })
        }
        x => {
            Err(
                From::from(
                    format!("Expected 2 args, received {:?} : {:?}", 
                        x,
                        env::args_os().collect::<Vec<_>>()
            )))
        }
    }
}

fn run() -> Result<(), Box<Error>> {
    let args = read_args()?;

    let mut reader = 
        match args.source {
            FileSource::ReadFromFile(p) => {
                csv::Reader::from_file(p)
            }
        }?;

    let mut writer = csv::Writer::from_writer(std::io::stdout());

    let headers = reader.headers()?.clone();

    writer.write(headers.iter())?;

    for res in reader.records() {
        let row = res?;
        writer.write(row.iter())?;
    }

    writer.flush()?;
    Ok(())
    }

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
