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
                        <h4><p>{"I am a student at the University of Pittsburgh pursuing as BS in \
                        Computer Science, a BS in Qauntum Computing and Physics, and a BA in Mathematics. \
                        My anticipated graduation date is Summer 2025. With my three degrees my primary pursuit \
                        is Software Engineering with a focus on Systems and Systems Design, \
                        however, I'm open to all opportunities. I have a mastery in multiple programming languages, \
                        which can be referenced in the Resume section. Rust is my perferred language for \
                        professional and personal applications. I pride myself in my work on programming languages and coding, as well as \
                        understanding the underlying concepts on the creation and design of languages."}</p></h4>
                    </section>
                </div>
                // Right side of the page
                <div class="col">
                    <section class = "sub">
                        <h2 id="unders">{"What are my interests?"}</h2>
                        <h4><p>{"Programming is not just career pursuit for me, it is also one of my primary hobbies. \
                         I am consistently expanding my GitHub with new projects and ideas. My pique interests at the moment \
                         involve System Design including embedded systems and logic controllers, \
                         as well as encryption and compression algorithms. \
                         I have done projects working on Arduino, Raspberry Pi and numerous other logic controllers. \
                         I have also done projects working with the Linux Kernel. "}</p></h4>

                        <h4><p>{" While the majority of my projects may be in C or Rust, I have a firm understanding
                        of OOP languages such as Java, Ruby, and Python. I also have sufficient experience doing web development
                        or front end development in languages such as React. "} </p></h4>

                    </section>
                </div>
            </section>
        </>
    }
}

