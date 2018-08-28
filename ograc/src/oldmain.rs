// main.rs
// Aldaron's OS
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern crate adi;

use adi::{ Window, Input, Sprite, Style, Transform, Key };

struct Context {
	window: Window,
	bubble: Sprite,
}

fn redraw(context: &mut Context) {
	let disp2 = context.window.pulse_full_linear(2.0);

	context.window.background(disp2, disp2, disp2);

	for i in 0..2 {
		for j in 0..3 {
			let x = (j as f32) / 1.5;
			let y = (i as f32);
			Transform::create()
				.translate(x - (2.0 / 3.0), y - 0.5, 0.0)
				.scale(0.4, 1.0 / 3.0, 1.0)
				.orthographic(&mut context.window)
				.apply(&mut context.window, &context.bubble,
					i + (j * 2));
		}
	}
}

fn main() {
	let mut window = Window::create("project_name",
		include_bytes!("res/logo.ppm"), &[]);
	// TODO: add something other than subtransparent, that ALSO changes the
	// opacity of the entire image.
	let style = Style::create().subtransparent(&mut window,
		include_bytes!("res/bubble.ppm"), (0, 0, 0));
	
	let bubble = Sprite::create(&mut window, &include!("res/bubble.data"),
		style, 6);

	let mut context = Context {
		window: window,
		bubble: bubble,
	};

	loop {
		let input = context.window.update();
		match input {
			Input::Back => break,
			Input::Redraw => redraw(&mut context),
			_ => {},
		}
		// Run the update functions
		// context.player.update(&mut context.window, input);
	}
}
