use yew::prelude::*;

#[function_component]
pub(crate) fn AboutMe() -> Html {
    html! {
        <>
            // Hide contact and resume sections
            <section class="view" id="sec:aboutme" style="display: block">
                <div class="col">
                        <section class = "sub">
                        <h2 id="unders">{"Who am I?"}</h2>
                        <h4><p>{"I am a dual degree student at the University of Pittsburgh studying \
                        Computer Science and Mathematics. I am currently a senior and will be graduating \
                        in May 2024. I am a Software Engineer with a focus on Systems/Systems Design. \
                        I am a proud Rustacean. While I have a mastery in many programming languages, \
                        all of which can be seen in the \"Resume\" section, Rust is my go to language for \
                        professional and personal applications (such as this website)."}</p></h4>

                        <h4><p>{"I pride myself on not just knowing programming languages and how to code, \
                        but also understanding the underlying concepts of the creation of the languages,\
                        the design of the languages, and the theory behind the languages. I enjoy learning \
                        the semantics of languages and how they are implemented. As well as the different \
                        paradigms of programming languages and how they can be used to solve problems."}</p></h4>
                    </section>
                </div>
                <div class="col">
                    <section class = "sub">
                        <h2 id="unders">{"What are my interests?"}</h2>
                        <h4><p>{"I personally program for fun all the time. My personal GitHub is always \
                        growing with new projects and ideas. I mainly enjoy working on projects that \
                        involve systems design/programming, encryption, compression, and low level \
                        programming (usually in ARM or embedded Rust). I have done projects \
                        working on Arduino, Raspberry Pi and numerous other logic controller.\
                        I have also done projects working with the Linux Kernel and Linux Kernel Modules. "}</p></h4>

                        <h4><p>{" While the majority of my projects may be in C or Rust, I have a firm understanding
                        of OOP languages such as Java, Ruby, and Python. I have no problem doing web development
                        or front end development in languages such as React or Python. "} </p></h4>

                    </section>
                </div>
            </section>
        </>
    }
}

