use super::turkish_chars::TurkishLetters;
use gloo::net::http::Request;
use log::info;
use serde::Deserialize;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Album {
    name: String,
    id: i32,
}

fn get_albums(state: &UseStateHandle<Vec<Album>>) {
    let state = state.clone();
    spawn_local(async move {
        let albums = Request::get("/get/albums")
            .send()
            .await
            .unwrap()
            .json::<Vec<Album>>()
            .await
            .unwrap();
        info!("{:?}", albums);
        state.set(albums);
    });
}

fn send_quote(song: String, quote: String, album: String) {
    spawn_local(async move {
        Request::post("/add/quote")
            .query([("album", album), ("song", song), ("quote", quote)])
            .send()
            .await
            .unwrap();
    });
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let albums_state: UseStateHandle<Vec<Album>> = use_state(|| vec![]);
    get_albums(&albums_state);

    let albums_ref: NodeRef = use_node_ref();
    let song_ref: NodeRef = use_node_ref();
    let quote_ref: NodeRef = use_node_ref();

    let submit_quote = {
        let song_ref = song_ref.clone();
        let quote_ref = quote_ref.clone();
        let albums_ref = albums_ref.clone();
        move || {
            let song = song_ref.cast::<HtmlInputElement>().unwrap().value();
            let quote = quote_ref.cast::<HtmlInputElement>().unwrap().value();
            let album = albums_ref.cast::<HtmlSelectElement>().unwrap().value();
            send_quote(song, quote, album);
        }
    };

    html! {
        <div>
            <h1 style="text-align: center;">{ "Sagopa API" }</h1>
            <div id="main-container" style="display: flex; justify-content: center;">
                <div id="albums-container" style="
                    background-color: #0b0e13;
                    width: 20%;
                    padding: 30px 20px;
                    justify-content: center;
                    align-items: center;
                    border-radius: 20px;
                    margin-right: 20px;
                    "
                >
                    <div id="albums-title" style="
                        text-align: center;
                        font-size: 30px;
                        color: #9fadc6;
                        "
                    >{ "Album" }</div>
                    <div id="albums-select" style="display: flex; flex-direction: column;">
                        <select id="albums" ref={albums_ref} style="align-self: center; justify-self: center">
                        {
                            albums_state.iter().map(|album: &Album| {
                                html! {
                                    <option id={album.id.to_string()}>{ album.name.clone() }</option>
                                }
                            }).collect::<Html>()
                        }
                        </select>
                    </div>
                </div>
                <div id="songs-container" style="
                    background-color: #0b0e13;
                    width: 35%;
                    padding: 30px 20px;
                    justify-content: center;
                    align-items: center;
                    border-radius: 20px;
                    margin-right: 20px;
                    "
                >
                    <div id="songs-title" style="
                        text-align: center;
                        font-size: 30px;
                        color: #9fadc6;
                        "
                    >{ "Song" }</div>
                    <div id="songs-input" style="display: flex; flex-direction: column;">
                        <input id="songs" ref={song_ref.clone()} type="text" />
                    </div>
                    <TurkishLetters input_ref={&song_ref} />
                </div>
                <div id="quote-container" style="
                    background-color: #0b0e13;
                    width: 35%;
                    padding: 30px 20px;
                    justify-content: center;
                    align-items: center;
                    border-radius: 20px;
                    margin-right: 20px;
                    "
                >
                    <div id="quote-title" style="
                        text-align: center;
                        font-size: 30px;
                        color: #9fadc6;
                        "
                    >{ "Quote" }</div>
                    <div id="quote-input" style="display: flex; flex-direction: column;">
                        <input id="quotes" ref={quote_ref.clone()} type="text" />
                    </div>
                    <TurkishLetters input_ref={&quote_ref} />
                </div>
            </div>
            <div id="submit-container" style="display: flex; justify-content: center; margin-top: 20px;">
                <button type="submit" style="
                    font-size: 30px;
                    background-color: #60759f;
                    color: #394760;
                    border-radius: 10px;
                    border: none;
                    height: 50px;
                    padding: 10px 20px;
                    text-align: center;
                    cursor: pointer;
                    "
                    onclick={move |_| submit_quote()}
                >{ "Submit" }</button>
            </div>
            <div id="links-container" style="display: flex; justify-content: center; margin-top: 20px; margin-bottom: 20px;">
                <div id="links" style="display: grid; grid-template-columns: 1fr;">
                    <a href="/get/quote">{ "See all the quotes!" }</a>
                    <a href="/get/albums">{ "See all the albums!" }</a>
                    <a href="/get/songs">{ "See all the songs!" }</a>
                    <a href="/get/random/quote">{ "Get a random quote!" }</a>
                </div>
            </div>
        </div>
    }
}
