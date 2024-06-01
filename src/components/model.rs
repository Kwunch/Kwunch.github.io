
use yew::prelude::*;

pub(crate) struct Model;

//Create ModelProps with CallBack to ShowComponent on App
#[derive(Clone, Properties, PartialEq)]
pub(crate) struct ModelProps {
    pub(crate) on_click: Callback<String>,
}

impl Component for Model {
    type Message = ();
    type Properties = ModelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                <header>
                    <h1>{"Kwunch's Website"}</h1>
                        <h3>{"Written in Rust with Yew"}</h3>
                            <nav>
                                <h2>
                                    // Link to AboutMe page, when clicked, show AboutMe component
                                    <a class="heading" href="#aboutme" onclick={
                                        ctx.props().on_click.reform(
                                            move |_| "aboutme".to_string().clone()
                                        )
                                    }>{"About Me"}</a>

                                    // Link to Projects page, when clicked, show Projects component
                                    <a class="heading" href="#other-projects" onclick={
                                       ctx.props().on_click.reform(
                                            move |_| "projects".to_string().clone()
                                        )
                                    }>{"KSLS Compression and Other Projects"}</a>

                                    // Link to GitHub page, when clicked open GitHub
                                    <a class="heading" href="https://GitHub.com/Kwunch" target="_blank">
                                        {"GitHub"}
                                    </a>

                                    // Link to Rust Tutorials page, when clicked, show Tutorials component
                                    <a class="heading" href="#rust-tutorials" onclick={
                                       ctx.props().on_click.reform(
                                            move |_| "tutorials".to_string().clone()
                                        )
                                    }>{"Rust Tutorials"}</a>

                                    // Link to Resume page, when clicked, show Resume component
                                    <a class="heading" href="#resume" onclick={
                                        ctx.props().on_click.reform(
                                            move |_| "resume".to_string().clone()
                                        )
                                    }>{"Resume"}</a>

                                    // Link to Contact page, when clicked, show Contact component
                                    <a class="heading" href="#contact" onclick={
                                        ctx.props().on_click.reform(
                                            move |_| "contact".to_string().clone()
                                        )
                                    }>{"Contact"}</a>
                                </h2>
                            </nav>
                </header>
        }
    }
}



