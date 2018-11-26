use std::io;

fn main() {
    let mut entrada = String::new();

    println!("Ingrese cuantos números de FizzBuzz desea");
    io::stdin().read_line(&mut entrada).unwrap();

    let n:i32 = entrada.trim().parse().unwrap();

    for i in 0..n {
        if i % 3 == 0 {
            println!(" Fizz! ");
        } else if i % 5 == 0 {
            println!(" Buzz! ");
        } else if i % 3 == 0 && i % 5 == 0 {
            println!(" FizzBuzz! ");
        } else {
            println!("{}",i);
        }
    }
}
