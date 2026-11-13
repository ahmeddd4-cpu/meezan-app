use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Dashboard,
    Tasks,
    Habits,
    Focus,
    Finance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn label_ar(&self) -> &'static str {
        match self {
            Priority::High => "عالية",
            Priority::Medium => "متوسطة",
            Priority::Low => "منخفضة",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Priority::High => "#ef4444",
            Priority::Medium => "#f59e0b",
            Priority::Low => "#10b981",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilter {
    All,
    Pending,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub category: String,
    pub priority: Priority,
    pub completed: bool,
    pub pinned: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Habit {
    pub id: u64,
    pub title: String,
    pub icon: String,
    pub streak: u32,
    pub completed_today: bool,
    pub history_7d: [bool; 7],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    Focus25,
    ShortBreak5,
    LongBreak15,
}

impl TimerMode {
    pub fn total_seconds(&self) -> u32 {
        match self {
            TimerMode::Focus25 => 25 * 60,
            TimerMode::ShortBreak5 => 5 * 60,
            TimerMode::LongBreak15 => 15 * 60,
        }
    }

    pub fn label_ar(&self) -> &'static str {
        match self {
            TimerMode::Focus25 => "تركيز عميق (25د)",
            TimerMode::ShortBreak5 => "استراحة قصيرة (5د)",
            TimerMode::LongBreak15 => "استراحة طويلة (15د)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub title: String,
    pub category: String,
    pub amount: f64,
    pub is_income: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppData {
    pub tasks: Vec<Task>,
    pub habits: Vec<Habit>,
    pub water_cups: u8,
    pub mood_index: usize,
    pub monthly_budget: f64,
    pub transactions: Vec<Transaction>,
    pub quick_note: String,
    pub focus_sessions_today: u32,
    pub dark_mode: bool,
    pub next_id: u64,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task {
                    id: 1,
                    title: "مراجعة أهداف الأسبوع وترتيب الأولويات".into(),
                    category: "تخطيط".into(),
                    priority: Priority::High,
                    completed: false,
                    pinned: true,
                },
                Task {
                    id: 2,
                    title: "إنجاز جلسة عمل عميق لمدة 50 دقيقة".into(),
                    category: "عمل".into(),
                    priority: Priority::High,
                    completed: true,
                    pinned: true,
                },
                Task {
                    id: 3,
                    title: "قراءة 15 صفحة قبل النوم".into(),
                    category: "تطوير".into(),
                    priority: Priority::Medium,
                    completed: false,
                    pinned: true,
                },
            ],
            habits: vec![
                Habit {
                    id: 101,
                    title: "صلاة الفجر في وقتها".into(),
                    icon: "🕌".into(),
                    streak: 14,
                    completed_today: true,
                    history_7d: [true, true, true, true, true, true, true],
                },
                Habit {
                    id: 102,
                    title: "رياضة أو مشي 30 دقيقة".into(),
                    icon: "🏃".into(),
                    streak: 6,
                    completed_today: false,
                    history_7d: [true, true, false, true, true, true, false],
                },
                Habit {
                    id: 103,
                    title: "تعلّم مهارة أو برمجة بلغة Rust".into(),
                    icon: "🦀".into(),
                    streak: 9,
                    completed_today: false,
                    history_7d: [true, true, true, true, false, true, false],
                },
            ],
            water_cups: 5,
            mood_index: 3,
            monthly_budget: 3000.0,
            transactions: vec![
                Transaction {
                    id: 201,
                    title: "راتب / دخل شهري".into(),
                    category: "دخل".into(),
                    amount: 4500.0,
                    is_income: true,
                },
                Transaction {
                    id: 202,
                    title: "مشتريات البقالة الأسبوعية".into(),
                    category: "طعام".into(),
                    amount: 320.0,
                    is_income: false,
                },
                Transaction {
                    id: 203,
                    title: "اشتراك إنترنت واتصالات".into(),
                    category: "فواتير".into(),
                    amount: 140.0,
                    is_income: false,
                },
            ],
            quick_note: "التركيز على عادة واحدة كل أسبوع يصنع فرقاً تراكمياً مذهلاً.".into(),
            focus_sessions_today: 2,
            dark_mode: true,
            next_id: 300,
        }
    }
}
