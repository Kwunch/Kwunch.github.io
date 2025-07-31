use yew::prelude::*;

#[function_component]
pub(crate) fn Contact() -> Html {
    html! {
        <section class="view" id="sec:contact">
          <h3>{"People are welcome to reach out to me any \
          time using one of the following methods."}</h3>
            <div class="col">
                <section class="sub">
                  <h3>{"Email"}</h3>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <li><a href="mailto:t1bur0n@proton.me" id="emails">
                            {"t1bur0n@proton.me"}</a></li>
                      </ul>
                    </li>
                  </ul>
                </section>
            </div>
            <div class="col">
                <section class="sub">
                  <h3>{"Socials"}</h3>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <h4>{"LinkedIn"}</h4>
                        <li>{"TODO"}</li>
                      </ul>
                    </li>
                  </ul>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <h4>{"Reddit"}</h4>
                        <li>{"TODO"}</li>
                      </ul>
                    </li>
                  </ul>
                </section>
                <section class="sub">
                  <h3>{"Other"}</h3>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <h4>{"Discord"}</h4>
                        <li>{".Kwunch"}</li>
                      </ul>
                    </li>
                  </ul>
                </section>
            </div>
        </section>
    }
}
