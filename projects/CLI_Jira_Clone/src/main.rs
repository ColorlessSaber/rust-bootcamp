use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();
        if let Some(current_page) = navigator.get_current_page(){
            if let Err(err) = current_page.draw_page() {
                println!("Error with rendering page: {}\nPress any key to continue...", err);
                wait_for_key_press();
            }

            let user_input = get_user_input();

            match current_page.handle_input(user_input.trim()) {
                Err(err) => {
                    println!("Error with processing user's input: {}\nPress any key to continue...", err);
                    wait_for_key_press();
                },
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(err) = navigator.handle_action(action) {
                            println!("Error with processing user's input: {}\nPress any key to continue...", err);
                            wait_for_key_press();
                        }
                    }
                }
            }

        } else { // if there is no page, break the loop
            break;
        }

    }
}