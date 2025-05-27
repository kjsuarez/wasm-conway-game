use crate::cell::Board;
use wasm_bindgen::Clamped;

    // The board exists as an array where every 4th element
    //  describes if a pixel is on or off

pub fn cell_index_to_pixel_address(cell_index:u32, board_metadata:&Board) -> u32{
    let pixel = cell_index_to_pixel_index(cell_index, board_metadata);
    3+(pixel*4)
}

pub fn cell_index_to_pixel_index(cell_index:u32, board_metadata:&Board) -> u32{
    let cell_length = board_metadata.cell_length;
    let row_length = board_metadata.row_length;
    let your_row = cell_index / row_length;
    let cells_in_row_before_you = cell_index%row_length;
    (cell_length * cell_length * your_row * row_length) + (cells_in_row_before_you * cell_length)
    
}

pub fn update_cell(index:u32, state:bool, board: &Board, mut output: Clamped<Vec<u8>>) -> Clamped<Vec<u8>> {
    let state_value = match state {
        true => 255,
        _ => 0
    };
    let start = cell_index_to_pixel_index(index, board);

    for row in 0..board.cell_length {
        let pixels_per_row = row * board.cell_length * board.row_length;
        let row_start = pixels_per_row + start;
        let row_end = row_start + board.cell_length;
        for pixel in row_start..row_end {
            let address = 3+(pixel*4);
            output[address as usize] = state_value;
        }
    }
    output
}

pub fn state_of_index(cell_index:u32, board:&Board) -> u8{
    let pix_i = cell_index_to_pixel_address(cell_index, board);
    let state:u8 = if board.data[pix_i as usize] != 0 {
        1
    } else {
        0
    };
    state
}

pub fn cell_coor_to_cell_index(board_data: &Board, x:u32, y:u32) -> u32{
    (y * board_data.row_length) + x
}

pub fn pixel_to_cell_index(cursor_x:u32, cursor_y:u32, board_data: &Board) -> u32{
    let cell_x:u32 = cursor_x / board_data.cell_length;
    let cell_y:u32 = cursor_y / board_data.cell_length;
    cell_coor_to_cell_index(board_data, cell_x, cell_y)
}