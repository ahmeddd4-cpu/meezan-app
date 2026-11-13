use dioxus::prelude::*;
use crate::models::ActiveTab;
use crate::state::AppState;

#[component]
pub fn NavBar() -> Element {
    let mut state = use_context::<AppState>();
    let current = *state.active_tab.read();

    let items = [
        (ActiveTab::Dashboard, "🏠", "اليوم"),
        (ActiveTab::Tasks, "✅", "المهام"),
        (ActiveTab::Habits, "🔥", "العادات"),
        (ActiveTab::Focus, "⏱️", "التركيز"),
        (ActiveTab::Finance, "💳", "المالية"),
    ];

    rsx! {
        nav {
            class: "bottom-nav",
            for (tab, icon, label) in items {
                button {
                    key: "{label}",
                    class: if current == tab { "nav-item active tap-spring" } else { "nav-item tap-spring" },
                    onclick: move |_| state.active_tab.set(tab),
                    span { class: "nav-icon", style: "font-size: 19px;", "{icon}" }
                    span { "{label}" }
                }
            }
        }
    }
}
