// fn main() {
//     println!("Hello, world!");
// }

// note ! std = standart library rust
// :: operator ini pemisah sepertinya
// io (input and output (io)) adalah module yg ada di standard library rust
use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // example
    // let guess = String::new() // ini immutable => ini mirip const yang tidak bisa diubah di js
    // let guess = String::new() // ini mutable => ini mirip let yang ada di js yang bisa di ubah

    let mut guess = String::new();

    // sedangkan stdin adalah function yang ada pada io module (modul io)
    io::stdin()
        .read_line(&mut guess) // membaca variable guess yang sudah di deklarasikan
        .expect("Failed to read line"); // jika gagal membaca variable akan muncul pesan gagal

    println!("You Guess : {}", guess);
}
