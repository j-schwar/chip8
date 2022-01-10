mod data;
mod disassemble;
mod emulation;
mod opcode;

use disassemble::Disassembler;
use emulation::Emulator;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::exit,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "chip8", about = "Chip8 Emulator")]
enum Opt {
    #[structopt(name = "dasm")]
    Disassemble {
        /// Prints address along with instructions.
        #[structopt(short = "a", long)]
        include_addresses: bool,

        /// The address to start at when printing addresses.
        #[structopt(long, default_value = "512")]
        start_address: u16,

        /// Prints binary along with instructions.
        #[structopt(short = "b", long)]
        include_binary: bool,

        /// Path to the binary to execute.
        bin_path: PathBuf,
    },

    Run {
        /// Path to the binary to execute.
        bin_path: PathBuf,
    },
}

fn read_file(path: &Path) -> Vec<u8> {
    match fs::read(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Disassemble {
            include_addresses,
            start_address,
            include_binary,
            bin_path,
        } => {
            let program = read_file(&bin_path);

            Disassembler::new()
                .with_addresses(include_addresses)
                .with_start_address(start_address)
                .with_binary(include_binary)
                .disassemble(&program, &mut io::stdout())
                .unwrap();
        }

        Opt::Run { bin_path } => {
            let program = read_file(&bin_path);

            Emulator::new().run(&program).unwrap();
        }
    }
}
