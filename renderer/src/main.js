const { invoke } = window.__TAURI__.core;

let board;
let pieces;

let piece_container;

let container;
let cells = [];
let rect

let possible_moves;

let CELL_WIDTH = 50;

function redraw_pieces() {
  rect = container.getBoundingClientRect()
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

function clamp(value) {
  if (value > 7.5) {
    return 7.5
  }
  if (value < -0.5) {
    return -0.5
  }
  return value
}

function reset_background() {
  for (let i = 0; i < cells.length; i++) {
    let cell = cells[i];
    cell.classList.remove("highlighter")
  }
}
function highlight_background() {
  for (let i = 0; i < possible_moves.length; i++) {
    console.log(i)
    let move = possible_moves[i]
    console.log(move)
    console.log(cells[move.y * 8 + move.x])
    cells[move.y * 8 + move.x].classList.add("highlighter")
  }
}

async function update_board() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  
  for (let i in pieces) {
    let piece = pieces[i]
    //piece.piece.removeEventListener("mousedown")
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
      piece.addEventListener("mousedown", piece_mousedown) 
      async function piece_mousedown(e) {
        e.preventDefault()

        piece.style.cursor = "grabbing"

        let start_x = piece_wrapper.pos.x
        let start_y = piece_wrapper.pos.y

        possible_moves = await invoke("get_possible_moves", {startX: start_x, startY: start_y})
        highlight_background()

        function mouse_move(mousemove_event) {
          piece_wrapper.pos.x = clamp((mousemove_event.clientX - rect.left) / CELL_WIDTH - 0.5)
          piece_wrapper.pos.y = clamp((mousemove_event.clientY - rect.top) / CELL_WIDTH - 0.5)
          redraw_pieces()
        }
        async function mouse_up() {
          let promote_to = "Blank"
          if (element.piece == "Pawn" && (Math.round(piece_wrapper.pos.y) == 0 || Math.round(piece_wrapper.pos.y) == 7)){
            promote_to = "Queen"
          }
          document.removeEventListener("mousemove", mouse_move)
          document.removeEventListener("mouseup", mouse_up)
          reset_background()
          piece.style.cursor = "grab"
          await invoke("move_piece", {
            startX: start_x,
            startY: start_y,
            endX: Math.round(piece_wrapper.pos.x),
            endY: Math.round(piece_wrapper.pos.y),
            promoteTo: promote_to,
          })
          board = await invoke("get_board")
          update_board()
        }
        document.addEventListener("mousemove", mouse_move)
        document.addEventListener("mouseup", mouse_up)
      }
      pieces.push(piece_wrapper)
      piece_container.appendChild(piece)
    }

  }
  redraw_pieces()
}

window.addEventListener("DOMContentLoaded", async () => {
  container = document.getElementsByClassName("container")[0]
  piece_container = document.getElementsByClassName("piece_container")[0]
  rect = container.getBoundingClientRect()

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
    
    cells.push(cell)
    container.appendChild(cell)
  }

  await invoke("initial_setup")
  board = await invoke("get_board");

  update_board();
});

window.addEventListener("resize", () => {
  redraw_pieces()
})
