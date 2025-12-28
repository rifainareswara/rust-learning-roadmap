fn main() {
    println!("Bismillah")
}

// Ini Komentar

/*
    Komentar ini tidak akan di eksekusi
 */

// Deklarasi Variable menggunakan let
#[test]
fn nama_variable() {
    let nama = "Rifai";
    println!("{}", nama);
}

#[test]
// Immutability pada variable
fn variable_mut(){
    let mut message_number = 9;
    let message1 = "Hallo";

    println!("Variable mut: {}: {}", message_number, message1);

    message_number = 3;
    let message2 = "Rifai";
    println!("Variable mut: {}: {}", message_number, message2);

    // Deklarasi Variabel dalam satu stetment
    let (var1, var2) = (24, "hello");
    println!("var1: {0}", var1); // hasilnya => var1: 24
    println!("var2: {0}", var2); // hasilnya => var2: hello

    // Vairable Shadowing
    let x = 5;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
}

// Variable _
// #[test]
// fn main(){
//     let _ = run_something;
// }

// Tipe Data
// Primitive Skalar
#[test]
    // integers
fn tipe_data(){
    let numerik1 = 24;
    let numerik2: i8 = 2;
    let numerik3: i64 = 12;

    println!("{} | {} | {}", numerik1, numerik2, numerik3);

    // Floating
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;

    println!("{} | {:.5}", fp1, fp2);

    // Bool
    let b1 = true;
    let b2 = false;

    println!("{} | {}", b1, b2);

    // Char
    let c1 = 'n';
    let c2 = '-';
    let c3 = '2';

    println!("{} | {} | {}", c1, c2, c3);
    // output ==> n | - | 2

    // Pointer Scalar
    let ptr1: &i32 = &24;
    println!("{}", ptr1);
    // output ==> 24
}

#[test]
fn test_boolean() {
    // Mendeklarasikan variabel dengan tipe data bool
    let is_active: bool = false;
    let is_raining = true; // Tipe data bisa ditebak otomatis oleh Rust

    // Mencetak nilai boolean
    println!("Status aktif: {}", is_active);
    println!("Apakah sedang hujan? {}", is_raining);

    // Boolean dalam ekspresi if-else
    if is_active {
        println!("Pengguna ini aktif. ✅");
    } else {
        println!("Pengguna ini tidak aktif. ❌");
    }

    if is_raining {
        println!("Jangan lupa bawa payung! ☔");
    } else {
        println!("Cuaca cerah, nikmati harimu! ☀️");
    }


    let has_key = true;
    let door_is_locked = false;

    // Menggunakan operator AND (&&)
    // Akan true hanya jika kedua kondisi true
    if has_key && !door_is_locked {
        println!("Pintu berhasil dibuka.");
    } else {
        println!("Pintu tidak bisa dibuka.");
    }

    // Menggunakan operator OR (||)
    let has_permission = false;
    let is_admin = true;
    if has_permission || is_admin {
        println!("Akses diberikan.");
    }
}

// String Literal (&str)
#[test]
fn tipe_data2 (){

    // Escape karakter menggunakan \
    let var2 = "hello \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var2);

    // Multiline string literal
    let var3 = "baris satu
    baris dua
    baris tiga";
    println!("{}", var3);
}

// Konstanta
#[test]
fn konstanta(){
    const LABEL: &str = "nilai phi adalah:";
    const PHI: f32 = 22.0/7.0;
    println!("{} {}", LABEL, PHI);
}

// Operator
#[test]
fn operators(){

    // Aritmatika
    let (num1, num2) = (11, 12);

    let value_addition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_addition);
    // output => 12 + 4 = 16

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);
    // output => 12 - 4 = 8

    let value_mut = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mut);
    // output => 12 * 4 = 48

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);
    // output => 12 / 4 = 3

    let value_mod = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_mod);
    // output => 12 % 4 = 0

    // Perbandingan
    let number_a = 11;
    let number_b = 12;

    let res_one = number_a == number_b;
    println!("res_one: {res_one}");

    let res_two = number_a != number_b;
    println!("res_two: {res_two}");

    let res_three = number_a > number_b;
    println!("res_three: {res_three}");

    let res_four = number_a < number_b;
    println!("res_four: {res_four}");

    let res_five = number_a >= number_b;
    println!("res_five: {res_five}");

    let res_six = number_a <= number_b;
    println!("res_six: {res_six}");

    // Operator Negasi
    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");
    // output => true true

    // Operator Logika
    let (bool_left, bool_right) = (false, true);
    println!("AND result \t: {}", bool_left && bool_right);
    println!("OR result \t: {}", bool_left || bool_right);
}

/*
If else
 */
#[test]
fn operator_logika() {
    let number_a = 3;
    if number_a < 5 {
        println!("number_a adalah di bawah 5");
    }

    let result_a = number_a >= 5;
    if result_a {
        println!("result_a adalah di atas atau sama dengan 5");
    }

    let number_b = 12;
    if number_b > 3 {
        println!("number_b di atas 3");
    }

    let result_b = number_a >= 3;
    if result_b {
        println!("number_a adalah di atas 5");
    }
}

// if else if
#[test]
fn operator_logika2() {
    let number_c = 4;
    if number_c == 3 {
        println!("number_b adalah 3");
    } else if number_c < 3 {
        println!("number_b adalah di bawah 3");
    } else {
        println!("number_b adalah di atas 3");
    }
}

#[test]
fn is_true() {
    let is_true: bool = false;
    if is_true {
        println!("is_true");
    } else {
        println!("is_false");
    }
}
/*
while
 */

#[test]
fn kondisi(){
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("nilai: {i}");
        i += 1;
    }

    // Nested While
    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let max_inner = i;

        while j <= max_inner {
            print!("* ");
            j += 1;
        }

        println!();
        i += 1;
    }
}

/*
loop, break, continue, label
 */
#[test]
// looping
fn looping(){
    let mut i = 0;

    loop {
        println!("nilai: {i}");
        i += 1;
    }
}

#[test]
//break
fn breaking_loop(){
    let mut i = 0;
    let max = 10;

    loop {
        println!("nilai: {i}");
        i += 1;
        if i > max {
            break;
        }
    }

    println!("perulangan selesai");
}
// Continue
#[test]
fn continue_loop(){
    let mut i = 0;
    let max = 15;

    loop {
        i += 1;

        if i % 2 == 1 {
            continue;
        }

        println!("nilai i: {i}");

        if i > max {
            break;
        }
    }
}

// Label
#[test]
fn label(){
    let mut i = 0;
    let max = 10;

    'mainLoop: loop {
        i += 1;
        let mut j = 0;

        loop {
            if i > max {
                break 'mainLoop;
            }

            j += 1;
            if j > i {
                break;
            }

            print!("{i} ");
        }

        println!();
    }
}