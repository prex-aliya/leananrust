/* GOTO bottom to read about this code */

/* Module Name */
mod my_mod {
    /* The items within a module are by default private */
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    /* To make a function public append pub to front of function */
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    /* Item can access other items within the same module, even if they are private */
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    /* Modules can be nested */
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        /* Functions declared using `pub(in path)` syntax are only visible within the given path.
         * `path` must be a parent or or ancestor module */
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_my_mod();
        }

        /* Functions declared using `pub(self)` syntax are only visible within the current module,
         * which is the same as leaving them private */
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        /* Function declared using `pub(super)` syntax are only visible within the parent module */
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
    
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    /* pub(crate) makes functions visible within the current crate */
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    /* Nested modules follow the same rules for viibility */
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        /* Private parent item will restrict the visibility of child items, even if it is
         * declared visible with the scope */
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    print!("called `function()`");
}

fn main() {
    /* Modules allow disambiguation between items that have the same name. */
    function();
    my_mod::function();

    /* Public items, including those inside nested modules, can be accessed from outside the parent
     * module*/
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    /* pub(crate) items can be called from anywhere in the same crate */
    my_mod::public_function_in_crate();

    /* pub(in path) items can only be called from within the module specified *
     * ERROR! function `public_function_in_my_mod()` is private. */
    //my_mod::pricate_function();
    /* TODO ^ Try uncommenting this line */

    /* Private items of a module cannot be directly accessed, even if nested in a publi module: */


    /* ERROR! `private_function` is private */
    //my_mod::private_function();
    /* TODO ^ Try uncommenting this line */

    /* ERROR! `private_function` is private */
    //my_mod::private_nested::function();
    /* TODO ^ Try uncommenting this line */

    /* ERROR! `private_function` is private */
    //my_mod::private_nested::restricted_function();
    /* TODO ^ Try uncommenting this line */
}


/*
 * This code is from the rust-lang docs 
 * https://doc.rust-lang.org/rust-by-example/mod/visibility.html
 * I have rewritten this entire segment of code. This is my version of that code, it still has many
 * of the same aspects of the original code. The only peices of code I did not copy myself are some
 * of the print! and println! macros, since they got repetitive. 
 */
