use ncurses::*;
mod extensions {
    use ncurses::*;
    pub fn begin() {
        /* calles external list */
        //addstr("begin!\n");
    }


    pub fn move_left() {
        addstr("moved left!");
    }
    pub fn move_right() {
        addstr("moved right!");
    }
    pub fn move_up() {
        addstr("moved up!");
    }
    pub fn move_down() {
        addstr("moved down!");
    }


    pub fn end() {
        /* calles external list */
        //addstr("\nend!\n");
    }
}



fn main() {
    initscr();
    noecho(); /* Stops showing of input keys */

    println!("Hello, world!");

    'main :loop {
        extensions::begin();

        let key = getch();
        match key as u8 as char {
            'h' => extensions::move_left(),
            'l' => extensions::move_right(),
            'k' => extensions::move_up(),
            'j' => extensions::move_down(),
            'q' => { break 'main; },
            _ => {},
        }

        extensions::end();
    }

    endwin(); /* Restores terminal behavior */
}
