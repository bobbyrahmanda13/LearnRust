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
