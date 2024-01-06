# Game of Life in Rust

This is an implementation of Conway's Game of Life in Rust programming language. The Game of Life is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/shawakash/game_of_life.git
   ```

2. Navigate to the project directory:
   ```bash
   cd game-of-life
   ```

3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Usage

To run the game, use the following command:
```bash
cargo run --release
```

## Controls

- Press `Enter` to start the simulation.
- Press `Space` to pause/unpause the simulation.
- Press `Esc` to exit the game.

## Rules

1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

## Configuration

You can customize the game parameters by modifying the following constants in the `main.rs` file:

- `GRID_WIDTH`: Width of the game grid.
- `GRID_HEIGHT`: Height of the game grid.
- `CELL_SIZE`: Size of each cell in pixels.
- `FPS`: Frames per second for the simulation.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). Feel free to use and modify the code as per the license terms.