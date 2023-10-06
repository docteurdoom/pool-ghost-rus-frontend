use dioxus::prelude::*;
use crate::parse;

pub fn home(cx: Scope) -> Element {
    let data = parse::deserialized_data().unwrap();
    cx.render(rsx!(
        head {
            title { "ПУЛ.ГОСТ.РУС" }
            link {
                rel: "icon",
                href: "/assets/favicon.ico",
            }
            link {
                rel: "stylesheet",
                href: "/assets/style.css",
            }
            meta {
                name: "viewport",
                content: "width=device-width",
            }
            meta {
                name: "referrer",
                content: "no-referrer",
            }
        }
        body {
            div {
                class: "fix",
                div {
                    style: "text-align: center !important; padding: 0 auto; margin-top: 13px; min-width: 100% !important",
                    img {
                        width: "50",
                        src: "/assets/ghost-logo-white.png",
                    }
                    h1 {
                        style: "color: white",
                        "Первый СНГ Пул Холодного Стейкинга"
                    } br {}
                    img {
                        width: "500",
                        src: "assets/staking-pool-black.jpg",
                    } br {} br {} br {} br {}
                }
                div {
                    style: "color: white; float: left;",
                    div { "Адрес пула: " strong { "gcs169zkwmr9zt8mz2epql8wnly3dyf4hkavcprrm2" } }
                    div { "Версия ядра: " strong { data.core_version } }
                    div { "Стейк бонус: " strong { "13%" } }
                    div { "Отчисление пулу: " strong { data.fee } } br {}
                    div { "Всего в активном стейкинге: " strong { data.currently_staking } }
                    div { "Всего в пуле: " strong { data.total_pooled_coin } } br {}
                    div { "Порог выплаты: " strong { "0.1 GHOST" } }
                    div { "Блоки между запусками платежей: " strong { data.blocks_between_payment_runs } }
                    div { "Высота синхронизации: " strong { data.real_height } }
                    div { "Найдено блоков: " strong { data.blocks_found } }
                    div { "Всего выплачено: " strong { data.total_disbursed } }
                    div { "Последний платёжный цикл: " strong { data.last_payment_run } } br {}
                    div { "Доход пула: " strong { data.total_pool_rewards } }
                    div { "Сумма оплаченных комиссий: " strong { data.total_pool_fees } } br {} br {}
                    div {
                        p {
                            style: "text-align: center;",
                            strong { "Последние Найденные Блоки" } br {} br {}
                            table {
                                thead {
                                    tr {
                                        th { "Высота" }
                                        th { "Хеш" }
                                        th { "Награда" }
                                    }
                                }
                                tbody {
                                    for i in 0..5 {
                                        tr {
                                            td { &data.recent_blocks.height[i][..] }
                                            td {
                                                a {
                                                    href: "https://ghostscan.io/block/{&data.recent_blocks.block_hash[i][..]}/",
                                                    style: "color: white;",
                                                    target: "blank",
                                                    &data.recent_blocks.block_hash[i][..]
                                                }
                                            }
                                            td { &data.recent_blocks.block_reward[i][..] }
                                        }
                                    }
                                }
                            }
                        }
                    } br {} br {}
                    div {
                        p {
                            style: "text-align: center;",
                            strong { "Последние Выплаты" } br {} br {}
                            table {
                                thead {
                                    tr {
                                        th { "Высота" }
                                        th { "Транзакция" }
                                        th { "Выплата" }
                                    }
                                }
                                tbody {
                                    for i in 0..5 {
                                        tr {
                                            td { &data.last_payments.height[i][..] }
                                            td {
                                                a {
                                                    href: "https://ghostscan.io/tx/{&data.last_payments.txid[i][..]}/",
                                                    style: "color: white;",
                                                    target: "blank",
                                                    &data.last_payments.txid[i][..]
                                                }
                                            }
                                            td { &data.last_payments.disbursed[i][..] }
                                        }
                                    }
                                }
                            }
                        }
                    } br {} br {}
                }
            }
        }
    ))
}

