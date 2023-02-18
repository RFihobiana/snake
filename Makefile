
build/main.o: snake-c/main.c snake-c/game.h snake-c/snake.h
	gcc -c -o build/main.o snake-c/main.c

build/snake.o: snake-c/snake.c snake-c/snake.h
	gcc -c -o build/snake.o snake-c/snake.c

build/game.o: snake-c/game.c snake-c/game.h snake-c/snake.h
	gcc -c -o build/game.o snake-c/game.c

build/snake: build/main.o build/snake.o build/game.o
	gcc -o build/snake build/main.o build/snake.o build/game.o -lSDL2 -lSDL2main

clean:
	rm build/*

run: build/snake
	build/snake

