use yew::prelude::*;

#[function_component]
pub(crate) fn Papers() -> Html {
    html! {
        <section class="view" id="sec:papers" alt="sections">
            <h3>{"Here are some of the papers that I have written."}</h3>
            <div class="col">
            <section class="sub">
              <h3>{"Public Key Cryptography Paper"}</h3>
              <ul class="about separate">
                <li>
                  <ul>
                    <li><a id="pdf_paper" href="https://drive.proton.me/urls/GME45A2B4R#ICJlo0k3GErZ" target="_blank">
                {"View PDF Here"}</a></li>
                    <li id="pdf_description">{"This research paper covers the math behind the creation of Public Key Cryptography.
                          Also known as Asymmetric Key Encryption. This method uses a dual key method consisting
                          of a public key and a private key. Where contents are encrypted using the former and
                          decrypted using the latter. Due to the nature of this paper and my role as a Computer
                          Scientist and a Mathematician. This paper will include programs written in Rust showing
                          how certain calculations are computed as well as the mathematical formulas behind those
                          implementations."}</li>
                  </ul>
                </li>
              </ul>
            </section>
            </div>
            <div class="col">
            <section class="sub">
              <h3>{"OOP in Rust"}</h3>
              <ul class="about separate">
                <li>
                  <ul>
                    <li><a id="pdf_paper" href="https://github.com/Kwunch/OOP_In_Rust/blob/main/README.md" target="_blank">
                {"View MD Here"}</a></li>
                    <li id="pdf_description">{"This paper offers a brief introduction to the Rust programming language as 
                    well as an introduction to the OOP paradigm. It shows how Rust is unable to achieve surface level OOP, and then
                    shows how one would 'mimick' OOP in Rust using standard data structures."}</li>
                  </ul>
                </li>
              </ul>
            </section>
            </div>
            <script type="module" src="https://md-block.verou.me/md-block.js"></script>
        </section>
    }
}