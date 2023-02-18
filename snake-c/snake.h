#pragma once

#include <SDL2/SDL.h>

enum Direction {
    DIR_UP,
    DIR_DOWN,
    DIR_LEFT,
    DIR_RIGHT,
};
typedef enum Direction Direction;

struct Coord {
    short x;
    short y;
};
typedef struct Coord Coord;

struct Size {
    short width;
    short height;
};
typedef struct Size Size;

struct Snake {
    Coord *ring_buffer;
    unsigned ring_buffer_size;
    unsigned head_idx;
    unsigned tail_idx;
};
typedef struct Snake Snake;

struct Board {
    Size size;
    Snake snake;
    Coord food;
};
typedef struct Board Board;

void Board_Init(Board *board, Size size, unsigned snake_length);
void Board_Close(Board *board);
int Board_Next(Board *board, Direction dir);
void Board_Draw(Board *board, SDL_Renderer *renderer);
