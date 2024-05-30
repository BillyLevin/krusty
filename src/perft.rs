use anyhow::Context;

use crate::{board::Board, move_generator::MoveList};

struct PerftMetadata<'a> {
    fen: &'a str,
    tests: Vec<Test>,
}

struct Test {
    depth: u64,
    expected_nodes: u64,
}

pub fn run_perft_tests(tests: &str) {
    let tests: Vec<_> = tests.lines().map(parse_perft_string).collect();

    for position in tests {
        let position = position.unwrap();

        let mut board = Board::default();
        board.parse_fen(position.fen).unwrap();

        for test in position.tests {
            let result = perft(&mut board, test.depth).unwrap();
            assert_eq!(result, test.expected_nodes);
        }
    }
}

fn perft(board: &mut Board, depth: u64) -> anyhow::Result<u64> {
    if depth == 0 {
        return Ok(1);
    }

    let mut nodes = 0;

    let mut move_list = MoveList::new();
    board.generate_all_moves(&mut move_list)?;

    for mv in move_list {
        if board.make_move(mv)? {
            nodes += perft(board, depth - 1)?;
        }

        board.unmake_move(mv)?;
    }

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
