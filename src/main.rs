use flexi_logger::{default_format, Logger};
use log::info;
use nebula_mdx::MDLXModel;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use took::Timer;

use optimizer::optimize_model;

mod optimizer;
mod traits;

pub fn parse_optimize_model(
    path: &Path,
    threshold: f32,
    outside: bool,
    linearize: bool,
) -> anyhow::Result<()> {
    // Read bytes from mdx file at specific path
    let file_name = path.file_stem().unwrap();
    let mut file = File::open(path)?;
    let buf_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut bytes = Vec::<u8>::with_capacity(buf_size);
    file.read_to_end(&mut bytes)?;

    let original_bytes_len = bytes.len();

    // Read mdx file into struct
    let mut timer = Timer::new();
    let mut model = MDLXModel::read_mdx_file(bytes)?;
    info!("Deserializing mdx file! Took {}", timer.took());

    // Optimize model
    timer = Timer::new();
    optimize_model(&mut model, threshold, linearize, outside);
    info!("Optimizing mdx file! Took {}", timer.took());

    // Write mdx file into bytes vec
    timer = Timer::new();
    let new_bytes = MDLXModel::write_mdx_file(model)?;
    info!("Serializing mdx file! Took {}", timer.took());

    let new_bytes_len = new_bytes.len();

    info!(
        "Original size: {} - Optimized size: {}",
        original_bytes_len, new_bytes_len
    );

    // Write bytes
    timer = Timer::new();
    let new_file_name =
        String::from(file_name.to_str().unwrap()) + String::from("_optimized.mdx").as_ref();

    info!("Writing data into file name: {}", &new_file_name);
    fs::write(new_file_name, new_bytes)?;
    info!("Writing bytes into mdx file! Took {}", timer.took());

    Ok(())
}

fn setup_logger(log: bool) -> anyhow::Result<()> {
    if log {
        Logger::with_env_or_str("mdxlroptimizer=debug, nebula_mdx=debug")
            .log_to_file()
            .directory("logs")
            .format(default_format)
            .start()?;
    } else {
        Logger::with_env_or_str("mdxlroptimizer=debug")
            .format(default_format)
            .start()?;
    }

    Ok(())
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

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    info!("{:#?}", opt);

    // Setup logging
    setup_logger(opt.log)?;

    info!("Processing file with name: {:?}", opt.file);
    parse_optimize_model(opt.file.as_ref(), opt.threshold, opt.outside, opt.linearize)?;

    Ok(())
}
