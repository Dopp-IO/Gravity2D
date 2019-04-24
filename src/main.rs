mod physics;

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::{Events, EventSettings};
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::{RenderEvent, UpdateEvent, MouseCursorEvent, ButtonEvent, ButtonState, Button, MouseButton};

fn main() {
    //program-wide variables
    let mut phys_objects: Vec<physics::PhysObject2d> = Vec::new();
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    let mut mouse_click_pos: [f64; 2] = [0.0, 0.0];
    let mut current_mass: f64 = 100000000000.0;
    let mut current_charge: f64 = 0.0;
    let mut draw_dir: bool = false;
    let mut drag_scale: f64 = 0.001;

    //colors
    let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

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
                //clear([1.0; 4], g);
                rectangle([1.0, 1.0, 1.0, 0.1], rectangle::square(0.0, 0.0, 1000.0), c.transform, g);

                //render all objectes in phys_objects
                for object in 0..phys_objects.len() {
                    rectangle(phys_objects[object].color, rectangle::square(phys_objects[object].pos[0] - 1.0, phys_objects[object].pos[1] - 1.0, 2.0), c.transform, g);
                }

                //draw force line
                if (draw_dir) {
                    line([1.0, 0.0, 0.0, 1.0], 0.5, [mouse_click_pos[0], mouse_click_pos[1], mouse_pos[0], mouse_pos[1]], c.transform, g);
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
                if args.button == Button::Mouse(MouseButton::Left) || args.button == Button::Mouse(MouseButton::Right){
                    mouse_click_pos = mouse_pos;
                    draw_dir = true;
                }
            } else {
                //Create new physics object
                //Left click for heavy object
                if args.button == Button::Mouse(MouseButton::Left) {
                    current_mass = 100000000000.0;
                    current_charge = 0.0;

                    phys_objects.push(physics::PhysObject2d::new(current_mass, mouse_click_pos, [(mouse_pos[0] - mouse_click_pos[0]) * drag_scale, (mouse_pos[1] - mouse_click_pos[1]) * drag_scale], current_charge, green));
                    draw_dir = false;
                }

                //Right click for light object
                if args.button == Button::Mouse(MouseButton::Right) {
                    current_mass = 10000.0;
                    current_charge = 0.0;

                    phys_objects.push(physics::PhysObject2d::new(current_mass, mouse_click_pos, [(mouse_pos[0] - mouse_click_pos[0]) * drag_scale, (mouse_pos[1] - mouse_click_pos[1]) * drag_scale], current_charge, blue));
                    draw_dir = false;
                }
            }
        }
    }
}
