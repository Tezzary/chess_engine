const { invoke } = window.__TAURI__.core;

let board;
let pieces;

let piece_container;
let container;

let CELL_WIDTH = 50;

function redraw_pieces() {
  let rect = container.getBoundingClientRect()
  let base_x = rect.left
  let base_y = rect.top
  console.log("redrawing")
  for (let i in pieces) {
    let piece = pieces[i]
    // 2 offset for 2px border
    piece.piece.style.top = (base_y + CELL_WIDTH * piece.pos.y + 2) + "px" 
    piece.piece.style.left = (base_x + CELL_WIDTH * piece.pos.x + 2) + "px"
  }
}
async function update_board() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("initial_setup")
  board = await invoke("get_board");
  
  for (let i in pieces) {
    let piece = pieces[i]
    piece.piece.remove()
  }
  pieces = []
  for (let i in board) {
    let element = board[i]
    if (element.piece != "Empty") {
      let piece = document.createElement("img")
      piece.src = "assets/" + element.piece + "_" + element.team + ".png"
      piece.classList.add("piece")
      let piece_wrapper = {
        piece: piece,
        pos: {x: i % 8, y: Math.floor(i / 8)}
      }
      pieces.push(piece_wrapper)
      piece_container.appendChild(piece)
    }
  }
  redraw_pieces()
}

window.addEventListener("DOMContentLoaded", () => {
  container = document.getElementsByClassName("container")[0]
  piece_container = document.getElementsByClassName("piece_container")[0]
  for(let i = 0; i < 64; i+=1) {
    let cell = document.createElement("div")
    cell.classList.add("cell")
    if (Math.floor(i / 8) % 2 == 0) {
      if (i % 2 == 0) {
        cell.classList.add("white")
      } else {
        cell.classList.add("black")
      }
    }
    else {
      if (i % 2 == 0) {
        cell.classList.add("black")
      } else {
        cell.classList.add("white")
      }
    }

    container.appendChild(cell)
  }
  update_board();
});
