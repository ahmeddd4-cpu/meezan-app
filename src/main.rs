mod components;
mod models;
mod state;
mod storage;

use dioxus::prelude::*;
use components::{
    dashboard::DashboardView,
    finance::FinanceView,
    focus::FocusView,
    habits::HabitsView,
    header::Header,
    navbar::NavBar,
    tasks::TasksView,
};
use models::ActiveTab;
use state::AppState;

const ANIMATIONS_CSS: &str = include_str!("../assets/animations.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let state = use_context_provider(AppState::init);
    let dark_mode = state.data.read().dark_mode;
    let active_tab = *state.active_tab.read();

    rsx! {
        style { "{ANIMATIONS_CSS}" }
        div {
            class: "app-shell",
            "data-theme": if dark_mode { "dark" } else { "light" },
            Header {}
            main {
                match active_tab {
                    ActiveTab::Dashboard => rsx! { DashboardView {} },
                    ActiveTab::Tasks => rsx! { TasksView {} },
                    ActiveTab::Habits => rsx! { HabitsView {} },
                    ActiveTab::Focus => rsx! { FocusView {} },
                    ActiveTab::Finance => rsx! { FinanceView {} },
                }
            }
            NavBar {}
        }
    }
}
