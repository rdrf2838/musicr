mod constants;
use clap::Parser;
use constants::{COMPUTER_MUSIC_ROOT, COMPUTER_PLAYLIST_ROOT, PHONE_MUSIC_ROOT};
use core::panic;
use musicr::process_playlists;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_dir: Option<String>,

    #[arg(short, long)]
    output_dir: Option<String>,
}

struct Params {
    input_dir: String,
    input_root: String,
    output_dir: String,
    output_root: String,
}

fn main() {
    let args = Args::parse();
    let params = if let Some(input_dir) = args.input_dir {
        Params {
            input_dir: input_dir,
            input_root: PHONE_MUSIC_ROOT.to_string(),
            output_dir: COMPUTER_PLAYLIST_ROOT.to_string(),
            output_root: COMPUTER_MUSIC_ROOT.to_string(),
        }
    } else {
        if let Some(output_dir) = args.output_dir {
            Params {
                input_dir: COMPUTER_PLAYLIST_ROOT.to_string(),
                input_root: COMPUTER_MUSIC_ROOT.to_string(),
                output_dir: output_dir,
                output_root: PHONE_MUSIC_ROOT.to_string(),
            }
        } else {
            panic!("Must specify either input or output directory");
        }
    };
    process_playlists(
        &params.input_dir,
        &params.input_root,
        &params.output_dir,
        &params.output_root,
    );
}
