#include "board/board.hpp"
#include "algorithm/algorithm.hpp"
#include "triqui/triqui.hpp"
#include <iostream>

Board::Board(std::array<std::array<char, Board::COL>, Board::ROW> board)
{
    this->board = board;
}

Board::Board(void) {}

std::array<std::array<char, Board::COL>, Board::ROW> Board::get_board(void)
{
    return board;
}

bool Board::find_winner(char symbol)
{
    char c = symbol;
    for (size_t i = 0; i < Board::ROW; i++)
        if (
            (board[i][0] == c && board[i][1] == c && board[i][2] == c) || (board[0][i] == c && board[1][i] == c && board[2][i] == c))
            return true;
    return (
        (board[0][0] == c && board[1][1] == c && board[2][2] == c) || (board[2][0] == c && board[1][1] == c && board[0][2] == c));
}

void Board::print_board(void)
{
    for (auto &i : board)
    {
        for (auto j : i)
            std::printf("[%c] ", (!j) ? ' ' : j);
        std::printf("\n");
    }
}

void Board::clean_board(void)
{
    board = {};
}

bool Board::is_full_board(void)
{
    int counter = 0;
    for (auto &i : board)
        for (auto j : i)
            if (j)
                counter++;
    return counter == Board::ROW * Board::COL;
}

char Board::find_winner(void)
{
    for (auto i : Triqui::symbols_played)
        if (find_winner(i))
            return i;
    return (is_full_board()) ? ' ' : '\0';
}