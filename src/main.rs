use krusty::cli::Cli;

fn main() -> anyhow::Result<()> {
    let mut cli = Cli::default();
    cli.start_loop();

    Ok(())
}
