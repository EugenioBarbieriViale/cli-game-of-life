const HEIGHT: usize = 10;
const WIDTH: usize = HEIGHT * 2;

fn main() {
    let alive: char = '@';
    let  dead: char = '.';

    let mut states = [[false; WIDTH]; HEIGHT];
    let mut next_states = states; 

    states[4][4] = true;
    states[4][5] = true;
    states[4][6] = true;
                
    loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = count_neighbours(x, y, &states);
                compute_next(x, y, n, &states, &mut next_states);
                draw(alive, dead, x, y, &states);
            }
            println!("");
        }
        states = next_states;
    }
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

fn draw(alive: char, dead: char, x: usize, y: usize, states: &[[bool; WIDTH]; HEIGHT]) {
    if states[y][x] {
        print!("{}", alive);
    }
    else {
        print!("{}", dead);
    }
}
