import init, { step, flip_cell } from "./pkg/wasm_conway_game.js";

let interval;
const cell_length = 10;
const pixel_length = 4;
const height = 500;
const width = 500;
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
let board_state = ctx.createImageData(height, width);
ctx.putImageData(board_state, 0, 0);
let step_interval = 50;
let running = false;

const step_button = document.getElementById("step-button");
const start_button = document.getElementById("start-button");

init().then(() => {
  canvas.addEventListener("click", async(e)=>{
    const cursor_pos = getCursorPosition(canvas, e);
    const { x: pixel_x, y: pixel_y } = cursor_pos;
    const next_board_state = flip_cell(
      pixel_x, pixel_y, 
      board_state.data, 
      width, height, 
      cell_length
    );
    board_state = new ImageData(next_board_state, width, height);
    ctx.putImageData(board_state, 0, 0);
  });

  step_button.addEventListener("click", async(e)=>{
    const next_board_state = step(board_state.data, width, height, cell_length);
    board_state = new ImageData(next_board_state, width, height);
    ctx.putImageData(board_state, 0, 0);
  });

  start_button.addEventListener("click", async(e)=>{
    running = !running;
    if(running) {
      interval = setInterval(take_step, step_interval);
      start_button.innerHTML = "stop";
    } else {
      clearInterval(interval);
      start_button.innerHTML = "start";
    }
  });

  function take_step(){
    if(running) {
      const next_board_state = step(board_state.data, width, height, cell_length);
      board_state = new ImageData(next_board_state, width, height);
      ctx.putImageData(board_state, 0, 0);
    }
  }
});

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const y = event.clientY - rect.top

  return {x,y}
}