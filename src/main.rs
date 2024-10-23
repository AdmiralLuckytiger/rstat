mod srcstats; 

use std::path::PathBuf;

use structopt::{self, StructOpt};

use srcstats::{errors::StatsError, get_summary_src_stats};

#[derive(Debug, structopt::StructOpt)]
#[structopt(name="rstat", about="Rust source statistics. Given a directory, it will generate a file count of Rust sources files
, and source code metrics such as the number of blanks, comments, and actual lines of code within the directory structure.")]
struct Opt {
    /// srcfolder: directory with the Rust files
    #[structopt(name = "src", default_value= ".", short)]
    src_folder: PathBuf,    
}


/// TODO: Add bin for binary analisys
fn main() -> Result<(), StatsError>{
    // 1. Accepts user inputs from the commandline
    let opt = Opt::from_args();

    // 2. Invokes the appropiate method to compute the source code metrics
    let stats = get_summary_src_stats(&opt.src_folder);

    // 3. Display the result to the user
    match stats {
        Ok(stats) => {
            println!("Summary stats: {}", stats);
        },
        Err(e) => {
            eprintln!("{}", e.warn);
        }
    }
    
    // 4. In the event of errors, a suitable error message is displayed to the use;
    Ok(())
}
