use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TurkishLetterProps {
    pub input_ref: NodeRef,
}

const TURKISH_CHARS: &[char] = &[
    'ı', 'î', 'İ', 'ç', 'Ç', 'ö', 'Ö', 'ü', 'Ü', 'ğ', 'ş', 'Ş', 'â',
];

#[function_component(TurkishLetters)]
pub fn turkish_chars(TurkishLetterProps { input_ref }: &TurkishLetterProps) -> Html {
    let append_char = {
        let input_ref = input_ref.clone();
        move |c: char| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.set_value(&format!("{}{}", input.value(), c));
            input.focus().unwrap();
        }
    };
    html! {
        <div>
            {
                TURKISH_CHARS
                    .iter()
                    .map(|c: &char| {
                        // We are cloning these functions here so that we can pass them to
                        // the callbacks
                        let append_char = append_char.clone();
                        html! {
                            <button
                                style="
                                    background-color: #60759f;
                                    color: #394760;
                                    border-radius: 10px;
                                    font-size: 20px;
                                    border: none;
                                    width: 50px;
                                    height: 50px;
                                    padding: 10px 20px;
                                    margin-right: 5px; 
                                    margin-top: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    "
                                onclick={move |_| { append_char(*c)}}
                            >{ *c }</button>
                        }
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
