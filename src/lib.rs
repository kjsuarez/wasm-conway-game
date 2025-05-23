extern crate console_error_panic_hook;
mod cell;
use crate::cell::Cell;
use crate::cell::Board;
mod engine;
use crate::engine::*;
use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn test(board: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> String {
    let mut output = String::new();
    let board_metadata = Board::new(board, width, height, cell_length);

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
pub fn step(board_data: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> Clamped<Vec<u8>> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut output = board_data.clone();
    let board = Board::new(board_data, width, height, cell_length);
    let cell_count = (board.cell_count) as usize;
    for i in 0..cell_count {
        let cell_index = i as u32;
        let cell_state = matches!(state_of_index(cell_index, &board), 1);
        let cell = Cell::new(&board, cell_index);
        let neighbor_count = cell.neighbor_count(&board);
        let updated_cell_state = apply_conway_rules(cell_state, neighbor_count);
        if cell_state != updated_cell_state {
            output = update_cell(cell_index, updated_cell_state, &board, output);
        }
    }

    output
}

#[wasm_bindgen]
pub fn neighbor_positions(board_data: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32, x:u32, y: u32) -> String{
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let board = Board::new(board_data, width, height, cell_length);
    let cell_index = cell_coor_to_cell_index(&board, x, y);
    let cell = Cell::new(&board, cell_index);
    let this_cell = Cell::new(&board, cell_index);
    let mut output = String::new();

    let count = cell.neighbor_count(&board);
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

fn apply_conway_rules(cell_state:bool, neighbor_count:u8) -> bool {
    match (cell_state, neighbor_count) {
        (true, n) if n < 2 => false,
        (true, n) if n > 3 => false,
        (true, 2..4) => true,
        (false,3) => true,
        _ => false
    }
}