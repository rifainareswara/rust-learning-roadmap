// RustQuest - Interactive Rust Learning System
// =============================================

mod exercises;
mod progress;
mod runner;

use colored::Colorize;
use std::io::{self, Write};

fn main() {
    clear_screen();
    print_banner();
    
    let mut progress = progress::Progress::load();
    
    loop {
        println!("\n{}", "═══════════════════════════════════════".cyan());
        println!("{}", "📚 MENU UTAMA".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("  1. 🎯 Mulai Quest");
        println!("  2. 📊 Lihat Progress");
        println!("  3. 🔄 Reset Progress");
        println!("  4. ❌ Keluar");
        println!();
        
        print!("{}", "Pilih (1-4): ".yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => run_quest_menu(&mut progress),
            "2" => progress.show_progress(),
            "3" => {
                progress.reset();
                println!("{}", "✅ Progress direset!".green());
            }
            "4" => {
                println!("{}", "\n👋 Sampai jumpa, Rustacean! 🦀\n".cyan());
                break;
            }
            _ => println!("{}", "❌ Pilihan tidak valid!".red()),
        }
    }
}

fn run_quest_menu(progress: &mut progress::Progress) {
    loop {
        clear_screen();
        println!("\n{}", "═══════════════════════════════════════".green());
        println!("{}", "⚔️  PILIH QUEST".green().bold());
        println!("{}", "═══════════════════════════════════════".green());
        
        let quests = exercises::get_all_quests();
        for (i, quest) in quests.iter().enumerate() {
            let status = if progress.is_completed(&quest.id) {
                "✅".to_string()
            } else {
                "⬜".to_string()
            };
            println!("  {}. {} {}", i + 1, status, quest.name);
        }
        println!("  0. ⬅️  Kembali");
        println!();
        
        print!("{}", "Pilih quest: ".yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() == "0" {
            break;
        }
        
        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= quests.len() {
                let quest = &quests[num - 1];
                runner::run_quest(quest, progress);
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_banner() {
    println!("{}", r#"
    ╔═══════════════════════════════════════════════════════╗
    ║                                                       ║
    ║   🦀  R U S T Q U E S T  🦀                          ║
    ║                                                       ║
    ║   Interactive Rust Learning System                    ║
    ║   Belajar Rust dengan cara yang menyenangkan!         ║
    ║                                                       ║
    ╚═══════════════════════════════════════════════════════╝
    "#.cyan().bold());
}
