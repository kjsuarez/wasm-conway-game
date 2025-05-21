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


pub enum Vertical {
    ceiling,
    middle,
    floor
}
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
            horizontal: Horizontal::middle,
        }
    }
    pub fn north(&self) -> u32 {
        match self.vertical {
            Vertical::ceiling => {
                self.id + (self.board_data.row_length * (self.board_data.column_height-1))
            }
            Vertical::middle => {
                self.id - self.board_data.row_length
            }
            Vertical::floor => {
                self.id - self.board_data.row_length
            }
        }
    }
    pub fn south(&self) -> u32 {
        match self.vertical {
            Vertical::ceiling => {
                self.id + self.board_data.row_length
            }
            Vertical::middle => {
                self.id + self.board_data.row_length
            }
            Vertical::floor => {
                self.id%self.board_data.row_length
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
}