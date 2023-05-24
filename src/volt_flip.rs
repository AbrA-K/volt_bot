use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Clone, Copy, Deserialize)]
enum CellValue {
    Bomb,
    Value(u32),
}

#[derive(Debug, Serialize, Copy, Clone, Deserialize)]
struct Cell {
    value: CellValue,
    revealed: bool,
}

impl Cell {
    fn to_char(self) -> char {
        match self.value {
            CellValue::Bomb => '',
            CellValue::Value(value) => char::from_digit(value, 10).unwrap(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: CellValue::Value(1),
            revealed: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
struct LineHint {
    value_sum: u8,
    bomb_sum: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoltGame {
    board: [[Cell; 5]; 5],
    xhints: [LineHint; 5],
    yhints: [LineHint; 5],
}

impl VoltGame {
    // new games could be done better but idc
    pub fn get_new_game() -> Self {
        let mut game = VoltGame {
            board: [[Cell::default(); 5]; 5],
            xhints: [LineHint::default(); 5],
            yhints: [LineHint::default(); 5],
        };
        game.new_game();
        game
    }

    pub fn new_game(&mut self) {
        let voltorb_count = 6;
        let threes = 3;
        let twos = 3;
        let mut rng = rand::thread_rng();

        // reset hints
        self.xhints.iter_mut().for_each(|hint| {
            hint.value_sum = 0;
            hint.bomb_sum = 0
        });

        self.yhints.iter_mut().for_each(|hint| {
            hint.value_sum = 0;
            hint.bomb_sum = 0
        });

        // reset everything to 1 unrevealed
        self.board.iter_mut().for_each(|arr| {
            arr.iter_mut().for_each(|elem| {
                elem.value = CellValue::Value(1);
                elem.revealed = false
            })
        });

        let mut randx = rng.gen_range(0..5);
        let mut randy = rng.gen_range(0..5);

        // add voltorbs
        for _ in 0..voltorb_count {
            //always check if random position is free
            while self.board[randx][randy].value != CellValue::Value(1) {
                randx = rng.gen_range(0..5);
                randy = rng.gen_range(0..5);
            }
            self.board[randx][randy].value = CellValue::Bomb;
        }

        // add 2
        for _ in 0..twos {
            while self.board[randx][randy].value != CellValue::Value(1) {
                randx = rng.gen_range(0..5);
                randy = rng.gen_range(0..5);
            }
            self.board[randx][randy].value = CellValue::Value(2);
        }

        // add 3
        for _ in 0..threes {
            while self.board[randx][randy].value != CellValue::Value(1) {
                randx = rng.gen_range(0..5);
                randy = rng.gen_range(0..5);
            }
            self.board[randx][randy].value = CellValue::Value(3);
        }

        //update hints
        for x in 0..5 {
            for y in 0..5 {
                let cell = self.board[x][y];
                match cell.value {
                    CellValue::Bomb => self.xhints[x].bomb_sum += 1,
                    CellValue::Value(value) => self.xhints[x].value_sum += value as u8,
                }
            }
        }
        //update hints
        for y in 0..5 {
            for x in 0..5 {
                let cell = self.board[x][y];
                match cell.value {
                    CellValue::Bomb => self.yhints[y].bomb_sum += 1,
                    CellValue::Value(value) => self.yhints[y].value_sum += value as u8,
                }
            }
        }
    }

    /// reveal bomb at x y coordinate
    /// return value if it was a value
    /// return 0 if it was a bomb
    pub fn reveal(&mut self, x: u8, y: u8) -> u8 {
        // return 1 if already revealed
        if self.board[x as usize][y as usize].revealed {
            return 1;
        }

        // reveal and return value
        self.board[x as usize][y as usize].revealed = true;
        match self.board[x as usize][y as usize].value {
            CellValue::Bomb => {
                self.new_game();
                0
            }
            CellValue::Value(value) => value as u8,
        }
    }

    pub fn pretty_string(&self) -> String {
        let mut result = "|-------------------||--------\n".to_string();
        let in_betweens = "|---+---+---+---+---||--------\n".to_string();

        for (x, arr) in self.board.iter().enumerate() {
            let mut line_str = "".to_string();
            for cell in arr.iter() {
                if cell.revealed {
                    line_str += format!("| {} ", cell.to_char()).as_str();
                } else {
                    line_str += "|   ";
                }
            }
            line_str += format!(
                "||v:{}; b:{} \n",
                self.xhints[x].value_sum, self.xhints[x].bomb_sum
            )
            .as_str();
            if x <= 3 {
                line_str += in_betweens.as_str();
            } else {
                line_str += "|-------------------||--------\n";
            }
            result += line_str.as_str();
        }
        result += "|-------------------||--------\n";
        let mut y_value_line = "".to_owned();
        let mut y_bomb_line = "".to_owned();
        for i in 0..5 {
            y_value_line += format!("|v:{}", self.yhints[i as usize].value_sum).as_str();
            y_bomb_line += format!("|b:{}", self.yhints[i as usize].bomb_sum).as_str();
        }
        y_value_line += "|| ɢʟ \n";
        y_bomb_line += "||    ʜғ \n";

        result += y_value_line.as_str();
        result += y_bomb_line.as_str();

        result
    }

    pub fn check_won(&self) -> bool {
        for arr in self.board {
            for cell in arr {
                if cell.value == CellValue::Value(2) && !cell.revealed {
                    return false;
                }
                if cell.value == CellValue::Value(3) && !cell.revealed {
                    return false;
                }
            }
        }
        true
    }
}
