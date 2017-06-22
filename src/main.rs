extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate gfx_device_gl;

use std::rc::Rc;
use std::path::PathBuf;

use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
};

use gear::Gear;
use gear::Rack;

use std::f64::consts::PI;

mod gear;

fn load_sprite_from_texture(window: &mut PistonWindow,
                            assets: &PathBuf,
                            target: &str) -> Sprite<Texture<gfx_device_gl::Resources>> {
    let tex = Rc::new(Texture::from_path(
        &mut window.factory,
        assets.join(target),
        Flip::None,
        &TextureSettings::new()
    ).unwrap());
    Sprite::from_texture(tex.clone())
}

fn main() {
    // Initialize the window
    let (width, height) = (540, 360);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("RustTv", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    // Get a reference to the assets folder
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    // Sprites values
    let gear_logo = Gear {
        pitch_circle_radius: 69.0,
        addendum: 3.0,
        teeth_num: 32
    };
    let gear_epicyc = Gear {
        pitch_circle_radius: 33.0,
        addendum: 3.0,
        teeth_num: 16
    };

    let rack = Rack {
        width: 386.0,
        height: 27.0,
        addendum: 3.0,
        teeth_num: 32,
    };

    // Create the drawing scene
    let mut scene = Scene::new();

    // Create background sprite
    let mut bg_sprite_white = load_sprite_from_texture(&mut window, &assets, "bg_white.png");
    bg_sprite_white.set_position(width as f64 / 2.0, height as f64 / 2.0);
    bg_sprite_white.set_opacity(0.0);
    let bg_white = scene.add_child(bg_sprite_white);

    // Create the gear logo sprites
    let mut gear_sprite_black = load_sprite_from_texture(&mut window, &assets, "rust.png");
    gear_sprite_black.set_position(width as f64 / 2.0,
                                   height as f64 / 2.0);
    gear_sprite_black.set_opacity(0.0);
    let gear_black = scene.add_child(gear_sprite_black);

    let mut gear_sprite_white = load_sprite_from_texture(&mut window, &assets, "rust_white.png");
    gear_sprite_white.set_position(width as f64 / 2.0 + rack.width,
                                   height as f64 / 2.0);
    let gear_white = scene.add_child(gear_sprite_white);

    // Create the linear rack sprites
    let mut rack_sprite_white = load_sprite_from_texture(&mut window, &assets, "rust_rack_white.png");
    let h_offset = rack.width / 2.0 - rack.pitch();
    let rack_gap = 2.0; // to improve appearance
    let v_offset = gear_logo.pitch_circle_radius + rack.height / 2.0 - rack.addendum * 2.0 + rack_gap;
    rack_sprite_white.set_position(width as f64 / 2.0 + h_offset,
                                   height as f64 / 2.0 + v_offset);
    rack_sprite_white.set_opacity(0.0);
    let rack_white = scene.add_child(rack_sprite_white);

    // Create the epicycloid gear
    let mut gear_sprite_epicyc = load_sprite_from_texture(&mut window, &assets, "rust_colosseum.png");
    let dist = gear_logo.pitch_circle_radius
        + gear_epicyc.pitch_circle_radius
        - gear_logo.addendum;
    //- gear_epicyc.addendum;
    let angle = PI / 3.68;
    let h_offset = dist * f64::cos(angle);
    let v_offset = dist * f64::sin(angle);
    gear_sprite_epicyc.set_position(width as f64 / 2.0 + h_offset,
                                    height as f64 / 2.0 + v_offset);
    gear_sprite_epicyc.set_opacity(0.0);
    let gear_epicyc = scene.add_child(gear_sprite_epicyc);


    // Animation for the white gear
    let epicyc_time = 0.0;//2.0;
    let animation_time = 1.0;//6.0;
    let start_pause_time = 0.5;
    let fade_time = 0.8;
    let seq_rotate = Sequence(vec![
        Wait(start_pause_time + fade_time),
        Action(RotateTo(animation_time, -360.0)),
        Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeOut(fade_time)))),
    ]);
    scene.run(gear_white, &seq_rotate);
    // run in parallel with the previous
    let seq_animation = Sequence(vec![
        Wait(start_pause_time + fade_time),
        Action(MoveBy(animation_time, -rack.width, 0.0)),
    ]);
    scene.run(gear_white, &seq_animation);

    // Animation for the white rack
    let seq_animation = Sequence(vec![
        Wait(start_pause_time),
        Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(fade_time)))),
        Wait(animation_time),
        Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeOut(fade_time)))),
    ]);
    scene.run(rack_white, &seq_animation);

    // Animation for the black gear and black background
    let seq_animation = Sequence(vec![
        Wait(start_pause_time + fade_time + animation_time),
        Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(fade_time)))),
    ]);
    scene.run(gear_black, &seq_animation);
    scene.run(bg_white, &seq_animation);

    // Animation for the epicycloid gear
    let seq_animation = Sequence(vec![
        Wait(start_pause_time + fade_time + animation_time + epicyc_time),
        Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(fade_time)))),
    ]);
    scene.run(gear_epicyc, &seq_animation);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            //            clear([0.5, 0.5, 0.5, 1.0], g);
            clear([0.0, 0.0, 0.0, 1.0], g);
            //              clear([1.0, 1.0, 1.0, 1.0], g);

            scene.draw(c.transform, g);
        });
    }
}
