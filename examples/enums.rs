enum Action {
    Drive,
    Turn(Direction),
    Stop,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: Action){
    match a {
        Action::Drive => println!("Driving ..."),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turning Left .."),
            Direction::Right => println!("Turning Right .."),
        }
        Action::Stop => println!("Stopping ..."),
    }
}

fn main() {
    let actions = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop
    ];

    for action in actions {
        print_action(action);
    }
}