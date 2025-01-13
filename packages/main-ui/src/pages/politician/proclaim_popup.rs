#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::{
    theme::Theme, 
    components::{
        checkbox::Checkbox,
        dropdown::Dropdown,
    },
};
use super::i18n::PoliticianStanceTranslate;

#[component]
pub fn ProclaimPopup(
    #[props(default = "proclaim_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    name: String,
    party: String,
    stance: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<PoliticianStanceTranslate>(&lang);
    let mut popup: PopupService = use_context();
    let mut agreed = use_signal(|| false);
    let mut stance_signal = use_signal(|| stance);

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[35px] pt-[10px]",
                div { class: "flex flex-col w-full gap-[10px]",

                    // NAME
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.name}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: "name",
                            readonly: true,
                        }
                    }

                    // PARTY
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.party}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: party,
                            readonly: true,
                        }
                    }

                    // STANCE ON CRYPTO
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.stance_on_crypto}" }
                        }
                        Dropdown {
                            // TODO: replace this data to CryptoStance
                            items: vec![
                                tr.supportive.to_string(),
                                tr.against.to_string(),
                                tr.neutral.to_string(),
                                tr.no_stance.to_string(),
                            ],
                            value: stance_signal(),
                            placeholder: "{tr.stance_placeholder}",
                            onselect: move |value| {
                                stance_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                    }

                    div { class: "flex flex-row gap-[6px] items-center",
                        Checkbox {
                            class: "cursor-pointer",
                            title: "{tr.agree_proclaim}",
                            onchange: move |check| {
                                agreed.set(check);
                            },
                        }
                    }
                }

                div { class: "flex w-full",
                    button {
                        class: "w-full h-[57px] text-white bg-[{theme.primary100}] text-[18px] font-extrabold leading-[24px] rounded-[12px]",
                        style: if agreed() {
                            "opacity: 0.5; cursor: pointer;"
                        } else {
                            "opacity: 0.2;"
                        },
                        onclick: move |_| {
                            tracing::debug!("proclaim button clicked");
                            if !agreed() {
                                return;
                            }
                            // TODO: send proclaim request
                        },
                        disabled: !agreed(),
                        "{tr.proclaim}"
                    }
                }
            }
        }
    }
}