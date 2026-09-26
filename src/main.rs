fn main() {
    let name: String = String::from("Aysu");

    println!("Hello, {}!", name);


    let age: u32 = 20; // u32 type inference
    let height: f64 = 1.65; // f64 type inference
    //u8 → 8 bit ; u16 → 16 bit ; u32 → 32 bit ; u64 → 64 bit ; u128 → 128 bit
    let delta_time:f32 = 1.25_f32;
    println!("delta_time: {}", delta_time);

    let total: u32 = 100+50+1;
    println!("total: {}", total);

    let a = 5;
    let c = 10;
    let b= a * c;
    println!("b: {}", b);


    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);


    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; 
    let remainder = 43 % 5;
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("remainder: {}", remainder);


    // bool → qərar vermək üçün istifadə olunan true/false tipi
    let t = true;
    let f: bool = false; 
    println!("t: {}", t);
    println!("f: {}", f);


    // char → Unicode dəyərləri üçün istifadə olunan tipdir yəni bircə dənə simvolu saxlaya bilir
    let y = 'z';
    let y: char = 'σ'; 
    println!("y: {}", y);

    // tuple → fərqli tipləri bir yerdə saxlaya bilən tipdir
    let person = ("Aysu", 20, 1.71);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    // array → eyni tipdəki dəyərləri bir yerdə saxlaya bilən tipdir
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Months: {:?}", months);


    let saylar = [1, 2, 3, 4, 5];
    let first = saylar[0];
    let second = saylar[1];
    println!("First: {}, Second: {}", first, second);

    use std::io;  // input/output kitabxanasını istifadə edirik
    let eded = [1,3,4,5,6,7,8,9,10];
    println!("{}", eded[0]);

    let mut index = String::new(); // boş bir String yaradırıq və mut içi dəyişəcək anlamına gəlir
    io::stdin() // standart inputdu 
    .read_line(&mut index) // istifadəçidən input oxuyur
    .expect("Failed to read line"); // əgər oxumaqda problem olsa bu mesajı verəcək

    let index: usize = index // usize - integer tipidir
    .trim() // boşluqları silir
    .parse() // istifadəçidən gələn məlumat String olur
    .expect("Index entered was not a number"); // əgər parse etməkdə problem olsa bu mesajı verəcək

}