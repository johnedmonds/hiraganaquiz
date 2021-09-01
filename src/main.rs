use hiragana::Hiragana;
use quiz::Quiz;
use rand::{prelude::ThreadRng, seq::IteratorRandom, thread_rng};
use web_sys::Url;
use yew::{html, Component, Properties};

use crate::romanji::Romanji;
mod hiragana;
mod quiz;
mod romanji;

struct Model {
    link: yew::ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
struct Props {
    hiragana: Vec<&'static Hiragana<'static>>,
    rng: ThreadRng,
}

impl Component for Model {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
                <Quiz hiragana=self.props.hiragana.clone() rng=self.props.rng.clone()/>
                <Romanji/>
            </>
        }
    }
}

fn main() {
    let size = Url::new(&web_sys::window().unwrap().location().href().unwrap())
        .unwrap()
        .search_params()
        .get("count")
        .map(|count| {
            count
                .parse::<usize>()
                .expect("Count param should be a positive number")
        })
        .unwrap_or(5);
    let single_char_hiragana = hiragana::data::SYMBOLS
        .iter()
        .filter(|h| h.jpn.chars().count() < 2);
    let mut rng = thread_rng();
    yew::start_app_with_props::<Model>(Props {
        hiragana: single_char_hiragana.choose_multiple(&mut rng, size),
        rng,
    });
}
