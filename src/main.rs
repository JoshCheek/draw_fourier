use std::io::{self};
use std::time::Duration;
use std::convert::TryInto;
use std::cmp::Ordering;

// extern crate json;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;

extern crate svg;
use svg::node::element::path::Position::{Relative, Absolute};
use svg::node::element::path::{Command, Data};
use svg::parser;

extern crate lyon_path;
use lyon_path::Path;
use lyon_path::iterator::*;
use lyon_path::builder::*;
use lyon_path::builder::{SvgBuilder, SvgPathBuilder};
use lyon_path::math::{point, vector};
use lyon_path::{FlattenedEvent};

extern crate lyon_geom;
use lyon_geom::{LineSegment};
use lyon_geom::euclid::TypedPoint2D;


fn draw(coefficients: Vec<(f64, f64)>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width  = 800;
    let height = 600;

    let window = video_subsystem.window("Fourier Drawing", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut iteration = 0;
    let len = coefficients.len();
    let mut drawn : Vec<(i32, i32)> = Vec::new();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        iteration += 1;
        iteration %= len;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut mag_and_angle: Vec<(f64, f64, f64, f64)> = Vec::new();
        for (i, (re, im)) in coefficients.iter().enumerate() {
            let angle = 2.0*3.14159*(i as f64)*(iteration as f64)/(len as f64);
            let magnitude = (re*re + im*im).sqrt();
            mag_and_angle.push((*re, *im, angle, magnitude));
        }

        mag_and_angle.sort_by(|(_,_,_,mag1), (_,_,_,mag2)|
            if mag1 < mag2 { Ordering::Greater } else { Ordering::Less }
        );

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let half_width  : f64 = (width/2).try_into().unwrap();
        let half_height : f64 = (height/2).try_into().unwrap();
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for (re, im, angle, mag) in mag_and_angle {
            let c = angle.cos();
            let s = angle.sin();
            let new_x = sum_x + c*re + s*im;
            let new_y = sum_y + c*im - s*re;

            // println!("{}, {}", sum_x, sum_y);
            canvas.set_draw_color(Color::RGB(100, 100, 100));

            // I can't figure out the type of the canvas, so I can't pass it as an arg
            // and I can't figure out how to get the circle drawing methods to work
            // on it either (this trait shit is confusing)
            let n = 50;
            let x = sum_x * half_height + half_width;
            let y = sum_y * half_height + half_height;
            let radius = mag * half_height;
            if 1.0 < radius {
                for i in 0..n {
                    let angle1 = 2.0*3.14159*((i+1) as f64)/(n as f64);
                    let angle2 = 2.0*3.14159*(i as f64)/(n as f64);
                    match canvas.draw_line(
                        Point::new(
                            (x + radius*angle1.cos()) as i32,
                            (y + radius*angle1.sin()) as i32,
                        ),
                        Point::new(
                            (x + radius*angle2.cos()) as i32,
                            (y + radius*angle2.sin()) as i32,
                        ),
                    ) { Ok(..) | Err(..) => () }
                }
            }

            canvas.set_draw_color(Color::RGB(255, 0, 0));
            match canvas.draw_line(
                Point::new(
                    (sum_x * half_height + half_width ) as i32,
                    (sum_y * half_height + half_height) as i32,
                ),
                Point::new(
                    (new_x * half_height + half_width) as i32,
                    (new_y * half_height + half_height) as i32,
                ),
            ) { Ok(..) | Err(..) => () }

            sum_x = new_x;
            sum_y = new_y;
        }


        drawn.push((
            (sum_x * half_height + half_width) as i32,
            (sum_y * half_height + half_height) as i32,
        ));

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 1..drawn.len() {
            let (x1, y1) = drawn[i-1];
            let (x2, y2) = drawn[i];
            match canvas.draw_line(
                Point::new(x1, y1),
                Point::new(x2, y2),
            ) { Ok(..) | Err(..) => () }
        }

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }
}

fn draw2(points: Vec<(f64, f64)>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width  = 800;
    let height = 600;
    let half_width  : f64 = (width/2).try_into().unwrap();
    let half_height : f64 = (height/2).try_into().unwrap();

    let window = video_subsystem.window("Fourier Drawing", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in 1..points.len() {
            let (x1, y1) = points[i-1];
            let (x2, y2) = points[i];
            match canvas.draw_line(
                Point::new(
                    (x1 * half_height + half_width) as i32,
                    (y1 * half_height + half_height) as i32,
                ),
                Point::new(
                    (x2 * half_height + half_width) as i32,
                    (y2 * half_height + half_height) as i32,
                ),
            ) { Ok(..) | Err(..) => () }
        }

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }
}


fn normalize(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    // get min and max values
    let mut min_x = 0f64;
    let mut max_x = 0f64;
    let mut min_y = 0f64;
    let mut max_y = 0f64;
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            min_x = *x;
            max_x = *x;
            min_y = *y;
            max_y = *y;
        } else {
            if *x < min_x { min_x = *x; }
            if *x > max_x { max_x = *x; }
            if *y < min_y { min_y = *y; }
            if *y > max_y { max_y = *y; }
        }
    }

    // convert to 0 .. 0.5 on x and y axis (assume same aspect ratio)
    let delta_x = max_x - min_x;
    let delta_y = max_y - min_y;
    let side    = if delta_x < delta_y { delta_y } else { delta_x };

    let mut new_points = Vec::new();
    for (x, y) in points {
        new_points.push((
            (x - min_x) / side - 0.5,
            (y - min_y) / side - 0.5,
        ));
    }

    new_points
}

fn calculate_dist(point1 : (f64, f64), point2 : (f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    (delta_x*delta_x + delta_y*delta_y).sqrt()

}

fn sample_points(line: Vec<(f64, f64)>, num_samples: usize) -> Vec<(f64, f64)> {
    let mut len = 0.0;

    for i in 1..line.len() {
        len += calculate_dist(line[i-1], line[i]);
    }

    let sample_dist = len / num_samples as f64;
    let mut points  = line.to_vec();
    points.reverse();
    let mut current = points.pop().unwrap();
    let mut samples = Vec::new();
    samples.push(current);

    while 0 < points.len() {
      let mut next_point = points.last().unwrap();
      let mut point_dist = calculate_dist(current, *next_point);
      let mut dist = sample_dist;

      while point_dist < dist && 2 < points.len() {
        dist      -= point_dist;
        current    = points.pop().unwrap();
        next_point = points.last().unwrap();
        point_dist = calculate_dist(current, *next_point);
      }

      let delta_x = (next_point.0-current.0)*dist/point_dist;
      let delta_y = (next_point.1-current.1)*dist/point_dist;
      current = (current.0+delta_x, current.1+delta_y);
      samples.push(current);
    }

    samples
}

fn fft(samples : Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let n = samples.len() as f64;
    let mut coefficients = Vec::new();
    for u in 0..samples.len() {
      let mut real = 0.0;
      let mut imaginary = 0.0;
      for (k, (f_real, f_imaginary)) in samples.iter().enumerate() {
        let p = -2.0*3.14159*(u as f64)*(k as f64)/n;
        let c = p.cos();
        let s = p.sin();
        real      += c*f_real      + s*f_imaginary;
        imaginary += c*f_imaginary - s*f_real;
      }
      coefficients.push((real/n, imaginary/n));
    }
    coefficients
}


fn main() {
    let mut builder = SvgPathBuilder::new(Path::builder());

    for event in svg::read(io::stdin()).unwrap() {
        match event {
            parser::Event::Tag(svg::node::element::tag::Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        Command::Move(position, params) => {
                            match position {
                                Relative => builder.relative_move_to(vector(params[0], params[1])),
                                Absolute => builder.move_to(point(params[0], params[1])),
                            }
                        },
                        Command::Line(position, params) => {
                            match position {
                                Relative => builder.relative_line_to(vector(params[0], params[1])),
                                Absolute => builder.line_to(point(params[0], params[1])),
                            }
                        },
                        Command::HorizontalLine(position, params) => {
                            println!("HorizontalLine({:?}, {:?})", position, params)
                        },
                        Command::VerticalLine(position, params) => {
                            println!("VerticalLine({:?}, {:?})", position, params)
                        },
                        Command::QuadraticCurve(position, params) => {
                            println!("QuadraticCurve({:?}, {:?})", position, params)
                        },
                        Command::SmoothQuadraticCurve(position, params) => {
                            println!("SmoothQuadraticCurve({:?}, {:?})", position, params)
                        },
                        Command::CubicCurve(position, params) => {
                            match position {
                                Relative => builder.relative_cubic_bezier_to(
                                    vector(params[0], params[1]),
                                    vector(params[2], params[3]),
                                    vector(params[4], params[5]),
                                ),
                                Absolute => builder.cubic_bezier_to(
                                    point(params[0], params[1]),
                                    point(params[2], params[3]),
                                    point(params[4], params[5]),
                                )
                            }
                        },
                        Command::SmoothCubicCurve(position, params) => {
                            println!("SmoothCubicCurve({:?}, {:?})", position, params)
                        },
                        Command::EllipticalArc(position, params) => {
                            println!("EllipticalArc({:?}, {:?})", position, params)
                        },
                        Command::Close => {
                            builder.close()
                        },
                    }
                }
            }
            _ => {}
        }
    }


    let path = builder.build();



    let num_samples = 200 as usize;
    // let samples = normalize(
    //     &sample_points(
    //         path.iter().flattened(0.01).map(|event|
    //             match event {
    //                 FlattenedEvent::MoveTo(TypedPoint2D { x, y, _unit }) => {
    //                     // println!("MoveTo({}, {})", x, y);
    //                     (x as f64, y as f64)
    //                 },
    //                 FlattenedEvent::Line(LineSegment { from: _, to: TypedPoint2D { x, y, _unit } }) => {
    //                     // println!("Line({}, {}, {})", f, x, y);
    //                     (x as f64, y as f64)
    //                 },
    //                 FlattenedEvent::Close(LineSegment { from: _, to: TypedPoint2D { x, y, _unit } }) => {
    //                     // println!("Close({}, {}, {})", f, x, y);
    //                     (x as f64, y as f64)
    //                 },
    //             }
    //         ).collect(),
    //         num_samples
    //     )
    // );
    let mut samples = Vec::new();
    for event in path.iter().flattened(0.1) {
        match event {
            FlattenedEvent::MoveTo(TypedPoint2D { x, y, _unit }) => {
                // println!("MoveTo({}, {})", x, y);
                samples.push((x as f64, y as f64));
            },
            FlattenedEvent::Line(LineSegment { from: _, to: TypedPoint2D { x, y, _unit } }) => {
                // println!("Line({}, {}, {})", f, x, y);
                samples.push((x as f64, y as f64));
            },
            FlattenedEvent::Close(LineSegment { from: _, to: TypedPoint2D { x, y, _unit } }) => {
                // println!("Close({}, {}, {})", f, x, y);
                samples.push((x as f64, y as f64));
            },
        }
    }

    // draw2(normalize(&samples));
    draw(fft(normalize(&samples)));
}
