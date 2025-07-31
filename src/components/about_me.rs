use yew::prelude::*;

#[function_component]
pub(crate) fn AboutMe() -> Html {
    html! {
        <>
            // Hide contact and resume sections
            <section class="view" id="sec:aboutme" style="display: block">
                // Left side of the page
                <div class="col">
                        <section class = "sub">
                        <h2 id="unders">{"Who am I?"}</h2>
                        <h4><p>{"TBD"}</p></h4>
                    </section>
                </div>
                // Right side of the page
                <div class="col">
                    <section class = "sub">
                        <h2 id="unders">{"What are my interests?"}</h2>
                        <h4><p>{"TBD"}</p></h4>

                        <h4><p>{"TBD"} </p></h4>
                    </section>
                </div>
            </section>
        </>
    }
}

