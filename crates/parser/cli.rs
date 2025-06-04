use clap::Parser;
use minilisp::cli::ParserDispatcher;
use minilisp::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "minilisp command-line")]
pub struct Cli {
    #[arg()]
    text: Vec<String>,
}
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text.join(" "));

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
