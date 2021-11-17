use crate::strategy::Strategy;
use crate::triqui::Triqui;

pub trait MiniMax {
    type Movement;
    fn minimax(
        &mut self,
        depth: i64,
        alpha: f64,
        beta: f64,
        is_maximizing: bool,
        board_state: i16,
    ) -> f64;
    fn best_movement(&mut self, depth: &i64) -> Self::Movement;
}

impl MiniMax for Triqui {
    type Movement = usize;
    fn minimax(
        &mut self,
        depth: i64,
        mut alpha: f64,
        mut beta: f64,
        is_maximizing: bool,
        board_state: i16,
    ) -> f64 {
        let available: Vec<Self::Movement> = self.available_movements();
        if board_state >= self.size as i16 * self.symbol_value(self.maximizer) {
            return 100.;
        }
        if board_state <= self.size as i16 * self.symbol_value(self.minimizer) {
            return -100.;
        }
        if depth == 0 || available.is_empty() {
            return 0.;
        }
        let mut max_eval: f64;
        if is_maximizing {
            max_eval = f64::NEG_INFINITY;
            available.iter().for_each(|mv| {
                let board_state = self.play(mv, true);
                let eval = self.minimax(depth - 1, alpha, beta, false, board_state);
                self.redo(mv);
                if max_eval < eval {
                    max_eval = eval;
                }
                if alpha < eval {
                    alpha = eval;
                }
                if beta <= alpha {
                    return;
                }
            });
        } else {
            max_eval = f64::INFINITY;
            available.iter().for_each(|mv| {
                let board_state = self.play(mv, false);
                let eval = self.minimax(depth - 1, alpha, beta, true, board_state);
                self.redo(mv);
                if max_eval > eval {
                    max_eval = eval;
                }
                if beta > eval {
                    beta = eval;
                }
                if beta <= alpha {
                    return;
                }
            });
        }
        max_eval
    }
    fn best_movement(&mut self, &depth: &i64) -> Self::Movement {
        let mut best_movement: Self::Movement = self.invalid_movement();
        let alpha: f64 = f64::NEG_INFINITY;
        let beta: f64 = f64::INFINITY;
        let mut max_eval = f64::NEG_INFINITY;
        self.available_movements().into_iter().for_each(|mv| {
            self.play(&mv, true);
            let eval = self.minimax(depth, alpha, beta, false, 0);
            self.redo(&mv);
            if max_eval < eval {
                max_eval = eval;
                best_movement = mv;
            }
        });
        best_movement
    }
}
