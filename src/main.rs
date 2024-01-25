mod utils;

fn main() {
    let args = utils::cli::parse();

    match args.target {
        utils::cli::CompilerAction::Default => {
            panic!("Invalid target");
        }
        utils::cli::CompilerAction::Scan => {
            todo!("scan");
        }
        utils::cli::CompilerAction::Parse => {
            todo!("parse");
        }
        utils::cli::CompilerAction::Inter => {
            todo!("inter");
        }
        utils::cli::CompilerAction::Assembly => {
            todo!("assembly");
        }
    }
}
