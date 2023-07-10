use game::Life;

mod game;
mod gui;

fn main() {
    let mut life = Life::new(vec![]);
    gui::show(&mut life); 
}
