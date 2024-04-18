use std::path::PathBuf;

use clap::Parser;

pub mod lex;
pub mod parse;
pub mod run;

#[derive(Debug, Parser)]
#[clap(name = "tula")]
pub enum Command {
    Lex {
        #[clap(short, long)]
        file: PathBuf,
    },

    Parse {
        #[clap(short, long)]
        file: PathBuf,
    },

    Run {
        #[clap(short, long)]
        src: PathBuf,

        #[clap(short, long)]
        tape: PathBuf,
    },
}
