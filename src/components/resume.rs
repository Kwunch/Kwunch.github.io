use yew::prelude::*;

#[function_component]
pub(crate) fn Resume() -> Html {
    html! {
        <section class="view" id="sec:resume">
          <h2 id="genny">{"General Information"}</h2>
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
              being my GOTOs."}</li></h4>
              <ul class="about separate">
                <li>
                  <ul>
                    <li>{"Rust"}</li>
                    <li>{"Eight Years Experience"}</li>
                    <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"C/C++"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                      <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Swift"}</li>
                    <li>{"Four Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"GO"}</li>
                    <li>{"Five Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Python"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                    <li>{"Mastery"}</li>
                    <li><h5>{"Note: I technically still use python regularly for scripting. However, \
                    It has become more of what I would consider a pseudo language. I use it \
                    for quick scripts and to test out ideas that I may later implement in a \
                    more robust language. At this point I do not use it for anything serious. "}
                    </h5></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Assembly: ARM, MIPS, x86"}</li>
                    <li>{"Mastery in ARM"}</li>
                    <li>{"Proficient in MIPS and x86"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"SQL, PostgreSQL, MongoDB"}</li>
                    <li>{"Proficient in All"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Shell Scripting"}</li>
                    <li>{"Bash, Zsh, Fish"}</li>
                    <li>{"Windows Batch"}</li>
                    <li>{"Proficient in All"}</li>
                  </ul>
                </li>
              </ul>
            </section>
          </div>
          <div class="col">
            <h4><p>{"For a more detailed resume, please contact me."}</p></h4>
            <h4><p>{"To see my projects, please visit my GitHub."}</p></h4>
            <section class="sub">
              <h3>{"Other Languages"}</h3>
              <h4><li>{"These are languages I have used in the past and have a good understanding of. \
              Though my use of them has decreased overtime or was miniscule to begin with. Some \
              I still maintain a mastery in but you will see very few, if any, projects \
               utilizing them."}</li></h4>
              <ul class="about separate">
                <li>
                  <ul>
                    <li>{"Java"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                    <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"React"}</li>
                    <li>{"Three Years Experience"}</li>
                    <li>{"Proficient"}</li>
                    <li><h5>{"Note: I have not used React in a while. I have been using Yew \
                    for all of my front end development. I have no problem using React, but \
                    I may need to brush up on it."}</h5></li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Ruby"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                      <li>{"Mastery"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"C#"}</li>
                    <li>{"Six Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"R"}</li>
                    <li>{"Five Years Experience"}</li>
                      <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Kotlin"}</li>
                    <li>{"Six Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Perl"}</li>
                    <li>{"Eight Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"MatLab"}</li>
                    <li>{"Three Years Experience"}</li>
                    <li>{"Proficient"}</li>
                  </ul>
                </li>
                <li>
                  <ul>
                    <li>{"Arduino"}</li>
                    <li>{"Seven Years Experience"}</li>
                    <li>{"Mastery"}</li>
                    <li><h5>{"Note: Still somewhat use regularly, but is being phased out \
                    by Embedded Rust and Raspberry Pi."}</h5></li>
                  </ul>
                </li>
              </ul>
            </section>
          </div>
        </section>
    }
}