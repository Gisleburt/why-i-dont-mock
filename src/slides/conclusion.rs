use crate::impress::Notes;
use dioxus::prelude::*;
use indoc::indoc;

pub fn Conclusion(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Conclusion" }
        ul {
            ol { "Mocks are brittle and messy" }
            ol { "DDD paradigms can extend to testing" }
            ol { "Daniel Loves Tests" }
        }
        Notes { 
            p { "What did we learn?" }
            p { "Mocks are, I think, brittle and messy " }
            p { "Domain Driven Development paradigms can extend to testing" }
            p { "I love testing" }
        }
    ))
}

pub fn Bonus(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Bonus usage" }
        Notes { 
            p {
                indoc! { "
                    A minor aside here, I wanted to add that Stub Adaptors actually _can_ be useful
                    _in_ production code. At the company where I originally learned this technique
                    we also used these adaptors for scripts that needed a store, but did not need a
                    database connection, for example, doing things like processing CSVs.
                " }
            }
            p {
                indoc! { "
                    Since we already wrote the adaptors for tests, and were testing them themselves,
                    we effectively got this behaviour for free.
                " }
            }
        }
    ))
}

pub fn ThankYou(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Thank you" }
        p {
            "Find these slides at "
            a { href: "https://why-i-dont-mock.danielmason.com", "why-i-dont-mock.danielmason.com" }
        }
        p {
            "Find the slide code at "
            a { href: "https://github.com/gisleburt/why-i-dont-mock",
                "github.com/gisleburt/why-i-dont-mock"
            }
        }
        p { "Any Questions?" }
        Notes { 
            p { "Thank you for your time, I've been Daniel" }
            p { "If you want to look through these slides later, you can find them on" }
            p { "why i dont mock dot daniel mason dot come" }
            p { "If you want to see how I made the slides, you can find them on my github" }
        }
    ))
}
