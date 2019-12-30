extern crate scroll;
#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fs;
use crate::model::MDLXModel;
use crate::optimizer::optimize_model;

mod model;
mod macros;
mod optimizer;

pub fn parse_optimize_model(path: &Path, threshold: f32, outside: bool, linearize: bool) {
    // Read bytes from mdx file at specific path
    let file_name = path.file_stem().unwrap();
    let mut file = File::open(path).expect("cannot find file");
    let buf_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut bytes = Vec::<u8>::with_capacity(buf_size);
    file.read_to_end(&mut bytes).unwrap();

    let original_bytes_len = bytes.len();

    // Read mdx file into struct
    let mut model = MDLXModel::read_mdx_file(bytes).unwrap();

    // Optimize model
    optimize_model(&mut model, threshold, linearize, outside);

    // Write mdx file into bytes vec
    let new_bytes = MDLXModel::write_mdx_file(model).unwrap();

    let new_bytes_len = new_bytes.len();

    println!("Original size: {} - Optimized size: {}", original_bytes_len, new_bytes_len);

    // Write bytes
    let new_file_name =
        String::from(file_name.to_str().unwrap()) + String::from("_optimized.mdx").as_ref();
    fs::write(new_file_name, new_bytes).unwrap();
}

fn main() {
    let matches = App::new("Mdxlroptimizer")
        .version(crate_version!())
        .about("Tool for optimizing mdx files.")
        .author("Nebula Venus (Github)")
        .arg(Arg::with_name("outside")
            .help("Delete redundant frames but outside anim sequences")
            .long("outside"))
        .arg(Arg::with_name("linearize")
            .help("Converts hermite/bezier to linear. Simplify keyframes")
            .long("linearize"))
        .arg(Arg::with_name("threshold")
            .takes_value(true)
            .short("t")
            .long("threshold"))
        .subcommand(SubCommand::with_name("optimize")
                        .about("Optimize mdl file")
                        .arg(
                            Arg::with_name("input")
                                .help("the file to optimize")
                                .index(1)
                                .required(true)
                        ),
        )
        .get_matches();

    let mut threshold = 0f32;
    if let Some(th) = matches.value_of("threshold") {
        let new_th = th.parse::<f32>()
            .expect("entered threshold value is not correct");
        if new_th.is_sign_negative() {
            println!("Threshold can't be negative, default value will be used");
        } else {
            threshold = new_th;
        }
    }

    let mut outside = matches.is_present("outside");
    let mut linearize = matches.is_present("linearize");

    if let Some(ref matches) = matches.subcommand_matches("optimize") {
        let file = matches.value_of("input").unwrap();
        parse_optimize_model(file.as_ref(), threshold, outside, linearize);
    }
}
