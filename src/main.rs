use std::path::PathBuf;
use structopt::StructOpt;

use bgzip;
use std::fs;
use std::io::{self, BufRead};

////////////////////
// VARIABLES
////////////////////

////////////////////
// CONFIGURATIONS
////////////////////

// let header_schema = json!({
//     "type": "object",
//     "properties": {
//         "version": {"type": "string"},
//         "metadata_blocks": {"type": "number"},
//         "k": {"type": "number"},
//         "total_kmers": {"type": "number"},
//         "unique_kmers": {"type": "number"},
//         "metadata": {"type": "boolean"},
//         "tags": {
//             "type": "array",
//             "items": {"type": "string"}
//         },
//         "files": {
//             "type": "array",
//             "items": {
//                 "type": "object",
//                 "properties": {
//                     "filename": {"type": "string"},
//                     "sha256": {
//                         "type": "string",
//                         "minLength": 64,
//                         "maxLength": 64
//                     },
//                     "md5": {
//                         "type": "string",
//                         "minLength": 32,
//                         "maxLength": 32
//                     },
//                     "total_reads": {"type": "number"},
//                     "total_kmers": {"type": "number"},
//                     "unique_kmers": {"type": "number"},
//                     "nullomers": {"type": "number"},
//                     "mononucleotides": {
//                         "type": "object",
//                         "properties": {
//                             "A": {"type": "number"},
//                             "C": {"type": "number"},
//                             "G": {"type": "number"},
//                             "T": {"type": "number"}
//                         }
//                     }
//                 },
//                 "required": ["filename", "sha256", "md5", "total_reads", "total_kmers", "unique_kmers", "nullomers"]
//             }
//         },
//         "comments": {
//             "type": "array",
//             "items": {"type": "string"}
//         }
//     },
//     "required": ["version", "metadata_blocks", "k", "tags", "files"]
// });

////////////////////
// CMDLINE OPTIONS
////////////////////

#[derive(Debug, StructOpt)]
pub struct Cmdline {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(subcommand)]
    subcommand: KmerDB,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kmerdb",
    about = "A k-mer count file format for artificial metagenomes."
)]
enum KmerDB {
    Profile {
        /// Output KDB file
        #[structopt(short, long)]
        outfile: PathBuf,
    },
    Header {
        /// JSONify
        #[structopt(short, long)]
        json: bool,

        /// Input KDB file
        #[structopt(parse(from_os_str))]
        infile: PathBuf,
    },
    View {
        /// Input
        #[structopt(short, long)]
        infile: Option<PathBuf>,

        /// Output
        #[structopt(short, long)]
        outfile: Option<PathBuf>,
    },
    // Matrix {

    // },
    // Distance {

    // },
    // Index {

    // },
    // Shuf {

    // },
    // Probability {

    // },
    // Citation {

    // }
}

////////////////////
// MAIN
////////////////////

fn main() {
    let opt = Cmdline::from_args();
    dbg!(&opt);
    eprintln!("{:#?}", opt);

    match opt.subcommand {
        KmerDB::View { infile, outfile } => {
            eprintln!("running view");
        }
        KmerDB::Header { json, infile } => {
            eprintln!("running header");
            let header = load_header(infile);
        }
        KmerDB::Profile { outfile } => {
            eprintln!("running profile");
        }
        _ => {
            eprintln!("Not implemented!");
        }
    }

    eprintln!("Done.")
}

////////////////////
// FUNCTIONS
////////////////////

fn load_header(infile: PathBuf) -> Result<(), bgzip::BGZFError> {
    //use jsonschema::{Draft, JSONSchema};
    //use serde_json::json;
    use std::io::Write;
    use std::str;

    let f = fs::File::open(&infile)?;
    let mut reader = bgzip::BGZFReader::new(f);
    let mut line = String::new();
    let mut lines = String::new();
    //let mut bytelines = reader.read_until("========================");
    let mut byte_vec: Vec<u8> = Vec::new(); // https://stackoverflow.com/questions/52846511/how-can-i-stop-a-bufreader-from-reading-in-rust-when-using-read-until
                                            // Thank you for the quick reference to vectors.
                                            // I have no idea what I'm doing.
                                            // I'm unemployed.
                                            // ...
                                            // Here I want to read a bunch of lines until a criterion matching the byte_vec is satisfied.

    // And I want to collect the byte_vecs
    // There is no implementation of read_until in the bgzf::BGZFReader.

    reader.read_line(&mut line)?;

    //let is_valid_utf8 = str::from_utf8(&byte_vec)?;
    while !line.ends_with(&("=".repeat(24) + "\n")) {
        reader.read_line(&mut line);
    }
    println!("{}", &line);
    // if line == ("=".repeat(24) + "\n") {
    //     println!("Header delimiter reached: {}", &line);
    // } else {
    //     println!("ERROR");
    // }

    // Read the first line

    //println!("{:#?}", reader);

    //let mut buff = [u8;
    //let block = reader.read_exact(

    //println!("{}", buf);
    //assert_eq!(header.operation_system, FILESYSTEM_UNIX);
    //assert_eq!(header.compression_method, 8);
    //assert_eq!(header.flags, FLAG_FNAME);
    //assert_eq!(header.extra_field_len, None);
    Ok(())
}
