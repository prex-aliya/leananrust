use ncurses::*;

#[derive(Default)]
struct Game {
    row: i32,
    col: i32,
}
impl Game {
    fn start(&mut self) {
        self.row = 0;
        self.col = 0;

        mv(self.row, self.col);
    }
    fn end(&self) {
    }
}



fn main() {
    initscr();
    noecho(); /* Stops showing of input keys */

    println!("Hello, world!");

    let mut game: Game = Game::default();

    'main :loop {
        game.start(); /* Defines start location "and size" */

        let input = getch();
        match input as u8 as char {
            'q' => break 'main,
            _   => {},
        }

        game.end();
    }

    endwin(); /* Restores terminal behavior */
}
