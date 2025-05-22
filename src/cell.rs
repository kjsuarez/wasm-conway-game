pub struct BoardData{
    pub cell_count:u32,
    pub row_length:u32,
    pub column_height:u32,
    pub cell_length:u32,
}
impl BoardData {
    pub fn new(board_length: u32, width: u32, height: u32, cell_length: u32) -> BoardData{
        BoardData{
            cell_count: (board_length/4)/(cell_length*cell_length),
            row_length: width / cell_length,
            column_height: height / cell_length,
            cell_length
        }
    }
}

#[derive(Copy, Clone)]
pub enum Vertical {
    ceiling,
    middle,
    floor
}
#[derive(Copy, Clone)]
pub enum Horizontal{
    left,
    middle,
    right
}

pub struct Cell <'a> {
    board_data: &'a BoardData,
    id: u32,
    pixel_length:u32,
    active: bool,
    vertical: Vertical,
    horizontal: Horizontal
}

impl Cell <'_> {
    pub fn new( board_data: &BoardData, id:u32) -> Cell {
        Cell{
            pixel_length: board_data.cell_length.clone(),
            board_data,
            id,
            active:false,
            vertical: Cell::determine_vertical_position(&board_data, id),
            horizontal: Cell::determine_horizontal_position(&board_data, id),
        }
    }
    pub fn north(&self) -> u32 {
        match self.vertical {
            Vertical::ceiling => {
                self.id + (self.board_data.row_length * (self.board_data.column_height-1))
            }
            _ => {
                self.id - self.board_data.row_length
            }
        }
    }
    pub fn south(&self) -> u32 {
        match self.vertical {
            Vertical::floor => {
                self.id%self.board_data.row_length
            }
            _ => {
                self.id + self.board_data.row_length
            }
        }
    }

    pub fn east(&self) -> u32 {
        match self.horizontal {
            Horizontal::right => {
                self.id - (self.board_data.row_length - 1)
            }
            _ => {
                self.id + 1
            }
        }
    }

    pub fn west(&self) -> u32 {
        match self.horizontal {
            Horizontal::left => {
                self.id + (self.board_data.row_length - 1)
            }
            _ => {
                self.id - 1
            }
        }
    }

    pub fn north_west(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::ceiling, Horizontal::left) => {
                self.board_data.cell_count - 1
            }
            (Vertical::ceiling, Horizontal::right | Horizontal::middle) => {
                (self.id + (self.board_data.row_length * (self.board_data.column_height-1))) - 1
            }
            (Vertical::floor | Vertical::middle, Horizontal::left) => {
                (self.id + (self.board_data.row_length - 1)) - self.board_data.row_length
            }
            _ => {
                (self.id - self.board_data.row_length) - 1
            }
        }
    }

    pub fn north_east(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::ceiling, Horizontal::right) => {
                self.board_data.cell_count - self.board_data.row_length
            }
            (Vertical::ceiling, Horizontal::left | Horizontal::middle) => {
                (self.id + (self.board_data.row_length * (self.board_data.column_height-1))) + 1
            }
            (Vertical::floor | Vertical::middle, Horizontal::right) => {
                self.id - ((self.board_data.row_length * 2) - 1)
            }
            _ => {
                (self.id - self.board_data.row_length) + 1
            }
        }
    }

    pub fn south_east(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::floor, Horizontal::right) => {
                0
            }
            (Vertical::floor, Horizontal::left | Horizontal::middle) => {
                (self.id % self.board_data.row_length) + 1
            }
            (Vertical::ceiling | Vertical::middle, Horizontal::right) => {
                self.id + 1
            }
            _ => {
                (self.id + self.board_data.row_length) + 1
            }
        }
    }

    pub fn south_west(&self) -> u32 {
        match (self.vertical, self.horizontal) {
            (Vertical::floor, Horizontal::left) => {
                self.board_data.row_length - 1
            }
            (Vertical::floor, Horizontal::right | Horizontal::middle) => {
                (self.id % self.board_data.row_length) - 1  
            }
            (Vertical::ceiling | Vertical::middle, Horizontal::left) => {
                self.id + (self.board_data.row_length*2) - 1
            }
            _ => {
                self.id + (self.board_data.row_length) - 1
            }
        }
    }

    pub fn determine_vertical_position(board_data: &BoardData, id:u32) -> Vertical{
        match id {
            n if n < board_data.row_length => {
                Vertical::ceiling
            }
            n if n >= (board_data.cell_count - board_data.row_length) => {
                Vertical::floor
            }
            _ => Vertical::middle
        }
    }

    pub fn determine_horizontal_position(board_data: &BoardData, id:u32) -> Horizontal{
        match id {
            n if n % board_data.row_length == 0 => {
                Horizontal::left
            }
            n if (n+1) % board_data.row_length == 0 => {
                Horizontal::right
            }
            _ => Horizontal::middle
        }
    }
}