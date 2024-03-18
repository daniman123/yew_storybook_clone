use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardProps {}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let CardProps {} = props;
    html! {
        <div
            class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700"
        >
            <a href="#">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                    { "Noteworthy technology acquisitions 2021" }
                </h5>
            </a>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                { "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order." }
            </p>
            <a
                href="#"
                class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
                { "Read more" }
                <svg
                    class="rtl:rotate-180 w-3.5 h-3.5 ms-2"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 14 10"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M1 5h12m0 0L9 1m4 4L9 9"
                    />
                </svg>
            </a>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct InputFieldsProps {}

#[function_component]
pub fn InputFields(props: &InputFieldsProps) -> Html {
    let InputFieldsProps {} = props;
    html! {
        <div class="w-1/2">
            <form>
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <label
                            for="first_name"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "First name" }
                        </label>
                        <input
                            type="text"
                            id="first_name"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="John"
                        />
                    </div>
                    <div>
                        <label
                            for="last_name"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "Last name" }
                        </label>
                        <input
                            type="text"
                            id="last_name"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="Doe"
                        />
                    </div>
                    <div>
                        <label
                            for="company"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "Company" }
                        </label>
                        <input
                            type="text"
                            id="company"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="Flowbite"
                        />
                    </div>
                    <div>
                        <label
                            for="phone"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "Phone number" }
                        </label>
                        <input
                            type="tel"
                            id="phone"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="123-45-678"
                            pattern="[0-9]{3}-[0-9]{2}-[0-9]{3}"
                        />
                    </div>
                    <div>
                        <label
                            for="website"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "Website URL" }
                        </label>
                        <input
                            type="url"
                            id="website"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="flowbite.com"
                        />
                    </div>
                    <div>
                        <label
                            for="visitors"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            { "Unique visitors (per month)" }
                        </label>
                        <input
                            type="number"
                            id="visitors"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder=""
                        />
                    </div>
                </div>
                <div class="mb-6">
                    <label
                        for="email"
                        class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >
                        { "Email address" }
                    </label>
                    <input
                        type="email"
                        id="email"
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        placeholder="john.doe@company.com"
                    />
                </div>
                <div class="mb-6">
                    <label
                        for="password"
                        class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >
                        { "Password" }
                    </label>
                    <input
                        type="password"
                        id="password"
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        placeholder="•••••••••"
                    />
                </div>
                <div class="mb-6">
                    <label
                        for="confirm_password"
                        class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                    >
                        { "Confirm password" }
                    </label>
                    <input
                        type="password"
                        id="confirm_password"
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        placeholder="•••••••••"
                    />
                </div>
                <div class="flex items-start mb-6">
                    <div class="flex items-center h-5">
                        <input
                            id="remember"
                            type="checkbox"
                            value=""
                            class="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800"
                        />
                    </div>
                    <label
                        for="remember"
                        class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                    >
                        { "I agree with the" }
                        <a href="#" class="text-blue-600 hover:underline dark:text-blue-500">
                            { "terms and conditions" }
                        </a>
                        { "." }
                    </label>
                </div>
                <button
                    type="submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                    { "Submit" }
                </button>
            </form>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct JumboTronProps {}

#[function_component]
pub fn JumboTron(props: &JumboTronProps) -> Html {
    let JumboTronProps {} = props;
    html! {
        <section
            class="bg-white dark:bg-gray-900 bg-[url('https://flowbite.s3.amazonaws.com/docs/jumbotron/hero-pattern.svg')] dark:bg-[url('https://flowbite.s3.amazonaws.com/docs/jumbotron/hero-pattern-dark.svg')]"
        >
            <div class="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 z-10 relative">
                <a
                    href="#"
                    class="inline-flex justify-between items-center py-1 px-1 pe-4 mb-7 text-sm text-blue-700 bg-blue-100 rounded-full dark:bg-blue-900 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-800"
                >
                    <span class="text-xs bg-blue-600 rounded-full text-white px-4 py-1.5 me-3">
                        { "New" }
                    </span>
                    <span class="text-sm font-medium">
                        { "Jumbotron component was launched! See what's new" }
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
                    { "We invest in the world’s potential" }
                </h1>
                <p
                    class="mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 lg:px-48 dark:text-gray-200"
                >
                    { "Here at Flowbite we focus on markets where technology, innovation, and capital can unlock long-term value and drive economic growth." }
                </p>
            </div>
            <div
                class="bg-gradient-to-b from-blue-50 to-transparent dark:from-blue-900 w-full h-full absolute top-0 left-0 z-0"
            />
        </section>
    }
}

#[derive(PartialEq, Properties)]
pub struct SearchInputProps {}

#[function_component]
pub fn SearchInput(props: &SearchInputProps) -> Html {
    let SearchInputProps {} = props;
    html! {
        <form class="max-w-md mx-auto">
            <label
                for="default-search"
                class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
            >
                { "Search" }
            </label>
            <div class="relative">
                <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                    <svg
                        class="w-4 h-4 text-gray-500 dark:text-gray-400"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 20 20"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
                        />
                    </svg>
                </div>
                <input
                    type="search"
                    id="default-search"
                    class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="Search Mockups, Logos..."
                />
                <button
                    type="submit"
                    class="text-white absolute end-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                    { "Search" }
                </button>
            </div>
        </form>
    }
}

#[derive(PartialEq, Properties)]
pub struct SearchInputSmallProps {}

#[function_component]
pub fn SearchInputSmall(props: &SearchInputSmallProps) -> Html {
    let SearchInputSmallProps {} = props;
    html! {
        <div class="pt-2 flex relative mx-auto text-gray-600">
            <input
                class="border-2 border-gray-300 bg-white h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none"
                type="search"
                name="search"
                placeholder="Search"
            />
            <button type="submit" class="relative right-7">
                <svg
                    class="text-gray-600 h-4 w-4 fill-current"
                    xmlns="http://www.w3.org/2000/svg"
                    xlink="http://www.w3.org/1999/xlink"
                    version="1.1"
                    id="Capa_1"
                    x="0px"
                    y="0px"
                    viewBox="0 0 56.966 56.966"
                    style="0 0 56.966 56.966;"
                    space="preserve"
                    width="51px"
                    height="512px"
                >
                    <path
                        d="M55.146,51.887L41.588,37.786c3.486-4.144,5.396-9.358,5.396-14.786c0-12.682-10.318-23-23-23s-23,10.318-23,23  s10.318,23,23,23c4.761,0,9.298-1.436,13.177-4.162l13.661,14.208c0.571,0.593,1.339,0.92,2.162,0.92  c0.779,0,1.518-0.297,2.079-0.837C56.255,54.982,56.293,53.08,55.146,51.887z M23.984,6c9.374,0,17,7.626,17,17s-7.626,17-17,17  s-17-7.626-17-17S14.61,6,23.984,6z"
                    />
                </svg>
            </button>
        </div>
    }
}
