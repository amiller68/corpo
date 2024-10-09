use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-[80vh] w-full">
            <div class="max-w-[50ch] w-full px-4">
                <h1
                    class="relative w-[max-content] font-mono text-4xl font-bold before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                    > "about"
                </h1>
                <div
                    class="mt-8 text-md text-left"
                >
                    <p class="mb-4">
                        "
                        hey there, welcomer to krondor.org!
                        "
                    </p>
                    <p class="mb-4">
                        "
                        my name is alex. 
                        i'm a software engineer with a background in product, 
                        system, and protocol design, having worked with teams of all sizes, 
                        all across the world.
                        i'm currently based in new york city, helping my clients turn their dreams into reality.
                        "               
                    </p>
                    <p class="mb-4">
                        "
                        interested in working together?
                        "
                    </p>

                    <a
                        class="text-blue-500 hover:underline"
                        href="https://docs.google.com/forms/d/1V16MGOz2V-JvivA7EMeaS-O-4c1duojn6V8hsGRcM2k/prefill"
                    >
                        "
                        get in touch!
                        "
                    </a>
                </div>
            </div>
            <div
                class="mt-8 flex justify-center text-2xl space-x-4 w-full"
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
