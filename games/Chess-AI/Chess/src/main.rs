extern crate args;
extern crate getopts;

use chess:: { Board, MoveGen, Color, BoardStatus, ChessMove, ALL_RANKS, Piece, get_rank };
use std::env;
use std::str::FromStr;
use std::time::{ Duration, Instant };
use args::{ Args, ArgsError };
use getopts::Occur;
use colored::{ ColoredString, Colorize };
mod piece_values;
mod benchmarks;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
const DEFAULT_DEPTH: i64 = 7;

const PROGRAM_DESC: &'static str = "Q good old fashioned Rust chess engine";
const PROGRAM_NAME: &'static str = "Amano";


fn calc_piece_value(pc_idx: usize, sq_idx:usize, color: Option<Color>) -> i64 {
    match color {
        Some(Color::White) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][sq_idx];
            return -(piece_values::PIECE_VALS[pc_idx] + sq_value);
        },
        Some(Color::Black) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][63 - sq_idx];
            return piece_values::PIECE_VALS[pc_idx] + sq_value;
        },
        None => {
            return 0;
        }
    }
}

fn calc_pieces_value(board: &Board) -> i64 {
    let mut result = 0;
    for pc_idx in 0..6 {
        let pc_type = piece_values::PIECES[pc_idx];
        let bboard = *board.pieces(pc_type);
        for square in bboard {
            let sq_idx = square.to_index();
            result += calc_piece_value(pc_idx, sq_idx, board.color_on(square));
        }
    }
    result
}

fn calc_board_value(board: &Board) -> i64 {
    let w_move = board.side_to_move() == Color::White;
    let result = match board.status() {
        BoardStatus::Checkmate => if w_move { 20000 } else { -20000 },
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => calc_pieces_value(board)
    };
    result
}

fn alpha_beta(
        board: &Board, depth: i8,
        is_max: bool, alpha: i64,
        beta: i64, total: &mut i64) -> i64 {
    if (depth == 0) || (board.status() != BoardStatus::Ongoing) {
        *total += 1;
        let val = calc_board_value[board];
        return val;
    }
    let mut alpha = alpha;
    let mut beta = beta;
    if is_max{
        let mut best_value = i64::MIN;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(&result_board, depth-1, false, alpha, beta, total);
            best_val = std::cmp::max(value, best_val);
        
            alpha = std::cmp::max(alpha, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    } else {
        let mut best_val = i64::MAX;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess:Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(&result_board, depth-1, true, alpha, beta. total);
            best_val = std::cmp::min(value, best_val);
            beta = std::cmp::min(beta, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    }
}

fn show_board(board: Board) {
    for (&rank, lbl) in ALL_RANKS.iter().zip("12345678".chars()) {
        print!("{}", lbl);
        print!(" ");
        for sq in get_rank(rank) {
            let piece = board.piece_on(sq);
            let sq_char = match board.color_on(sq) {
                Some(Color::Black) => match piece {
                    Some(Piece::King) => "",
                    Some(Piece::Queen) => "",
                    Some(Piece::Rook) => "",
                    Some(Piece::Bishop) => "",
                    Some(Piece::Knight) => "",
                    Some(Piece::Pawn) => "",
                    _ => "?"
                },
                Some(Color::White) => match piece {
                    Some(Piece::King) => "",
                    Some(Piece::Queen) => "",
                    Some(Piece::Rook) => "",
                    Some(Piece::Bishop) => "",
                    Some(Piece::Knight) => "",
                    Some(Piece::Pawn) => "",
                    _ => "?"
                },
                _ => "."
            };
            print!("{} ", sq_chat);
        }
        print!("\n");
    }
    println!("  a b c d e f g h");
}

fn find_best_move(board: &Board, depth: i8) -> Option<ChessMove> {
    let black_move = board.side_to_move() == Color::Black;
    let moves = MoveGen::new_legal(board).nth(0);
    let mut best_val;
    let is_better = {
        if black_move {
            best_val = i64::MIN;
            |x: i64, y: i64| -> bool { x < y }
        }
    };
    let mut total = 0;
    for mv in moves {
        let mut new_board = Board::default();
        board.make_move(mv, &mut new_board);
        let val = alpha_beta(&new_board, depth, black_move, i64::MIN, i64::MAX);
        if is_better(val, best_val) {
            best_val = val;
            best_move = Some(mv);
        }
    }
    best_move
}

fn parse(
        input: &Vec<String>,
        ) -> Result<(bool, bool, bool, String, i8), ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Print the usage menu");
    args.flag("i", "interactive", "Run the engine in interactive mode");
    args.flag("s", "selfplay", "Run the engine in self play mode");
    args.flag("b", "interactive", "Run Benchmarks");
    args.flag("d", "depth", "Set the depth of the tree search. Default 4",
        "DEPTH", Occur::Req, Some("4".to_string())
    );
    args.flag("f", "fen", "The State of the game as FEN",
            "FEN", Occur::Optionnal, Some(STARTING_FEN.to_string())
    );
    args.parse(input)?;
    let is_help = args.value_of("help")?;
    if is_help {
        args.full_usage();
    }
    let is_interactive = args.value_of("interactive")?;
    let is_self_play = args.value_of("selfplay")?;
    let run_benchmarks = args.value_of("bench")?;
    let fen_str: String = args.value_of("fen")?;
    let play_count: i8 = args.value_of::<String>("depth")?.parse::<i8>().unwrap();
    println!("Depth: {}", ply_count);
    Ok((is_interactive, is_self_play, run_benchmarks, fen_str, ply_count));
}

fn exec_ai_turn(board: &mut Board, ply_count: i8) {
    match find_best_move(board, ply_count) {
        Some(n) => { *board = board.make_move_new(n) }
        None => { println!("ERROR: No move found") }
    }
    println!("----------------");
    show_board(*board);
}

fn interactive_loop(mut board: Board, ply_count: i8) {
    let mut ai_turn = true;
    loop {
        if ai_turn {
            exec_ai_turn(&mut board, ply_count);
        } else {
            println!("Your turn");
            exec_user_turn(&mut board, ply_count);
        
        }
        ai_turn = !ai_turn;
    }
}

fn self_play_loop(mut board: Board, ply_count: i8) {
    loop {
        if board.status() == BoardStatus::Ongoing {
            exec_ai_turn(&mut board, ply_count);
        } else {
            return;
        }
    }
}

fn run_bench() {
    println!("name\tdepth\tduration");
    for (name, fen) in benchmarks::cases {
        let start = Instant::now();
        let mut duration = 0;
        match Board::from_str(fen) {
            Ok(board) => {
                for &depth in benchmarks::depths {
                    find_best_move(&board, depth);
                    duration = start.elapsed().as_millis();
                    println!("{}\t{}\t{}", name, depth, duration);
                }
            }
            Err(_) => { }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (is_interactive, is_self_play, run_benchmarks, fen_str, ply_count) = parse(&args).unwrap();

    if run_benchmarks {
        run_bench();
        return;
    }
    let board = match Board::from_str(fen_str.as_str()) {
        Ok(b) => b,
        Err(_) => {
            println!("Bad Fen");
            return;
        }
    };

    if is_self_play {
        self_play_loop(board, ply_count);
        println!("Game Over");
        return;
    }

    if !is_interactive {
        match find_best_move(&board, ply_count) {
            Some(n) => { println!("Best move: {}", n) },
            None => { println!("ERROR: No move found") },
        }
    } else {
        interactive_loop(board, ply_count);
    }
}
