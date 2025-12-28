// Quest Exercises Definition
// ==========================

pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hint: String,
    pub exercises: Vec<Exercise>,
}

pub struct Exercise {
    pub id: String,
    pub question: String,
    pub code_template: String,
    pub expected_output: String,
    pub hint: String,
}

pub fn get_all_quests() -> Vec<Quest> {
    vec![
        quest_variables(),
        quest_control_flow(),
        quest_functions(),
        quest_ownership(),
    ]
}

fn quest_variables() -> Quest {
    Quest {
        id: "1.1".to_string(),
        name: "📦 Variables & Data Types".to_string(),
        description: "Pelajari cara mendeklarasikan variable dan tipe data di Rust".to_string(),
        hint: "Ingat: variable di Rust immutable by default!".to_string(),
        exercises: vec![
            Exercise {
                id: "1.1.1".to_string(),
                question: r#"
🎯 QUEST 1.1.1: Deklarasi Variable

Buat variable `nama` dengan tipe String yang berisi namamu,
lalu print dengan format: "Halo, {nama}!"

Contoh output yang diharapkan (jika namamu Rifai):
Halo, Rifai!
"#.to_string(),
                code_template: r#"fn main() {
    // TODO: Buat variable nama di sini
    let nama = ____;
    
    println!("Halo, {}!", nama);
}"#.to_string(),
                expected_output: "Halo,".to_string(), // Partial match
                hint: "Gunakan String::from(\"nama\") atau \"nama\".to_string()".to_string(),
            },
            Exercise {
                id: "1.1.2".to_string(),
                question: r#"
🎯 QUEST 1.1.2: Mutability

Buat variable mutable `counter` dengan nilai awal 0.
Tambahkan 1 ke counter, lalu print hasilnya.

Output yang diharapkan:
Counter: 1
"#.to_string(),
                code_template: r#"fn main() {
    // TODO: Buat variable mutable counter
    let ____ counter = 0;
    
    // TODO: Tambahkan 1 ke counter
    counter = ____;
    
    println!("Counter: {}", counter);
}"#.to_string(),
                expected_output: "Counter: 1".to_string(),
                hint: "Gunakan keyword 'mut' untuk membuat variable mutable".to_string(),
            },
            Exercise {
                id: "1.1.3".to_string(),
                question: r#"
🎯 QUEST 1.1.3: Data Types

Buat variable untuk menyimpan:
- umur (u8): 25
- tinggi (f64): 175.5
- is_student (bool): true

Print semua dengan format yang benar.

Output yang diharapkan:
Umur: 25, Tinggi: 175.5, Student: true
"#.to_string(),
                code_template: r#"fn main() {
    let umur: u8 = ____;
    let tinggi: f64 = ____;
    let is_student: bool = ____;
    
    println!("Umur: {}, Tinggi: {}, Student: {}", umur, tinggi, is_student);
}"#.to_string(),
                expected_output: "Umur: 25, Tinggi: 175.5, Student: true".to_string(),
                hint: "u8 untuk angka positif kecil, f64 untuk decimal, bool untuk true/false".to_string(),
            },
            Exercise {
                id: "1.1.4".to_string(),
                question: r#"
🎯 QUEST 1.1.4: Tuple & Destructuring

Buat tuple `person` berisi (nama, umur, kota).
Destructure tuple tersebut ke variable terpisah.

Output yang diharapkan:
Nama: Rifai, Umur: 25, Kota: Jakarta
"#.to_string(),
                code_template: r#"fn main() {
    let person = ("Rifai", 25, "Jakarta");
    
    // TODO: Destructure tuple
    let (nama, ____, ____) = person;
    
    println!("Nama: {}, Umur: {}, Kota: {}", nama, umur, kota);
}"#.to_string(),
                expected_output: "Nama: Rifai, Umur: 25, Kota: Jakarta".to_string(),
                hint: "Destructuring: let (a, b, c) = tuple;".to_string(),
            },
            Exercise {
                id: "1.1.5".to_string(),
                question: r#"
🎯 QUEST 1.1.5: Array

Buat array `hari` berisi 7 hari dalam seminggu (Senin-Minggu).
Print hari pertama dan hari terakhir.

Output yang diharapkan:
Hari pertama: Senin, Hari terakhir: Minggu
"#.to_string(),
                code_template: r#"fn main() {
    let hari: [&str; 7] = [
        "Senin", "Selasa", "Rabu", "Kamis", 
        "Jumat", "Sabtu", ____
    ];
    
    println!("Hari pertama: {}, Hari terakhir: {}", hari[0], hari[____]);
}"#.to_string(),
                expected_output: "Hari pertama: Senin, Hari terakhir: Minggu".to_string(),
                hint: "Array index dimulai dari 0. Untuk 7 elemen, index terakhir adalah 6.".to_string(),
            },
        ],
    }
}

fn quest_control_flow() -> Quest {
    Quest {
        id: "1.2".to_string(),
        name: "🔀 Control Flow".to_string(),
        description: "Pelajari if, loop, while, for, dan match".to_string(),
        hint: "Match adalah pattern matching yang sangat powerful di Rust!".to_string(),
        exercises: vec![
            Exercise {
                id: "1.2.1".to_string(),
                question: r#"
🎯 QUEST 1.2.1: If Expression

Buat program yang mengecek apakah angka genap atau ganjil.
Untuk angka = 7, output yang diharapkan:

7 adalah bilangan ganjil
"#.to_string(),
                code_template: r#"fn main() {
    let angka = 7;
    
    if angka % 2 == ____ {
        println!("{} adalah bilangan genap", angka);
    } else {
        println!("{} adalah bilangan ganjil", angka);
    }
}"#.to_string(),
                expected_output: "7 adalah bilangan ganjil".to_string(),
                hint: "Modulo 2: jika hasilnya 0 berarti genap, selain itu ganjil".to_string(),
            },
            Exercise {
                id: "1.2.2".to_string(),
                question: r#"
🎯 QUEST 1.2.2: Loop dengan Break

Buat loop yang menghitung dari 1 sampai 5, lalu break.
Print "Selesai!" setelah loop.

Output yang diharapkan:
1
2
3
4
5
Selesai!
"#.to_string(),
                code_template: r#"fn main() {
    let mut i = 1;
    
    loop {
        println!("{}", i);
        i += 1;
        
        if i > ____ {
            break;
        }
    }
    
    println!("Selesai!");
}"#.to_string(),
                expected_output: "1\n2\n3\n4\n5\nSelesai!".to_string(),
                hint: "Break ketika i lebih dari 5".to_string(),
            },
            Exercise {
                id: "1.2.3".to_string(),
                question: r#"
🎯 QUEST 1.2.3: For Loop

Gunakan for loop untuk print angka 1 sampai 3.

Output yang diharapkan:
Angka: 1
Angka: 2
Angka: 3
"#.to_string(),
                code_template: r#"fn main() {
    for i in 1..=____ {
        println!("Angka: {}", i);
    }
}"#.to_string(),
                expected_output: "Angka: 1\nAngka: 2\nAngka: 3".to_string(),
                hint: "1..=3 berarti range inklusif dari 1 sampai 3".to_string(),
            },
            Exercise {
                id: "1.2.4".to_string(),
                question: r#"
🎯 QUEST 1.2.4: Match Expression

Gunakan match untuk mengkonversi nilai angka ke grade:
- 90-100: "A"
- 80-89: "B"  
- 70-79: "C"
- < 70: "D"

Untuk nilai = 85, output:
Grade: B
"#.to_string(),
                code_template: r#"fn main() {
    let nilai = 85;
    
    let grade = match nilai {
        90..=100 => "A",
        80..=89 => "____",
        70..=79 => "C",
        _ => "D",
    };
    
    println!("Grade: {}", grade);
}"#.to_string(),
                expected_output: "Grade: B".to_string(),
                hint: "Pattern 80..=89 cocok untuk nilai 85".to_string(),
            },
        ],
    }
}

fn quest_functions() -> Quest {
    Quest {
        id: "1.3".to_string(),
        name: "⚡ Functions".to_string(),
        description: "Pelajari cara membuat dan menggunakan functions".to_string(),
        hint: "Function di Rust harus mendeklarasikan tipe return!".to_string(),
        exercises: vec![
            Exercise {
                id: "1.3.1".to_string(),
                question: r#"
🎯 QUEST 1.3.1: Function Dasar

Buat function `sapa` yang menerima nama dan mengembalikan greeting.

Output yang diharapkan:
Halo, Rustacean!
"#.to_string(),
                code_template: r#"fn sapa(nama: &str) -> String {
    format!("Halo, {}!", ____)
}

fn main() {
    let greeting = sapa("Rustacean");
    println!("{}", greeting);
}"#.to_string(),
                expected_output: "Halo, Rustacean!".to_string(),
                hint: "Gunakan parameter nama di dalam format!".to_string(),
            },
            Exercise {
                id: "1.3.2".to_string(),
                question: r#"
🎯 QUEST 1.3.2: Function dengan Kalkulasi

Buat function `luas_persegi` yang menghitung luas persegi.
Rumus: sisi * sisi

Output yang diharapkan (sisi = 5):
Luas: 25
"#.to_string(),
                code_template: r#"fn luas_persegi(sisi: i32) -> i32 {
    ____ * sisi
}

fn main() {
    let luas = luas_persegi(5);
    println!("Luas: {}", luas);
}"#.to_string(),
                expected_output: "Luas: 25".to_string(),
                hint: "Return expression tidak perlu semicolon di akhir".to_string(),
            },
            Exercise {
                id: "1.3.3".to_string(),
                question: r#"
🎯 QUEST 1.3.3: Multiple Return (Tuple)

Buat function yang mengembalikan min dan max dari 2 angka.

Output yang diharapkan (a=10, b=5):
Min: 5, Max: 10
"#.to_string(),
                code_template: r#"fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (____, ____)
    }
}

fn main() {
    let (min, max) = min_max(10, 5);
    println!("Min: {}, Max: {}", min, max);
}"#.to_string(),
                expected_output: "Min: 5, Max: 10".to_string(),
                hint: "Jika a >= b, maka b adalah min dan a adalah max".to_string(),
            },
        ],
    }
}

fn quest_ownership() -> Quest {
    Quest {
        id: "1.4".to_string(),
        name: "👑 Ownership (CRITICAL!)".to_string(),
        description: "Konsep paling penting di Rust - JANGAN SKIP!".to_string(),
        hint: "Setiap value di Rust hanya punya SATU owner!".to_string(),
        exercises: vec![
            Exercise {
                id: "1.4.1".to_string(),
                question: r#"
🎯 QUEST 1.4.1: Clone untuk Menghindari Move

String s1 sudah di-move ke s2, sehingga s1 tidak valid lagi.
Gunakan clone() agar s1 tetap bisa digunakan.

Output yang diharapkan:
s1: hello
s2: hello
"#.to_string(),
                code_template: r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1.____();  // Clone s1
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}"#.to_string(),
                expected_output: "s1: hello\ns2: hello".to_string(),
                hint: "Gunakan method .clone() untuk membuat copy dari String".to_string(),
            },
            Exercise {
                id: "1.4.2".to_string(),
                question: r#"
🎯 QUEST 1.4.2: Borrowing dengan Reference

Buat function yang meminjam String tanpa mengambil ownership.
Gunakan reference (&) agar string asli masih bisa digunakan.

Output yang diharapkan:
Panjang hello world: 11
String masih valid: hello world
"#.to_string(),
                code_template: r#"fn hitung_panjang(s: ____String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello world");
    let panjang = hitung_panjang(&s);
    
    println!("Panjang {}: {}", s, panjang);
    println!("String masih valid: {}", s);
}"#.to_string(),
                expected_output: "Panjang hello world: 11".to_string(),
                hint: "Gunakan & untuk membuat reference (borrowing)".to_string(),
            },
            Exercise {
                id: "1.4.3".to_string(),
                question: r#"
🎯 QUEST 1.4.3: Mutable Reference

Buat function yang mengubah String menggunakan mutable reference.
Tambahkan " world" ke string "hello".

Output yang diharapkan:
Hasil: hello world
"#.to_string(),
                code_template: r#"fn tambah_world(s: &____ String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    tambah_world(&mut s);
    
    println!("Hasil: {}", s);
}"#.to_string(),
                expected_output: "Hasil: hello world".to_string(),
                hint: "Gunakan &mut untuk mutable reference".to_string(),
            },
            Exercise {
                id: "1.4.4".to_string(),
                question: r#"
🎯 QUEST 1.4.4: Ownership Rules

Isi bagian yang kosong dengan jawaban yang benar:
1. Setiap value di Rust hanya punya ____ owner
2. Ketika owner keluar dari scope, value akan di-____

Output yang diharapkan:
Rule 1: satu
Rule 2: drop
"#.to_string(),
                code_template: r#"fn main() {
    let rule1 = "____";  // satu
    let rule2 = "____";  // drop
    
    println!("Rule 1: {}", rule1);
    println!("Rule 2: {}", rule2);
}"#.to_string(),
                expected_output: "Rule 1: satu\nRule 2: drop".to_string(),
                hint: "Ownership rules: 1 owner, drop when out of scope".to_string(),
            },
        ],
    }
}
