use dioxus::prelude::*;
use crate::models::{ActiveTab, AppData, Habit, Priority, Task, Transaction};
use crate::storage::{load_data, save_data};

#[derive(Clone, Copy)]
pub struct AppState {
    pub data: Signal<AppData>,
    pub active_tab: Signal<ActiveTab>,
}

impl AppState {
    pub fn init() -> Self {
        Self {
            data: Signal::new(load_data()),
            active_tab: Signal::new(ActiveTab::Dashboard),
        }
    }

    pub fn update<F: FnOnce(&mut AppData)>(&mut self, f: F) {
        let mut d = self.data.write();
        f(&mut d);
        save_data(&d);
    }

    pub fn daily_progress_percent(&self) -> u32 {
        let d = self.data.read();
        let total = d.tasks.len() + d.habits.len();
        if total == 0 {
            return 0;
        }
        let completed_tasks = d.tasks.iter().filter(|t| t.completed).count();
        let completed_habits = d.habits.iter().filter(|h| h.completed_today).count();
        (((completed_tasks + completed_habits) as f64 / total as f64) * 100.0).round() as u32
    }

    pub fn toggle_task(&mut self, id: u64) {
        self.update(|d| {
            if let Some(t) = d.tasks.iter_mut().find(|t| t.id == id) {
                t.completed = !t.completed;
            }
        });
    }

    pub fn toggle_pin_task(&mut self, id: u64) {
        self.update(|d| {
            let pinned_count = d.tasks.iter().filter(|t| t.pinned).count();
            if let Some(t) = d.tasks.iter_mut().find(|t| t.id == id) {
                if t.pinned {
                    t.pinned = false;
                } else if pinned_count < 3 {
                    t.pinned = true;
                }
            }
        });
    }

    pub fn add_task(&mut self, title: String, category: String, priority: Priority) {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return;
        }
        self.update(|d| {
            let id = d.next_id;
            d.next_id += 1;
            let auto_pin = d.tasks.iter().filter(|t| t.pinned).count() < 3;
            d.tasks.insert(
                0,
                Task {
                    id,
                    title: trimmed.to_string(),
                    category: if category.trim().is_empty() {
                        "عام".into()
                    } else {
                        category.trim().to_string()
                    },
                    priority,
                    completed: false,
                    pinned: auto_pin,
                },
            );
        });
    }

    pub fn delete_task(&mut self, id: u64) {
        self.update(|d| d.tasks.retain(|t| t.id != id));
    }

    pub fn toggle_habit(&mut self, id: u64) {
        self.update(|d| {
            if let Some(h) = d.habits.iter_mut().find(|h| h.id == id) {
                h.completed_today = !h.completed_today;
                h.history_7d[6] = h.completed_today;
                if h.completed_today {
                    h.streak += 1;
                } else if h.streak > 0 {
                    h.streak -= 1;
                }
            }
        });
    }

    pub fn add_habit(&mut self, title: String, icon: String) {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return;
        }
        self.update(|d| {
            let id = d.next_id;
            d.next_id += 1;
            d.habits.push(Habit {
                id,
                title: trimmed.to_string(),
                icon,
                streak: 0,
                completed_today: false,
                history_7d: [false; 7],
            });
        });
    }

    pub fn set_water_cups(&mut self, cups: u8) {
        self.update(|d| {
            d.water_cups = if d.water_cups == cups {
                cups.saturating_sub(1)
            } else {
                cups.min(8)
            };
        });
    }

    pub fn set_mood(&mut self, idx: usize) {
        self.update(|d| d.mood_index = idx);
    }

    pub fn add_transaction(&mut self, title: String, category: String, amount: f64, is_income: bool) {
        if title.trim().is_empty() || amount <= 0.0 {
            return;
        }
        self.update(|d| {
            let id = d.next_id;
            d.next_id += 1;
            d.transactions.insert(
                0,
                Transaction {
                    id,
                    title: title.trim().to_string(),
                    category: if category.trim().is_empty() {
                        "عام".into()
                    } else {
                        category.trim().to_string()
                    },
                    amount,
                    is_income,
                },
            );
        });
    }

    pub fn set_budget(&mut self, budget: f64) {
        if budget > 0.0 {
            self.update(|d| d.monthly_budget = budget);
        }
    }

    pub fn set_note(&mut self, note: String) {
        self.update(|d| d.quick_note = note);
    }

    pub fn toggle_theme(&mut self) {
        self.update(|d| d.dark_mode = !d.dark_mode);
    }

    pub fn increment_focus_session(&mut self) {
        self.update(|d| d.focus_sessions_today += 1);
    }
}
