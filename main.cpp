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

#define R_CONTROLLER_ZERO_PAD 13

#define R_AUDIO_FREQ 48000
#define R_AUDIO_CHANNELS 2
#define R_AUDIO_SMAPLES 4096
#define R_AUDIO_BUFFSIZE R_AUDIO_CHANNELS * R_AUDIO_SMAPLES

#define internal static
#define persist static
#define global static

#define r_bool32 int32_t

global bool running;

struct rel_bitmap_pixels
{
	int  w;
	int  h;
	void *data;
	int  pitch;
};

internal void rel_render_weird_gradiant(rel_bitmap_pixels bitmap,
		int x_off, int y_off)
{
	uint32_t *iter;

	iter = (uint32_t *) bitmap.data;
	for (int y = 0; y < bitmap.h; ++y)
	{
		for (int x = 0; x < bitmap.w; ++x)
		{
			uint8_t blue =  x + x_off;
			uint8_t green = y + y_off;
			uint8_t red = x + x_off;
			*iter++ = blue | (green << 8) | (red << 16);
		}
	}
}

internal void rel_update_gw_from_bm(SDL_Window *game_window,
		rel_bitmap_pixels bitmap)
{
	SDL_Surface *gw_surface;
	SDL_Surface *bm_surface;

	gw_surface = SDL_GetWindowSurface(game_window);
	bm_surface = SDL_CreateRGBSurfaceFrom(bitmap.data, bitmap.w, bitmap.h,
			R_PIXEL_BITDEPTH, R_PIXEL_WIDTH * bitmap.w, 0, 0, 0, 0);
	SDL_BlitSurface(bm_surface, NULL, gw_surface, NULL);
	SDL_UpdateWindowSurface(game_window);
	SDL_FreeSurface(gw_surface);
	SDL_FreeSurface(bm_surface);
}

internal void rel_resize_gw(SDL_Window *game_window, int w, int h,
		rel_bitmap_pixels *bitmap)
{
	if (bitmap->data)
		free(bitmap->data);
	bitmap->w = w;
	bitmap->h = h;
	bitmap->pitch = R_PIXEL_WIDTH * w;
	posix_memalign(&bitmap->data, getpagesize(), w * h * R_PIXEL_WIDTH);
	rel_render_weird_gradiant(*bitmap, 0, 0);
	rel_update_gw_from_bm(game_window, *bitmap);
}

int main(int argc, char **argv)
{
	SDL_Event          event;
	SDL_Window         *game_window;
	SDL_GameController *controller;
	SDL_AudioDeviceID  audiodev_id;
	SDL_AudioSpec      desired, obtained;
	rel_bitmap_pixels  bitmap;
	int16_t            audio_buff[R_AUDIO_BUFFSIZE];
	int                x_off, y_off;
	int                controller_id;
	bool               dpad_up;
	bool               dpad_down;
	bool               dpad_left;
	bool               dpad_right;
	bool               start;
	bool               back;
	bool               l_shoulder;
	bool               r_shoulder;
	bool               a_button;
	bool               b_button;
	bool               x_button;
	bool               y_button;
	int16_t            l_axis_x, l_axis_y;

	SDL_InitSubSystem(SDL_INIT_EVERYTHING);
	game_window = SDL_CreateWindow(R_GAME_WINDOW_TITLE, 0, 0, R_GAME_WINDOW_W,
								   R_GAME_WINDOW_H,
								   SDL_WINDOW_RESIZABLE | SDL_WINDOW_UTILITY);
	bitmap = {R_GAME_WINDOW_W, R_GAME_WINDOW_H, NULL,
			R_PIXEL_WIDTH * R_GAME_WINDOW_W};
	posix_memalign(&bitmap.data, getpagesize(), R_GAME_WINDOW_W *
			R_GAME_WINDOW_H * R_PIXEL_WIDTH);

	/* Audio Stuff */
	desired = {};
	obtained = {};
	desired.freq = R_AUDIO_FREQ;
	desired.format = AUDIO_S16;
	desired.channels = R_AUDIO_CHANNELS;
	desired.samples = R_AUDIO_SMAPLES;
	audiodev_id = SDL_OpenAudioDevice(NULL, 0, &desired, &obtained, 0);

	x_off = 0;
	y_off = 0;
	running = true;
	while (running)
	{
		for (int i = 0; i < SDL_NumJoysticks(); ++i)
			if (SDL_IsGameController(i))
				controller = SDL_GameControllerOpen(i);
		event = {};
		while (SDL_PollEvent(&event) > 0)
		{
			if (event.type == SDL_QUIT)
			{
				running = false;
				break ;
			}
			else if (event.type == SDL_WINDOWEVENT
				&& event.window.event == SDL_WINDOWEVENT_SIZE_CHANGED)
			{
				rel_resize_gw(game_window, event.window.data1,
						event.window.data2, &bitmap);
			}
			else if (event.type == SDL_CONTROLLERBUTTONDOWN)
			{
				dpad_up = false;
				dpad_down = false;
				dpad_left = false;
				dpad_right = false;
				start = false;
				back = false;
				l_shoulder = false;
				r_shoulder = false;
				a_button = false;
				b_button = false;
				x_button = false;
				y_button = false;

				switch (event.cbutton.button)
				{
					case SDL_CONTROLLER_BUTTON_DPAD_UP:       dpad_up = true;
					case SDL_CONTROLLER_BUTTON_DPAD_DOWN:     dpad_down = true;
					case SDL_CONTROLLER_BUTTON_DPAD_LEFT:     dpad_left = true;
					case SDL_CONTROLLER_BUTTON_DPAD_RIGHT:    dpad_right = true;
					case SDL_CONTROLLER_BUTTON_START:         start = true;
					case SDL_CONTROLLER_BUTTON_BACK:          back = true;
					case SDL_CONTROLLER_BUTTON_LEFTSHOULDER:  l_shoulder = true;
					case SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: r_shoulder = true;
					case SDL_CONTROLLER_BUTTON_A:             a_button = true;
					case SDL_CONTROLLER_BUTTON_B:             b_button = true;
					case SDL_CONTROLLER_BUTTON_X:             x_button = true;
					case SDL_CONTROLLER_BUTTON_Y:             y_button = true;
				}
			}
			else if (event.type == SDL_KEYDOWN)
			{
				switch (event.key.keysym.sym)
				{
					case SDLK_w: y_off -= 2;
					case SDLK_a: x_off -= 2;
					case SDLK_s: y_off += 2;
					case SDLK_d: x_off += 2;
					case SDLK_q:;
					case SDLK_e:;
					case SDLK_UP:;
					case SDLK_DOWN:;
					case SDLK_LEFT:;
					case SDLK_RIGHT:;
					case SDLK_ESCAPE: running = false;
				}
			}
			else if (event.type == SDL_CONTROLLERAXISMOTION)
			{
				if (event.caxis.axis == SDL_CONTROLLER_AXIS_LEFTX)
					l_axis_x = event.caxis.value;
				else if (event.caxis.axis == SDL_CONTROLLER_AXIS_LEFTY)
					l_axis_y = event.caxis.value;
				x_off += l_axis_x >> R_CONTROLLER_ZERO_PAD;
				y_off += l_axis_y >> R_CONTROLLER_ZERO_PAD;
			}
		}
		/* SDL_GameControllerRumble(controller, 2000, 60000, 200); */
		rel_render_weird_gradiant(bitmap, x_off, y_off);
		rel_update_gw_from_bm(game_window, bitmap);
		usleep(16667);
	}
	SDL_Quit();
	exit(EXIT_SUCCESS);
}
