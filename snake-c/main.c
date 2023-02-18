#include <SDL2/SDL.h>

#include "game.h"

void InitSDL() {}

int main(int argc, char **argv) {
    srand(time(NULL));

    int renderer_flags = SDL_RENDERER_ACCELERATED;
    int window_flags = 0;

    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        printf("Could not initialize SDL: %s\n", SDL_GetError());
        exit(1);
    }

    Size board_size = (Size){.width = 16, .height = 16};
    Size cell_size = (Size){.width = 40, .height = 40};
    SDL_Window *window = SDL_CreateWindow(
        "Snake", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
        board_size.width * cell_size.width,
        board_size.height * cell_size.height, window_flags);
    if (!window) {
        printf("Failed to open window: %s\n", SDL_GetError());
        exit(2);
    }

    SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");

    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, renderer_flags);
    if (!renderer) {
        printf("Failed to create renderer: %s\n", SDL_GetError());
        exit(3);
    }

    Game game;
    Game_Init(&game, board_size, cell_size);

    Game_Redraw(&game, renderer);
    SDL_RenderPresent(renderer);

    while (1) {
        int end_loop = 0;
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            switch (event.type) {
            case SDL_QUIT:
                end_loop = 1;
                break;

            case SDL_KEYDOWN:
                switch (event.key.keysym.sym) {
                case SDLK_ESCAPE:
                    end_loop = 1;
                    break;
                case SDLK_UP:
                    Game_SetDirection(&game, DIR_UP);
                    break;
                case SDLK_DOWN:
                    Game_SetDirection(&game, DIR_DOWN);
                    break;
                case SDLK_LEFT:
                    Game_SetDirection(&game, DIR_LEFT);
                    break;
                case SDLK_RIGHT:
                    Game_SetDirection(&game, DIR_RIGHT);
                    break;
                }
            default:
                break;
            }
        }

        if (end_loop) {
            break;
        }

        switch (Game_Poll(&game)) {
        case GAME_REDRAW:
            Game_Redraw(&game, renderer);
            break;
        case GAME_END:
            exit(0);
            break;
        case GAME_CONTINUE:
            break;
        }

        SDL_RenderPresent(renderer);
        SDL_Delay(16);
    }

    return 0;
}
