use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic name:");
    let user_input_epic_name = get_user_input();
    println!("Epic description:");
    let user_input_epic_description = get_user_input();
    Epic::new(user_input_epic_name.trim().to_owned(), user_input_epic_description.trim().to_owned())
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story name:");
    let user_input_story_name = get_user_input();
    println!("Story description:");
    let user_input_story_description = get_user_input();
    Story::new(user_input_story_name.trim().to_owned(), user_input_story_description.trim().to_owned())
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");
    let user_input = get_user_input();
    if user_input.to_lowercase().trim().eq("y") {
        return true;
    }
    false
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]:");
    let user_input = get_user_input();
    if user_input.to_lowercase().trim().eq("y") {
        return true;
    }
    false
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let user_input = get_user_input();
    match user_input.to_lowercase().trim() {
        "1" => Some(Status::Open),
        "2" => Some(Status::InProgress),
        "3" => Some(Status::Resolved),
        "4" => Some(Status::Closed),
        _ => None
    }
}