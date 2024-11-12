#[derive(Debug, clap::Parser)]
pub struct Args {
    /// The shell for which to generate the completion script.
    shell: clap_complete::Shell,
}

pub async fn run(args: Args) -> anyhow::Result<()> {
    generate_completions(args.shell);
}

fn generate_completions(shell: clap_complete::Shell) -> ! {
    clap_complete::generate(
        shell,
        &mut <crate::Args as clap::CommandFactory>::command(),
        clap::crate_name!(),
        &mut std::io::stdout(),
    );
    std::process::exit(0);
}
