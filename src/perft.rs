use std::io::Write;

use anyhow::Context;
use colored::Colorize;

use crate::{
    board::Board,
    move_generator::MoveList,
    transposition_table::{TranspositionTable, TranspositionTableEntry},
};

struct PerftMetadata<'a> {
    fen: &'a str,
    tests: Vec<Test>,
}

struct Test {
    depth: u8,
    expected_nodes: u64,
}

pub fn run_perft_tests(tests: &str) {
    let start_time = std::time::Instant::now();

    let tests: Vec<_> = tests.lines().map(parse_perft_string).collect();

    let number_of_tests = tests.len();

    let mut pass_count = 0;
    let mut fail_count = 0;

    let mut board = Board::default();
    let mut transposition_table = TranspositionTable::new(128);

    for (i, position) in tests.into_iter().enumerate() {
        let position = position.unwrap();

        let progress = format!("[{}/{}]", i + 1, number_of_tests);
        println!("{} FEN: {}", progress.cyan(), position.fen);

        board.parse_fen(position.fen).unwrap();

        // position.tests.reverse();
        // position.tests.sort_by_key(|t| t.depth);
        // let test = position.tests.last().unwrap();

        for test in position.tests {
            print!(
                "\r\tdepth: {}, expected nodes: {}",
                test.depth, test.expected_nodes
            );
            std::io::stdout().flush().unwrap();

            let result = perft(&mut board, test.depth, &mut transposition_table).unwrap();
            assert_eq!(result, test.expected_nodes);
            let passed = result == test.expected_nodes;

            let passed_icon = match passed {
                true => "\u{2713}".green(),
                false => "\u{2717}".red(),
            };

            if passed {
                pass_count += 1;
            } else {
                fail_count += 1;
            }

            println!(
                "\r\tdepth: {}, expected nodes: {} {}",
                test.depth, test.expected_nodes, passed_icon
            );
        }
    }

    let total_tests = pass_count + fail_count;
    let pass_count = format!("{} passed", pass_count).green();
    let fail_count = format!("{} failed", fail_count).red();

    println!(
        "\nTests: {}, {}, {} total",
        pass_count, fail_count, total_tests
    );

    println!("Time: {:.2?}", start_time.elapsed());
}

fn perft(
    board: &mut Board,
    depth: u8,
    transposition_table: &mut TranspositionTable,
) -> anyhow::Result<u64> {
    if depth == 0 {
        return Ok(1);
    }

    let entry = transposition_table.probe(board.hash());
    if entry.depth == depth && entry.hash == board.hash() {
        return Ok(entry.node_count);
    }

    let mut nodes = 0;

    let mut move_list = MoveList::new();
    board.generate_all_moves(&mut move_list)?;

    for mv in move_list {
        if board.make_move(mv)? {
            nodes += perft(board, depth - 1, transposition_table)?;
        }

        board.unmake_move(mv)?;
    }

    transposition_table.store(TranspositionTableEntry::new(board.hash(), nodes, depth));

    Ok(nodes)
}

fn parse_perft_string(perft_string: &str) -> anyhow::Result<PerftMetadata> {
    let (fen, tests) = perft_string
        .split_once(';')
        .context("invalid perft string")?;

    let fen = fen.trim();

    let tests = tests
        .split(" ;")
        .map(|test| {
            let (depth, expected_nodes) = test.trim().split_once(' ').unwrap();

            let depth = depth[1..].parse().unwrap();
            let expected_nodes = expected_nodes.parse().unwrap();

            Test {
                depth,
                expected_nodes,
            }
        })
        .collect();

    Ok(PerftMetadata { fen, tests })
}
