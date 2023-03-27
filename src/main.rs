use rand::Rng;
use std::io;

fn main() {
    let arr = [1, 2, 89, 11, 32, 9];
    let name = String::from("David");

    println!("{name}");

    for i in arr {
        println!("{i}");
    }
    println!("------------");

    for n in 1..10 {
        println!("{n}");
    }

    // let mut input_string = String::new();

    // println!("Enter number: ");

    // io::stdin()
    //     .read_line(&mut input_string)
    //     .expect("Failed to read line");

    // let number: i32 = input_string
    //     .trim()
    //     .parse()
    //     .expect("Please enter valid number");

    // match number {
    //     1 => println!("One"),
    //     2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    //     13..=19 => println!("Teen"),
    //     _ => println!("Ain't special"),
    // }

    const HOURS_IN_SECONDS: u32 = 60 * 60;
    println!("{}", HOURS_IN_SECONDS);

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);

    println!("{}", random_number);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    //-------------------------------------
    let mut x = [0; 6];

    let mut rng = rand::thread_rng();

    for _i in 1..=20 {
        let face = rng.gen_range(1..=6);

        match face {
            1 => x[0] += 1,
            2 => x[1] += 1,
            3 => x[2] += 1,
            4 => x[3] += 1,
            5 => x[4] += 1,
            6 => x[5] += 1,
            _ => (),
        }
    }

    println!("one={}", x[0]);
    println!("two={}", x[1]);
    println!("three={}", x[2]);
    println!("four={}", x[3]);
    println!("five={}", x[4]);
    println!("six={}", x[5]);

    // --- gerbi safasuri ----
    let mut gerbi = 0;
    let mut safasuri = 0;

    for _i in 1..20 {
        let n = rng.gen_range(1..=2);

        match n {
            1 => gerbi += 1,
            2 => safasuri += 1,
            _ => (),
        }
    }

    println!("gerbi={gerbi}");
    println!("safasuri={safasuri}");

    // 4
    let mut input = String::new();

    println!("Enter any number from 1 to 5: ");
    io::stdin().read_line(&mut input).unwrap();

    let a = input.trim().parse::<i32>().unwrap();

    match a {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Different Number"),
    }

    // 5
    let mut jami = 0;
    let mut namravli = 1;
    let mut sashualo = 0;

    for i in 1..16 {
        jami += i;
        namravli *= i
    }

    sashualo = jami / 15;

    println!("Jami={}", jami);
    println!("Namravli={}", namravli);
    println!("Sashualo={}", sashualo);
}
