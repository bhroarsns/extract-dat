use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::env;
use std::path::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    let inputfile = &args[1];
    let outputfile = if inputfile.ends_with(".dat") {
        PathBuf::from(&format!("{}_extracted.dat", inputfile.trim_end_matches(".dat")))
    } else {
        Path::new(inputfile).with_extension("dat")
    };

    let mut f = BufReader::new(File::open(inputfile)?);
    let mut data = String::new();
    f.read_to_string(&mut data)?;

    let mut out = BufWriter::new(File::create(outputfile)?);
    
    let pattern = [','];

    for line in data.split("\n") {
        if line.trim().is_empty() {
            writeln!(out, "#")?;
        } else {
            if line.split_terminator(&pattern).flat_map(|strip| {
                strip.split_whitespace()
            }).all(|strip| {
                strip.trim().parse::<f64>().is_ok()
            }) {
                writeln!(out, "{}", line.replace(pattern, " "))?;
            } else {
                writeln!(out, "# {}", line)?;
            }
        }
    }

    Ok(())
}