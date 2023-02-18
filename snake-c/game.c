

#include "game.h"

static Direction Direction_Invert(Direction dir) {
    switch (dir) {
    case DIR_UP:
        return DIR_DOWN;
    case DIR_DOWN:
        return DIR_UP;
    case DIR_LEFT:
        return DIR_RIGHT;
    case DIR_RIGHT:
        return DIR_LEFT;
    }
}

void Game_Init(Game *game, Size board_size, Size cell_size) {
    Board_Init(&game->board, board_size, 3);
    game->cell_size = cell_size;
    game->speed = 600;
    game->latest_move = SDL_GetTicks();
    game->current_direction = DIR_RIGHT;
    game->next_direction = DIR_RIGHT;
}

void Game_Close(Game *game) { Board_Close(&game->board); }

void Game_SetDirection(Game *game, Direction dir) {
    if (dir != Direction_Invert(game->current_direction)) {
        if (dir == game->current_direction) {
            game->latest_move = SDL_GetTicks() - game->speed;
        }
        game->next_direction = dir;
    }
}

GamePollResult Game_Poll(Game *game) {
    if (SDL_GetTicks() - game->latest_move > game->speed) {
        game->latest_move += game->speed;
        game->current_direction = game->next_direction;

        if (Board_Next(&game->board, game->current_direction) == 0) {
            return GAME_END;
        } else {
            return GAME_REDRAW;
        }
    } else {
        return GAME_CONTINUE;
    }
}

static void Game_GetRect(const Game *game, const Coord *cell,
                         SDL_Rect *into_rect) {
    into_rect->x = game->cell_size.width * cell->x + 1;
    into_rect->y =
        (game->board.size.height - 1 - cell->y) * game->cell_size.height + 1;
    into_rect->w = game->cell_size.width - 2;
    into_rect->h = game->cell_size.height - 2;
}

void Game_Redraw(const Game *game, SDL_Renderer *renderer) {
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
    SDL_RenderClear(renderer);

    const Board *board = &game->board;
    SDL_Rect rect;

    // food
    SDL_SetRenderDrawColor(renderer, 120, 0, 0, 255);
    Game_GetRect(game, &board->food, &rect);
    SDL_RenderFillRect(renderer, &rect);

    // snake
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
    const Snake *snake = &board->snake;
    unsigned snake_length =
        (snake->head_idx - snake->tail_idx) % snake->ring_buffer_size + 1;
    for (unsigned i = 0; i < snake_length; ++i) {
        unsigned idx = (snake->head_idx - i) % snake->ring_buffer_size;
        Game_GetRect(game, &snake->ring_buffer[idx], &rect);
        SDL_RenderFillRect(renderer, &rect);
    }
}
