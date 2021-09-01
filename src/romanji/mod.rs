use itertools::Itertools;
use yew::{html, Component, InputData};

use crate::romanji::convert::to_hiragana;

pub mod convert;

pub struct Romanji {
    romanji: String,
    link: yew::ComponentLink<Self>,
}

impl Component for Romanji {
    type Message = String;

    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            romanji: "".to_string(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        self.romanji = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let input_handler = self.link.callback(|e: InputData| e.value);
        let hiragana = to_hiragana(&self.romanji);
        let hiragana = hiragana
            .map(|(_remaining, hiragana)| {
                hiragana.into_iter().map(|hiragana| hiragana.jpn).join("")
            })
            .unwrap_or("".to_string());
        html! {
            <div>
                <input type="text" value=self.romanji.clone() oninput=input_handler/>
                <span>{hiragana}</span>
            </div>
        }
    }
}
