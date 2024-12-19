#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::Topic;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::{
        button::{Button, RoundedNoButton, RoundedYesButton},
        icon_text::IconText,
        icons,
    },
    theme::Theme,
};

#[component]
pub fn HighlightedTopics(
    #[props(default ="highlighted_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    topics: Vec<Topic>,
    onselect: EventHandler<usize>,
) -> Element {
    let mut selected = Signal::new(0);
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div {
            id,
            class,
            div {
                class: "flex flex-col w-full max-w-[1440px]",
                for (i, topic) in topics.iter().enumerate() {
                    if i == selected() {
                        HighlightedTopic {
                            id: topic.id.clone(),
                            // image: topic.image.clone(),
                            // title: topic.title.clone(),
                            // description: topic.description.clone(),
                            // period: topic.period.clone(),
                            // donations: topic.donations,
                            // replies: topic.replies,
                            // yes: topic.yes,
                            // no: topic.no,
                        }
                    }
                }

                div {
                    class: "flex flex-row w-full items-center justify-center gap-[10px] p-[10px]",
                    for i in 0..topics.len() {
                        div {
                            class: format!(
                                "h-[8px] transition-all rounded-full cursor-pointer {} bg-[{}] hover:bg-white",
                                if i == selected() {
                                    "w-[90px]"
                                } else {
                                    "w-[52px] hover:w-[70px]"
                                },
                                theme_data.primary06
                            ),
                            onclick: move |_| {
                                tracing::debug!("selected: {}", i);
                                selected.set(i);
                                onselect(i);
                            },
                        }
                    }
                }
            }

        }
    }
}

#[component]
pub fn HighlightedTopic(
    #[props(default = "highlighed-topic".to_string())] id: String,
    #[props(default = "https://dev.democrasee.me/images/sample.png".to_string())] image: String,
    #[props(default = "윤대통령 2차 탄핵안 절차 게시될까?".to_string())] title: String,
    #[props(default = "민주당과 조국혁신당, 개혁신당 등 야 6당이 함께 윤석열 대통령에 대한 두 번째 탄핵소추안을 국회에 제출했습니다.  지난 7일, 국민의힘 의원 대부분이 표결에 불참해 1차 탄핵소추안이 의결정족수 미달로...".to_string())]
    description: String,
    #[props(default = "12/15 - 1/22".to_string())] period: String,
    #[props(default = 25991291)] donations: u64,
    #[props(default = 200)] replies: u64,
    #[props(default = 64)] yes: u64,
    #[props(default = 36)] no: u64,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    #[allow(unused_mut)]
    let mut visible = true;

    #[cfg(feature = "web-only")]
    {
        let window_size = dioxus_sdk::utils::window::use_window_size();
        visible = window_size().width > 560;
    }

    rsx! {
        div {
            id,

            class: "w-full grid grid-cols-12 grid-rows-11 gap-x-[20px] gap-y-[40px] h-[496px]",
            img {
                src: image,
                class: "row-start-2 row-span-8 col-start-1 col-span-5 w-full h-full rounded-[8px] z-[10] object-cover",
            }
            div {
                class: "col-start-6 row-start-3 col-span-6 row-span-7 flex flex-col justify-start items-start z-[10] gap-[33px]",
                ContentWrapper { title, description, period, donations, replies }
                div {
                    class: "flex flex-row w-full justify-start items-center gap-[17px]",
                    VoteResultBars {
                        class: "grow",
                        yes,
                        no,
                    }
                    if visible {
                        Button {
                            background: "rgba(130, 143, 165, 0.05)",
                            onclick: |_| {},
                            div {
                                class: "flex flex-row items-center justify-center gap-[10px]",
                                span {
                                    class: "text-[14px] font-bold",
                                    "더보기"
                                }
                                icons::RightArrow {}
                            }
                        }
                    }
                }
            }

            div {
                class: "col-start-1 col-span-12 row-start-1 row-span-11 ml-[71px] z-[9] flex flex-row gap-[16px] items-end justify-center backdrop-blur-[10px] rounded-[8px] py-[32px] px-[10px]",
                style: "background: {theme_data.primary05};",
                RoundedYesButton { onclick: |_| {} }
                RoundedNoButton { onclick: |_| {} }
            }

            div {
                class: "flex flex-row items-center justify-center col-start-4 col-span-5 row-start-10 row-span-2 px-[18px] py-[13px] gap-[16px] z-[10]",
            }
        }
    }
}

#[component]
pub fn ContentWrapper(
    title: String,
    description: String,
    period: String,
    donations: u64,
    replies: u64,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div {
            class: "flex flex-col gap-[22px] items-start justify-start h-[209px]",
            h1 {
                class: "text-[42px] font-extrabold tracking-normal line-clamp-1",
                "{title}"
            }
            p {
                class: "text-[16px] max-w-[674px] font-regular leading-[24px] tracking-[0.5px] line-clamp-2",
                style: "color: {theme_data.primary00};",
                "{description}"
            }
            div {
                class: "flex flex-row gap-[8px] justify-start items-center",
                IconText {
                    text: "{period}",
                    background: "{theme_data.primary06}",
                    icons::Clock{ }
                }
                IconText {
                    text: format!("누적기부금 {}₩", donations.to_formatted_string(&Locale::en)),
                    icons::Money{ }
                }
                IconText {
                    class: "",
                    text: "{replies}",
                    icons::ChatBubble { }
                }
            }
        }
    }
}

#[component]
pub fn VoteResultBars(
    yes: u64,
    no: u64,
    #[props(default = "w-[580px]".to_string())] class: String,
) -> Element {
    let sum = yes + no;
    let yes = (yes as f64 / sum as f64) * 100.0;
    let no = (no as f64 / sum as f64) * 100.0;

    rsx! {
        div {
            class: "flex flex-row justify-around {class}",
            div {
                class: "w-[calc(50%-6px)]",
                div {
                    class: "relative animate-grow flex flex-row justify-end items-center px-[20px] text-[15px] font-bold w-[calc(50%-6px)] h-[28px] rounded-[6px]",
                    style: "background: linear-gradient(90deg, #212231 0%, rgba(104, 211, 108, 0.5) 100%);",
                    div {
                        class: "absolute z-[20] h-[22px] w-[22px] right-[2.46px] top-[3px] rounded-[6px] bg-[#68D36C] opacity-50",
                    }
                    span { class: "z-[30]", "{yes}%" }
                }
            }

            div {
                class: "relative w-[calc(50%-6px)]",
                div {
                    class: "absolute right-[0px] relative animate-grow-to-left flex flex-row justify-start items-center px-[20px] text-[15px] font-bold w-[calc(50%-6px)] h-[28px] rounded-[6px]",
                    style: "background: linear-gradient(90deg, rgba(255, 90, 93, 0.5) 0%, #212231 100%);",
                    div {
                        class: "absolute z-[20] h-[22px] w-[22px] left-[2.46px] top-[3px] rounded-[6px] bg-[#FF5A5D] opacity-50",
                    }
                    span { class: "z-[30]", "{no}%" }
                }

            }
        }
    }
}
