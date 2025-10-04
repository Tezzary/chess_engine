const { invoke } = window.__TAURI__.core;

let board;

async function update_board() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("initial_setup")
  let board = await invoke("get_board");
}

window.addEventListener("DOMContentLoaded", () => {
  let container = document.getElementsByClassName("container")[0]
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
});
