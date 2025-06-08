use gloo_net::http;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Default, Clone, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub preparation: Vec<String>,
    pub source: String,
    pub tags: Vec<String>,
}

#[function_component]
fn App() -> Html {
    let recipe = use_state(|| Recipe::default());

    let fetch_random = {
        let recipe = recipe.clone();
        move |_| {
            let recipe = recipe.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_recipe: Recipe =
                    http::Request::get("http://127.0.0.1:8888/api/v1/recipe/random")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                recipe.set(fetched_recipe);
            });
        }
    };

    let fetch_tagged = {
        let recipe = recipe.clone();
        move |_| {
            let recipe = recipe.clone();
            wasm_bindgen_futures::spawn_local(async move {
                //let body = vec!["egg".to_string(),"berry".to_string()];
                let fetched_recipe: Recipe =
                    http::Request::get("http://127.0.0.1:8888/api/v1/recipe/with-tags")
                        //.body(body).unwrap() //NOTE: I'm not sure why this doesn't work.
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                recipe.set(fetched_recipe);
            });
        }
    };

    html! {
        <div>
            <span>
                <button onclick={fetch_random}>{"Random Recipe"}</button>
            </span><br/><br/>
            <span>
                <label>{"Search recipes by tag: "}</label>
                <input type="text" name="tags" />
                <button onclick={fetch_tagged}>{"Find"}</button>
            </span><br/>
            <h2>{ recipe.name.clone() }</h2>
            <span>
                <ul>
                    { for recipe.ingredients.iter().map(|item| { html! { <li>{ item }</li> }}) }
                </ul>
            </span>
            <ol>
                { for recipe.preparation.iter().map(|item| { html! { <li>{ item }</li> }}) }
            </ol><br/>
            <ul>
                { for recipe.tags.iter().map(|item| { html! { <li>{ item }</li> }}) }
            </ul><br/>
            <p>{ format!("Source: {}", recipe.source) }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
