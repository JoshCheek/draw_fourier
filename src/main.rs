use std::io::{self, Read};
use std::process::exit;
use std::time::Duration;
use std::convert::TryInto;
use std::cmp::Ordering;

extern crate svg2polylines;
use svg2polylines::{Polyline};

extern crate rustfft;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

// extern crate json;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

fn draw(coefficients: Vec<Complex<f64>>) {
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
        for (i, Complex { re, im }) in coefficients.iter().enumerate() {
            let angle = 2.0*3.14159*(i as f64)*(iteration as f64)/(len as f64);
            let magnitude = (re*re + im*im).sqrt();
            mag_and_angle.push((*re, *im, angle, magnitude));
        }

        mag_and_angle.sort_by(|(_,_,_,mag1), (_,_,_,mag2)|
            if mag1 < mag2 { Ordering::Less } else { Ordering::Greater }
        );

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let half_width  : f64 = (width/2).try_into().unwrap();
        let half_height : f64 = (height/2).try_into().unwrap();
        let mut sum_x = half_width;
        let mut sum_y = half_height;
        for (re, im, angle, _mag) in mag_and_angle {
            let x = sum_x + (half_width  * re * angle.cos());
            let y = sum_y + (half_height * im * angle.sin());

            match canvas.draw_line(
                Point::new(sum_x  as i32, sum_y  as i32),
                Point::new(x as i32, y as i32),
            ) { Ok(..) | Err(..) => () }

            sum_x = x;
            sum_y = y;
        }

        drawn.push((sum_x as i32, sum_y as i32));

        canvas.set_draw_color(Color::RGB(0, 255, 255));
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
                    (x1 * half_width + half_width) as i32,
                    (y1 * half_height + half_height) as i32,
                ),
                Point::new(
                    (x2 * half_width + half_width) as i32,
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

    // convert to -0.5 .. 0.5 on x and y axis (assume same aspect ratio)
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

    for _i in 1..num_samples {
      let mut next_point = points.last().unwrap();
      let mut point_dist = calculate_dist(current, *next_point);
      let mut dist = sample_dist;

      if point_dist < sample_dist {
        dist       = sample_dist - point_dist;
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

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let polylines: Vec<Polyline> = svg2polylines::parse(&buffer).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);
    });


    // current expectation: there is only one line
    for line in polylines {
        let num_samples = 1000 as usize;
        let samples = normalize(
            &sample_points(
                line.into_iter().map(|pair| (pair.x, pair.y)).collect(),
                num_samples
            )
        );

        let mut input:  Vec<Complex<f64>> = vec![Complex::zero(); num_samples];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); num_samples];

        for (i, (x, y)) in samples.iter().enumerate() {
            input[i] = Complex::new(*x, *y);
        }

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(num_samples);
        fft.process(&mut input, &mut output);

        // draw(output);
        draw2(samples);
    }

}
