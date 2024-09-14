use leptos::prelude::*;
use leptos::web_sys;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug)]
pub enum MainSearchStyle {
    Header,
    Page,
}

#[derive(Clone, PartialEq)]
pub struct SearchResult {
    pub link: String,
    pub main_text: String,
    pub secondary_text: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    pub link: String,
    pub main_text: String,
    pub secondary_text: String,
}

#[island]
pub fn SearchComponent() -> impl IntoView {
    use leptos::html::Div;

    let (search_text, set_search_text) = signal("".to_string());

    let searches = LocalResource::<Result<Vec<SearchResponse>, ServerFnError>>::new(move || {
        perform_search(search_text())
    });

    let _reffy = NodeRef::<Div>::new();

    view! {

        <div class="relative" node_ref=_reffy>
            <div>
                <div class="py-4 relative">
                    <div class="flex gap-2">
                        <div class="pl-4" aria-hidden="true">

                        </div>
                        <form id="searchForm" class="contents">
                            <input
                                id="searchQuery"
                                name="query"
                                autocomplete="off"
                                style="border-color:black; border-width:2px; border-style:solid;"
                                placeholder="Enter address, community, city, etc"
                                autofocus=true
                                on:input=debounce(std::time::Duration::from_millis(200), move |ev| { set_search_text(event_target_value(&ev)) })
                                on:focus=move |ev| set_search_text(event_target_value(&ev))
                            />
                            <button type="submit" class="sr-only">
                                Search
                            </button>

                        </form>
                        // <script>{include_str!("./javascript/SearchInput.js")}</script>
                    </div>
                </div>
            </div>
            <div class="absolute top-16 rounded-lg overflow-hidden z-10 shadow-2xl w-full">

                {move || match searches.get().map(|w| w.take()) {
                    Some(Ok(response)) => {
                        let search_results:Vec<SearchResult> = response
                            .into_iter()
                            .map(|data| {

                                let address_link = data.link.clone();
                                let main_text = data.main_text.clone();
                                let province = data.secondary_text.clone();
                                SearchResult {
                                    link: address_link,
                                    main_text: main_text,
                                    secondary_text: province,
                                }
                            })
                            .collect();
                        let no_results = search_results.is_empty();
                        if no_results && !search_text().is_empty() {
                            view! { <div class="text-sm py-4 pl-4 bg-white text-navy hover:cursor-not-allowed font-bold">"No results found"</div> }.into_any()
                        } else {
                            view! {
                                <div>

                                    {search_results
                                        .iter()
                                        .map(|result| {
                                            view! {
                                                <a href=result.link.clone() class="block">
                                                    <div class="text-sm py-4 pl-4 bg-white text-navy hover:bg-navy hover:text-white group font-bold overflow-hidden whitespace-nowrap overflow-ellipsis">
                                                        {result.main_text.clone()} " " <span class="ml-1 #aaa group-hover:text-white font-normal">{result.secondary_text.clone()}</span>
                                                    </div>
                                                </a>
                                            }
                                        })
                                        .collect_view()}

                                </div>
                            }
                                .into_any()
                        }
                    }
                    Some(Err(_)) => {
                        if !search_text().is_empty() {
                            view! { <div class="text-sm py-4 pl-4 bg-white text-navy hover:cursor-not-allowed font-bold">"There was an error with the search"</div> }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }
                    None => view! { <div></div> }.into_any(),
                }}

            </div>
        </div>

    }
}

#[server(PerformSearch, "/api")]
pub async fn perform_search(query: String) -> Result<Vec<SearchResponse>, ServerFnError> {
    let empty_search_results = SearchResponse {
        link: "https://www.houski.com".to_string(),
        main_text: "No results found".to_string(),
        secondary_text: "".to_string(),
    };

    let url = format!("https://www.google.com/",);
    let response = reqwest::get(&url).await;

    match response {
        Ok(response) => {
            let text = response.text().await;
            match text {
                Ok(text) => {
                    let dummy_result = SearchResponse {
                        link: "https://example.com".to_string(),
                        main_text: "Example search result".to_string(),
                        secondary_text: "This is a dummy result".to_string(),
                    };
                    Ok(vec![dummy_result])
                }
                Err(_) => Ok(vec![empty_search_results]),
            }
        }
        Err(_) => Ok(vec![empty_search_results]),
    }
}
