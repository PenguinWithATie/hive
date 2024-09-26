use crate::components::{atoms::simple_link::SimpleLink, molecules::banner::Banner};
use crate::i18n::*;
use leptos::*;

#[component]
pub fn Faq() -> impl IntoView {
    let i18n = use_i18n();
    let header_class = "text-lg leading-6 font-medium";
    let paragraph_class = "mt-2 text-base";
    let div_class = "p-3";
    let source_link = |children: ChildrenFn| {
        view! { <SimpleLink link="https://github.com/hiveboardgame/hive" children=children.into()/> }
    };
    let discord_link = |children: ChildrenFn| {
        view! { <SimpleLink link="https://discord.gg/jNTjr5vj9Z" children=children.into()/> }
    };
    let donate_link = |children: ChildrenFn| {
        view! { <SimpleLink link="/donate" children=children.into()/> }
    };
    let gen42_link = |_| {
        view! { <SimpleLink link="https://www.gen42.com/" text="Gen42"/> }
    };
    let glicko2_link = |_| {
        view! { <SimpleLink link="https://en.wikipedia.org/wiki/Glicko-2" text="Glicko-2"/> }
    };
    let resources_link = |children: ChildrenFn| {
        view! { <SimpleLink link="/resources" children=children.into()/> }
    };
    view! {
        <div class="pt-20">
            <div class="px-4 mx-auto max-w-4xl sm:px-6 lg:px-8">
                <Banner title=t!(i18n, faq.title).into_view()/>

                <div class="space-y-10 md:space-y-0 md:grid md:grid-cols-1 md:gap-x-6 lg:gap-x-8">
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.what_is_hivegame.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.what_is_hivegame.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_to_help.question)}</h3>
                        <p class=paragraph_class>
                            <ul class="mt-2 list-disc list-inside">
                                <li>{t!(i18n, faq.how_to_help.answers.item1, < source_link >)}</li>
                                <li>{t!(i18n, faq.how_to_help.answers.item2, < discord_link >)}</li>
                                <li>{t!(i18n, faq.how_to_help.answers.item3, < donate_link >)}</li>
                            </ul>
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_does_it_operate.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.how_does_it_operate.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_to_get_crown.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.how_to_get_crown.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.what_is_hive.question)}</h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.what_is_hive.answer, < gen42_link >)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>
                            {t!(i18n, faq.how_to_play_with_friends.question)}
                        </h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.how_to_play_with_friends.answers.item1)}
                        </p>
                        <p class=paragraph_class>
                            {t!(i18n, faq.how_to_play_with_friends.answers.item2)}
                        </p>
                        <p class=paragraph_class>
                            {t!(i18n, faq.how_to_play_with_friends.answers.item3)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_to_check_stack.question)}</h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.how_to_check_stack.answers.item1)}
                        </p>
                        <p class=paragraph_class>
                            {t!(i18n, faq.how_to_check_stack.answers.item2)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>
                            {t!(i18n, faq.what_are_cornfirmation_modes.question)}
                        </h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.what_are_cornfirmation_modes.description)}
                            <ol class="list-decimal list-inside">
                                <li>{t!(i18n, faq.what_are_cornfirmation_modes.answers.item1)}</li>
                                <li>{t!(i18n, faq.what_are_cornfirmation_modes.answers.item2)}</li>
                                <li>{t!(i18n, faq.what_are_cornfirmation_modes.answers.item3)}</li>
                            </ol>
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.can_i_play_base_game.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.can_i_play_base_game.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.what_rating_system.question)}</h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.what_rating_system.answer, < glicko2_link >)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_to_recover_password.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.how_to_recover_password.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.why_basic_chat.question)}</h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.why_basic_chat.details)}
                            <ol class="list-decimal list-inside">
                                <li>{t!(i18n, faq.why_basic_chat.answers.item1)}</li>
                                <li>{t!(i18n, faq.why_basic_chat.answers.item2)}</li>
                                <li>{t!(i18n, faq.why_basic_chat.answers.item3)}</li>
                            </ol>
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.opponent_is_abusive.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.opponent_is_abusive.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>
                            {t!(i18n, faq.where_to_meet_other_players.question)}
                        </h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.where_to_meet_other_players.answer, < resources_link >)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>
                            {t!(i18n, faq.why_elo_has_questionmark.question)}
                        </h3>
                        <p class=paragraph_class>{t!(i18n, faq.why_elo_has_questionmark.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.why_not_in_top_players.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.why_not_in_top_players.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.how_to_review_games.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.how_to_review_games.answer)}</p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.where_to_learn_more.question)}</h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.where_to_learn_more.answer, < resources_link >)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>
                            {t!(i18n, faq.can_i_set_up_a_tournament.question)}
                        </h3>
                        <p class=paragraph_class>
                            {t!(i18n, faq.can_i_set_up_a_tournament.answer)}
                        </p>
                    </div>
                    <div class=div_class>
                        <h3 class=header_class>{t!(i18n, faq.can_i_play_with_bots.question)}</h3>
                        <p class=paragraph_class>{t!(i18n, faq.can_i_play_with_bots.answer)}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
