extern crate flexi_logger;
extern crate log;
extern crate nebula_mdx;
extern crate structopt;

use crate::optimizer::optimize_model;
use flexi_logger::{default_format, Logger};
use log::info;
use nebula_mdx::MDLXModel;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

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

    info!(
        "Original size: {} - Optimized size: {}",
        original_bytes_len, new_bytes_len
    );

    // Write bytes
    let new_file_name =
        String::from(file_name.to_str().unwrap()) + String::from("_optimized.mdx").as_ref();

    info!("Writing data into file name: {}", &new_file_name);
    fs::write(new_file_name, new_bytes).unwrap();
}

fn setup_logger(log: bool) {
    if log {
        Logger::with_env_or_str("mdxlroptimizer=debug, nebula_mdx=debug")
            .log_to_file()
            .directory("logs")
            .format(default_format)
            .start()
            .unwrap();
    } else {
        Logger::with_env_or_str("mdxlroptimizer=debug")
            .format(default_format)
            .start()
            .unwrap();
    }
}

/// Tool for optimizing mdx files.
#[derive(StructOpt, Debug)]
#[structopt(name = "Mdlxroptimizer", author)]
struct Opt {
    /// Log everything into a file
    #[structopt(long)]
    log: bool,

    /// Deletes keyframes outside of animation sequences
    #[structopt(long)]
    outside: bool,

    /// Converts Hermite/Bezier interpolation to Linear
    #[structopt(long)]
    linearize: bool,

    /// Similar keyframes with a threshold difference
    #[structopt(short, long, default_value = "0")]
    threshold: f32,

    // By default postfix will be used after file name
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str), required = true)]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    info!("{:#?}", opt);

    // Setup logging
    setup_logger(opt.log);

    info!("Processing file with name: {:?}", opt.file);
    parse_optimize_model(opt.file.as_ref(), opt.threshold, opt.outside, opt.linearize);
}
