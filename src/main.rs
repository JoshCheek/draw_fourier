extern crate svg2polylines;
use std::io::{self, Read};
use std::process::exit;
use svg2polylines::{CoordinatePair, Polyline};

extern crate rustfft;
use std::sync::Arc;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;


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

        // for num in line {
        //     println!("- {:?}", num);
        // }
        for num in &output {
            println!("- {}", num);
        }
    }

}
