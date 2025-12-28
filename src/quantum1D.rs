use crate::quantum_class::{base_new, gaussian_pulse, harmonic_oscillator, WaveVector};
use crate::screen_config::{correct_coord_tuple, draw_line, scatter_plot, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let (start, end, res) = (-2., 2., 80);
    let (center, std_dev) = (-1., 0.1);
    let base = base_new(start, end, res);
    let values_0 = gaussian_pulse(&base, center, std_dev);
    let mass = 1.;
    let omega = 9.;
    let potential = base.map(|x| harmonic_oscillator(*x, mass, omega));
    let mut wave_vector = WaveVector::new(start, end, res, base, values_0, mass);

    //let mut wave_vector = Arc::new(Mutex::new(wave_vector));

    let mut center = (0, 0);
    let mut zoom = 200.;

    let vertical_scale = 1.;
    const DT: f32 = 0.003;

    // let other_wave = Arc::clone(&wave_vector);
    // thread::spawn(move || {
    //     loop {
    //         let mut this_wave = other_wave.lock().unwrap();
    //         this_wave.update_rk4(&potential, DT);
    //         thread::sleep(Duration::from_secs_f32(0.001));
    //     }
    // });

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        let mouse = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                        zoom *= 1.2;
                        println!("zoom factor: {}", zoom);
                    }

                    zoom *= 0.9;
                    println!("zoom factor: {}", zoom);
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

        draw_line(&mut canvas, (-100., 0.), (100., 0.), center, zoom);
        draw_line(&mut canvas, (start, -10.), (start, 10.), center, zoom);
        draw_line(&mut canvas, (end, -10.), (end, 10.), center, zoom);
        //let this_wave = wave_vector.lock().unwrap();
        wave_vector.update_rk4(&potential, DT);
        scatter_plot(
            &mut canvas,
            &wave_vector.base,
            &wave_vector.probability,
            true,
            center,
            zoom,
            vertical_scale,
        );

        let test_point = correct_coord_tuple((0., 10.), center, zoom);

        canvas
            .draw_rect(Rect::new(test_point.0, test_point.1, 10, 10))
            .unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
