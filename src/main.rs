use clap::{App, Arg};
use flate2::read::GzDecoder;
use mail_extractor;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;

mod read_s3;

fn get_files(filename: &str) -> HashMap<String, Vec<u8>> {
    let mht_file = read_s3::init(filename);
    let mut z = GzDecoder::new(&mht_file[..]);
    let mut s = String::new();
    match z.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => panic!("Unable to read string {}", e), 
    };
    let extracted_file: HashMap<String, Vec<u8>> = mail_extractor::rewrite(s.as_bytes().to_vec());
    // extracted_file = mail_extractor::rewrite(mht_file);
    extracted_file
}

fn main() {
    let mut link_to_hash: HashMap<String, Vec<u8>> = HashMap::new();
    let matches = App::new("Email File Extractor")
        .version("1.0")
        .author("IamSaquib <saquib@distill.io>")
        .about("Extracts file from provided MHT file and stores it locally")
        .args(&[
            Arg::with_name("input-file")
                .index(1)
                .value_name("string")
                .help("Takes an input string"),
            Arg::with_name("output-file")
                .short(String::from("o"))
                .long("output")
                .value_name("string")
                .help("Extracts mht file and gives out map containing filename and data in binary")
                .takes_value(true),
        ])
        .get_matches();

    if matches.is_present("input-file") {
        let in_val = matches.value_of("input-file").unwrap();
        link_to_hash = get_files(&in_val.to_string());
    } 
    if matches.is_present("output-file") {
        let dirname = matches.value_of("output-file").unwrap();
        match fs::create_dir_all(dirname) {
            Ok(_) => (),
            Err(e) => panic!("Unable to create Directory {}", e),
        }
        for filename in link_to_hash.keys() {
            let content = link_to_hash.get(filename);
            match fs::write(dirname.to_owned()+ "/" + filename, content.unwrap()) {
                Ok(_) => (),
                Err(e) => panic!("Unable to write into file, {}", e),
            };
        }
    } 
}