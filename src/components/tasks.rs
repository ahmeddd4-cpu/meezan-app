use dioxus::prelude::*;
use crate::models::{Priority, TaskFilter};
use crate::state::AppState;

#[component]
pub fn TasksView() -> Element {
    let mut state = use_context::<AppState>();
    let mut title = use_signal(String::new);
    let mut category = use_signal(|| "عمل".to_string());
    let mut priority = use_signal(|| Priority::Medium);
    let mut filter = use_signal(|| TaskFilter::All);

    let current_filter = *filter.read();
    let tasks: Vec<_> = state
        .data
        .read()
        .tasks
        .iter()
        .filter(|t| match current_filter {
            TaskFilter::All => true,
            TaskFilter::Pending => !t.completed,
            TaskFilter::Completed => t.completed,
        })
        .cloned()
        .collect();

    rsx! {
        div {
            class: "page-transition",

            // إضافة مهمة جديدة
            div {
                class: "card stagger-item",
                style: "--i: 0;",
                h3 { style: "margin: 0 0 12px; font-size: 16px; font-weight: 800;", "➕ مهمة جديدة" }
                input {
                    class: "input-field",
                    style: "margin-bottom: 10px;",
                    placeholder: "ما الذي تريد إنجازه؟",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                }
                div {
                    style: "display: flex; gap: 8px; margin-bottom: 12px;",
                    input {
                        class: "input-field",
                        style: "flex: 1;",
                        placeholder: "التصنيف (عمل، دراسة...)",
                        value: "{category}",
                        oninput: move |e| category.set(e.value()),
                    }
                    select {
                        class: "input-field",
                        style: "width: 120px;",
                        onchange: move |e| {
                            let p = match e.value().as_str() {
                                "High" => Priority::High,
                                "Low" => Priority::Low,
                                _ => Priority::Medium,
                            };
                            priority.set(p);
                        },
                        option { value: "Medium", "متوسطة" }
                        option { value: "High", "عالية 🔥" }
                        option { value: "Low", "منخفضة" }
                    }
                }
                button {
                    class: "btn-primary tap-spring",
                    style: "width: 100%;",
                    onclick: move |_| {
                        state.add_task(title.read().clone(), category.read().clone(), *priority.read());
                        title.set(String::new());
                    },
                    "إضافة المهمة"
                }
            }

            // أزرار التصفية
            div {
                style: "display: flex; gap: 8px; margin-bottom: 14px;",
                for (f, label) in [
                    (TaskFilter::All, "الكل"),
                    (TaskFilter::Pending, "المتبقية"),
                    (TaskFilter::Completed, "المكتملة"),
                ] {
                    button {
                        key: "{label}",
                        class: "tap-spring",
                        style: if current_filter == f {
                            "flex: 1; padding: 9px; border-radius: 12px; border: none; background: var(--accent); color: #fff; font-weight: 700; font-size: 13px;"
                        } else {
                            "flex: 1; padding: 9px; border-radius: 12px; border: 1px solid var(--border-subtle); background: var(--bg-card); color: var(--text-secondary); font-weight: 600; font-size: 13px;"
                        },
                        onclick: move |_| filter.set(f),
                        "{label}"
                    }
                }
            }

            // قائمة المهام المتحركة بالتتابع
            for (idx, task) in tasks.into_iter().enumerate() {
                div {
                    key: "{task.id}",
                    class: "card stagger-item",
                    style: "--i: {idx}; display: flex; align-items: center; gap: 12px; padding: 14px;",
                    button {
                        class: if task.completed { "tap-spring check-pop" } else { "tap-spring" },
                        style: "background: none; border: none; font-size: 20px; padding: 0;",
                        onclick: move |_| state.toggle_task(task.id),
                        if task.completed { "✅" } else { "⬜" }
                    }
                    div {
                        style: "flex: 1;",
                        div {
                            style: if task.completed {
                                "font-size: 14.5px; font-weight: 600; text-decoration: line-through; opacity: 0.55;"
                            } else {
                                "font-size: 14.5px; font-weight: 700;"
                            },
                            "{task.title}"
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 8px; margin-top: 5px; font-size: 11.5px; color: var(--text-secondary);",
                            span {
                                style: "background: var(--bg-card-elevated); padding: 2px 8px; border-radius: 8px;",
                                "{task.category}"
                            }
                            span {
                                style: "color: {task.priority.color()}; font-weight: 700;",
                                "● {task.priority.label_ar()}"
                            }
                        }
                    }
                    button {
                        class: "tap-spring",
                        style: if task.pinned {
                            "background: var(--accent-soft); border: none; border-radius: 10px; padding: 6px 8px; font-size: 14px;"
                        } else {
                            "background: transparent; border: none; opacity: 0.45; font-size: 14px; padding: 6px 8px;"
                        },
                        onclick: move |_| state.toggle_pin_task(task.id),
                        "📌"
                    }
                    button {
                        class: "tap-spring",
                        style: "background: transparent; border: none; font-size: 15px; opacity: 0.65; padding: 6px;",
                        onclick: move |_| state.delete_task(task.id),
                        "🗑️"
                    }
                }
            }
        }
    }
}
