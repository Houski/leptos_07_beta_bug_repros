use leptos::prelude::*;

use leptos_meta::*;

use crate::api_doc_request_and_response::ApiDocRequestAndResponse;
use std::env::var;

#[component]
pub fn ApiDocProperties() -> impl IntoView {
    // let api_doc_properties_type_declarations = Resource::<Result<String, ServerFnError>>::new(
    //     move || code_language(),
    //     move |code_language| api_doc_properties_type_declarations(code_language),
    // );

    view! {
        <Transition fallback=move || ()>

            <Title text="Properties endpoint - API documentation"/>
            <div class="relative flex flex-col min-h-screen bg-eggshell justify-between h-full w-full ">


                    <div class=" mb-auto mx-auto max-w-5xl w-full ">
                        <div class="block w-full md:flex h-full">
                            <div class="hidden md:block ">
                                // <ApiDocMenu/>
                            </div>
                            <div class="mt-8 mx-4 md:mx-8 max-w-full">
                                <div class="flex items-center gap-2">
                                    <h1 class="text-2xl  text-navy font-bold tracking-tight leading-none">"/properties"</h1>
                                    <div class="mt-2 p-2 rounded-md bg-green font-bold text-white text-sm flex items-center h-6">"GET"</div>

                                </div>

                                <div class="text-[#8f9196]">

                                    {format!("{}{}", var("PROPERTY_API_URL").unwrap_or_default(), "/properties")}

                                </div>


                                <div class="mt-4 ">
                                    <p class="mt-4 text-navy text-sm">
                                        "This endpoint returns detailed information about properties. It can be used to get information for multiple properties, or just one."
                                    </p>

                                    <p class="mt-4 text-sm text-navy">
                                        "This endpoint conforms to Houski API's standard " <a href="/api-documentation/filter" class="underline font-bold">
                                            "filtering"
                                        </a> ", " <a href="/api-documentation/sort" class="underline font-bold">
                                            "sorting"
                                        </a> ", and " <a href="/api-documentation/select" class="underline font-bold">
                                            "selection"
                                        </a> " functionality."
                                    </p>
                                    <p class="mt-4 text-sm text-navy">
                                        "To see all the fields that can be selected, filtered or sorted by, see the " <a href="/api-documentation/fields" class="underline font-bold">
                                            "fields"
                                        </a> " page."
                                    </p>
                                    <p class="mt-4 text-sm text-navy">
                                        "To find data on all available location values, such as
                                        country_abbreviation, province_abbreviation, city and community,
                                        you can use the " <a href="/api-documentation/locations" class="underline font-bold">
                                            "/locations"
                                        </a> " endpoint."
                                    </p>

                                    <h2 class="text-lg mt-8  text-navy font-bold tracking-tight leading-none">"Example use cases"</h2>
                                    <div class="max-w-fit mt-2">

                                    </div>
                                    <ol class="mt-4 text-sm  list-disc leading-tight max-w-[60ch] ">
                                        <li class="text-navy mt-2 ml-6">"Build a real estate website and provide your users with detailed information about properties they are interested in."</li>
                                        <li class="text-navy mt-2 ml-6">"Analyze property market trends, identify areas with high demand or low supply, and make informed investment decisions."</li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Government agencies or urban planners can use this endpoint to gather data on population density, average income, and other factors that influence city planning decisions."
                                        </li>
                                        <li class="text-navy mt-2 ml-6">"Insurance companies can use this endpoint to gather data on property values to help determine insurance rates."</li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Appraisal services can use this endpoint API to gather data on property values and trends to help determine the value of a property."
                                        </li>
                                        <li class="text-navy mt-2 ml-6">"Create customized Automated Valuation Models (AVM)."</li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Target developers or business owners looking for properties in certain zones using the " <a href="/api-documentation/fields#zoning" class="underline">
                                                "zoning"
                                            </a> " field."
                                        </li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Appeal to workshop enthusiasts with information on workshop types using the " <a href="/api-documentation/fields#workshop_type" class="underline">
                                                "workshop_type"
                                            </a> " field."
                                        </li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Leverage " <a href="/api-documentation/fields#score_transit" class="underline">
                                                "score_transit"
                                            </a> " to attract commuters or those who prefer not to own a vehicle."
                                        </li>
                                        <li class="text-navy mt-2 ml-6">
                                            "Utilize various " <a href="/api-documentation/fields#score_fields" class="underline">
                                                "score fields"
                                            </a> " to create a property recommendation engine."
                                        </li>

                                    </ol>

                                    // REQUEST PARAMETERS.
                                    <h2 class="text-lg mt-8 text-navy font-bold tracking-tight leading-none">"Request parameters"</h2>

                                    <div class="overflow-x-auto mt-2">
                                        <table class="text-sm w-full">
                                            <thead>
                                                <tr>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Name"</th>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Required"</th>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Type"</th>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Description"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"api_key"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Yes"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"UUID v4"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "Authorization " <a href="/api-documentation/authentication" class="underline">
                                                            "API key"
                                                        </a>
                                                    </td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"country_abbreviation"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"String"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "A " <a href="/api-documentation/fields#country_abbreviation" class="underline">
                                                            "country abbreviation"
                                                        </a>
                                                    </td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"province_abbreviation"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"String"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "A " <a href="/api-documentation/fields#province_abbreviation" class="underline">
                                                            "province abbreviation"
                                                        </a> " within the country"
                                                    </td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"city"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"String"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "A " <a href="/api-documentation/fields#city" class="underline">
                                                            "city"
                                                        </a> " within the province"
                                                    </td>

                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"community"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"String"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "A " <a href="/api-documentation/fields#community" class="underline">
                                                            "community"
                                                        </a> " within the city"
                                                    </td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"bbox_ne_lng"</td>

                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Northeast longitude for bounding box."</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"bbox_sw_lat"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Southwest latitude for bounding box."</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"bbox_sw_lng"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Southwest longitude for bounding box."</td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"polygon"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        <a href="/api-documentation/filter" class="underline">
                                                            "Polygon filter string"
                                                        </a>
                                                    </td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Get results inside a polygon."</td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"results_per_page"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Integer"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number of results per page"</td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"select"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"No"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">
                                                        "String - see " <a href="/api-documentation/select" class="underline">
                                                            "selecting"
                                                        </a>
                                                    </td>
                                                    <td class="border px-4 py-2 border-light-gray">
                                                        "The fields to return - " <a href="/api-documentation/fields#address" class="underline">
                                                            "address"
                                                        </a> " and " <a href="/api-documentation/fields#property_id" class="underline">
                                                            "property_id"
                                                        </a> " are always selected"
                                                    </td>
                                                </tr>

                                            </tbody>
                                        </table>
                                    </div>

                                    // RESPONSE OBJECT.

                                    <h2 class="text-lg mt-8 text-navy font-bold tracking-tight leading-none">"Response object"</h2>

                                    <p class=" text-sm w-full text-black">
                                        <a href="#types" class="underline font-bold text-black">
                                            "Type declarations"
                                        </a>
                                        " are available at the bottom of this page."
                                    </p>

                                    <div class="overflow-x-auto mt-2">
                                        <table class="text-sm w-full">
                                            <thead>
                                                <tr>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Name"</th>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Type"</th>
                                                    <th class="border px-4 py-2 border-light-gray text-left">"Description"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"cache_hit"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Boolean"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Indicates if the response was a cache hit"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"cost_cents"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Cost of the request in cents"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"data"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Array of objects"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Contains the data for the properties"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"error"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"String"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Details about the error. Empty if no error"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"pagination"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Object"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Contains pagination info like current page and total pages"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"price_quote"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Boolean"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Indicates whether the response is a price quote"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"result_total"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"The total number of results"</td>
                                                </tr>
                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"time_ms"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Number"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Time taken for the request to complete in milliseconds"</td>
                                                </tr>

                                                <tr>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"ui_info"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Object"</td>
                                                    <td class="border px-4 py-2 border-light-gray whitespace-nowrap align-top">"Useful metadata to create user interfaces"</td>
                                                </tr>
                                            </tbody>
                                        </table>
                                    </div>

                                </div>

                                <h2 class="text-lg mt-8 text-navy font-bold tracking-tight leading-none">"Example requests and responses"</h2>



                                        <ApiDocRequestAndResponse
                                            params=vec![
                                                ("country_abbreviation".into(), "ca".into()),
                                                ("province_abbreviation".into(), "ab".into()),
                                                ("city".into(), "calgary".into()),
                                                ("community".into(), "riverbend".into()),
                                                ("results_per_page".into(), "3".into()),
                                                ("select".into(), "bedroom,den,estimate_list_price".into()),
                                            ]


                                            title="Get a list of properties".into()
                                            func_name="houski_get_properties".into()

                                            user_api_key="YOUR_API_KEY".to_string()
                                        >
                                            <p class="text-sm text-navy">"Get a list of properties in a given location."</p>
                                        </ApiDocRequestAndResponse>


                                <h2 id="types" class="text-lg mt-8 text-navy font-bold tracking-tight leading-none">
                                    "Response type declarations"
                                </h2>


                                <div class="mb-16"></div>

                            </div>
                        </div>
                    </div>
                </div>

        </Transition>
    }
}
