// Quest Runner - Execute and Validate Code
// =========================================

use crate::exercises::{Exercise, Quest};
use crate::progress::Progress;
use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

pub fn run_quest(quest: &Quest, progress: &mut Progress) {
    println!("\n{}", "═══════════════════════════════════════".yellow());
    println!("{} {}", "⚔️".to_string(), quest.name.yellow().bold());
    println!("{}", "═══════════════════════════════════════".yellow());
    println!("{}", quest.description);
    println!("{} {}", "💡 Hint:".cyan(), quest.hint);
    
    for (i, exercise) in quest.exercises.iter().enumerate() {
        println!("\n{}", "───────────────────────────────────────".white());
        
        let status = if progress.is_completed(&exercise.id) {
            "✅".to_string()
        } else {
            "⬜".to_string()
        };
        
        println!("{} Exercise {}/{}: {}", status, i + 1, quest.exercises.len(), exercise.id);
        
        if !progress.is_completed(&exercise.id) {
            run_exercise(exercise, progress);
        }
    }
    
    println!("\n{}", "Tekan Enter untuk kembali...".white());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn run_exercise(exercise: &Exercise, progress: &mut Progress) {
    println!("{}", exercise.question.cyan());
    println!("\n{}", "📝 Code Template:".yellow().bold());
    println!("{}", "```rust".white());
    println!("{}", exercise.code_template.white());
    println!("{}", "```".white());
    
    loop {
        println!("\n{}", "Pilihan:".yellow());
        println!("  1. ✏️  Tulis jawaban");
        println!("  2. 💡 Lihat hint");
        println!("  3. ⏭️  Skip exercise ini");
        
        print!("{}", "Pilih (1-3): ".yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                if write_and_test_code(exercise, progress) {
                    break;
                }
            }
            "2" => {
                println!("\n{} {}", "💡 Hint:".cyan().bold(), exercise.hint);
            }
            "3" => {
                println!("{}", "⏭️  Exercise di-skip.".yellow());
                break;
            }
            _ => println!("{}", "❌ Pilihan tidak valid!".red()),
        }
    }
}

fn write_and_test_code(exercise: &Exercise, progress: &mut Progress) -> bool {
    println!("\n{}", "═══════════════════════════════════════".green());
    println!("{}", "✏️  TULIS KODE KAMU".green().bold());
    println!("{}", "═══════════════════════════════════════".green());
    println!("{}", "Tulis kode Rust lengkap (termasuk fn main)".white());
    println!("{}", "Ketik 'END' di baris baru untuk selesai:".white());
    println!();
    
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }
    
    // Write code to temp file
    let temp_file = "/tmp/rustquest_test.rs";
    let temp_bin = "/tmp/rustquest_test";
    
    fs::write(temp_file, &code).expect("Gagal menulis file temp");
    
    // Compile
    println!("\n{}", "🔧 Compiling...".yellow());
    let compile = Command::new("rustc")
        .args([temp_file, "-o", temp_bin])
        .output();
    
    match compile {
        Ok(output) => {
            if !output.status.success() {
                println!("{}", "❌ COMPILE ERROR!".red().bold());
                println!("{}", String::from_utf8_lossy(&output.stderr).red());
                println!("\n{}", "💡 Coba lagi! Periksa syntax kamu.".yellow());
                return false;
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Gagal compile:".red(), e);
            return false;
        }
    }
    
    // Run
    println!("{}", "🚀 Running...".yellow());
    let run = Command::new(temp_bin).output();
    
    match run {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let expected = exercise.expected_output.trim();
            
            println!("\n{}", "📤 Output kamu:".cyan().bold());
            println!("{}", stdout.white());
            
            if stdout.contains(expected) {
                println!("{}", "═══════════════════════════════════════".green());
                println!("{}", "🎉 BENAR! Quest selesai!".green().bold());
                println!("{}", "═══════════════════════════════════════".green());
                progress.mark_completed(&exercise.id);
                return true;
            } else {
                println!("{}", "═══════════════════════════════════════".red());
                println!("{}", "❌ SALAH! Output tidak sesuai.".red().bold());
                println!("{}", "═══════════════════════════════════════".red());
                println!("{} {}", "Expected:".yellow(), expected);
                println!("\n{}", "💡 Coba lagi!".yellow());
                return false;
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Gagal menjalankan:".red(), e);
            return false;
        }
    }
}
