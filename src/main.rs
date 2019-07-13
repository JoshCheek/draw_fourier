use std::io::{self, Read};
use std::process::exit;
use std::time::Duration;

use std::cmp::Ordering;

extern crate svg2polylines;
use svg2polylines::{CoordinatePair, Polyline};

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

fn draw_line(coefficients: Vec<Complex<f64>>) {
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
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        iteration = iteration + 1;
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

        let mut mag_and_angle: Vec<(f64, f64, f64, f64)> = Vec::new();
        for (i, Complex { re, im }) in coefficients.iter().enumerate() {
            let angle = 2.0*3.14159*(i as f64)*(iteration as f64);
            let magnitude = (re*re + im*im).sqrt();
            mag_and_angle.push((*re, *im, angle, magnitude));
        }

        mag_and_angle.sort_by(|(_,_,_,mag1), (_,_,_,mag2)|
            if mag1 < mag2 { Ordering::Less } else { Ordering::Greater }
        );

        // let mut sum_x = width/2;
        // let mut sum_y = height/2;
        // for (re, im, angle, _mag) in mag_and_angle {
        //     let x = ((width/2) * re * angle.cos()) as i32;
        //     let y = ((height/2) * im * angle.sin()) as i32;

        //     match canvas.draw_line(
        //         Point::new(sum_x, sum_y),
        //         Point::new(x, y),
        //     ) {
        //         Ok(..) => (),
        //         Err(..) => (),
        //     }

        //     sum_x += x;
        //     sum_y += y;
        // }

        match canvas.draw_line(
            Point::new(0, 0),
            Point::new((width/2) as i32, (height/2) as i32),
        ) {
            Ok(..) => (),
            Err(..) => (),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn normalize(points: &mut Vec<CoordinatePair>) {
    // get min and max values
    let mut min_x = 0f64;
    let mut max_x = 0f64;
    let mut min_y = 0f64;
    let mut max_y = 0f64;
    for (i, CoordinatePair { x, y }) in points.iter().enumerate() {
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

    for mut coord in points {
        coord.x = (coord.x - min_x) / side - 0.5;
        coord.y = (coord.y - min_y) / side - 0.5;
    }
}


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let polylines: Vec<Polyline> = svg2polylines::parse(&buffer).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);
    });


    // current expectation: there is only one line
    for mut line in polylines {
        let num_samples = line.len();
        let mut input:  Vec<Complex<f64>> = vec![Complex::zero(); num_samples];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); num_samples];

        normalize(&mut line);

        for (i, coord) in line.iter().enumerate() {
            match coord {
                CoordinatePair { x, y } => input[i] = Complex::new(*x, *y)
            }
        }

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(num_samples);
        fft.process(&mut input, &mut output);

        draw_line(output);

        // for num in line {
        //     println!("- {:?}", num);
        // }
        // let mut result: Vec<Vec<f64>> = Vec::new();
        // for Complex { re, im } in &output {
        //     result.push(vec![*re, *im]);
        // }
        // println!("{}", json::stringify(result));
    }

}
