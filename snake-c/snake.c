#include <stdlib.h>
#include <string.h>

#include "snake.h"

static int Coord_Equals(const Coord *left, const Coord *right) {
    return left->x == right->x && left->y == right->y;
}

// static void DumpSnake(Snake *snake) {
//     unsigned snake_length =
//         (snake->head_idx - snake->tail_idx) % snake->ring_buffer_size + 1;
//     printf("Snake [%d]", snake_length);
//     for (unsigned i = 0; i < snake_length; ++i) {
//         unsigned idx = (snake->head_idx - i) % snake->ring_buffer_size;
//         Coord cell = snake->ring_buffer[idx];
//         printf(" >(%d)[%d, %d]", idx, cell.x, cell.y);
//     }
//     printf("\n");
// }

static void Snake_Init(Snake *snake, unsigned length) {
    const unsigned buffer_size = 8;
    snake->ring_buffer_size = buffer_size;
    snake->ring_buffer = malloc(buffer_size * sizeof(Coord));
    snake->tail_idx = 0;
    snake->head_idx = length - 1;
    for (unsigned i = 0; i < length; ++i) {
        snake->ring_buffer[i] = (Coord){.x = i, .y = 0};
    }
}

static void Snake_Close(Snake *snake) { free(snake->ring_buffer); }

// return the index of `cell` in the ring buffer or -1 if not found
static int Snake_FindCell(Snake *snake, Coord cell) {
    unsigned idx = snake->head_idx;
    for (;;) {
        if (Coord_Equals(&cell, &snake->ring_buffer[idx])) {
            return idx;
        }
        if (idx == snake->tail_idx) {
            return -1;
        }
        idx = (idx - 1) % snake->ring_buffer_size;
    }
}

/// return non-zero value iff no collision (with the board boundaries)
static int MoveCell(Coord *cell, Direction direction, Size board_size) {
    switch (direction) {
    case DIR_UP:
        cell->y += 1;
        return cell->y < board_size.height;

    case DIR_DOWN:
        cell->y -= 1;
        return cell->y >= 0;

    case DIR_LEFT:
        cell->x -= 1;
        return cell->x >= 0;

    case DIR_RIGHT:
        cell->x += 1;
        return cell->x < board_size.width;
    }

    return 0;
}

static void Board_MoveFood(Board *board) {
    Coord new_food = (Coord){.x = rand() % board->size.width,
                             .y = rand() % board->size.height};
    if (Snake_FindCell(&board->snake, new_food) != -1) {
        Board_MoveFood(board);
    } else {
        board->food = new_food;
    }
}

void Board_Init(Board *board, Size size, unsigned snake_length) {
    board->size = size;
    Snake_Init(&board->snake, snake_length);
    board->food = (Coord){.x = 0, .y = 1};
    Board_MoveFood(board);
}

void Board_Close(Board *board) { Snake_Close(&board->snake); }

// return non-zero value iff no collision
static int Board_Extend(Board *board, Direction dir) {
    Snake *snake = &board->snake;
    Coord head = board->snake.ring_buffer[board->snake.head_idx];
    if (MoveCell(&head, dir, board->size) == 0) {
        // collision
        return 0;
    }

    if (Snake_FindCell(snake, head) != -1) {
        // self-collision
        return 0;
    }

    unsigned next_head_idx = (snake->head_idx + 1) % snake->ring_buffer_size;
    if (next_head_idx == snake->tail_idx) {
        // buffer is full
        // re-allocate
        unsigned buffer_size = snake->ring_buffer_size << 1;
        Coord *buffer =
            realloc(snake->ring_buffer, buffer_size * sizeof(Coord));
        if (!buffer) {
            printf("Failed to allocate memory\n");
            exit(1);
        }
        if (snake->head_idx < snake->tail_idx) {
            memcpy(&buffer[snake->ring_buffer_size], buffer,
                   snake->tail_idx * sizeof(Coord));
            snake->head_idx += snake->ring_buffer_size;
            next_head_idx = (snake->head_idx + 1) % buffer_size;
        }
        snake->ring_buffer_size = buffer_size;
        snake->ring_buffer = buffer;
    }

    snake->ring_buffer[next_head_idx] = head;
    snake->head_idx = next_head_idx;

    return 1;
}

// return 0 on collision, 1 otherwize
int Board_Next(Board *board, Direction dir) {
    if (Board_Extend(board, dir) == 0) {
        // collision
        return 0;
    }

    if (!Coord_Equals(&board->snake.ring_buffer[board->snake.head_idx],
                      &board->food)) {
        // not food -> remove tail
        board->snake.tail_idx =
            (board->snake.tail_idx + 1) % board->snake.ring_buffer_size;
    } else {
        // food -> MoveFood
        Board_MoveFood(board);
    }
    return 1;
}
