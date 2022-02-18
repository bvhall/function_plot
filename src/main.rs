// Just a simple graphing calculator written in Rust
// main.rs

const MAX_X_GRID: i32 = 120;
const MAX_Y_GRID: i32 = 120;

// test function that returns the result of x^2
fn x_squared (input: f64) -> f64 {
    let y_val: f64 = input * input;
    y_val
}

// fn sine_wave (input: f64) -> f64 {
//     let y_val: f64 = 
// }

fn plot (x_start: i32, x_end: i32, y_start: i32, y_end: i32, function: fn(f64) -> f64, step: f64) {
    
    // Sanity check parameters
    assert!(x_start < x_end);
    assert!(y_start < y_end);
    assert!(step > 0.0);

    // calculate number of "pixels" to be used
    let step_multiplier = (1.0 / step) as i32;
    let x_points: i32 = (x_end - x_start) * step_multiplier;
    let y_points: i32 = (y_end - y_start) * step_multiplier;

    // keeps track of the current x and y coordinates with respect to the absolute terminal points
    // kept track by y_pos and x_pos
    let mut x_curr = x_end as f64;
    let mut y_curr = y_end as f64;

    // draw graph
    // outer loop prints the rows (y), while the inner loop prints the columns (x)

    // to ensure that each point on the function is actually printed, we allow a variance of 
    // +- the step value. This should also mitigate floating point error

    if x_points < MAX_X_GRID && y_points < MAX_Y_GRID {

            let mut output: f64; // declare output as mutable outside loop to save complexity

            for _y_pos in 0..y_points {
                
                for _x_pos in 0..x_points {

                    // get function output for the current x
                    output = function(x_curr);

                    if output == y_curr {
                        print!("*")
                    }

                    else if output == 0.0 {
                        print!("|")
                    }

                    else if y_curr == 0.0 {
                        print!("―")
                    }

                    else {
                        print!(" ")
                        //print!("({}, {})", x_curr, y_curr); // debug
                    }

                    x_curr -= step;
                }
                x_curr = x_end as f64;
                y_curr -= step;
                println!(); // print newline
        }
    }

}

fn main() {
    plot(-5, 5, -5, 5, x_squared, 0.5);
}
