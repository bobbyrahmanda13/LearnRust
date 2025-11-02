fn main() {
    // note :
    // menggunakan print!(text) = ini jika digunakan akan menulis tapi tanpa di enter jadi
    // hasilnya akan gabung
    // jika menggunakan println!(text) = ini jika digunakan akan menulis tapi nanti di enter dan hasil nya tidak akan di gabung
    println!("Hello, world!");
    print!("Hello, world!");
    print!("Hello, world!");
}

#[test] // ini adalah attribut / anotation (pada typescript ini disebut decorator)
fn hello_test() {
    println!("hello test")
}

#[test]
/*
# Variable
variable adalah tempat untuk menyimpan data
cara membuat variable di Rust bisa menggunakan kata kunci "let"
setelah variable diisi data, maka data itu tidak bisa diubah lagi
note : jika sudah buat variable menggunakan "let" maka data itu tidak bisa di ubah lagi
*/
fn test_variable() {
    /*
        let name = "Bobby Rahmanda";
        let name2 = "Bobby Rahmanda";
        println!("Hello {} {}", name, name2);
    */
    let first_name = "Bobby";
    let last_name = "Rahmanda";
    let lala = "pergi";
    println!("Hello {} {} {}", first_name, last_name, lala);
}

#[test]
fn test_mutable() {
    /*
    # Mutable
        seperti dijelaskan sebelumnya, Variable yang sudah di isi datanya tidak bisa diubah lagi atau disebut "immutable"
        Namun Rust juga memperbolehkan jika kita ingin membuat variable yang bisa diubah lagi, atau disebut Mutable
        Caranya kita bisa gunakan kata kunci "let mut" ketika membuat variable
    */
    let mut namee = "Bobby Rahmanda";
    println!("Hello {}", namee);
    namee = "Lidya Jelek";
    println!("Hello {}", namee);

    let mut taik = "taik";
    println!("hello {}", taik);

    taik = "jancok";
    println!("hello {}", taik);
}

#[test]
fn static_typing() {
    let mut name = "windy an-nisa";
    println!("hello {}", name);

    // name = 10;
    name = "bobby rahmanda";
    println!("selamat pagi sayang {}", name);
}

#[test]
fn shadowing() {
    let names = "bobby rahmanda";
    println!("hello {}", names);

    let names = 10;
    println!("hello {}", names);
}

/*
ini adalah komentar dari satu baris
ini adalah komentar dari satu baris
ini adalah komentar dari satu baris
*/
#[test]
fn comment() {
    let windy = "sayang";
    println!("hello {}", windy); // ini adalah komentar
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age);

    let ag: i32 = 222222222;
    println!("{}", ag);
}

/*
* Type Data
- Integer Type (Bilangan Bulat) (default: i32)
Panjang | Signed | Unsigned|
8-bit   | i8     | u8
16-bit  | i16    | u16
32-bit  | i32    | u32
64-bit  | i64    | u64
128-bit | i128   | u128

note:
Panjang = type bilangan bulat
Signed = type bisa negatif bisa positif ( bisa dimulai dari negatif )
Unsigned = type yg hanya bisa positif aja (dimulai dari 0)

* Float Type (default: 64-bit (f64))

Panjang | FLoat |
32-bit   | f32
64-bit   | f64



*/
