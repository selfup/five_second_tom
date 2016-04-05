use std::io;

struct Tom {
    hello: (),
    good_bye: (),
    greet_person: (),
}

fn main() {
    loop {
        let tom = Tom {hello: hello(), good_bye: good_bye(), greet_person: greet_person()};

        println!("Your name?!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess == "STOP\n" {
            break;
        } else {
            five_second_tom(tom)
        }
    }
}

fn hello() {
    println!("Hey there");
}

fn good_bye() {
    println!("Goodbye");
}

fn greet_person() {
    println!("I'm Tom!");
}

fn five_second_tom(tom: Tom) {
    tom.hello;
    tom.good_bye;
    tom.greet_person;
}
