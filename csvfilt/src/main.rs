extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::path::PathBuf;

mod query;

use query::QueryString;

enum FileSource {
    ReadFromFile(PathBuf)
}

struct Args {
    source : FileSource,
    query : QueryString
}

fn read_args() -> Result<Args, Box<Error>> {
    let args : Vec<OsString> = 
        env::args_os().skip(1).collect(); // first arg is the exe
    match args.len() {
        2 => {
            let q = args[0].clone().into_string();
            let p = args[1].clone();
            Ok(Args {
                source : FileSource::ReadFromFile(From::from(p)), 
                query : query::QueryString(q.unwrap())
                })
        }
        x => {
            Err(
                From::from(
                    format!("Expected 2 args, received {:?} : {:?}", 
                        x,
                        args
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
