use dioxus::prelude::*;
use crate::models::TimerMode;
use crate::state::AppState;

#[component]
pub fn FocusView() -> Element {
    let mut state = use_context::<AppState>();
    let mut mode = use_signal(|| TimerMode::Focus25);
    let mut seconds_left = use_signal(|| TimerMode::Focus25.total_seconds());
    let mut is_running = use_signal(|| false);

    // مؤقت غير متزامن بلغة Rust يعمل بسلاسة كل ثانية
    use_future(move || async move {
        loop {
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::TimeoutFuture::new(1000).await;

            if *is_running.read() {
                let current = *seconds_left.read();
                if current > 0 {
                    seconds_left.set(current - 1);
                } else {
                    is_running.set(false);
                    if *mode.read() == TimerMode::Focus25 {
                        state.increment_focus_session();
                    }
                    seconds_left.set(mode.read().total_seconds());
                }
            }
        }
    });

    let total = mode.read().total_seconds() as f64;
    let remaining = *seconds_left.read();
    let radius = 92.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let progress = if total > 0.0 {
        (total - remaining as f64) / total
    } else {
        0.0
    };
    let offset = circumference * (1.0 - progress);

    let mins = remaining / 60;
    let secs = remaining % 60;
    let sessions = state.data.read().focus_sessions_today;

    rsx! {
        div {
            class: "page-transition",

            div {
                class: "card stagger-item",
                style: "--i: 0; text-align: center; padding: 22px 16px;",

                // اختيار وضع المؤقت
                div {
                    style: "display: flex; gap: 6px; margin-bottom: 22px;",
                    for m in [TimerMode::Focus25, TimerMode::ShortBreak5, TimerMode::LongBreak15] {
                        button {
                            key: "{m.label_ar()}",
                            class: "tap-spring",
                            style: if *mode.read() == m {
                                "flex: 1; padding: 8px 6px; border-radius: 12px; border: none; background: var(--accent-soft); color: var(--accent); font-weight: 700; font-size: 12px;"
                            } else {
                                "flex: 1; padding: 8px 6px; border-radius: 12px; border: 1px solid var(--border-subtle); background: transparent; color: var(--text-secondary); font-size: 12px;"
                            },
                            onclick: move |_| {
                                is_running.set(false);
                                mode.set(m);
                                seconds_left.set(m.total_seconds());
                            },
                            "{m.label_ar()}"
                        }
                    }
                }

                // الحلقة الدائرية المتحركة للمؤقت
                div {
                    class: if *is_running.read() { "timer-active-glow" } else { "" },
                    style: "position: relative; width: 220px; height: 220px; margin: 0 auto 22px;",
                    svg {
                        width: "220",
                        height: "220",
                        view_box: "0 0 220 220",
                        circle {
                            cx: "110",
                            cy: "110",
                            r: "{radius}",
                            fill: "none",
                            stroke: "var(--border-subtle)",
                            stroke_width: "12",
                        }
                        circle {
                            class: "ring-progress",
                            cx: "110",
                            cy: "110",
                            r: "{radius}",
                            fill: "none",
                            stroke: "var(--accent)",
                            stroke_width: "12",
                            stroke_linecap: "round",
                            stroke_dasharray: "{circumference}",
                            stroke_dashoffset: "{offset}",
                        }
                    }
                    div {
                        style: "position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center;",
                        span {
                            style: "font-size: 42px; font-weight: 800; font-variant-numeric: tabular-nums;",
                            "{mins:02}:{secs:02}"
                        }
                        span {
                            style: "font-size: 12px; color: var(--text-secondary); margin-top: 4px;",
                            if *is_running.read() { "جارٍ التركيز العميق..." } else { "مستعد للانطلاق" }
                        }
                    }
                }

                // أزرار التحكم
                div {
                    style: "display: flex; justify-content: center; gap: 12px;",
                    button {
                        class: "btn-primary tap-spring",
                        style: "min-width: 135px; font-size: 15px;",
                        onclick: move |_| {
                            let next = !*is_running.read();
                            is_running.set(next);
                        },
                        if *is_running.read() { "⏸️ إيقاف مؤقت" } else { "▶️ ابدأ الجلسة" }
                    }
                    button {
                        class: "tap-spring",
                        style: "padding: 11px 16px; border-radius: 14px; border: 1px solid var(--border-subtle); background: var(--bg-card-elevated); color: var(--text-primary); font-weight: 700;",
                        onclick: move |_| {
                            is_running.set(false);
                            seconds_left.set(mode.read().total_seconds());
                        },
                        "🔄 إعادة"
                    }
                }
            }

            div {
                class: "card stagger-item",
                style: "--i: 1; display: flex; justify-content: space-between; align-items: center;",
                span { style: "font-size: 14px; font-weight: 700;", "🍅 جلسات التركيز المنجزة اليوم" }
                span {
                    style: "font-size: 18px; font-weight: 800; color: var(--accent);",
                    "{sessions} جلسات"
                }
            }
        }
    }
}
