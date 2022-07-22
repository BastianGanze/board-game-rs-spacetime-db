use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter, Read};
use std::time::Instant;

use crate::board::{Board, Player};
use crate::games::ataxx::{AtaxxBoard, Move};
use crate::interface::uai::command::{Command, GoTimeSettings, Position};

pub fn run(
    mut bot: impl FnMut(&AtaxxBoard, u32) -> (Move, String),
    name: &str,
    author: &str,
    input: impl Read,
    output: impl Write,
    log: impl Write,
) -> std::io::Result<()> {
    // wrap everything
    let input = &mut BufReader::new(input);
    let output = &mut BufWriter::new(output);
    let log = &mut BufWriter::new(log);

    //warmup
    bot(&AtaxxBoard::default(), 1000);

    let mut line = String::new();
    let mut curr_board = None;

    loop {
        log.flush()?;
        output.flush()?;

        line.clear();
        input.read_line(&mut line)?;
        let line = line.trim();
        writeln!(log, "> {}", line).unwrap();
        println!("> {}", line);

        let command = match Command::parse(line) {
            Ok(command) => command,
            Err(_) => {
                writeln!(output, "info < failed to parse command '{}'", line)?;
                continue;
            }
        };

        let command = Command::parse(line).unwrap_or_else(|_| panic!("Failed to parse command '{}'", line));

        match command {
            Command::Uai => {
                writeln!(output, "id name {}", name)?;
                writeln!(output, "id author {}", author)?;
                writeln!(output, "uaiok")?;
            }
            Command::IsReady => {
                writeln!(output, "readyok")?;
            }
            Command::SetOption { name, value } => {
                writeln!(
                    output,
                    "info < ignoring command setoption, name={}, value={}",
                    name, value
                )?;
            }
            Command::NewGame => {
                curr_board = Some(AtaxxBoard::default());
            }
            Command::Position(position) => {
                curr_board = Some(match position {
                    Position::StartPos => AtaxxBoard::default(),
                    Position::Fen(fen) => AtaxxBoard::from_fen(fen).unwrap(),
                });
            }
            Command::Go(time_settings) => {
                let curr_board = match curr_board.as_ref() {
                    Some(curr_board) => curr_board,
                    None => {
                        writeln!(output, "info < received go command without having a board",)?;
                        continue;
                    }
                };

                let time_to_use = match time_settings {
                    GoTimeSettings::Move(time) => time * 95 / 100,
                    GoTimeSettings::Clock { w_time, b_time, .. } => {
                        let time_left = match curr_board.next_player() {
                            Player::A => w_time,
                            Player::B => b_time,
                        };
                        time_left / 30
                    }
                };

                writeln!(log, "time_to_use: {}", time_to_use)?;

                let start = Instant::now();
                let (best_move, info) = bot(curr_board, time_to_use);
                let time_used = (Instant::now() - start).as_secs_f32();

                writeln!(log, "best_move: {:?}, time_used: {}, {}", best_move, time_used, info)?;
                writeln!(output, "bestmove {}", best_move.to_uai())?;
            }
            Command::Quit => {
                //nothing to do here
            }
        }
    }
}
