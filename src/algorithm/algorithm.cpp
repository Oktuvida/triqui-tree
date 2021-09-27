#include "algorithm/algorithm.hpp"
#include "triqui/triqui.hpp"
#include <limits>
#include <iostream>
#include <algorithm>
#include <map>

std::int_fast64_t AI::minimax(Board *board, std::int_fast64_t alpha, std::int_fast64_t beta, bool is_maximizing)
{
    std::map<char, std::int_fast8_t> valores = {
        {Triqui::player_symbol, -1},
        {Triqui::ai_symbol, 1},
        {' ', 0},
    };
    char winner_symbol = board->find_winner();
    if (winner_symbol)
        return valores[winner_symbol];
    auto new_board = board->get_board();
    std::int_fast64_t max_eval;
    if (is_maximizing)
    {
        max_eval = std::numeric_limits<std::int_fast64_t>::min();
        for (auto &row : new_board)
            for (auto &square : row)
                if (!square)
                {
                    square = Triqui::ai_symbol;
                    std::int_fast64_t eval = minimax(new Board(new_board), alpha, beta, false);
                    max_eval = std::max(max_eval, eval);
                    alpha = std::max(alpha, eval);
                    square = '\0';
                    if (beta <= alpha) {
                        return max_eval;
                    }
                }
    }
    else
    {
        max_eval = std::numeric_limits<std::int_fast64_t>::max();
        for (auto &row : new_board)
            for (auto &square : row)
                if (!square)
                {
                    square = Triqui::player_symbol;
                    std::int_fast64_t eval = minimax(new Board(new_board), alpha, beta, true);
                    max_eval = std::min(max_eval, eval);
                    beta = std::min(beta, eval);
                    square = '\0';
                    if (beta <= alpha) {
                        return max_eval;
                    }
                }
    }
    return max_eval;
}

std::pair<size_t, size_t> AI::best_movement(Board *board)
{
    std::pair<size_t, size_t> pos;
    auto new_board = board->get_board();
    std::int_fast64_t best_mv = std::numeric_limits<std::int_fast64_t>::min();
    std::int_fast64_t alpha = std::numeric_limits<std::int_fast64_t>::min();
    std::int_fast64_t beta = std::numeric_limits<std::int_fast64_t>::max();
    for (size_t i = 0; i < Board::ROW; i++)
        for (size_t j = 0; j < Board::COL; j++)
            if (!new_board[i][j])
            {
                new_board[i][j] = Triqui::ai_symbol;
                std::int_fast64_t mov = minimax(new Board(new_board), alpha, beta, false);
                new_board[i][j] = '\0';
                best_mv = std::max(best_mv, mov);
                if (best_mv == mov)
                    pos = std::make_pair(i, j);
            }
    return pos;
}