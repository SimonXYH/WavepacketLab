use crate::drag_point::DragPoint;
use crate::draw3d::Scene3d;
use crate::quantum_class2d::{
    create_grid, gaussian2d_pulse, DoubleSlit, RampPotential, WaveVector2D,
};
use crate::screen_config::{draw_axis_all, draw_envelope, draw_point_3d, HEIGHT, WIDTH};
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use ndarray::Array2;
use num_complex::Complex32;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut center = (0, 0);
    let mut zoom_factor = 50.;

    let (a_start, a_end) = (-5., 5.);

    let scene_center = Vec2::new(0., 0.);
    let scene_zoom = 1.;

    let mut scene3d = Scene3d::new(scene_center, scene_zoom, 30., (2.0, 3.0));

    let mut drag_point = DragPoint::new((2, 3), 1.);

    let (width, height) = (30., 30.);
    let shape = (80, 80);
    let grid = create_grid((0.0, 0.0), width, height, shape);
    let (dx, dy) = (width / shape.0 as f32, height / shape.1 as f32);

    let values = gaussian2d_pulse((0.3, 1.0), (5.0, 0.0), &grid);

    let mut wave_vector = WaveVector2D::new(grid.clone(), values, 1.);

    let double_slit = DoubleSlit::new()
        .slit_spacing(1.0)
        .slit_width(1.5)
        .height(3.)
        .center((0., 0.))
        .angle(0.0)
        .thickness(1.)
        .build();

    let ramp_thickness = 4.5;
    let ramp_gradient = 1.;
    let ramp_height = ramp_thickness * ramp_gradient;

    let ramp = RampPotential::new()
        .vertical_offset(ramp_height)
        .gradient(ramp_gradient)
        .width(3.)
        .thickness(ramp_thickness)
        .center((6.0, 0.0))
        .angle(0.)
        .build();

    let potential_vec: Vec<Vec<Complex32>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|grid_point| {
                    //harmonic_oscialltor_2d(grid_point.0, grid_point.1, (0., 0.), 1., 1.)+
                    double_slit.potential_value(grid_point.0, grid_point.1)
                        + ramp.potential_value(grid_point.0, grid_point.1)
                })
                .collect::<Vec<Complex32>>()
        })
        .collect();

    let potential = Array2::from_shape_vec(
        (grid.len(), grid[0].len()),
        potential_vec.into_iter().flatten().collect(),
    )
    .unwrap();

    const DT: f32 = 0.01;

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        let mouse = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            drag_point.event_update(&event, &mouse);
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                        zoom_factor *= 1.2;
                        println!("zoom factor: {}", zoom_factor);
                    }

                    zoom_factor *= 0.9;
                    println!("zoom factor: {}", zoom_factor);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => center.0 -= 2,
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => center.0 += 2,
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => center.1 += 2,
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => center.1 -= 2,
                _ => {}
            }
        }

        drag_point.frame_update(&event_pump.mouse_state());

        scene3d.angles = (
            drag_point.pos.0 as f32 / 100.,
            drag_point.pos.1 as f32 / 100.,
        );

        draw_axis_all(&mut canvas, &scene3d, a_start, a_end, center, zoom_factor);

        draw_point_3d(
            &mut canvas,
            scene3d.project_correct(Vec3::new(25., 0., 0.), center, zoom_factor),
        );
        draw_point_3d(
            &mut canvas,
            scene3d.project_correct(Vec3::new(0., 15., 0.), center, zoom_factor),
        );
        draw_point_3d(
            &mut canvas,
            scene3d.project_correct(Vec3::new(0., 0., 15.), center, zoom_factor),
        );

        // for inner_vec in &grid {
        //     for point in inner_vec {
        //         draw_point_3d(
        //             &mut canvas,
        //             scene3d.project_correct(Vec3::new(point.0, point.1, 0.), center, zoom_factor),
        //         )
        //     }
        // }

        draw_envelope(
            &mut canvas,
            &scene3d,
            &grid,
            &wave_vector.values,
            center,
            zoom_factor,
            150.,
            "mag".to_string(),
        );

        canvas.set_draw_color(Color::RGBA(0, 255, 255, 100));
        draw_envelope(
            &mut canvas,
            &scene3d,
            &grid,
            &potential,
            center,
            zoom_factor,
            0.5,
            "re".to_string(),
        );

        wave_vector.update_rk4(&potential, DT, dx, dy);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
