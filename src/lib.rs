extern crate console_error_panic_hook;
mod cell;
use crate::cell::Cell;
use crate::cell::BoardData;
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
    let row_length:u32 = width / cell_length as u32;
    let column_height:u32 = height / cell_length;

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

fn cell_coor_to_cell_index(board_data: &BoardData, x:u32, y:u32) -> u32{
    (y * board_data.row_length) + x
}

#[wasm_bindgen]
pub fn neighbor_positions(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32, x:u32, y: u32) -> String{
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let board_data = BoardData::new(board.len() as u32, width, height, cell_length);
    let cell_index = cell_coor_to_cell_index(&board_data, x, y);
    println!("{}",cell_index);
    let this_cell = Cell::new(&board_data, cell_index);
    let mut output = String::new();
    // NW
    output.push_str(" NorthWest:");
    output.push_str(&this_cell.north_west().to_string());
    // N
    output.push_str(" North:");
    output.push_str(&this_cell.north().to_string());
    // NE
    output.push_str(" NorthEast:");
    output.push_str(&this_cell.north_east().to_string());
    // E
    output.push_str(" East:");
    output.push_str(&this_cell.east().to_string());
    // SE
    output.push_str(" SouthEast:");
    output.push_str(&this_cell.south_east().to_string());
    // S
    output.push_str(" South:");
    output.push_str(&this_cell.south().to_string());
    // SW
    output.push_str(" SouthWest:");
    output.push_str(&this_cell.south_west().to_string());
    // W
    output.push_str(" West:");
    output.push_str(&this_cell.west().to_string());

    output
}

fn cell_index_to_pixel_index(cell_index:u32, cell_length:u32, row_length:u32) -> u32{
    let your_row = cell_index / row_length;
    let cells_in_row_before_you = cell_index%row_length;
    let pixel = (cell_length * cell_length * your_row * row_length) + (cells_in_row_before_you * cell_length);
    3+(pixel*4)
}
