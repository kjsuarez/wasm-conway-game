extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn test(name: &str) -> u8 {
    2
}

#[wasm_bindgen]
pub fn step(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> Vec<u32> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // width: board width in pixels
    // height: board height in pixels
    // cell_length: length of cell in pixels
    let output = board.clone();
    let cell_count = (board.len()/4)/(cell_length*cell_length) as usize;
    let cell_iter = 0..cell_count;
    let row_length:u32 = width as u32 / cell_length as u32;

    let mut xxx: Vec<u32> = Vec::new();
    for cell_index in cell_iter {
        let pix_i = cell_index_to_pixel_index(cell_index as u32, cell_length, row_length);
        let state:u32 = if board[pix_i as usize] != 0 {
            1
        } else {
            0
        };
        xxx.push(state);
    }

    // board is an array where every 4th element
    //  describes if a pixel is on or off
    // iterate over elements such that 1 element = 1 cell

    xxx
    //output
}

fn cell_index_to_pixel_index(cell_index:u32, cell_length:u32, row_length:u32) -> u32{
    let your_row = cell_index / row_length;
    let cells_in_row_before_you = cell_index%row_length;
    let pixel = (cell_length * cell_length * your_row * row_length) + (cells_in_row_before_you * cell_length);
    3+(pixel*4)
}

enum Vertical {
    ceiling,
    middle,
    floor
}
enum Horizontal{
    left,
    middle,
    right
}

struct Cell {
    id: u32,
    active: bool,
    vertical: Vertical,
    horizontal: Horizontal
}

