import {Cell} from "day3";
import {memory} from "day3/day3_bg"

export {drawGrid, drawCells}

const drawGrid = (ctx, width, height, cell_size, grid_color) => {
  ctx.beginPath();
  ctx.strokeStyle = grid_color;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (cell_size + 1) + 1, 0);
    ctx.lineTo(i * (cell_size + 1) + 1, (cell_size + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (cell_size + 1) + 1);
    ctx.lineTo((cell_size + 1) * width + 1, j * (cell_size + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = (ctx, grid_world, cell_size, occupied_color, intersection_color) => {
  const cellsPtr = grid_world.cells();
  const width = grid_world.width();
  const height = grid_world.height();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  const getIndex = (row, column) => {
    return row * width + column;
  };

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (cells[idx] !== Cell.Empty) {
        ctx.fillStyle = cells[idx] === Cell.Occupied
        ? occupied_color
        : intersection_color;

        ctx.fillRect(
          col * (cell_size + 1) + 1,
          row * (cell_size + 1) + 1,
          cell_size,
          cell_size
        );
      }
    }
  }
  ctx.stroke();
  
};