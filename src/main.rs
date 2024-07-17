use rand::Rng;
use std::io::{self, Write};

const SIZE: usize = 15;

fn imprimir(matriu: [[i32; SIZE]; SIZE]) {
    println!("  01 02 03 04 05 06 07 08 09 10 11 12 13 14 15");
    for i in 0..SIZE {
        print!("{}  ", i+1);
        for j in 0..SIZE {
            match matriu[i][j] {
                0 => print!("_  "),
                1 => print!("o  "),
                2 => print!("x  "),
                _ => (),
            }
        }
        println!();
    }
}

fn victoria(matriu: [[i32; SIZE]; SIZE], x: usize, y: usize, player: i32) -> bool {
    let mut conseq = 0;
    for i in 0..SIZE {
        if matriu[x][i] == player {
            conseq += 1;
            if conseq == 5 {
                return true;
            }
        } else {
            conseq = 0;
        }

    }

    conseq = 0;
    for i in 0..SIZE {
        if matriu[i][y] == player {
            conseq += 1;
            if conseq == 5 {
                return true;
            }
        } else {
            conseq = 0;
        }
    }

    false
}

fn count_consecutive(matrix: [[i32; SIZE]; SIZE], x: usize, y: usize, num: i32) -> i32 {
    let mut count_x = 0;
    for j in (y + 1)..SIZE {
        if matrix[x][j] == num {
            count_x += 1;
        } else {
            break;
        }
    }
    for j in (0..y).rev() {
        if matrix[x][j] == num {
            count_x += 1;
        } else {
            break;
        }
    }

    let mut count_y = 0;
    for i in (x + 1)..SIZE {
        if matrix[i][y] == num {
            count_y += 1;
        } else {
            break;
        }
    }
    for i in (0..x).rev() {
        if matrix[i][y] == num {
            count_y += 1;
        } else {
            break;
        }
    }

    count_x + count_y
}

fn ia_play(matriu: [[i32; SIZE]; SIZE], x: &mut usize, y: &mut usize) {
    let mut valoracion = [[0; SIZE]; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            if matriu[i][j] != 0 {
                valoracion[i][j] = -1;
            } else {
                valoracion[i][j] = 5 * count_consecutive(matriu, i, j, 2);
                if count_consecutive(matriu, i, j, 1) >= 3 {
                    valoracion[i][j] += 1000;
                }

                if count_consecutive(matriu, i, j, 2) >= 4 {
                    valoracion[i][j] += 10000;
                }
            }
        }
    }

    let mut max_num = 0;
    let mut max_coords = Vec::new();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if valoracion[i][j] > max_num {
                max_num = valoracion[i][j];
                max_coords.clear();
                max_coords.push((i, j));
            } else if valoracion[i][j] == max_num {
                max_coords.push((i, j));
            }
        }
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..max_coords.len());
    *x = max_coords[random_index].0;
    *y = max_coords[random_index].1;
}

fn main() {
    let mut matriu = [[0; SIZE]; SIZE];
    let mut player_actual = 1;
    let mut x=0;
    let mut y=0;
    let mut finalizado = false;

    while !finalizado {
        if player_actual == 1 {
            imprimir(matriu);
            println!("tu turno");
            loop {
                print!("Introduce x y: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let inputs: Vec<usize> = match input.trim().split_whitespace().map(|s| s.parse()).collect() {
                    Err(_) => continue,
                    Ok(res) => res,
                };

                if inputs.len() != 2 {
                    continue;
                }
                
                x = inputs[0] - 1;
                y = inputs[1] - 1;
                if matriu[x][y] == 0 {
                    break;
                } else {
                    println!("Casilla ocupada, elige otra.");
                }
            }
            matriu[x][y] = 1;
            if victoria(matriu, x, y, player_actual) {
                println!("You win");
                finalizado = true;
            }
            player_actual = 2;
        } else {
            ia_play(matriu, &mut x, &mut y);
            matriu[x][y] = 2;
            if victoria(matriu, x, y, player_actual) {
                println!("IA win");
                finalizado = true;
            }
            player_actual = 1;
        }
    }
}
