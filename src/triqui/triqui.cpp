#include "triqui/triqui.hpp"
#include "algorithm/algorithm.hpp"
#include <iostream>
#include <string>

#if defined _WIN32
#include <windows.h>
#endif

void clean_console(void)
{
#if defined _WIN32
    /* Extracted from https://www.cplusplus.com/forum/general/273223/ */
    HANDLE hStdOut;
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    DWORD count;
    DWORD cellCount;
    COORD homeCoords = {0, 0};

    hStdOut = GetStdHandle(STD_OUTPUT_HANDLE);
    if (hStdOut == INVALID_HANDLE_VALUE)
        return;

    /* Get the number of cells in the current buffer */
    if (!GetConsoleScreenBufferInfo(hStdOut, &csbi))
        return;
    cellCount = csbi.dwSize.X * csbi.dwSize.Y;

    /* Fill the entire buffer with spaces */
    if (!FillConsoleOutputCharacter(
            hStdOut,
            (TCHAR)' ',
            cellCount,
            homeCoords,
            &count))
        return;

    /* Fill the entire buffer with the current colors and attributes */
    if (!FillConsoleOutputAttribute(
            hStdOut,
            csbi.wAttributes,
            cellCount,
            homeCoords,
            &count))
        return;

    /* Move the cursor home */
    SetConsoleCursorPosition(hStdOut, homeCoords);
#else
    std::printf("\033c");
#endif
}

void wait_on_console(void)
{
    std::printf("Oprime cualquier tecla para continuar...");
    std::cin.get();
}

void Triqui::player_turn(void)
{
    std::int_fast64_t square;
    do
    {
        std::printf("\nDigita una casilla: ");
        std::cin >> square;
    } while (square < 1 || square > 9);

    square--;
    size_t col = square % 3, row = square / 3;
    if (!board[row][col])
        board[row][col] = Triqui::player_symbol;
    else
    {
        std::cout << "\nLa casilla " << square + 1 << " ya está en juego. Vuelve a intentarlo\n";
        player_turn();
    }
}

void Triqui::ai_turn(void)
{
    std::pair<size_t, size_t> pos = AI::best_movement(this);
    board[pos.first][pos.second] = Triqui::ai_symbol;
}

void Triqui::start_game(void)
{
    clean_console();
    std::printf("¡Bienvenido a triki! \
            \n~~~~~~~~~~~~~~~~~~~~~ \
            \nIngrese la posición en la cual desea poner su movimiento. \
            \nLas posiciones son: \
            \n[1] [2] [3] \
            \n[4] [5] [6] \
            \n[7] [8] [9] \n");
    wait_on_console();
    char winner_symbol;
    std::string str;
    while (true)
    {
        while (true)
        {
            clean_console();
            print_board();
    
            player_turn();
            winner_symbol = find_winner();
            if (winner_symbol)
                break;
    
            ai_turn();
            winner_symbol = find_winner();
            if (winner_symbol)
                break;
    
        }
        clean_console();
        print_board();
        if (winner_symbol == ' ')
            std::printf("\n¡Fin del juego! \
                    \nCulminó en empate\n");
        else
            std::printf("\n¡Fin del juego!\
                    \nEl ganador fue %s\n",
                        (winner_symbol == Triqui::player_symbol) ? "el jugador" : "la máquina");
        std::printf("¿Desea continuar jugando? ");
        std::cin >> str;
        if (str=="no" || str=="No" || str=="n" || str=="N")
            break;
        clean_board();
    }
}
