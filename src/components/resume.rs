use yew::prelude::*;

#[function_component]
pub(crate) fn Resume() -> Html {
    html! {
        <section class="view" id="sec:resume">
          <h2 id="genny">{"General Information"}</h2>
          // Viewable hyperlink to view PDF version of resume
          // Link is https://drive.proton.me/urls/ZNW5NYVAFR#ypYOygHRhAki
          <h3><a id="pdf_resume" href="https://drive.proton.me/urls/ZNW5NYVAFR#ypYOygHRhAki" target="_blank">
                {"View PDF Version"}</a></h3>
          <div class="col">
            <section class="sub">
              <h3>{"My Education"}</h3>
              <ul class="about separate">
                <li>
                  <ul>
                    <li>{"BS Computer Science"}</li>
                    <li id="school"><a href="https://www.pitt.edu/about" id="school">
                        {"University of Pittsburgh"}</a></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"BA Mathematics"}</li>
                    <li id="school"><a href="https://www.pitt.edu/about" id="school">
                        {"University of Pittsburgh"}</a></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"BS Quantum Computing and Physics"}</li>
                    <li id="school"><a href="https://www.pitt.edu/about" id="school">
                        {"University of Pittsburgh"}</a></li>
                  </ul>
                </li>
              </ul>
            </section>
            <section class="sub">
              <h3>{"Main Languages"}</h3>
              <h4><li>{"These are languages that I regularly use. With Rust and C \
              being my primary languages."}</li></h4>
              <ul class="about separate">
                <li>
                  <ul>
                    <li id="language">{"Rust"}</li>
                    <li>{"Eight Years Experience"}</li>
                    <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"C/C++"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                      <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Swift"}</li>
                    <li>{"Four Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"GO"}</li>
                    <li>{"Five Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Python"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                    <li>{"Mastery"}</li>
                    <li><h5>{"Note: I still use python regularly for scripting, however, \
                    it has become more of what I would consider a pseudo language. I use it \
                    for quick scripts and to test out ideas that I may later implement in a \
                    more robust language."}
                    </h5></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Assembly: ARM, MIPS, x86"}</li>
                    <li>{"Mastery in ARM"}</li>
                    <li>{"Proficient in MIPS and x86"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"SQL, PostgreSQL, MongoDB"}</li>
                    <li>{"Proficient in All"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Shell Scripting"}</li>
                    <li id="language">{"Bash, Zsh, Fish"}</li>
                    <li id="language">{"Windows Batch"}</li>
                    <li>{"Proficient in All"}</li>
                  </ul>
                </li>
              </ul>
            </section>
          </div>
          <div class="col">
            <h4><p>{"To see my projects, please visit my GitHub."}</p></h4>
            <section class="sub">
              <h3>{"Other Languages"}</h3>
              <h4><li>{"These are languages I have used in the past and have a good understanding of. \
              I maintain a mastery in them but you will see very few, if any, projects \
              utilizing them."}</li></h4>
              <ul class="about separate">
                <li>
                  <ul>
                    <li id="language">{"Java"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                    <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"React"}</li>
                    <li>{"Four Years Experience"}</li>
                    <li>{"Proficient"}</li>
                    <li><h5>{"Note: I have not used React in a while. I have been using Yew \
                    for all of my front end development. I have no problem using React, but \
                    I may need to brush up on it."}</h5></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Ruby"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                      <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"C#"}</li>
                    <li>{"Six Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"R"}</li>
                    <li>{"Five Years Experience"}</li>
                      <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Kotlin"}</li>
                    <li>{"Six Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Perl"}</li>
                    <li>{"Eight Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"MatLab"}</li>
                    <li>{"Three Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Arduino"}</li>
                    <li>{"Seven Years Experience"}</li>
                    <li>{"Mastery"}</li>
                    <li><h5>{"Note: I still use this somewhat use regularly, but it is being phased out \
                    by Embedded Rust."}</h5></li>
                  </ul>
                </li>
              </ul>
            </section>
          </div>
        </section>
    }
}