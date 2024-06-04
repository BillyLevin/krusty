use krusty::cli::CLI;

fn main() -> anyhow::Result<()> {
    let mut cli = CLI::default();
    cli.start_loop();

    Ok(())
}
