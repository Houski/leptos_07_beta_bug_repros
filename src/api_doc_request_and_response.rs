use leptos::prelude::*;

pub enum MainSearchStyle {
    Header,
    Page,
}

use leptos::{prelude::ServerFnError, server};

use std::env;

#[cfg(feature = "ssr")]
use reqwest;
#[cfg(feature = "ssr")]
use std::collections::HashMap;

#[cfg(feature = "ssr")]
pub struct ApiCodeDisplayGenerator {
    pub params: Vec<(String, String)>,
    pub func_name: String,
}

#[server(ApiDocGenerateResponseCode, "/api")]
pub async fn api_doc_generate_response_code(
    params: Vec<(String, String)>,
) -> Result<String, ServerFnError> {
    // do random reqwest out to google, to simulate the call to the property db
    let response = reqwest::get("https://google.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(
        r#"<div class="code-block"><div class="language-tag">JSON</div><pre class="code-block-inner" data-lang="json">{
  <i class="hh4">"cache_hit"</i>: true,
  <i class="hh4">"cost_cents"</i>: 1.5,
  <i class="hh4">"data"</i>: [
    {
      <i class="hh4">"address"</i>: <i class="hh10">"20 Rivergreen Crescent SE"</i>,
      <i class="hh4">"bedroom"</i>: 3,
      <i class="hh4">"den"</i>: 0,
      <i class="hh4">"estimate_list_price"</i>: 664035,
      <i class="hh4">"property_id"</i>: <i class="hh10">"cb703fdf1467d168"</i>
    },
    {
      <i class="hh4">"address"</i>: <i class="hh10">"24 Riverview Drive SE"</i>,
      <i class="hh4">"bedroom"</i>: 1,
      <i class="hh4">"den"</i>: 0,
      <i class="hh4">"estimate_list_price"</i>: 654176,
      <i class="hh4">"property_id"</i>: <i class="hh10">"4fbfdec714cf7a82"</i>
    },
    {
      <i class="hh4">"address"</i>: <i class="hh10">"160 Riverwood Circle SE"</i>,
      <i class="hh4">"bedroom"</i>: 3,
      <i class="hh4">"den"</i>: 0,
      <i class="hh4">"estimate_list_price"</i>: 616593,
      <i class="hh4">"property_id"</i>: <i class="hh10">"1ac664801c3dbd51"</i>
    }
  ],
  <i class="hh4">"error"</i>: <i class="hh10">""</i>,
  <i class="hh4">"pagination"</i>: {
    <i class="hh4">"current_page"</i>: 1,
    <i class="hh4">"has_next_page"</i>: true,
    <i class="hh4">"has_previous_page"</i>: false,
    <i class="hh4">"page_total"</i>: 1196
  },
  <i class="hh4">"price_quote"</i>: false,
  <i class="hh4">"result_total"</i>: 3588,
  <i class="hh4">"time_ms"</i>: 27,
  <i class="hh4">"ui_info"</i>: {
    <i class="hh4">"city"</i>: <i class="hh10">"Calgary"</i>,
    <i class="hh4">"city_id"</i>: <i class="hh10">"6ec95b53075d062c"</i>,
    <i class="hh4">"city_link"</i>: <i class="hh10">"ca/ab/calgary"</i>,
    <i class="hh4">"city_slug"</i>: <i class="hh10">"calgary"</i>,
    <i class="hh4">"community"</i>: <i class="hh10">"Riverbend"</i>,
    <i class="hh4">"community_id"</i>: <i class="hh10">"f5c930e9ed9a38e4"</i>,
    <i class="hh4">"community_link"</i>: <i class="hh10">"ca/ab/calgary/riverbend"</i>,
    <i class="hh4">"community_slug"</i>: <i class="hh10">"riverbend"</i>,
    <i class="hh4">"country"</i>: <i class="hh10">"Canada"</i>,
    <i class="hh4">"country_abbreviation"</i>: <i class="hh10">"CA"</i>,
    <i class="hh4">"country_abbreviation_id"</i>: <i class="hh10">"9ace2b6431b7f1be"</i>,
    <i class="hh4">"country_abbreviation_link"</i>: <i class="hh10">"ca"</i>,
    <i class="hh4">"country_slug"</i>: <i class="hh10">"canada"</i>,
    <i class="hh4">"province"</i>: <i class="hh10">"Alberta"</i>,
    <i class="hh4">"province_abbreviation"</i>: <i class="hh10">"AB"</i>,
    <i class="hh4">"province_abbreviation_id"</i>: <i class="hh10">"aae1f05a0f89d2c7"</i>,
    <i class="hh4">"province_abbreviation_link"</i>: <i class="hh10">"ca/ab"</i>,
    <i class="hh4">"province_slug"</i>: <i class="hh10">"alberta"</i>
  }
}
</pre></div>"#.to_string()
    )
}

#[server(ApiDocGenerateRequestCode, "/api")]
pub async fn api_doc_generate_request_code(
    params: Vec<(String, String)>,
    func_name: String,
) -> Result<String, ServerFnError> {
    Ok(r#"<div class="code-block"><div class="language-tag">Rust code</div><pre class="code-block-inner" data-lang="rust"><i class="hh4">use</i> std<i class="hh9">::</i>collections<i class="hh9">::</i>HashMap<i class="hh9">;</i>
<i class="hh4">use</i> reqwest<i class="hh9">;</i>
<i class="hh4">use</i> serde_json<i class="hh9">::</i>Value<i class="hh9">;</i>
<i class="hh4">use</i> serde_urlencoded<i class="hh9">;</i>

<i class="hh0">#<i class="hh8">[</i>tokio<i class="hh9">::</i>main<i class="hh8">]</i></i>
<i class="hh4">async</i> <i class="hh4">fn</i> <i class="hh3">main</i><i class="hh8">(</i><i class="hh8">)</i> <i class="hh8">{</i>
    <i class="hh4">let</i> <i class="hh4">mut</i> params = <i class="hh13">HashMap</i><i class="hh9">::</i><i class="hh3">new</i><i class="hh8">(</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"api_key"</i><i class="hh9">,</i> <i class="hh10">"YOUR_API_KEY"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"city"</i><i class="hh9">,</i> <i class="hh10">"calgary"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"community"</i><i class="hh9">,</i> <i class="hh10">"riverbend"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"country_abbreviation"</i><i class="hh9">,</i> <i class="hh10">"ca"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"province_abbreviation"</i><i class="hh9">,</i> <i class="hh10">"ab"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"results_per_page"</i><i class="hh9">,</i> <i class="hh10">"3"</i><i class="hh8">)</i><i class="hh9">;</i>
    params<i class="hh9">.</i><i class="hh3">insert</i><i class="hh8">(</i><i class="hh10">"select"</i><i class="hh9">,</i> <i class="hh10">"bedroom,den,estimate_list_price"</i><i class="hh8">)</i><i class="hh9">;</i>

    <i class="hh4">let</i> client = reqwest<i class="hh9">::</i><i class="hh13">Client</i><i class="hh9">::</i><i class="hh3">new</i><i class="hh8">(</i><i class="hh8">)</i><i class="hh9">;</i>
    <i class="hh4">let</i> query = <i class="hh4">match</i> serde_urlencoded<i class="hh9">::</i><i class="hh3">to_string</i><i class="hh8">(</i><i class="hh5">&amp;</i>params<i class="hh8">)</i> <i class="hh8">{</i>
        Ok<i class="hh8">(</i>q<i class="hh8">)</i> =&gt; q<i class="hh9">,</i>
        Err<i class="hh8">(</i>err<i class="hh8">)</i> =&gt; <i class="hh4">return</i> <i class="hh3">println</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"Failed to serialize query parameters: {}"</i><i class="hh9">,</i> err<i class="hh8">)</i><i class="hh9">,</i>
    <i class="hh8">}</i><i class="hh9">;</i>

    <i class="hh4">let</i> url = <i class="hh3">format</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"https://api.houski.ca/properties?{}"</i><i class="hh9">,</i> query<i class="hh8">)</i><i class="hh9">;</i>
    <i class="hh4">let</i> <i class="hh4">mut</i> res = <i class="hh4">match</i> client<i class="hh9">.</i><i class="hh3">get</i><i class="hh8">(</i><i class="hh5">&amp;</i>url<i class="hh8">)</i><i class="hh9">.</i><i class="hh3">send</i><i class="hh8">(</i><i class="hh8">)</i><i class="hh9">.</i><i class="hh4">await</i> <i class="hh8">{</i>
        Ok<i class="hh8">(</i>response<i class="hh8">)</i> =&gt; response<i class="hh9">,</i>
        Err<i class="hh8">(</i>err<i class="hh8">)</i> =&gt; <i class="hh4">return</i> <i class="hh3">println</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"Failed to send HTTP request: {}"</i><i class="hh9">,</i> err<i class="hh8">)</i><i class="hh9">,</i>
    <i class="hh8">}</i><i class="hh9">;</i>

    <i class="hh4">let</i> res_value = <i class="hh4">match</i> res<i class="hh9">.</i><i class="hh3">json</i><i class="hh9">::</i><i class="hh8">&lt;</i><i class="hh13">Value</i><i class="hh8">&gt;</i><i class="hh8">(</i><i class="hh8">)</i><i class="hh9">.</i><i class="hh4">await</i> <i class="hh8">{</i>
        Ok<i class="hh8">(</i>value<i class="hh8">)</i> =&gt; value<i class="hh9">,</i>
        Err<i class="hh8">(</i>err<i class="hh8">)</i> =&gt; <i class="hh4">return</i> <i class="hh3">println</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"Failed to parse JSON response: {}"</i><i class="hh9">,</i> err<i class="hh8">)</i><i class="hh9">,</i>
    <i class="hh8">}</i><i class="hh9">;</i>

    <i class="hh18">// You must copy the PropertiesResponse type declarations from the </i>
    <i class="hh18">// Houski API documentation to strongly type the response</i>
    <i class="hh4">let</i> data = <i class="hh4">match</i> serde_json<i class="hh9">::</i><i class="hh3">from_value</i><i class="hh9">::</i><i class="hh8">&lt;</i><i class="hh13">PropertiesResponse</i><i class="hh8">&gt;</i><i class="hh8">(</i>res_value<i class="hh8">)</i> <i class="hh8">{</i>
        Ok<i class="hh8">(</i>d<i class="hh8">)</i> =&gt; d<i class="hh9">,</i>
        Err<i class="hh8">(</i>err<i class="hh8">)</i> =&gt; <i class="hh4">return</i> <i class="hh3">println</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"Failed to deserialize to PropertiesResponse struct: {}"</i><i class="hh9">,</i> err<i class="hh8">)</i><i class="hh9">,</i>
    <i class="hh8">}</i><i class="hh9">;</i>

    <i class="hh18">// Log the response</i>
    <i class="hh3">println</i><i class="hh3">!</i><i class="hh8">(</i><i class="hh10">"{:#?}"</i><i class="hh9">,</i> data<i class="hh8">)</i><i class="hh9">;</i>
<i class="hh8">}</i>
</pre></div>"#.to_string())
}

#[server(ApiDocGenerateRequestCurl, "/api")]
pub async fn api_doc_generate_request_curl(
    params: Vec<(String, String)>,
) -> Result<String, ServerFnError> {
    Ok(r#"<div class="code-block"><div class="language-tag">Shell session</div><pre class="code-block-inner" data-lang="shell">curl -X GET "https://api.houski.ca/properties?api_key=YOUR_API_KEY&amp;city=calgary&amp;community=riverbend&amp;country_abbreviation=ca&amp;province_abbreviation=ab&amp;results_per_page=3&amp;select=bedroom,den,estimate_list_price"
    </pre></div>"#.to_string())
}

#[component]
pub fn ApiDocRequestAndResponse(
    params: Vec<(String, String)>,
    title: String,
    children: Children,
    func_name: String,
    user_api_key: String,
) -> impl IntoView {
    let params_copy_1 = params.to_owned();
    let params_copy_2 = params.to_owned();
    let params_copy_3 = params.to_owned();

    let user_api_key_copy_1 = user_api_key.to_owned();
    let user_api_key_copy_2 = user_api_key.to_owned();

    let api_doc_generate_request_curl = Resource::<Result<String, ServerFnError>>::new(
        move || (),
        move |_| {
            api_doc_generate_request_curl({
                let mut params = params_copy_1.to_owned();
                params.push(("api_key".to_string(), user_api_key_copy_1.clone()));
                params.sort();
                params
            })
        },
    );

    let api_doc_generate_request_code = Resource::<Result<String, ServerFnError>>::new(
        move || (),
        move |_| {
            api_doc_generate_request_code(
                {
                    let mut params = params_copy_2.to_owned();
                    params.push(("api_key".to_string(), user_api_key_copy_2.clone()));
                    params.sort();
                    params
                },
                func_name.to_owned(),
            )
        },
    );

    let api_doc_generate_response_code: Resource<Result<String, ServerFnError>> = Resource::new(
        move || (),
        move |_| api_doc_generate_response_code(params_copy_3.to_owned()),
    );

    view! {
        <div class="font-bold text-lg text-navy mt-8">{title}</div>
        {children()}

        <div class="mb-8">

            <div class="text-navy text-sm font-bold">"Request"</div>

            <div class="flex flex-col gap-2">

                <div inner_html=move || {
                    match api_doc_generate_request_curl.get() {
                        Some(Ok(result)) => result,
                        _ => "".to_string(),
                    }
                }></div>

                <div inner_html=move || {
                    match api_doc_generate_request_code.get() {
                        Some(Ok(result)) => result,
                        _ => "".to_string(),
                    }
                }></div>

            </div>

            <div class="text-navy text-sm font-bold mt-2">"Response"</div>


                <div inner_html=move || {
                    match api_doc_generate_response_code.get() {
                        Some(Ok(result)) => {
                            log!("result ON THE ACTUAL PAGE: {}", result);
                            result
                        }
                        Some(Err(e)) => {
                            log!("error ON THE ACTUAL PAGE: {}", e);
                            "".to_string()
                        }
                        None => {
                            log!("NONE RESULT");
                            "".to_string()
                        }
                    }
                }></div>



        </div>
    }
}
