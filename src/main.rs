extern crate rand;
use std::io;
use rand::Rng;
use std::fmt;
fn main() {
    let mut state = Field::new();
    loop {
        state.print(PrintType::Guesses);
        println!("Please input your guess as x,y[,b]");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let v: Vec<&str> = guess[..].split(",").collect();
        let x: usize = match v[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let y: usize = match v[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let mut bombflag = false;
        if v.len() > 2 {
            bombflag = v[2].trim() == "b";
        }
        if bombflag {
            state.click(x-1,y-1, FieldState::Bomb);
        } else {
            state.click(x-1,y-1, FieldState::Show);
        }
    }
}

#[derive(Debug)]
struct Field {
    field: [[bool; 9]; 9],
    state: [[FieldState; 9]; 9],
}

#[derive(Copy, Clone, Debug)]
enum FieldState {
    Bomb,
    Show,
    None,
}

#[derive(Copy, Clone)]
enum PrintType {
    Guesses,
    Solution,
}

impl Field {
    fn new() -> Field {
        let mut field = [[false; 9]; 9];
        for _ in 0..9 {
            let (x,y) = (rand::thread_rng().gen_range(0,9), rand::thread_rng().gen_range(0,9));
            field[x][y] = true;
        }
        let mut state = [[FieldState::None; 9]; 9];
        Field {field, state}
    }

    fn print(&self, print_type: PrintType) {
        let mut line = String::from("  |");
        for i in 0..9 {
            line.push_str(&format!(" {} ", i+1)[..]);
        }
        println!("{}",line );
        println!("--+---------------------------");
        for y in 0..9 {
            let mut line = String::from(format!("{} |", y+1));
            for x in 0..9 {
                match print_type {
                    PrintType::Guesses => {
                        match self.state[x][y] {
                            FieldState::Bomb => line.push_str(" B "),
                            FieldState::Show => line.push_str(&format!(" {} ", self.get_number(x,y))[..]),
                            FieldState::None => line.push_str("   "),
                        }
                    },
                    PrintType::Solution => {
                        match self.field[x][y] {
                            true => line.push_str(" X "),
                            false => line.push_str(&format!(" {} ", self.get_number(x,y))[..]),
                        }
                    }
                }
            }
            println!("{}",line );
        }
    }

    fn in_range(&self, x: usize, y: usize) -> bool {
        // x and y are not zero-based because I do not know how to loop from -1 to 1
        // Therefore, lower bound comparison is greater than zero (so it accepts 1++)
        // And upper bound comparison is smaller or equal to the length
        x > 0 && x <= self.state[0].len() && y > 0 && y <= self.state[0].len() 
    }

    fn get_number(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;
        for x_incr in 0..3 {
            for y_incr in 0..3 {
                if self.in_range(x+x_incr,y+y_incr) {
                    if self.field[x+x_incr - 1][y + y_incr - 1] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn click(&mut self, x: usize, y: usize, click_type: FieldState) {
        let has_been_clicked = match self.state[x][y] {
            FieldState::Show => true,
            _ => false,
        };
        match click_type {
            FieldState::Show => {
                if self.field[x][y] {
                    println!("YOU NOOB");
                    self.print(PrintType::Solution);
                    std::process::exit(1);
                }
                self.state[x][y] = click_type;
            },
            FieldState::Bomb => {
                self.state[x][y] = click_type;
                return
            },
            FieldState::None => {},
        }
        if self.get_number(x, y) == 0 && !has_been_clicked {
            for x_incr in 0..3 {
                for y_incr in 0..3 {
                    if self.in_range(x+x_incr, y+y_incr) {
                        self.click(x+x_incr - 1, y+y_incr - 1, FieldState::Show);
                    }
                }
            }
        }
    }
}
