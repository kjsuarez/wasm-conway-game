const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const cell_length = 10;
const pixel_length = 4;
const height = 300;
const width = 300;
const board_state = ctx.createImageData(height, width);


// Draw image data to the canvas
ctx.putImageData(board_state, 0, 0);

canvas.addEventListener("click", async(e)=>{
  const cursor_pos = getCursorPosition(canvas, e);
  const { x: pixel_x, y: pixel_y } = cursor_pos;
  console.log("clicked [" + pixel_x + "," + pixel_y + "]");
  let {x: cell_x, y: cell_y} = pixel_to_cell(pixel_x, pixel_y)
  console.log("cell [" + cell_x + "," + cell_y + "]");
  fill_cell_pixels(cell_x, cell_y);
  ctx.putImageData(board_state, 0, 0);
});

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const y = event.clientY - rect.top

  return {x,y}
}

function pixel_to_cell(x,y){
  return { x: Math.floor(x/cell_length), y: Math.floor(y/cell_length) }
}

function fill_pixel(x,y){
  const cell = (y * width) + x;
  // console.log("cell: " + cell);
  const address = 3 + (cell * pixel_length);
  board_state.data[address] = 255;
  // console.log("address: " + address);
}

function fill_cell_pixels(x,y){
  const x_floor = x * cell_length;
  const x_ciel = x_floor + cell_length - 1;
  const y_floor = y * cell_length;
  const y_ciel = y_floor + cell_length - 1;

  for (let x_index = x_floor; x_index < x_ciel; x_index++) {
    for (let y_index = y_floor; y_index < y_ciel; y_index++) {
      fill_pixel(x_index,y_index);
    }
  }
}