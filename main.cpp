/*
	Relative: A music Video Game
	Copyright (C) 2024 A. Gar <>
*/

#include "SDL.h"
#include <stdlib.h>
#include <unistd.h>

#define R_GAME_WINDOW_TITLE "rElAtiVe"
#define R_GAME_WINDOW_W 400
#define R_GAME_WINDOW_H 320
#define R_PIXEL_WIDTH 4
#define R_PIXEL_BITDEPTH R_PIXEL_WIDTH * 8


#define internal static
#define persist static
#define global static

global bool running;

global SDL_Surface *bitmap_surface;
global SDL_Window  *game_window;
global SDL_Surface *gw_surface;
global void        *bitmap_mem;

internal void rel_update_gw(void)
{
	if (gw_surface)
	{
		SDL_FreeSurface(gw_surface);
	}
	gw_surface = SDL_GetWindowSurface(game_window);
	if (bitmap_surface)
	{
		SDL_BlitSurface(bitmap_surface, NULL, gw_surface, NULL);
	}
	SDL_UpdateWindowSurface(game_window);
}

internal void rel_resize_gw(int w, int h)
{
	uint8_t *row;
	uint8_t *pixel;
	int pitch = R_PIXEL_WIDTH * w;

	if (bitmap_mem)
	{
		free(bitmap_mem);
	}
	if (bitmap_surface)
	{
		SDL_FreeSurface(bitmap_surface);
	}

	posix_memalign(&bitmap_mem, getpagesize(), w * h * R_PIXEL_WIDTH);
	row = (uint8_t *) bitmap_mem;
	for (int y = 0; y < h; ++y)
	{
		pixel = row;
		for (int x = 0; x < w; ++x)
		{
			*pixel++ = (uint8_t) x;
			*pixel++ = (uint8_t) y;
			*pixel++ = (uint8_t) x * y;
			*pixel++ = (uint8_t) 0;
		}
		row += pitch;
	}
	bitmap_surface = SDL_CreateRGBSurfaceFrom(bitmap_surface, w, h,
			R_PIXEL_BITDEPTH, R_PIXEL_WIDTH * w, 0, 0, 0, 0);
	bitmap_surface->pixels = bitmap_mem;
	rel_update_gw();
}

void rel_first_bitmap(SDL_Window *game_window, SDL_Surface *gw_surface)
{
	uint8_t *pixel;
	uint8_t *row;
	int     pitch = R_PIXEL_WIDTH * R_GAME_WINDOW_W;

	posix_memalign(&bitmap_mem, getpagesize(), R_GAME_WINDOW_W *
			R_GAME_WINDOW_H * R_PIXEL_WIDTH);
	row  = (uint8_t *) bitmap_mem;
	/* bitmap_mem = malloc(R_PIXEL_WIDTH * w * h); */
	for (int y = 0; y < R_GAME_WINDOW_H; ++y)
	{
		pixel = row;
		for (int x = 0; x < R_GAME_WINDOW_W; ++x)
		{
			*pixel++ = (uint8_t) x;
			*pixel++ = (uint8_t) y;
			*pixel++ = (uint8_t) y * x;
			*pixel++ = 0x00;
		}
		row += pitch;
	}
	bitmap_surface = SDL_CreateRGBSurfaceFrom(bitmap_surface, R_GAME_WINDOW_W,
			R_GAME_WINDOW_H, R_PIXEL_BITDEPTH, R_PIXEL_WIDTH * R_GAME_WINDOW_W,
			0, 0, 0, 0);
	bitmap_surface->pixels = bitmap_mem;
	rel_update_gw();
}

int main(int argc, char **argv)
{
	SDL_Event       event        = {};

	SDL_InitSubSystem(SDL_INIT_EVERYTHING);
	game_window = SDL_CreateWindow(R_GAME_WINDOW_TITLE, 0, 0, R_GAME_WINDOW_W,
			R_GAME_WINDOW_H, SDL_WINDOW_RESIZABLE | SDL_WINDOW_UTILITY);
	gw_surface  = SDL_GetWindowSurface(game_window);

	rel_first_bitmap(game_window, gw_surface);

	running = true;
	while (running)
	{
		while (SDL_PollEvent(&event) > 0)
		{
			if (event.type == SDL_QUIT)
			{
				running = false;
				break ;
			}
			if (event.type == SDL_WINDOWEVENT
				&& event.window.event == SDL_WINDOWEVENT_SIZE_CHANGED)
			{
				rel_resize_gw(event.window.data1, event.window.data2);
			}
			else
				break ;
		}
		SDL_UpdateWindowSurface(game_window);
	}
	SDL_Quit();
	exit(EXIT_SUCCESS);
}
