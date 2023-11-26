use gloo::utils::document;
use yew::prelude::*;
use crate::components::*;

pub(crate) struct App {
    sections: Vec<String>,
}

pub(crate) enum Msg {
    ShowComponent(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sections: vec![
                "aboutme".to_string(),
                "projects".to_string(),
                "tutorials".to_string(),
                "resume".to_string(),
                "contact".to_string(),
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowComponent(component_id) => {
                println!("Component ID: {}", component_id);
                // Check if component_id is valid
                // If valid, then show component and hide others
                // If invalid, then do nothing
                if let Some(component) = document().get_element_by_id(
                    format!("sec:{}", component_id).as_str()
                ) {
                    for s in &self.sections {
                        if *s != component_id {
                            let section = document().get_element_by_id(
                                format!("sec:{}", s).as_str()
                            ).unwrap();
                            section.set_attribute("style", "display: none").unwrap();
                        }
                    }
                    component.set_attribute("style", "display: block").unwrap();
                }
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click = ctx.link().callback(Msg::ShowComponent);
        html! {
            <body>
                <img id="pitt-rep" src="img/logo.png" alt="logo" class="logo" height="150" width="400"/>
                <img id="rust-crab" src="img/rust-mascot.png" alt="rust-mascot" class="rust-mascot" height="150" width="200"/>
                <model::Model on_click={on_click}/>
                <section id="info">
                    <about_me::AboutMe/>
                    <other_projects::Projects/>
                    <tutorials::Tutorials/>
                    <resume::Resume/>
                    <contacts::Contact/>
                </section>
            </body>

        }
    }
}