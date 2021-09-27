#pragma once
#include "board/board.hpp"
#include "iostream"

namespace AI
{
    std::int_fast64_t minimax(Board *board, int_fast64_t alpha, int_fast64_t beta, bool is_maximizing);
    std::pair<size_t, size_t> best_movement(Board *board);
}