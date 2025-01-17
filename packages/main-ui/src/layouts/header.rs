#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::{
    components::{button::Button, logo::LogoWrapper},
    layouts::{signup_popup::SignupPopup, user_setup_popup::UserSetupPopup},
    theme::Theme,
};

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "flex flex-row items-center justify-between w-full pt-[47px] pb-[39px]",
            LogoWrapper { }
            HeaderTails { }
        }
    }
}

#[component]
pub fn HeaderTails() -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let mut popup: PopupService = use_context();

    #[cfg(feature = "web")]
    let firebase: dioxus_oauth::prelude::FirebaseService = use_context();

    rsx! {
        div {
            class: "flex flex-row gap-[30px] justify-start items-center",
            Button {
                color: "{theme.primary00}",
                onclick: move |_| {},
                "로그인"
            }
            Button {
                color: "{theme.primary00}",
                background: "{theme.primary06}",
                onclick: move |_| {
                    tracing::debug!("회원가입 버튼 클릭");
                    let mut p = popup.clone();
                    popup.open(rsx! {
                        SignupPopup {
                            class: "w-[400px]",
                            onclick:move  |_| async move {
                                tracing::debug!("Google로 계속하기 버튼 클릭");
                                #[cfg(feature = "web")]
                                {
                                    firebase.sign_in_with_popup(vec![
                                        "https://www.googleapis.com/auth/drive.appdata".to_string(),
                                    ]).await;
                                    p.open(rsx!{
                                        UserSetupPopup {}
                                    }).with_id("user_setup_popup").with_title("계정 생성하기");
                                }
                            },
                        }
                    }).with_id("signup").with_title("회원가입");
                },
                "회원가입"
            }
        }
    }
}
