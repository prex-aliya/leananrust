use ncurses::*;
use std::process::exit;

const START_ROW: i32 = 0;
const START_COL: i32 = 0;
const MENU_LENGTH: i32 = 5;

fn error(msg: &str) {
    endwin();
    eprintln!("ERROR: {}", msg);
    exit(1);
}

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

    fn print(&self, map: &mut Vec<u8>) {
        for row in 0..32 {
            for col in 0..32 {
                if map[col] == 1 {
                    addstr("@@@@");
                } else {
                    addstr("####");
                }
            }
            addch('\n' as u32);
        }
    }

    fn end(&self) {
    }
}
fn game() {
    clear();
    addstr("GAME START");

    let mut game: Game = Game::default();
    let mut map: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    'game :loop {
        game.start(START_COL, START_ROW); /* Defines start location "and size" */
        {
            game.print(&mut map); /* Print the game */
        }

        let input = getch();
        match input as u8 as char {
            'q' => break 'game,
            'w'|'k' => {},
            's'|'j' => {},
            'a'|'h' => {},
            'd'|'l' => {},
            _   => {},
        }

        game.end();
    }
}
/* }}} */
/* Menu {{{ */
struct Menu {
    row: i32,
}
impl Menu {
    //fn start(self) {}
    fn print(&self) {
        let menu: Vec<&str> = vec!["START GAME", "1\t", "2\t", "3\t", "4\t", "QUIT\t"];
        for x in 0..=MENU_LENGTH {
            if self.row == x {
                attron(COLOR_PAIR(1));
                addstr(format!("\t>{}\t\n", menu[x as usize]).as_str());
                attroff(COLOR_PAIR(1));
            } else {
                addstr(format!("\t {}\t\n", menu[x as usize]).as_str());
            }
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
            5 => { addstr("Layer 5"); },
            _ => {}
        }
    }
}
/* Menu }}} */

fn main() {
    initscr();
    noecho(); /* Stops showing of input keys */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    /* Makes The Terminal Cursor Invisible */

    start_color();
    /* Exits if terminal doesnt suport color */
    if has_colors() == false {
        endwin();
        error("Terminal Does not support color");
    }

    /*      pair  forground   background */
    init_pair(1, COLOR_BLACK, COLOR_WHITE);
    init_pair(2, COLOR_WHITE, COLOR_BLACK);


    /* Initilize Menu */
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
    }

    endwin(); /* Restores terminal behavior */
}

#[cfg(test)]
mod test;
