/*
	Relative: A music Video Game
	Copyright (C) 2024 A. Gar <>
*/

#include "SDL2/SDL.h"
#define R_GAME_WINDOW_TITLE "rElAtiVe"
#define R_GAME_WINDOW_W 400
#define R_GAME_WINDOW_H 320

int
main(int argc, char **argv)
{
	SDL_Window	*game_window;

	SDL_InitSubSystem(SDL_INIT_EVERYTHING);
	game_window = SDL_CreateWindow(
		R_GAME_WINDOW_TITLE,
		0, 0, R_GAME_WINDOW_W, R_GAME_WINDOW_H,
		SDL_WINDOW_RESIZABLE
		);
	return (0);
}
