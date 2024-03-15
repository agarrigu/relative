/*
	Relative: A music Video Game
	Copyright (C) 2024 A. Gar <>
*/

#include "SDL.h"
#define R_GAME_WINDOW_TITLE "rElAtiVe"
#define R_GAME_WINDOW_W 400
#define R_GAME_WINDOW_H 320

#define internal static
#define persist static

#define ui8 uint8_t
#define ui16 uint16_t
#define ui32 uint32_t
#define ui64 uint64_t

#define i8 int8_t
#define i16 int16_t
#define i32 int32_t
#define i64 int64_t

/*
 * TODO(al):
 *  - add audio buffer
 *  - add clean exit on input
 *  - add controller stuff
 * */
int
main(int argc, char **argv)
{
	SDL_Window	*game_window;
	SDL_Surface *gw_surface;

	SDL_InitSubSystem(SDL_INIT_EVERYTHING);
	game_window = SDL_CreateWindow(R_GAME_WINDOW_TITLE,
			0, 0,
			R_GAME_WINDOW_W,
			R_GAME_WINDOW_H,
			SDL_WINDOW_RESIZABLE
			);
	/* pixels are accesible through gw_surface->pixels :) */
	gw_surface = SDL_GetWindowSurface(game_window);
	SDL_Quit();
	exit(EXIT_SUCCESS);
}
