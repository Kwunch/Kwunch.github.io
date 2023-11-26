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
                        <li><h4>{"Professional Development"}</h4></li>
                        <li><a href="mailto:kwunch_devs.moqsz@passmail.net" id="emails">
                            {"kwunch_devs.moqsz@passmail.net"}</a></li>
                      </ul>
                    </li>
                    <li>
                      <ul>
                        <li><h4>{"General Inquiries"}</h4></li>
                        <li><a href="mailto:Kwunch@outlook.com" id="emails">
                            {"Kwunch@outlook.com"}</a></li>
                      </ul>
                    </li>
                  </ul>
                </section>
                <section class="sub">
                  <h3>{"Phone"}</h3>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <li><h4>{"Main Phone"}</h4></li>
                        <li>{"+1 (412)-606-4704"}</li>
                      </ul>
                    </li>
                    <li>
                      <ul>
                        <li><h4>{"Backup"}</h4></li>
                        <li>{"TODO"}</li>
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
                        <li><h4>{"LinkdIn"}</h4></li>
                        <li>{"TODO"}</li>
                      </ul>
                    </li>
                  </ul>
                  <ul class="about separate">
                    <li>
                      <ul>
                        <li><h4>{"Reddit"}</h4></li>
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
                        <li><h4>{"Discord"}</h4></li>
                        <li>{"Kwunch#8600"}</li>
                      </ul>
                    </li>
                    <li>
                      <ul>
                        <li><h4>{"IRC"}</h4></li>
                        <li>{"TODO"}</li>
                      </ul>
                    </li>
                  </ul>
                </section>
            </div>
        </section>
    }
}