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
                        "Первый и Единственный" br {}
                        "AutoZAP Пул Холодного Стейкинга"
                    }
                    a {
                        href: "/en",
                        img {
                            style: "width: 30px;",
                            src: "assets/en_button.png",
                        }
                    } br {}
                    img {
                        width: "500",
                        src: "assets/staking-pool-black.jpg",
                    } br {} br {} br {} br {}
                }
                div {
                    style: "color: white; text-align: center",
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://гост.рус/",
                        strong { "GHOST" }
                    }
                    " – это анонимная криптовалюта, основанная на технологии " br {}
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://гост.рус/2021/02/15/что-такое-proof-of-stake-pos/",
                        b { "Proof Of Stake" }
                    }
                    ". "
                    "Благодаря данной технологии, отсутствует необходимость в покупке"
                    " оборудования для майнинга. Новые монеты начисляются благодаря уже имеющимся на Вашем счету."
                    br {} br {}
                    b { "ПУЛ.ГОСТ.РУС" }
                    " – это первый и единственный пул, в котором реализована возможность"
                    " автоматического добавления новых монет в процесс стейкинга."
                    br {} br {}
                    "После первого нажатия кнопки ZAP (реинвестирование полученных монет), "
                    "все средства, которые сгенерируются в будущем, автоматически будут размещены в пуле."
                    " Это делает пассивный доход в GHOST полностью автоматизированным и безопасным."
                    br {} br {}
                    b { "Пулы холодного стейкинга" }
                    " созданы для объединения усилий и увеличения общей доходности держателей."
                    " При размещении монет в пуле, физически они остаются на Вашем кошельке,"
                    " и вы можете продолжать использовать их без ограничений. "
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://гост.рус/",
                        b { "Подробнее" }
                    }
                    "."
                    br {} br {} hr {} br {}
                }
                div {
                    style: "color: white; float: left;",
                    div { "Адрес пула: " strong { "gcs169zkwmr9zt8mz2epql8wnly3dyf4hkavcprrm2" } }
                    div { "Версия ядра: " strong { data.core_version } }
                    div { "AutoZAP: " strong { "Активирован" } }
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
                    div { "Сумма оплаченных комиссий: " strong { data.total_pool_fees } } br {} hr {}
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
                    }
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

pub fn en(cx: Scope) -> Element {
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
                        "First and The Only" br {}
                        "AutoZAP Cold Staking Pool"
                    }
                    a {
                        href: "/",
                        img {
                            style: "width: 30px;",
                            src: "assets/ru_button.png",
                        }
                    } br {}
                    img {
                        width: "500",
                        src: "assets/staking-pool-black.jpg",
                    } br {} br {} br {} br {}
                }
                div {
                    style: "color: white; text-align: center",
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://ipfs.ghostbyjohnmcafee.com/#/",
                        strong { "GHOST" }
                    }
                    " – is an anonymous cryptocurrency based on "
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://en.wikipedia.org/wiki/Proof_of_stake",
                        b { "Proof Of Stake" }
                    }
                    " consensus algorithm. "
                    "Thanks to this technology, there is no need in mining equipment. "
                    "New coins are minted by staking the coins you already have in your wallet. "
                    br {} br {}
                    b { "POOL.GHOST.RUS" }
                    " is the first and the only pool in which "
                    "the rewards produced in cold staking are reinvested automatically."
                    br {} br {}
                    "After once clicking the ZAP button (reinvesting the received coins), "
                    "all coins that are generated in the future will automatically join the cold staking. "
                    "This makes passive income in GHOST fully automated and secure."
                    br {} br {}
                    b { "Cold Staking Pools" }
                    " are created to increase the overall profitability of holders. "
                    "Staked coins never leave your wallet and you can"
                    " spend them any time without restrictions. "
                    a {
                        style: "color: white;",
                        target: "blank",
                        href: "https://ghostveterans.net/staking/",
                        b { "More info" }
                    }
                    "."
                    br {} br {} hr {} br {}
                }
                div {
                    style: "color: white; float: left;",
                    div { "Pool Address: " strong { "gcs169zkwmr9zt8mz2epql8wnly3dyf4hkavcprrm2" } }
                    div { "Core Version: " strong { data.core_version } }
                    div { "AutoZAP: " strong { "Activated" } }
                    div { "Stake Bonus: " strong { "13%" } }
                    div { "Pool Fee: " strong { data.fee } } br {}
                    div { "Total in active staking: " strong { data.currently_staking } }
                    div { "Total staked in pool: " strong { data.total_pooled_coin } } br {}
                    div { "Minimum payout: " strong { "0.1 GHOST" } }
                    div { "Blocks between payouts: " strong { data.blocks_between_payment_runs } }
                    div { "Synced height: " strong { data.real_height } }
                    div { "Found blocks: " strong { data.blocks_found } }
                    div { "Total disbursed: " strong { data.total_disbursed } }
                    div { "Last payout run: " strong { data.last_payment_run } } br {}
                    div { "Pool revenue: " strong { data.total_pool_rewards } }
                    div { "Total fees paid: " strong { data.total_pool_fees } } br {} hr {}
                    div {
                        p {
                            style: "text-align: center;",
                            strong { "Latest Blocks Found" } br {} br {}
                            table {
                                thead {
                                    tr {
                                        th { "Height" }
                                        th { "Blockhash" }
                                        th { "Reward" }
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
                    }
                    div {
                        p {
                            style: "text-align: center;",
                            strong { "Latest Payouts" } br {} br {}
                            table {
                                thead {
                                    tr {
                                        th { "Height" }
                                        th { "Transaction ID" }
                                        th { "Paid" }
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

