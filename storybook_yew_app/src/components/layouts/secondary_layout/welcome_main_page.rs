use crate::utils::load_seo::SEO_CONTENT;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WelcomePageProps {}

#[function_component]
pub fn WelcomePage(_props: &WelcomePageProps) -> Html {
    // let WelcomePageProps {} = props;
    let seo_content = SEO_CONTENT.clone();

    html! {
        <section
            class="bg-white dark:bg-gray-900 bg-[url('https://flowbite.s3.amazonaws.com/docs/jumbotron/hero-pattern.svg')] dark:bg-[url('https://flowbite.s3.amazonaws.com/docs/jumbotron/hero-pattern-dark.svg')]"
        >
            <div class="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 z-10 relative">
                <a
                    href="https://github.com/daniman123/yew_storybook_clone"
                    class="inline-flex justify-between items-center py-1 px-1 pe-4 mb-7 text-sm text-blue-700 bg-blue-100 rounded-full dark:bg-blue-900 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-800"
                >
                    <span class="text-xs bg-blue-600 rounded-full text-white px-4 py-1.5 me-3">
                        { seo_content.page_banner_tag }
                    </span>
                    <span class="text-sm font-medium">
                        { seo_content.page_banner_description }
                    </span>
                    <svg
                        class="w-2.5 h-2.5 ms-2 rtl:rotate-180"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 6 10"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="m1 9 4-4-4-4"
                        />
                    </svg>
                </a>
                <h1
                    class="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white"
                >
                    { seo_content.page_title }
                </h1>
                <p
                    class="mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 lg:px-48 dark:text-gray-200"
                >
                    { seo_content.page_description }
                </p>
            </div>
            <div
                class="bg-gradient-to-b from-blue-50 to-transparent dark:from-blue-900 w-full h-full absolute top-0 left-0 z-0"
            />
        </section>
    }
}
