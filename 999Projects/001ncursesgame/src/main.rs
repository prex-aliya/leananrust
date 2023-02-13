use ncurses::*;

const START_ROW: i32 = 0;
const START_COL: i32 = 0;
const MENU_LENGTH: i32 = 5;


/* Menu {{{ */
struct Menu {
    row: i32,
}
impl Menu {
    fn start(&mut self, row: i32) {
        self.row = row;

        mv(self.row, 8);
    }
    fn print(&self) {
        for x in 0..=MENU_LENGTH {
            if self.row == x {
                addstr("> ");
            }
            addstr(format!("{}\n", x).as_str());
        }
    }

    fn up(&mut self) { 
        if self.row > 0 {
            self.row -= 1;
        }
    }
    fn down(&mut self) {
        if self.row < MENU_LENGTH {
            self.row += 1;
        }
    }

    fn select(&self) {
        match self.row {
            0 => { game(); },
            1 => { addstr("Layer 1"); },
            2 => { addstr("Layer 2"); },
            3 => { addstr("Layer 3"); },
            4 => { addstr("Layer 4"); },
            _ => {}
        }
    }
}
/* Menu }}} */
/* Game {{{ */
#[derive(Default)]
struct Game {
    row: i32,
    col: i32,
}
impl Game {
    fn start(&mut self, col: i32, row: i32) {
        self.row = row;
        self.col = col;

        mv(self.row, self.col);
    }
    fn end(&self) {
    }
}
fn game() {
    addstr("GAME START");
    let mut game: Game = Game::default();
    'game :loop {
        game.start(START_COL, START_ROW); /* Defines start location "and size" */
        {
            //game.print(); /* Print the game */
        }

        let input = getch();
        match input as u8 as char {
            'q' => break 'game,
            _   => {},
        }

        game.end();
    }
}
/* }}} */

fn main() {
    initscr();
    noecho(); /* Stops showing of input keys */

    println!("Hello, world!");

    let mut menu: Menu = Menu {row: 0 };

    'main :loop {
        clear();
        //menu.start(0);
        menu.print();

        match getch() as u8 as char {
            'q' => { break 'main },
            'k' => { menu.up(); },
            'j' => { menu.down(); },
            '\n' => { menu.select(); },
            _ => {}
        }
        addstr(format!("{}", menu.row).as_str());
    }

    endwin(); /* Restores terminal behavior */
}
