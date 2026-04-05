use brainfuck_generator::*;
use serde::Deserialize;
use std::io::BufRead;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Eq, PartialEq, Deserialize)]
enum Opt {
    Encode,
    Decode,
}
impl std::str::FromStr for Opt {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encode" => Ok(Opt::Encode),
            "decode" => Ok(Opt::Decode),
            _ => Err(format!("invalid opt: {}", s)),
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "args", about = "config and option")]
struct Args {
    config_path: PathBuf,
    option: Opt,
    file_path: PathBuf,
}

fn main() {
    let Args {
        config_path,
        option,
        file_path,
    } = Args::from_args();
    let config: Config = toml::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    println!("################## Brainfuck Generator ##################");
    println!("{:?}", config);
    println!("######################### Input #########################");
    let mut s = std::fs::read_to_string(&file_path).unwrap();
    println!("{}", s);
    println!("######################## Output #########################");
    if option == Opt::Encode {
        let mut encoded = std::str::from_utf8(&encode(&s))
            .expect("encode failed")
            .to_string();
        swap_chars(&mut encoded, &config);
        println!("{}", encoded);
    } else {
        unswap_chars(&mut s, &config);
        println!(
            "{}",
            std::str::from_utf8(&decode(&s)).expect("decode failed")
        );
    }
}
