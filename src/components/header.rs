use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn Header() -> Element {
    let mut state = use_context::<AppState>();
    let dark_mode = state.data.read().dark_mode;

    rsx! {
        header {
            style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 18px;",
            div {
                style: "display: flex; align-items: center; gap: 10px;",
                div {
                    style: "width: 42px; height: 42px; border-radius: 14px; background: linear-gradient(135deg, #10b981, #059669); display: flex; align-items: center; justify-content: center; font-size: 22px; box-shadow: 0 6px 18px rgba(16, 185, 129, 0.35);",
                    "⚖️"
                }
                div {
                    h1 {
                        style: "margin: 0; font-size: 20px; font-weight: 800; letter-spacing: -0.3px;",
                        "مِيزان"
                    }
                    p {
                        style: "margin: 0; font-size: 12px; color: var(--text-secondary);",
                        "توازن يومك، خطوة بخطوة"
                    }
                }
            }
            button {
                class: "tap-spring",
                style: "width: 40px; height: 40px; border-radius: 13px; border: 1px solid var(--border-subtle); background: var(--bg-card); color: var(--text-primary); font-size: 18px; display: flex; align-items: center; justify-content: center;",
                onclick: move |_| state.toggle_theme(),
                if dark_mode { "☀️" } else { "🌙" }
            }
        }
    }
}
