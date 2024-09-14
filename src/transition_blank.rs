use leptos::prelude::*;
use leptos_meta::*;

use crate::SearchComponent::SearchComponent;

#[component]
pub fn TransitionBlank() -> impl IntoView {
    view! {
        <Transition fallback=move || ()>

            <Title text="Search property data"/>
            <div class="relative flex flex-col min-h-screen bg-eggshell justify-between h-full w-full ">

                // {move || view! { <Header site_section=SiteSection::Base is_user=is_user() is_admin=is_admin()/> }} <div class="h-full flex flex-grow w-full">
                    <div class=" mx-auto lg:my-auto max-w-5xl mb-auto px-4  ">
                        <header class="block pt-12 ">
                            <h1 class="font-title text-navy text-xl md:text-3xl font-bold tracking-tight leading-none">"Search for property data"</h1>
                        </header>

                        <div class="mt-4 lg:mt-8">{move || view! { <SearchComponent /> }}</div>
                        <main class=" pb-20   mb-auto mx-auto max-w-5xl text-navy  pt-8">
                            <p class="max-w-[50ch]">"Houski is the largest " <b>"open-access database of detailed property information"</b> " on Earth."</p>
                            <p class="text-navy mt-6 max-w-[50ch]">
                                "Our goal is to make real estate information more accessible, so
                                people can make better decisions about where to live, work, and invest."
                            </p>
                        </main>

                    </div>

                </div>

        </Transition>
    }
}
