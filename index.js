import init, { test, step, neighbor_positions } from "./pkg/wasm_conway_game.js";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const cell_length = 10;
const pixel_length = 4;
const height = 500;
const width = 500;
let board_state = ctx.createImageData(height, width);
let running = false;

const step_button = document.getElementById("step-button");
const start_button = document.getElementById("start-button");

init().then(() => {
  let interval;
  // Draw image data to the canvas
  ctx.putImageData(board_state, 0, 0);

  canvas.addEventListener("click", async(e)=>{
    const cursor_pos = getCursorPosition(canvas, e);
    const { x: pixel_x, y: pixel_y } = cursor_pos;
    console.log("clicked [" + pixel_x + "," + pixel_y + "]");
    let {x: cell_x, y: cell_y} = pixel_to_cell(pixel_x, pixel_y)
    console.log("cell [" + cell_x + "," + cell_y + "]");
    flip_cell_pixels(cell_x, cell_y);
    ctx.putImageData(board_state, 0, 0);
    console.log("rust: " + neighbor_positions(board_state.data, width, height, cell_length, cell_x, cell_y ));
  });

  step_button.addEventListener("click", async(e)=>{
    board_state = new ImageData(step(board_state.data, width, height, cell_length), width, height);
    ctx.putImageData(board_state, 0, 0);
  });

  start_button.addEventListener("click", async(e)=>{
    running = !running;
    if(running) {
      interval = setInterval(take_step, 70);
      start_button.innerHTML = "stop";
    } else {
      clearInterval(interval);
      start_button.innerHTML = "start";
    }
  });

  function take_step(){
    if(running) {
      board_state = new ImageData(step(board_state.data, width, height, cell_length), width, height);
      ctx.putImageData(board_state, 0, 0);
      console.log("Running")
      
    }
  }

  function getCursorPosition(canvas, event) {
    const rect = canvas.getBoundingClientRect()
    const x = event.clientX - rect.left
    const y = event.clientY - rect.top

    return {x,y}
  }

  function pixel_to_cell(x,y){
    return { x: Math.floor(x/cell_length), y: Math.floor(y/cell_length) }
  }

  function set_pixel(x,y, state){
    const address = coor_to_pixel_address(x,y);
    board_state.data[address] = state;
  }

  function coor_to_pixel_address(x,y){
    const pixel_index = (y * width) + x;
    return 3 + (pixel_index * pixel_length)

  }

  function flip_cell_pixels(x,y){
    const x_floor = x * cell_length;
    const x_ciel = x_floor + cell_length - 1;
    const y_floor = y * cell_length;
    const y_ciel = y_floor + cell_length - 1;

    const first_address = coor_to_pixel_address(x_floor, y_floor);
    const last_address = coor_to_pixel_address(x_ciel, y_ciel);
    console.log("address of 1st pixel: "+first_address);
    console.log("address of last pixel: "+last_address)
    const flipped_value = board_state.data[first_address] == 0 ? 255 : 0;

    for (let x_index = x_floor; x_index <= x_ciel; x_index++) {
      for (let y_index = y_floor; y_index <= y_ciel; y_index++) {
        set_pixel(x_index,y_index, flipped_value);
      }
    }
  }
});

