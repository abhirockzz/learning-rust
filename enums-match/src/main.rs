enum Choice {
    One,
    Two,
    Three,
}

enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let num = 101;
    if num < 100 {
        println!("{} is less than 100", num)
    } else if num > 100 {
        println!("{} is more than 100", num)
    } else if num == 100 {
        println!("{} is equal 100", num)
    }

    let choice = Choice::One;
    match choice {
        Choice::One => println!("Option 1"),
        Choice::Two => println!("Option 2"),
        Choice::Three => println!("Option 3"),
    }
    let name = parse_name_arg();
    let greeting = match name {
        Some(n) => n,
        None => String::from("there"),
    };
    println!("Hello {}!", greeting);

    let today = Days::Monday;
    match today {
        Days::Friday => println!("thank god its Friday!"),
        _ => (),
    }

    if let today = Days::Monday {
        println!("its Monday already! :(");
    }
}

fn parse_name_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        None
    } else {
        //assuming cargo run
        let name = &args[1];
        Some(String::from(name))
    }
}
struct Player {
    name: String,
    rank: i32,
}
