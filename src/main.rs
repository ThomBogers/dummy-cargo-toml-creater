extern crate toml;
extern crate toml_edit;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

use toml_edit::{value, Document};

enum Emode {
    Lock,
    Toml,
}

fn main() {
    let mut mode: Option<Emode> = None;
    let mut package: Option<String> = None;

    for (i, arg) in env::args().enumerate() {
        if i == 0 {
            // Skip binary name
            continue;
        }

        let sarg: Vec<&str> = arg.split("=").collect();
        if sarg.len() != 2 {
            panic!("Invalid option: {}", arg);
        }

        if sarg[0] == "--mode" {
            if sarg[1] == "lock" {
                mode = Some(Emode::Lock);
            } else if sarg[1] == "toml" {
                mode = Some(Emode::Toml);
            } else {
                panic!("Unkown mode: {}", sarg[1]);
            }
        } else if sarg[0] == "--package" {
            package = Some(String::from(sarg[1]))
        } else {
            panic!("Unkown option: {}", sarg[0]);
        }
    }

    match mode {
        Some(Emode::Toml) => {
            println!("Running in TOML mode");
            ctoml_creater();
        }
        Some(Emode::Lock) => {
            let package = if let Some(package) = package {
                package
            } else {
                panic!("No package set");
            };
            println!("Running in LOCK mode for: {}", package);
            clock_creater(&package);
        }
        None => println!("No valid mode set"),
    }
}

fn ctoml_creater() {
    let manifest_str = match File::open("./Cargo.toml") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            do_cat(&mut buf_file)
        }
        Err(e) => panic!("{}", e),
    };
    let mut doc = manifest_str.parse::<Document>().expect("invalid doc");
    doc["package"]["version"] = value("0.0.0");
    let mut file = std::fs::File::create("./Cargo.toml").unwrap();
    file.write_all(doc.to_string().as_bytes())
        .expect("Could not write to file!");
}

fn clock_creater(package_name: &str) {
    let manifest_str = match File::open("./Cargo.lock") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            do_cat(&mut buf_file)
        }
        Err(e) => panic!("{}", e),
    };
    let mut doc = manifest_str.parse::<Document>().expect("invalid doc");
    let mut counter = 0;
    let mut idx = 0;
    let tables = doc["package"].as_array_of_tables_mut().unwrap();
    for t in tables.iter() {
        for v in t.iter() {
            if v.0 == "name" {
                if let Some(s) = v.1.as_str() {
                    if s == package_name {
                        idx = counter;
                        break;
                    }
                }
            }
        }
        counter += 1;
    }
    doc["package"][idx]["version"] = value("0.0.0");
    let mut file = std::fs::File::create("./Cargo.lock").unwrap();
    file.write_all(doc.to_string().as_bytes())
        .expect("Could not write to file!");
}

fn do_cat(stream: &mut dyn BufRead) -> String {
    let mut buffer = String::new();
    let mut result = String::new();
    loop {
        match stream.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                result = format!("{}{}", result, buffer);
                buffer.clear();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    result
}
