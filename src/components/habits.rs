use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn HabitsView() -> Element {
    let mut state = use_context::<AppState>();
    let mut new_habit = use_signal(String::new);
    let mut selected_icon = use_signal(|| "🔥".to_string());

    let icons = ["🔥", "📖", "🏃", "💧", "🧘", "🦀", "🌿"];
    let habits = state.data.read().habits.clone();

    rsx! {
        div {
            class: "page-transition",

            div {
                class: "card stagger-item",
                style: "--i: 0;",
                h3 { style: "margin: 0 0 12px; font-size: 16px; font-weight: 800;", "🌱 بناء عادة يومية جديدة" }
                div {
                    style: "display: flex; gap: 6px; margin-bottom: 10px; overflow-x: auto;",
                    for ic in icons {
                        button {
                            key: "{ic}",
                            class: "tap-spring",
                            style: if *selected_icon.read() == ic {
                                "width: 38px; height: 38px; border-radius: 12px; border: 1.5px solid var(--accent); background: var(--accent-soft); font-size: 18px;"
                            } else {
                                "width: 38px; height: 38px; border-radius: 12px; border: 1px solid var(--border-subtle); background: var(--bg-card-elevated); font-size: 18px;"
                            },
                            onclick: move |_| selected_icon.set(ic.to_string()),
                            "{ic}"
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 8px;",
                    input {
                        class: "input-field",
                        placeholder: "اسم العادة (مثلاً: قراءة 10 صفحات)",
                        value: "{new_habit}",
                        oninput: move |e| new_habit.set(e.value()),
                    }
                    button {
                        class: "btn-primary tap-spring",
                        onclick: move |_| {
                            state.add_habit(new_habit.read().clone(), selected_icon.read().clone());
                            new_habit.set(String::new());
                        },
                        "إضافة"
                    }
                }
            }

            for (idx, habit) in habits.into_iter().enumerate() {
                div {
                    key: "{habit.id}",
                    class: "card stagger-item",
                    style: "--i: {idx}; display: flex; align-items: center; justify-content: space-between; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px; flex: 1;",
                        div {
                            style: "width: 46px; height: 46px; border-radius: 15px; background: var(--bg-card-elevated); display: flex; align-items: center; justify-content: center; font-size: 22px;",
                            "{habit.icon}"
                        }
                        div {
                            div {
                                style: "font-size: 15px; font-weight: 700; margin-bottom: 4px;",
                                "{habit.title}"
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span {
                                    style: "font-size: 12px; font-weight: 700; color: var(--warning);",
                                    span { class: "streak-flame", "🔥" }
                                    " {habit.streak} يوم متتالي"
                                }
                                div {
                                    style: "display: flex; gap: 3px;",
                                    for done in habit.history_7d {
                                        span {
                                            style: if done {
                                                "width: 7px; height: 7px; border-radius: 50%; background: var(--accent);"
                                            } else {
                                                "width: 7px; height: 7px; border-radius: 50%; background: var(--border-subtle);"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: if habit.completed_today { "tap-spring check-pop" } else { "tap-spring" },
                        style: if habit.completed_today {
                            "padding: 10px 16px; border-radius: 14px; border: none; background: var(--accent); color: #fff; font-weight: 800; font-size: 13px; box-shadow: 0 4px 14px var(--accent-glow);"
                        } else {
                            "padding: 10px 16px; border-radius: 14px; border: 1.5px solid var(--accent); background: transparent; color: var(--accent); font-weight: 700; font-size: 13px;"
                        },
                        onclick: move |_| state.toggle_habit(habit.id),
                        if habit.completed_today { "تم ✓" } else { "إنجاز" }
                    }
                }
            }
        }
    }
}
