use structopt::StructOpt;

const PROGRAM_NAME: &str = "mcp";

mod cmds;
/// taken from share-secrets-safely/tools
mod options;
mod shared;

use options::Args;
use options::SubCommand::*;

#[paw::main]
fn main(args: Args) {
    simple_logger::init_with_level(args.log_level).ok();
    let res = match args.cmd {
        MapApiIndex(args) => cmds::map_index::execute(args),
        Completions(args) => cmds::completions::execute(Args::clap(), args),
        FetchApiSpecs(args) => cmds::fetch_specs::execute(args),
        Generate(args) => cmds::generate::execute(args),
        Substitute(args) => cmds::substitute::execute(args),
    };
    failure_tools::ok_or_exit(res);
}
