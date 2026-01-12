mod cli;

fn main() {
    let args: cli::Args = argh::from_env();

    match args.command {
        cli::Command::Run(run_args) => {
            println!("Workflow: {:?}", run_args.workflow);
            println!("Images: {:?}", cli::parse_overrides(&run_args.image));
            println!("CPUs: {:?}", cli::parse_overrides(&run_args.cpus));
            println!("Memory: {:?}", cli::parse_overrides(&run_args.mem));
            println!("Default CPUs: {}", cli::default_cpus());
            println!("Default Memory: {} MB", cli::DEFAULT_MEM_MB);
        }
    }
}
