use crate::strategy::Strategy;

pub trait MiniMax: Strategy {
    fn minimax(&mut self, depth: i64,alpha: f64, beta: f64, is_maximizing: bool) -> f64;
    fn best_movement(&mut self, depth: i64) -> <Self as Strategy>::Movement;
}

impl <T: Strategy> MiniMax for T {
    fn minimax(&mut self, depth: i64, mut alpha: f64, mut beta: f64, is_maximizing: bool) -> f64{
        let available: Vec<<T as Strategy>::Movement> = self.available_movements();
        if depth == 0 || self.completed_game() || available.is_empty() {
            return self.evaluate();
        }
        let mut max_eval: f64;
        if is_maximizing {
            max_eval = f64::NEG_INFINITY;
            available.iter().for_each(|mv| {
                self.play(mv, true);
                let eval = self.minimax(depth-1, alpha, beta, false);
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
                self.play(mv, false);
                let eval = self.minimax(depth-1, alpha, beta, true);
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

    fn best_movement(&mut self, depth: i64) -> <Self as Strategy>::Movement {
        let mut best_movement: <Self as Strategy>::Movement = self.invalid_movement();
        if self.completed_game() {
            return best_movement;
        }
        let alpha: f64 = f64::NEG_INFINITY;
        let beta: f64 = f64::INFINITY;
        let mut max_eval = f64::NEG_INFINITY;
        self.available_movements().into_iter().for_each(|mv| {
            self.play(&mv, true);
            let eval = self.minimax(depth, alpha, beta, false);
            self.redo(&mv);
            if max_eval <= eval {
                max_eval = eval;
                best_movement = mv;
            }
        });
        best_movement
    }
}
