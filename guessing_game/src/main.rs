// fn main() {
//     println!("Hello, world!");
// }

// note ! std = standart library rust
// :: operator ini pemisah sepertinya
// io (input and output (io)) adalah module yg ada di standard library rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Baris ini mencetak string yang sekarang berisi masukan pengguna. Kumpulan tanda kurung kurawal {} adalah pengganti: bayangkan {} sebagai penjepit kepiting kecil yang menyimpan nilai pada tempatnya. Saat mencetak nilai suatu variabel, nama variabel dapat dimasukkan ke dalam tanda kurung kurawal. Saat mencetak hasil evaluasi ekspresi, tempatkan tanda kurung kurawal kosong di string format, lalu ikuti string format dengan daftar ekspresi yang dipisahkan koma untuk dicetak di setiap placeholder tanda kurung kurawal kosong dalam urutan yang sama. Mencetak variabel dan hasil ekspresi dalam satu panggilan ke println! akan terlihat seperti ini:

    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

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

    let guess: u32 = guess.trim().parse().expect("please type a number!");

    // println!("You Guess : {}", guess);
    println!("You Guess : {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too Small!"),
        Ordering::Greater => println!("too Big"),
        Ordering::Equal => println!("You Win"),
    }
}
