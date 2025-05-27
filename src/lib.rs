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
pub fn flip_cell(cursor_x:u32, cursor_y:u32,board_data: Clamped<Vec<u8>>, width: u32, height: u32, cell_length: u32) -> Clamped<Vec<u8>> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut output = board_data.clone();
    let board = Board::new(board_data, width, height, cell_length);
    let cell_index = pixel_to_cell_index(cursor_x, cursor_y, &board);
    let cell_state = matches!(state_of_index(cell_index, &board), 1);
    output = update_cell(cell_index, !cell_state, &board, output);

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