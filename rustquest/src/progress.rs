// Progress Tracking System
// ========================

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const PROGRESS_FILE: &str = "progress.json";

#[derive(Serialize, Deserialize, Default)]
pub struct Progress {
    pub completed: HashSet<String>,
}

impl Progress {
    pub fn load() -> Self {
        if Path::new(PROGRESS_FILE).exists() {
            let data = fs::read_to_string(PROGRESS_FILE).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Progress::default()
        }
    }
    
    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(PROGRESS_FILE, data).expect("Gagal menyimpan progress");
    }
    
    pub fn mark_completed(&mut self, id: &str) {
        self.completed.insert(id.to_string());
        self.save();
    }
    
    pub fn is_completed(&self, id: &str) -> bool {
        self.completed.contains(id)
    }
    
    pub fn reset(&mut self) {
        self.completed.clear();
        self.save();
    }
    
    pub fn show_progress(&self) {
        use crate::exercises::get_all_quests;
        
        println!("\n{}", "═══════════════════════════════════════".magenta());
        println!("{}", "📊 PROGRESS KAMU".magenta().bold());
        println!("{}", "═══════════════════════════════════════".magenta());
        
        let quests = get_all_quests();
        let mut total_exercises = 0;
        let mut total_completed = 0;
        
        for quest in &quests {
            let completed_count = quest.exercises
                .iter()
                .filter(|e| self.is_completed(&e.id))
                .count();
            let total = quest.exercises.len();
            
            total_exercises += total;
            total_completed += completed_count;
            
            let status = if completed_count == total {
                "✅ SELESAI".green().to_string()
            } else if completed_count > 0 {
                format!("🔄 {}/{}", completed_count, total).yellow().to_string()
            } else {
                "⬜ Belum mulai".white().to_string()
            };
            
            println!("  {} - {}", quest.name, status);
        }
        
        println!("{}", "───────────────────────────────────────".magenta());
        
        let percentage = if total_exercises > 0 {
            (total_completed as f64 / total_exercises as f64 * 100.0) as i32
        } else {
            0
        };
        
        println!(
            "  {} {}/{}  ({}%)",
            "Total Progress:".bold(),
            total_completed,
            total_exercises,
            percentage
        );
        
        // Achievement badges
        println!("\n{}", "🏆 BADGES:".yellow().bold());
        
        if total_completed >= 5 {
            println!("  {} Rust Seedling - Selesaikan 5 quest pertama", "🌱".to_string());
        }
        if total_completed >= 10 {
            println!("  {} Flow Master - Selesaikan 10 quest", "🔧".to_string());
        }
        if self.completed.iter().any(|id| id.starts_with("1.4")) {
            println!("  {} Ownership King - Menguasai Ownership", "👑".to_string());
        }
        if total_completed == total_exercises {
            println!("  {} Fundamentals Complete! - Semua quest selesai!", "🎖️".to_string());
        }
        
        println!();
    }
}
