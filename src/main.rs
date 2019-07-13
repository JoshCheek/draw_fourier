extern crate svg2polylines;
use std::io::{self, Read};
use std::process::exit;
use svg2polylines::{CoordinatePair, Polyline};

extern crate rustfft;
use std::sync::Arc;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let polylines: Vec<Polyline> = svg2polylines::parse(&buffer).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);
    });


    for line in polylines {
        let num_samples = line.len();
        let mut input:  Vec<Complex<f64>> = vec![Complex::zero(); num_samples];
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); num_samples];

        for (i, coord) in line.iter().enumerate() {
            match coord {
                CoordinatePair { x, y } => input[i] = Complex::new(*x, *y)
            }
        }

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(num_samples);
        fft.process(&mut input, &mut output);

        for num in output {
            println!("- {}", num);
        }
    }

}
