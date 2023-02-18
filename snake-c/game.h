#pragma once

#include <SDL2/SDL.h>
#include <time.h>

#include "snake.h"

struct Game {
    Board board;
    Size cell_size;
    Uint32 speed;
    Uint32 latest_move;
    Direction current_direction;
    Direction next_direction;
};
typedef struct Game Game;

enum GamePollResult {
    GAME_CONTINUE,
    GAME_REDRAW,
    GAME_END,
};
typedef enum GamePollResult GamePollResult;

void Game_Init(Game *game, Size board_size, Size cell_size);
void Game_Close(Game *game);
void Game_SetDirection(Game *game, Direction dir);
GamePollResult Game_Poll(Game *game);
void Game_Redraw(const Game *game, SDL_Renderer *renderer);
