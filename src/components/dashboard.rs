use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn DashboardView() -> Element {
    let mut state = use_context::<AppState>();
    let progress = state.daily_progress_percent();
    let data = state.data.read().clone();

    let radius = 44.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference - (progress as f64 / 100.0) * circumference;

    let pinned_tasks: Vec<_> = data.tasks.iter().filter(|t| t.pinned).cloned().collect();
    let moods = [
        ("😫", "مرهق"),
        ("😕", "متشتت"),
        ("🙂", "مستقر"),
        ("😄", "نشيط"),
        ("🚀", "في القمة"),
    ];

    rsx! {
        div {
            class: "page-transition",

            // بطاقة الإنجاز اليومي الدائري المتحرك
            div {
                class: "card stagger-item",
                style: "--i: 0; display: flex; align-items: center; justify-content: space-between; gap: 16px;",
                div {
                    span {
                        style: "font-size: 12px; color: var(--accent); font-weight: 700; background: var(--accent-soft); padding: 4px 10px; border-radius: 99px;",
                        "مؤشر التوازن اليومي"
                    }
                    h2 {
                        style: "margin: 10px 0 6px; font-size: 20px; font-weight: 800;",
                        if progress >= 80 { "أداء استثنائي اليوم! 🌟" }
                        else if progress >= 40 { "تقدم ممتاز، استمر 💪" }
                        else { "ابدأ بأصغر خطوة الآن ✨" }
                    }
                    p {
                        style: "margin: 0; font-size: 13px; color: var(--text-secondary);",
                        "أنجزت {progress}% من مهامك وعاداتك اليومية"
                    }
                }

                div {
                    style: "position: relative; width: 104px; height: 104px; flex-shrink: 0;",
                    svg {
                        width: "104",
                        height: "104",
                        view_box: "0 0 104 104",
                        circle {
                            cx: "52",
                            cy: "52",
                            r: "{radius}",
                            fill: "none",
                            stroke: "var(--border-subtle)",
                            stroke_width: "9",
                        }
                        circle {
                            class: "ring-progress",
                            cx: "52",
                            cy: "52",
                            r: "{radius}",
                            fill: "none",
                            stroke: "var(--accent)",
                            stroke_width: "9",
                            stroke_linecap: "round",
                            stroke_dasharray: "{circumference}",
                            stroke_dashoffset: "{offset}",
                        }
                    }
                    div {
                        style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-weight: 800; font-size: 19px;",
                        "{progress}%"
                    }
                }
            }

            // أهم 3 أولويات مثبتة
            div {
                class: "card stagger-item",
                style: "--i: 1;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                    h3 { style: "margin: 0; font-size: 15px; font-weight: 700;", "📌 أهم أولويات اليوم" }
                    span { style: "font-size: 12px; color: var(--text-secondary);", "{pinned_tasks.len()}/3 مثبتة" }
                }
                if pinned_tasks.is_empty() {
                    p {
                        style: "margin: 0; font-size: 13px; color: var(--text-secondary); text-align: center; padding: 10px 0;",
                        "ثبّت أهم مهامك من تبويب المهام لتظهر هنا."
                    }
                } else {
                    for task in pinned_tasks {
                        div {
                            key: "{task.id}",
                            class: "tap-spring",
                            style: "display: flex; align-items: center; gap: 10px; padding: 10px 12px; border-radius: 14px; background: var(--bg-card-elevated); margin-bottom: 8px;",
                            onclick: move |_| state.toggle_task(task.id),
                            span {
                                class: if task.completed { "check-pop" } else { "" },
                                style: "font-size: 18px;",
                                if task.completed { "✅" } else { "⬜" }
                            }
                            span {
                                style: if task.completed {
                                    "flex: 1; font-size: 14px; text-decoration: line-through; opacity: 0.55; transition: opacity 0.25s ease;"
                                } else {
                                    "flex: 1; font-size: 14px; font-weight: 600;"
                                },
                                "{task.title}"
                            }
                            span {
                                style: "width: 8px; height: 8px; border-radius: 50%; background: {task.priority.color()};",
                            }
                        }
                    }
                }
            }

            // متتبع المزاج والطاقة
            div {
                class: "card stagger-item",
                style: "--i: 2;",
                h3 { style: "margin: 0 0 12px; font-size: 15px; font-weight: 700;", "🧠 حالتك الذهنية ومستوى الطاقة" }
                div {
                    style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 8px;",
                    for (idx, (emoji, label)) in moods.iter().enumerate() {
                        button {
                            key: "{label}",
                            class: "tap-spring",
                            style: if data.mood_index == idx {
                                "border: 1.5px solid var(--accent); background: var(--accent-soft); border-radius: 14px; padding: 8px 4px; color: var(--text-primary); display: flex; flex-direction: column; align-items: center; gap: 4px; transform: translateY(-2px);"
                            } else {
                                "border: 1px solid var(--border-subtle); background: var(--bg-card-elevated); border-radius: 14px; padding: 8px 4px; color: var(--text-secondary); display: flex; flex-direction: column; align-items: center; gap: 4px;"
                            },
                            onclick: move |_| state.set_mood(idx),
                            span { style: "font-size: 20px;", "{emoji}" }
                            span { style: "font-size: 11px; font-weight: 600;", "{label}" }
                        }
                    }
                }
            }

            // متتبع شرب الماء التفاعلي (8 أكواب)
            div {
                class: "card stagger-item",
                style: "--i: 3;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                    h3 { style: "margin: 0; font-size: 15px; font-weight: 700;", "💧 ترطيب الجسم (8 أكواب)" }
                    span {
                        style: "font-size: 13px; font-weight: 700; color: var(--water);",
                        "{data.water_cups} / 8"
                    }
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(8, 1fr); gap: 6px;",
                    for cup in 1..=8u8 {
                        button {
                            key: "{cup}",
                            class: "water-cup tap-spring",
                            onclick: move |_| state.set_water_cups(cup),
                            div {
                                class: "water-fill",
                                style: if cup <= data.water_cups { "height: 100%;" } else { "height: 0%;" }
                            }
                            span { class: "water-icon", "💧" }
                        }
                    }
                }
            }

            // مفكرة الأفكار السريعة
            div {
                class: "card stagger-item",
                style: "--i: 4;",
                h3 { style: "margin: 0 0 10px; font-size: 15px; font-weight: 700;", "📝 مفكرة سريعة" }
                textarea {
                    class: "input-field",
                    rows: "3",
                    placeholder: "دوّن فكرة طارئة أو تذكيراً سريعاً...",
                    value: "{data.quick_note}",
                    oninput: move |e| state.set_note(e.value()),
                }
            }
        }
    }
}
