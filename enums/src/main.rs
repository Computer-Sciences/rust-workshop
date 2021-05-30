enum Controller {
    Turbo,
    Up,
    Down,
    Left,
    Right,
    X,
    Y,
    A,
    B,
}

fn push_button_notify(c: &Controller) {
    // pattern mathing (equivalent to swith in JavaScript)
    match c {
        Controller::Turbo => println!("Turbo button pushed."),
        Controller::Up => println!("Up button pushed."),
        Controller::Down => println!("Down button pushed."),
        Controller::Left => println!("Left button pushed."),
        Controller::Right => println!("Right button pushed."),
        Controller::Y => println!("Y button pushed."),
        Controller::X => println!("X button pushed."),
        Controller::A => println!("A button pushed."),
        Controller::B => println!("B button pushed."),
    }
}

fn main() {
    let secret_push_combo = [
        Controller::Up,
        Controller::Left,
        Controller::A,
        Controller::Turbo,
        Controller::Y,
        Controller::B,
        Controller::Turbo,
        Controller::Down,
        Controller::Right,
        Controller::X,
    ];

    for push in secret_push_combo.iter() {
        push_button_notify(push);
    }
}
