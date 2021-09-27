#pragma once
#include "board/board.hpp"
#include <array>

void clean_console(void);
void wait_on_console(void);

class Triqui : public Board
{
public:
    static char const player_symbol = 'X';
    static char const ai_symbol = 'O';
    static std::array<char, 2> constexpr symbols_played = {player_symbol, ai_symbol};
    void start_game(void);
private:
    void player_turn(void);
    void ai_turn(void);
};