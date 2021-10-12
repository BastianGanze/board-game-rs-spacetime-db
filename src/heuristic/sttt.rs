use std::cmp::max;

use crate::ai::minimax::Heuristic;
use crate::ai::solver::SolverHeuristic;
use crate::board::Board;
use crate::games::sttt::{Coord, STTTBoard};

#[derive(Debug)]
pub struct STTTTileHeuristic {
    oo_factors: [i32; 3],
    macro_factor: i32,
}

impl Default for STTTTileHeuristic {
    fn default() -> Self {
        STTTTileHeuristic {
            oo_factors: [1, 3, 4],
            macro_factor: 1000,
        }
    }
}

impl Heuristic<STTTBoard> for STTTTileHeuristic {
    type V = i32;

    fn value(&self, board: &STTTBoard, length: u32) -> i32 {
        // done
        if board.is_done() {
            return SolverHeuristic.value(board, length).to_i32();
        }

        // tile
        let tile_value = Coord::all()
            .map(|c| {
                self.oo_factor(c.om())
                    * self.oo_factor(c.os())
                    * board.tile(c).map_or(0, |p| p.sign(board.next_player()))
            })
            .sum::<i32>();

        // macro
        let macr_value = (0..9)
            .map(|om| self.oo_factor(om) * board.macr(om).map_or(0, |p| p.sign(board.next_player())))
            .sum::<i32>()
            * self.macro_factor;

        tile_value + macr_value
    }

    fn value_update(
        &self,
        board: &STTTBoard,
        board_value: i32,
        board_length: u32,
        mv: Coord,
        child: &STTTBoard,
    ) -> i32 {
        // win
        if board.outcome().is_some() {
            return self.value(board, board_length + 1);
        }

        let mut neg_child_value = board_value;

        // tile
        neg_child_value += self.oo_factor(mv.om()) * self.oo_factor(mv.os());

        // macro
        if child.macr(mv.om()).is_some() {
            neg_child_value += self.macro_factor * self.oo_factor(mv.om());
        }

        -neg_child_value
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, bool) {
        (max(old, new), new >= old)
    }
}

impl STTTTileHeuristic {
    fn oo_factor(&self, oo: u8) -> i32 {
        let index = match oo {
            1 | 3 | 5 | 7 => 0,
            0 | 2 | 6 | 8 => 1,
            4 => 2,
            _ => panic!("Invalid oo value {}", oo),
        };
        self.oo_factors[index]
    }
}
