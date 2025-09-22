use yew::prelude::*;

#[function_component]
pub(crate) fn Resume() -> Html {
    html! {
        <section class="view" id="sec:resume">
          <h2 id="genny">{"General Information"}</h2>
          <div class="top_container">
          <div class="top_left_col">
            // Viewable hyperlink to view PDF version of resume
            // Link is https://drive.proton.me/urls/ZNW5NYVAFR#ypYOygHRhAki
            <h3 class="resume_headers"><a id="pdf_resume" href="https://drive.proton.me/urls/P0K8162VER#LIRno515cxTV" target="_blank">
                  {"View PDF Version"}</a></h3>
          </div>
          <div class="top_right_col">
            <h3 class="resume_headers">{"To see my projects, please visit my GitHub."}</h3>
          </div>
          </div>
          <div class="middle_container">
          <div class="middle_left">
            <section class="sub">
              <h3 class="resume_headers">{"My Education"}</h3>
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
              </ul>
            </section>
            </div>
            <div class="middle_right">
            <section class="sub">
                <h3 class="resume_headers">{"My Experience"}</h3>
                <ul class="about separate">
                  <li>
                    <ul>
                      <li id="language">{"Undergraduate Directed Research"}</li>
                      <li id="date_range">{"January 2023 - Present"}</li>
                      <h5 id="note">
                      {"
                         Working on a compression algorithm that I created with 3 professors, \
                         2 in Pitt SCi and 1 in Pitt Math Department. I am doing the research \
                         and they are there as mentors guiding me so I can eventually create \
                         a published paper and patent it which is what they think I should do \
                         with it.
                      "}</h5>
                    </ul>
                  </li>
                  <li>
                    <ul>
                      <li id="language">{"Undergraduate Research Assistant"}</li>
                      <li id="date_range">{"August 2023 - December 2023"}</li>
                      <h5 id="note">
                      {"
                         Assisted a professor and worked on GPS signaling with drones which \
                         involved embedded programming in C and Rust.
                      "}</h5>
                    </ul>
                  </li>
                  <li>
                    <ul>
                      <li id="language">{"Undergraduate Research Assistant"}</li>
                      <li id="date_range">{"August 2023 - May 2024"}</li>
                      <h5 id="note">
                      {"
                         Assisted a professor in research on a second and separate
                         compression algorithm which specialized in
                         compressing DBs and their data. During this, with his input
                         I created RandomSQL (see GitHub) which can generate tens of
                         thousands of tuples for every table in a schema in a matter of
                         minutes.
                      "}</h5>
                    </ul>
                  </li>
                  <li>
                    <ul>
                      <li id="language">{"Undergraduate Teaching Assistant"}</li>
                      <li id="date_range">{"January 2021 - December 2023"}</li>
                      <h5 id="note">
                      {"
                         I was a TA for the following classes;
                         Assembly Programming, Introduction to Systems Software,
                         Data Structures 1 and 2, and Intro to Java.
                      "}</h5>
                    </ul>
                  </li>
                </ul>
            </section>
            </div>
            </div>
            <div class="bottom_container">
            <div class="bottom_left">
            <section class="sub">
              <h3 class="resume_headers" id="mains">{"Main Languages"}</h3>
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
                    <li id="language">{"Ruby"}</li>
                    <li>{"More Than 10 Years Experience"}</li>
                      <li>{"Mastery"}</li>
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
                  </ul>
                </li>
                <li>
                  <ul>
                    <li id="language">{"Assembly: ARM, MIPS, x86"}</li>
                    <li>{"Mastery in ARM"}</li>
                    <li>{"Proficient in ARM and x86"}</li>
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
                    <li id="language">{"Bash Scripting"}</li>
                    <li id="language">{"Windows Batch"}</li>
                    <li>{"Proficient in Both"}</li>
                  </ul>
                </li>
              </ul>
            </section>
            </div>
            <div class="bottom_right">
            <section class="sub">
              <h3 class="resume_headers" id="others">{"Other Languages"}</h3>
              <h4><li>{"These are languages I have used in the past and have a good understanding of. \
              I maintain a solid foundation (or mastery depending on language) in them but you will see very few, if any, projects \
              utilizing them."}</li></h4>
              <ul class="about separate">
                <li>
                  <ul>
                    <li id="language">{"C#"}</li>
                    <li>{"Six Years Experience"}</li>
                      <li>{"Advanced"}</li>
                  </ul>
                </li>                
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
                    <li>{"Six Years Experience"}</li>
                    <li>{"Proficient"}</li>
                    <h5 id="note">{"Note: I have not used React in a while. I have been using Yew \
                    for all of my front end development. I have no problem using React, but \
                    it is not part of my daily use."}</h5>
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
                    <li id="language">{"R"}</li>
                    <li>{"Five Years Experience"}</li>
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
              </ul>
            </section>
          </div>
        </div>
        </section>
    }
}
