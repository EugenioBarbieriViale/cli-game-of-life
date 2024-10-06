use std::{thread, time};
use rand::Rng;

const HEIGHT: usize = 25;
const WIDTH: usize = HEIGHT * 2;

fn main() {
    let alive: char = '0';
    let  dead: char = ' ';

    let mut states = init_states();
    let mut next_states = states;

    let wait_time = time::Duration::from_millis(500);

    loop {
        thread::sleep(wait_time);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = count_neighbours(x, y, &states);
                compute_next(x, y, n, &states, &mut next_states);
                draw(alive, dead, x, y, states[y][x]);
            }
            println!("");
        }
        states = next_states;
        print!("{}[2J", 27 as char); // clear terminal
    }
}

fn init_states() -> [[bool; WIDTH]; HEIGHT] {
    let mut states = [[false; WIDTH]; HEIGHT];
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let val: bool = rand::thread_rng().gen_bool(0.5);
            states[j][i] = val;
        }
    }
    states
}

fn count_neighbours(x: usize, y: usize, states: &[[bool; WIDTH]; HEIGHT]) -> usize {
    let mut n = 0;

    for j in -1i8..=1 {
        for i in -1i8..=1 {
            if x as i8 -1 >= 0 && y as i8 -1 >= 0 && x+1 <= WIDTH-1 && y+1 <= HEIGHT-1 {
                let new_y = i + (y as i8);
                let new_x = j + (x as i8);
                
                if states[new_y as usize][new_x as usize] && (j != 0 || i != 0) {
                    n += 1;
                }
            }
        }
    }
    n
}

fn compute_next(x: usize, y: usize, n: usize, 
    states: &[[bool; WIDTH]; HEIGHT], next_states: &mut [[bool; WIDTH]; HEIGHT]) {

    if n < 2 && states[y][x] {
        next_states[y][x] = false;
    }

    if (n == 2 || n == 3) && states[y][x] {
        next_states[y][x] = true;
    }

    if n > 3 && states[y][x] {
        next_states[y][x] = false;
    }

    if n == 3 && !states[y][x] {
        next_states[y][x] = true;
    }
}

fn draw(alive: char, dead: char, x: usize, y: usize, state: bool) {
    if state {
        print!("{}", alive);
    }
    else {
        if y == 0 || y == HEIGHT - 1 {
            print!("-");
        }
        else if x == 0 || x == WIDTH - 1 {
            print!("|");
        }
        else {
            print!("{}", dead);
        }
    }
}
