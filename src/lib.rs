extern crate console_error_panic_hook;
mod cell;
use crate::cell::Cell;
use crate::cell::BoardData;
use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn test(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> String {
    let mut output = String::new();
    let board_metadata = BoardData::new(board.len() as u32, width, height, cell_length);

    let first = cell_index_to_pixel_address(0, &board_metadata);
    output.push_str("first:");
    output.push_str(&first.to_string());

    let pixels_in_cell = board_metadata.cell_length * board_metadata.cell_length;
    let last = first + (pixels_in_cell*4);
    output.push_str(" Last:");
    output.push_str(&last.to_string());
    output
}

#[wasm_bindgen]
pub fn step(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> Clamped<Vec<u8>> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // width: board width in pixels
    // height: board height in pixels
    // cell_length: length of cell in pixels
    let mut output = board.clone();
    let cell_count = (board.len()/4)/(cell_length*cell_length) as usize;
    let cell_iter = 0..cell_count;
    let board_metadata = BoardData::new(board.len() as u32, width, height, cell_length);
    for i in cell_iter {
        let cell_index = i as u32;
        let cell_state = matches!(state_of_index(cell_index, &board_metadata, &board), 1);
        let neighbor_count = neighbor_count(cell_index, &board_metadata, &board);
        let updated_cell_state = cell_state_from_context(cell_state, neighbor_count);
        if cell_state != updated_cell_state {
            //index:u32, state:bool, board_metadata: &BoardData, mut board: Clamped<Vec<u8>>
            output = update_cell(cell_index, updated_cell_state, &board_metadata, output);
        }
    }

    // board is an array where every 4th element
    //  describes if a pixel is on or off
    // iterate over elements such that 1 element = 1 cell
    output
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

    let count = neighbor_count(cell_index, &board_data, &board);
    output.push_str("Neightbor Count:");
    output.push_str(&count.to_string());

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

fn cell_index_to_pixel_address(cell_index:u32, board_metadata:&BoardData) -> u32{
    let pixel = cell_index_to_pixel_index(cell_index, board_metadata);
    3+(pixel*4)
}

fn cell_index_to_pixel_index(cell_index:u32, board_metadata:&BoardData) -> u32{
    let cell_length = board_metadata.cell_length;
    let row_length = board_metadata.row_length;
    let your_row = cell_index / row_length;
    let cells_in_row_before_you = cell_index%row_length;
    (cell_length * cell_length * your_row * row_length) + (cells_in_row_before_you * cell_length)
    
}

fn neighbor_count(cell_index: u32, board_metadata:&BoardData, board:&Clamped<Vec<u8>>) -> u8 {
    let cell: Cell = Cell::new(board_metadata, cell_index);
    let mut count = 0;
    count += state_of_index(cell.north(), board_metadata, board);
    count += state_of_index(cell.north_east(), board_metadata, board);
    count += state_of_index(cell.east(), board_metadata, board);
    count += state_of_index(cell.south_east(), board_metadata, board);
    count += state_of_index(cell.south(), board_metadata, board);
    count += state_of_index(cell.south_west(), board_metadata, board);
    count += state_of_index(cell.west(), board_metadata, board);
    count += state_of_index(cell.north_west(), board_metadata, board);

    count
}

fn state_of_index(cell_index:u32, board_metadata:&BoardData, board:&Clamped<Vec<u8>>) -> u8{
    let pix_i = cell_index_to_pixel_address(cell_index, board_metadata);
    let state:u8 = if board[pix_i as usize] != 0 {
        1
    } else {
        0
    };
    state
}

fn cell_state_from_context(cell_state:bool, neighbor_count:u8) -> bool {
    match (cell_state, neighbor_count) {
        (true, n) if n < 2 => false,
        (true, n) if n > 3 => false,
        (true, 2..4) => true,
        (false,3) => true,
        _ => false
    }
}

fn update_cell(index:u32, state:bool, board_metadata: &BoardData, mut board: Clamped<Vec<u8>>) -> Clamped<Vec<u8>> {
    let state_value = match state {
        true => 255,
        _ => 0
    };
    let start = cell_index_to_pixel_index(index, board_metadata);

    for row in 0..board_metadata.cell_length {
        let pixels_per_row = row * board_metadata.cell_length * board_metadata.row_length;
        let row_start = pixels_per_row + start;
        let row_end = row_start + board_metadata.cell_length;
        for pixel in row_start..row_end {
            let address = 3+(pixel*4);
            board[address as usize] = state_value;
        }
    }
    board
}