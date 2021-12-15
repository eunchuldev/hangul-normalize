use clap::Clap;
use derive_more::From;
use hangul_normalize::{normalize, NormalizeConfig};
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, Stdout, Write},
};

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
pub struct Opts {
    #[clap(short, long)]
    pub input_file_path: Option<String>,
    #[clap(short, long)]
    pub output_file_path: Option<String>,
    #[clap(short, long)]
    pub hangul_to_jamo: bool,
    #[clap(short, long)]
    pub control_chars: Option<String>,
    #[clap(short, long)]
    pub repeat: Option<usize>,
    #[clap(short, long)]
    pub whitespace_less: bool,
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}


#[derive(From)]
enum Writer {
    File(File),
    Stdout(Stdout),
}
impl Writer {
    fn write_all(&mut self, text: &[u8]) -> anyhow::Result<()> {
        match self {
            Writer::File(writer) => writer.write_all(text),
            Writer::Stdout(writer) => writer.write_all(text),
        }?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    let normalize_config = NormalizeConfig {
        hangul_to_jamo: opts.hangul_to_jamo,
        control_chars: opts.control_chars,
        repeat: opts.repeat,
        whitespace_less: opts.whitespace_less,
    };
    let mut writer: Writer = if let Some(output_file_path) = &opts.output_file_path {
        let file = File::create(&output_file_path)?;
        file.into()
    } else {
        stdout().into()
    };
    if let Some(input_file_path) = &opts.input_file_path {
        let file = File::open(&input_file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            writer.write_all(normalize(line, &normalize_config).as_bytes())?;
            writer.write_all(b"\n")?;
        }
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        writer.write_all(normalize(buffer, &opts).as_bytes())?;
    };
    Ok(())
}
