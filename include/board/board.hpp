#pragma once
#include <array>
#include <iostream>

class Board
{
public:
    static size_t const ROW = 3;
    static size_t const COL = 3;
    Board(std::array<std::array<char, Board::COL>, Board::ROW> board);
    Board(void);
    std::array<std::array<char, Board::COL>, Board::ROW> get_board(void);
    char find_winner(void);
    void print_board(void);
    void clean_board(void);
protected:
    std::array<std::array<char, Board::COL>, Board::ROW> board = {};
    bool is_full_board(void);
    bool find_winner(char symbol);
};