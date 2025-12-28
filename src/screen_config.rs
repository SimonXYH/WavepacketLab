use crate::draw3d::Scene3d;
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use ndarray::{Array1, Array2};
use num_complex::{Complex32, ComplexFloat};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 700;

pub fn correct_coord(pos: Vec2<f32>, center: (i32, i32), zoom: f32) -> Vec2<i32> {
    Vec2::new(
        (((pos.x + center.0 as f32) * zoom) + WIDTH as f32 / 2.) as i32,
        (((pos.y + center.1 as f32) * zoom) + HEIGHT as f32 / 2.) as i32, //potential negative sign here
    )
}

pub fn correct_coord_tuple(pos: (f32, f32), center: (i32, i32), zoom: f32) -> (i32, i32) {
    (
        ((pos.0 + center.0 as f32) * zoom + WIDTH as f32 / 2.) as i32,
        (-(pos.1 + center.1 as f32) * zoom + HEIGHT as f32 / 2.) as i32,
    )
}

pub fn draw_point_3d(canvas: &mut WindowCanvas, pos_corrected: Vec2<i32>) {
    canvas
        .draw_point(Point::new(pos_corrected.x, pos_corrected.y))
        .unwrap()
}
pub fn draw_line(
    canvas: &mut WindowCanvas,
    start: (f32, f32),
    end: (f32, f32),
    center: (i32, i32),
    zoom: f32,
) {
    canvas
        .draw_line(
            correct_coord_tuple(start, center, zoom),
            correct_coord_tuple(end, center, zoom),
        )
        .unwrap()
}

pub fn draw_line3d(
    canvas: &mut WindowCanvas,
    scene3d: &Scene3d,
    start: Vec3,
    end: Vec3,
    center: (i32, i32),
    zoom: f32,
) {
    let point1 = scene3d.project_correct(start, center, zoom).into_tuple();
    let point2 = scene3d.project_correct(end, center, zoom).into_tuple();
    canvas.draw_line(point1, point2).unwrap()
}

pub fn draw_axis_3d(
    canvas: &mut WindowCanvas,
    scene3d: &Scene3d,
    axis_num: usize,
    axis_start: f32,
    axis_end: f32,
    center: (i32, i32),
    zoom: f32,
) {
    let (start, end) = match axis_num {
        0 => (Vec3::new(axis_start, 0., 0.), Vec3::new(axis_end, 0., 0.)),
        1 => (Vec3::new(0., axis_start, 0.), Vec3::new(0., axis_end, 0.)),
        2 => (Vec3::new(0., 0., axis_start), Vec3::new(0., 0., axis_end)),
        _ => panic!("Invalid axis"),
    };
    draw_line3d(canvas, scene3d, start, end, center, zoom)
}

pub fn draw_axis_all(
    canvas: &mut WindowCanvas,
    scene3d: &Scene3d,
    axis_start: f32,
    axis_end: f32,
    center: (i32, i32),
    zoom: f32,
) {
    for i in (0..3) {
        draw_axis_3d(canvas, scene3d, i, axis_start, axis_end, center, zoom)
    }
}

pub fn draw_envelope(
    mut canvas: &mut WindowCanvas,
    scene3d: &Scene3d,
    grid: &Vec<Vec<(f32, f32)>>,
    values: &Array2<Complex32>,
    center: (i32, i32),
    zoom: f32,
    vertical_scale: f32,
    kind: String,
) {
    let (rows, cols) = values.dim();
    for i in 0..rows {
        for j in 0..cols - 1 {
            let grid_point1 = grid[i][j];
            let grid_point2 = grid[i][j + 1];
            let (value1, value2) = match kind.as_str() {
                "re" => (
                    values[[i, j]].re * vertical_scale,
                    values[[i, j + 1]].re * vertical_scale,
                ),
                "im" => (
                    values[[i, j]].im * vertical_scale,
                    values[[i, j + 1]].im * vertical_scale,
                ),
                "mag" => (
                    values[[i, j]].abs().powi(2) * vertical_scale,
                    values[[i, j + 1]].abs().powi(2) * vertical_scale,
                ),
                _ => panic!("Must be either re, im, or mag"),
            };
            let start = Vec3::new(grid_point1.0, grid_point1.1, value1);
            let end = Vec3::new(grid_point2.0, grid_point2.1, value2);
            draw_line3d(canvas, scene3d, start, end, center, zoom)
        }
    }
    for i in 0..rows - 1 {
        for j in 0..cols {
            let grid_point1 = grid[i][j];
            let grid_point2 = grid[i + 1][j];
            let (value1, value2) = match kind.as_str() {
                "re" => (
                    values[[i, j]].re * vertical_scale,
                    values[[i + 1, j]].re * vertical_scale,
                ),
                "im" => (
                    values[[i, j]].im * vertical_scale,
                    values[[i + 1, j]].im * vertical_scale,
                ),
                "mag" => (
                    values[[i, j]].abs().powi(2) * vertical_scale,
                    values[[i + 1, j]].abs().powi(2) * vertical_scale,
                ),
                _ => panic!("Must be either re, im, or mag"),
            };
            let start = Vec3::new(grid_point1.0, grid_point1.1, value1);
            let end = Vec3::new(grid_point2.0, grid_point2.1, value2);
            draw_line3d(canvas, scene3d, start, end, center, zoom)
        }
    }
}

pub fn scatter_plot(
    mut canvas: &mut WindowCanvas,
    x_array: &Array1<f32>,
    y_array: &Array1<f32>,
    join: bool,
    center: (i32, i32),
    zoom: f32,
    vertical_scale: f32,
) {
    if join {
        for i in 0..x_array.len() - 1 {
            let point1 =
                correct_coord_tuple((x_array[i], y_array[i] * vertical_scale), center, zoom);
            let point2 = correct_coord_tuple(
                (x_array[i + 1], y_array[i + 1] * vertical_scale),
                center,
                zoom,
            );
            canvas.draw_point(Point::new(point1.0, point1.1)).unwrap();
            //println!("hello")
            canvas.draw_line(point1, point2).unwrap()
        }
    } else {
        for i in 0..x_array.len() {
            let point =
                correct_coord_tuple((x_array[i], y_array[i] * vertical_scale), center, zoom);
            //  canvas.draw_point(Point::new(point.0, point.1)).unwrap()
        }
    }
}
