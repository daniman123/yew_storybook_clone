#[derive(Debug, Clone)]
pub struct SeoContent {
    pub page_title: &'static str,
    pub page_description: &'static str,
    pub page_banner_tag: &'static str,
    pub page_banner_description: &'static str,
}

pub const SEO_CONTENT: SeoContent = SeoContent {
    page_banner_tag: "New",
    page_banner_description: "Attention: Early stages of development",
    page_title: "Your Visual Testing Suite for Yew Components",
    page_description: "Welcome to the Rust-Yew Storybook Clone, your go-to visual testing suite for Yew components. Designed to streamline the development process, our platform empowers developers to conduct comprehensive visual tests on Yew components in isolation from the main Yew project. Whether you're a seasoned developer or just starting out, our platform provides the tools you need to ensure the visual integrity of your components.",
};
