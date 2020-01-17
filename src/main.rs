use clap::{App, Arg};
use mail_extractor;
use std::collections::HashMap;
use std::fs;

fn create_output_files(dirname: String, link: HashMap<String, Vec<u8>>) {
    match fs::create_dir_all(dirname.clone()) {
        Ok(_) => (),
        Err(e) => panic!("Unable to create Directory {}", e),
    }
    for filename in link.keys() {
        let content = link.get(filename);
        match fs::write(dirname.clone()+ "/" + filename, content.unwrap()) {
            Ok(_) => (),
            Err(e) => panic!("Unable to write into file, {}", e),
        };
    }
}

fn main() {
    let mut link_to_hash: HashMap<String, Vec<u8>> = HashMap::new();
    let matches = App::new("Mail File Extractor Binary")
        .version("1.0")
        .author("IamSaquib <saquib@distill.io>")
        .about("Extracts file from provided MHT file and stores it locally")
        .args(&[
            Arg::with_name("input-file")
                .index(1)
                .value_name("string")
                .required(true)
                .help("Takes FileName as input"),
            Arg::with_name("output-file")
                .index(2)
                .value_name("string")
                .help("Extracts mht file and gives out map containing filename and data in binary")
        ])
        .get_matches();

    if matches.is_present("input-file") {
        let in_val = matches.value_of("input-file").unwrap();
        let content = fs::read_to_string(in_val).unwrap_or("File not present".to_string());
        link_to_hash = mail_extractor::rewrite(content.as_bytes().to_vec());
    } 
    if matches.is_present("output-file") {    
        let dirname = matches.value_of("output-file").unwrap();
        create_output_files(dirname.to_string(), link_to_hash);
    } else {
        let dirname = matches.value_of("input-file").unwrap().split(".").next().unwrap();
        create_output_files(dirname.to_string(), link_to_hash);
    }
}