/*
	Relative: A music Video Game
	Copyright (C) 2024 A. Gar <>
*/

#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <unistd.h>

int main(void)
{
	Display *display = XOpenDisplay(nullptr);
	Window root_window = XDefaultRootWindow(display);

	int win_x = 0;
	int win_y = 0;
	int win_w = 800;
	int win_h = 600;
	int win_border_w = 0;
	int win_depth = CopyFromParent;
	int win_class = CopyFromParent;
	Visual *win_visual = CopyFromParent;

	int attr_value_mask = CWBackPixel | CWEventMask;
	XSetWindowAttributes win_attrs = {};
	win_attrs.background_pixel = 0xffeecc22;
	win_attrs.event_mask = StructureNotifyMask | KeyPressMask
			| KeyReleaseMask | ExposureMask;

	Window window = XCreateWindow(display, root_window, win_x, win_y,
			win_w, win_h, win_border_w, win_depth, win_class, win_visual,
			attr_value_mask, &win_attrs);
	
	XMapWindow(display, window);

	for(bool running = true; running;)
	{
		XEvent event = {};
		XNextEvent(display, &event);

		switch(event.type)
		{
			case KeyPress:
			case KeyRelease:
			{
				XKeyPressedEvent *kpevent = (XKeyPressedEvent *) &event;
				if (kpevent->keycode == XKeysymToKeycode(display, XK_Escape))
					running = false;
			}
			break;
		}
	}

	return (0);
}
