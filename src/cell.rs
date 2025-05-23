use wasm_bindgen::Clamped;
use crate::engine::*;

pub struct Board{
    pub data: Clamped<Vec<u8>>,
    pub cell_count:u32,
    pub row_length:u32,
    pub column_height:u32,
    pub cell_length:u32,
}
impl Board {
    pub fn new(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> Board{
        Board{
            cell_count: (board.len() as u32/4)/(cell_length*cell_length),
            row_length: width / cell_length,
            column_height: height / cell_length,
            cell_length,
            data: board,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Vertical {
    Ceiling,
    Middle,
    Floor
}
#[derive(Copy, Clone)]
pub enum Horizontal{
    Left,
    Middle,
    Right
}

pub struct Cell <'a> {
    board_data: &'a Board,
    id: u32,
    vertical: Vertical,
    horizontal: Horizontal
}

impl Cell <'_> {
    pub fn new( board_data: &Board, id:u32) -> Cell {
        Cell{
            board_data,
            id,
            vertical: Cell::determine_vertical_position(board_data, id),
            horizontal: Cell::determine_horizontal_position(board_data, id),
        }
    }
    pub fn north(&self) -> u32 {
        match self.vertical {
            Vertical::Ceiling => {
                self.id + (self.board_data.row_length * (self.board_data.column_height-1))
            }
            _ => {
                self.id - self.board_data.row_length
            }
        }
    }
    pub fn south(&self) -> u32 {
        match self.vertical {
            Vertical::Floor => {
                self.id%self.board_data.row_length
            }
            _ => {
                self.id + self.board_data.row_length
            }
        }
    }
    pub fn east(&self) -> u32 {
        match self.horizontal {
            Horizontal::Right => {
                self.id - (self.board_data.row_length - 1)
            }
            _ => {
                self.id + 1
            }
        }
    }
    pub fn west(&self) -> u32 {
        match self.horizontal {
            Horizontal::Left => {
                self.id + (self.board_data.row_length - 1)
            }
            _ => {
                self.id - 1
            }
        }
    }
    pub fn north_west(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::Ceiling, Horizontal::Left) => {
                self.board_data.cell_count - 1
            }
            (Vertical::Ceiling, Horizontal::Right | Horizontal::Middle) => {
                (self.id + (self.board_data.row_length * (self.board_data.column_height-1))) - 1
            }
            (Vertical::Floor | Vertical::Middle, Horizontal::Left) => {
                (self.id + (self.board_data.row_length - 1)) - self.board_data.row_length
            }
            _ => {
                (self.id - self.board_data.row_length) - 1
            }
        }
    }
    pub fn north_east(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::Ceiling, Horizontal::Right) => {
                self.board_data.cell_count - self.board_data.row_length
            }
            (Vertical::Ceiling, Horizontal::Left | Horizontal::Middle) => {
                (self.id + (self.board_data.row_length * (self.board_data.column_height-1))) + 1
            }
            (Vertical::Floor | Vertical::Middle, Horizontal::Right) => {
                self.id - ((self.board_data.row_length * 2) - 1)
            }
            _ => {
                (self.id - self.board_data.row_length) + 1
            }
        }
    }
    pub fn south_east(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::Floor, Horizontal::Right) => {
                0
            }
            (Vertical::Floor, Horizontal::Left | Horizontal::Middle) => {
                (self.id % self.board_data.row_length) + 1
            }
            (Vertical::Ceiling | Vertical::Middle, Horizontal::Right) => {
                self.id + 1
            }
            _ => {
                (self.id + self.board_data.row_length) + 1
            }
        }
    }
    pub fn south_west(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::Floor, Horizontal::Left) => {
                self.board_data.row_length - 1
            }
            (Vertical::Floor, Horizontal::Right | Horizontal::Middle) => {
                (self.id % self.board_data.row_length) - 1  
            }
            (Vertical::Ceiling | Vertical::Middle, Horizontal::Left) => {
                self.id + (self.board_data.row_length*2) - 1
            }
            _ => {
                self.id + (self.board_data.row_length) - 1
            }
        }
    }
    pub fn determine_vertical_position(board_data: &Board, id:u32) -> Vertical{
        match id {
            n if n < board_data.row_length => {
                Vertical::Ceiling
            }
            n if n >= (board_data.cell_count - board_data.row_length) => {
                Vertical::Floor
            }
            _ => Vertical::Middle
        }
    }
    pub fn determine_horizontal_position(board_data: &Board, id:u32) -> Horizontal{
        match id {
            n if n % board_data.row_length == 0 => {
                Horizontal::Left
            }
            n if (n+1) % board_data.row_length == 0 => {
                Horizontal::Right
            }
            _ => Horizontal::Middle
        }
    }
    pub fn neighbor_count(&self, board:&Board) -> u8 {
        let mut count = 0;
        count += state_of_index(self.north(), board);
        count += state_of_index(self.north_east(), board);
        count += state_of_index(self.east(), board);
        count += state_of_index(self.south_east(), board);
        count += state_of_index(self.south(), board);
        count += state_of_index(self.south_west(), board);
        count += state_of_index(self.west(), board);
        count += state_of_index(self.north_west(), board);
    
        count
    }
}