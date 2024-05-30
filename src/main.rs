use krusty::perft::run_perft_tests;

fn main() -> anyhow::Result<()> {
    let perft_contents = include_str!("../perft.epd");
    run_perft_tests(perft_contents);

    Ok(())
}
