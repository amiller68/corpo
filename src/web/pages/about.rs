use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-[80vh]">
                <h1
                    class="relative w-[max-content] font-mono text-4xl font-bold before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                    > "about me"
                </h1>
            <div
                class="mt-8 text-md text-left max-w-[50ch]"
            >
                <p class="mb-4">
                    "
                    hey there, my name is alex, im a software engineer with a product-minded focus. 
                    "
                </p>
                <p class="mb-4">
                    "
                    i have a passion for creating software that empowers, engages, and excels. 
                    i'm currently based in new york city, helping people all over the world turn bold ideas into reality.
                    "               
                </p>
                <p class="mb-4">
                    "
                    Interested in working together? Let's connect.
                    "
                </p>
            </div>
            <div
                class="mt-8 flex text-2xl space-x-4"
            >
                <div class="hover:scale-110 transition-transform transform-gpu">
                    <a href="https://github.com/amiller68">
                        <Icon icon=i::AiGithubFilled/>
                    </a>
                </div>
                <div class="hover:scale-110 transition-transform transform-gpu">
                    <a href="https://twitter.com/lord_krondor">
                        <Icon icon=i::AiTwitterOutlined/>
                    </a>
                </div>
                <div class="hover:scale-110 transition-transform transform-gpu">
                    <a href="https://www.linkedin.com/in/alex-miller-110953171/">
                        <Icon icon=i::AiLinkedinFilled/>
                    </a>
                </div>
                <div class="hover:scale-110 transition-transform transform-gpu">
                    <a href="mailto:al@krondor.org">
                        <Icon icon=i::AiMailOutlined/>
                    </a>
                </div>
                <div class="hover:scale-110 transition-transform transform-gpu">
                    <a href="tg://resolve?domain=lord_krondor">
                        <Icon icon=i::BiTelegram/>
                    </a>
                </div>
            </div>
        </div>
    }
}
