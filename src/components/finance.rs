use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn FinanceView() -> Element {
    let mut state = use_context::<AppState>();
    let mut title = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut category = use_signal(|| "طعام".to_string());
    let mut is_income = use_signal(|| false);

    let data = state.data.read().clone();
    let total_expenses: f64 = data
        .transactions
        .iter()
        .filter(|t| !t.is_income)
        .map(|t| t.amount)
        .sum();
    let total_income: f64 = data
        .transactions
        .iter()
        .filter(|t| t.is_income)
        .map(|t| t.amount)
        .sum();

    let budget_ratio = if data.monthly_budget > 0.0 {
        ((total_expenses / data.monthly_budget) * 100.0).min(100.0)
    } else {
        0.0
    };

    let bar_color = if budget_ratio > 90.0 {
        "var(--danger)"
    } else if budget_ratio > 70.0 {
        "var(--warning)"
    } else {
        "var(--accent)"
    };

    rsx! {
        div {
            class: "page-transition",

            // ملخص الميزانية مع شريط تقدم متحرك
            div {
                class: "card stagger-item",
                style: "--i: 0;",
                div {
                    style: "display: flex; justify-content: space-between; margin-bottom: 12px;",
                    div {
                        span { style: "font-size: 12px; color: var(--text-secondary);", "المصروفات مقابل الميزانية" }
                        h3 {
                            style: "margin: 4px 0 0; font-size: 20px; font-weight: 800;",
                            "{total_expenses:.0} / {data.monthly_budget:.0}"
                        }
                    }
                    div {
                        style: "text-align: left;",
                        span { style: "font-size: 12px; color: var(--text-secondary);", "إجمالي الدخل" }
                        div {
                            style: "margin-top: 4px; font-size: 16px; font-weight: 800; color: var(--accent);",
                            "+{total_income:.0}"
                        }
                    }
                }
                div {
                    style: "height: 10px; border-radius: 99px; background: var(--bg-card-elevated); overflow: hidden;",
                    div {
                        class: "bar-smooth",
                        style: "height: 100%; width: {budget_ratio}%; background: {bar_color}; border-radius: 99px;",
                    }
                }
            }

            // إضافة معاملة مالية
            div {
                class: "card stagger-item",
                style: "--i: 1;",
                h3 { style: "margin: 0 0 12px; font-size: 15px; font-weight: 800;", "💸 تسجيل حركة مالية" }
                div {
                    style: "display: flex; gap: 8px; margin-bottom: 10px;",
                    button {
                        class: "tap-spring",
                        style: if !*is_income.read() {
                            "flex: 1; padding: 8px; border-radius: 12px; border: none; background: var(--danger); color: #fff; font-weight: 700;"
                        } else {
                            "flex: 1; padding: 8px; border-radius: 12px; border: 1px solid var(--border-subtle); background: transparent; color: var(--text-secondary);"
                        },
                        onclick: move |_| is_income.set(false),
                        "مصروف (-)"
                    }
                    button {
                        class: "tap-spring",
                        style: if *is_income.read() {
                            "flex: 1; padding: 8px; border-radius: 12px; border: none; background: var(--accent); color: #fff; font-weight: 700;"
                        } else {
                            "flex: 1; padding: 8px; border-radius: 12px; border: 1px solid var(--border-subtle); background: transparent; color: var(--text-secondary);"
                        },
                        onclick: move |_| is_income.set(true),
                        "دخل (+)"
                    }
                }
                input {
                    class: "input-field",
                    style: "margin-bottom: 8px;",
                    placeholder: "الوصف (مثلاً: قهوة، فاتورة، مكافأة)",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                }
                div {
                    style: "display: flex; gap: 8px; margin-bottom: 10px;",
                    input {
                        class: "input-field",
                        r#type: "number",
                        placeholder: "المبلغ",
                        value: "{amount}",
                        oninput: move |e| amount.set(e.value()),
                    }
                    input {
                        class: "input-field",
                        placeholder: "الفئة",
                        value: "{category}",
                        oninput: move |e| category.set(e.value()),
                    }
                }
                button {
                    class: "btn-primary tap-spring",
                    style: "width: 100%;",
                    onclick: move |_| {
                        if let Ok(val) = amount.read().parse::<f64>() {
                            state.add_transaction(
                                title.read().clone(),
                                category.read().clone(),
                                val,
                                *is_income.read(),
                            );
                            title.set(String::new());
                            amount.set(String::new());
                        }
                    },
                    "حفظ الحركة"
                }
            }

            // سجل الحركات المالية
            for (idx, tx) in data.transactions.into_iter().enumerate() {
                div {
                    key: "{tx.id}",
                    class: "card stagger-item",
                    style: "--i: {idx}; display: flex; justify-content: space-between; align-items: center; padding: 13px 16px;",
                    div {
                        div { style: "font-weight: 700; font-size: 14px;", "{tx.title}" }
                        span {
                            style: "font-size: 11.5px; color: var(--text-secondary);",
                            "{tx.category}"
                        }
                    }
                    span {
                        style: if tx.is_income {
                            "font-weight: 800; font-size: 15px; color: var(--accent);"
                        } else {
                            "font-weight: 800; font-size: 15px; color: var(--danger);"
                        },
                        if tx.is_income { "+{tx.amount:.0}" } else { "-{tx.amount:.0}" }
                    }
                }
            }
        }
    }
}
