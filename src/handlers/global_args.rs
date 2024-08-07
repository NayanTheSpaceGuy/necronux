use crate::cli::GlobalArgs;

pub fn handle_global_args(args: &GlobalArgs) {
    if args.debug {
        println!("Debug mode is on");
    }
    if args.verbose {
        println!("Verbose mode is on");
    }
}
