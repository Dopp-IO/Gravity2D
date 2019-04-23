mod physics;

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::{Events, EventSettings};
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::{RenderEvent, UpdateEvent, MouseCursorEvent, ButtonEvent, ButtonState};

fn main() {
    //program-wide variables
    let mut phys_objects: Vec<physics::PhysObject2d> = Vec::new();
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    let mut mouse_click_pos: [f64; 2] = [0.0, 0.0];
    let mut current_mass: f64 = 1000000000000000.0; //a bit less than Earth's mass
    let mut current_charge: f64 = 0.0;

    //graphics
    use graphics::*;

    let opengl = OpenGL::V3_2;

    let mut settings = WindowSettings::new("Gravity", [500, 500])
        .exit_on_esc(true)
        .opengl(opengl);

    let mut window: Window = settings.build()
        .expect("Could not create window");

    //GL must be initialized after the window is created or it will not know what to draw to
    let mut gl = GlGraphics::new(opengl);

    let mut render_settings = EventSettings::new();
    //default 500hz refresh
    render_settings.ups = 2000;
    //default 60hz render_settings
    render_settings.max_fps = 60;
    //disable lazy updating
    render_settings.lazy = false;

    let mut render_events = Events::new(render_settings);

    while let Some(e) = render_events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);

                for object in 0..phys_objects.len() {
                    rectangle([0.0, 0.0, 0.0, 1.0], rectangle::square(phys_objects[object].pos[0] - 5.0, phys_objects[object].pos[1] - 5.0, 10.0), c.transform, g);
                }
            });
        }
        if let Some(args) = e.update_args() {
            phys_objects = physics::phys_step(phys_objects, args.dt);
        }
        if let Some(args) = e.mouse_cursor_args() {
            mouse_pos = args;
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                mouse_click_pos = mouse_pos;
            } else {
                //Create new physics object
                phys_objects.push(physics::PhysObject2d::new(current_mass, mouse_click_pos, [(mouse_pos[0] - mouse_click_pos[0]) * 0.001, (mouse_pos[1] - mouse_click_pos[1]) * 0.001], 0.0));
            }
        }
    }
}
