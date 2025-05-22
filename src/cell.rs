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
    board_data: &'a BoardData,
    id: u32,
    vertical: Vertical,
    horizontal: Horizontal
}

impl Cell <'_> {
    pub fn new( board_data: &BoardData, id:u32) -> Cell {
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

    pub fn determine_vertical_position(board_data: &BoardData, id:u32) -> Vertical{
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

    pub fn determine_horizontal_position(board_data: &BoardData, id:u32) -> Horizontal{
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
}