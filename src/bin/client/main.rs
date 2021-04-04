use hecs::World;
use glam::{Vec2, Mat3};
use anura::*;
use anura::components::{
    auto_attacker::{AutoAttacker, AutoAttackerTarget},
    auto_pather::{AutoPather, AutoPatherTarget},
    health_user::{HealthUser},
};
use std::sync::mpsc;
use std::thread;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (game_tx, graphics_rx) = mpsc::channel::<(String, String)>();
    let (input_tx, game_rx) = mpsc::channel::<(String, String)>();
    //game runs in its own thread
    thread::spawn(move || {
        let mut game = AnuraGame::default();
        let mut transform = Mat3::from_translation(Vec2::new(1.0, 3.0)) * Mat3::IDENTITY;
        println!("{:?}", transform);

        game.world.spawn((transform,) );

        let e = game.world.spawn((
            Mat3::from_angle(1.0) * Mat3::from_translation(Vec2::new(1.0, 0.0)),
            AutoAttacker {
                target: AutoAttackerTarget::NoTarget,
                speed: 1.0,
                range: 170.0,
                animation_time: 0.5,
                bullet_velocity: 150.0,
            },
            AutoPather {
                target: AutoPatherTarget::LocationTarget(Vec2::new(5.0, 5.0)),
                move_speed: 550.0,
            },
            HealthUser::new(500.0),
        ));

        //gotta connect this. Or have them running in different threads??? pretty neat idea
        game.run();
    });

    //window in this thread
    let mut events_loop = glutin::EventsLoop::new();
    let windowbuilder = glutin::WindowBuilder::new()
        .with_title("Anura".to_string())
        .with_dimensions(512, 512);
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(windowbuilder, contextbuilder, &events_loop);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        window.swap_buffers().unwrap();
        device.cleanup();
    }


}
