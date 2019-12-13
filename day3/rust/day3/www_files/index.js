import {GridWorld} from "day3";
import {drawGrid, drawCells} from "./utils";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const OCCUPIED_COLOR1 = "#0000ff";
const OCCUPIED_COLOR2 = "#ff0000";

const canvas = document.getElementById("day3-aoc");

function runGridWorld() {
    const grid_world = GridWorld.parse(document.getElementById("FirstString").value);
	const grid_world2 = GridWorld.parse(document.getElementById("SecondString").value);

	// const grid_world = GridWorld.parse("R8,U5,L5,D3");
	// const grid_world2 = GridWorld.parse("U7,R6,D4,L4");

	const grid_world_superimposed = grid_world.superimpose(grid_world2);

	const width = grid_world_superimposed.width();
	const height = grid_world_superimposed.height();

	canvas.height = (CELL_SIZE + 1) * height + 1;
	canvas.width = (CELL_SIZE + 1) * width + 1;
	const ctx = canvas.getContext('2d');

	drawGrid(ctx, width, height, CELL_SIZE, GRID_COLOR);
	drawCells(ctx, grid_world_superimposed, CELL_SIZE, OCCUPIED_COLOR1, OCCUPIED_COLOR2);
	const result = grid_world_superimposed.compute_intersections();
  	setTimeout(function() {
    if(!alert("Shortest distance is: " + result)){
        location.reload();
    }}, 5);
}

const submit = document.getElementById("SubmitQuery")
submit.addEventListener("click", runGridWorld);