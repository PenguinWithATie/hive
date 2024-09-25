use leptos::*;

use crate::components::{layouts::base_layout::COMMON_LINK_STYLE, molecules::banner::Banner};

#[component]
pub fn Donate() -> impl IntoView {
    view! {
        <div class="pt-20">
            <div class="px-4 mx-auto max-w-4xl sm:px-6 lg:px-8">
                <Banner
                    title="Free Hive® for everyone, forever!".into_view()
                    text="No ads, no subscriptions; but open-source and passion."
                />
                <p class="my-4 text-lg text-center">
                    We are a community project and we believe everyone should have access to a free, world-class hive platform.
                    We rely on support from people like you to make it possible. If you enjoy using hivegame, please consider supporting us by donating.
                </p>

                <div class="flex justify-center items-center my-4">
                    <a href="https://ko-fi.com/hivedevs" class=COMMON_LINK_STYLE>
                        Ko-fi
                    </a>
                    <a href="https://www.patreon.com/HiveDevs" class=COMMON_LINK_STYLE>
                        Patreon
                    </a>
                </div>

                <div class="p-3">
                    <h3 class="text-lg font-medium leading-6">Where does the money go?</h3>
                    <p class="mt-2 text-base">
                        First of all, the server,
                        then our developers.
                    </p>
                </div>

                <div class="p-3">
                    <h3 class="text-lg font-medium leading-6">
                        Are some features reserved for Patrons?
                    </h3>
                    <p class="mt-2 text-base">
                        "No, because hivegame is entirely free, forever, and for everyone. That's a promise. You do get a 👑 though."
                    </p>
                </div>

                <div class="mt-4 text-center">
                    We are a small team, so your support makes a huge difference!
                </div>
            </div>
        </div>
    }
}
