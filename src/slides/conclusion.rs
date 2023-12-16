use crate::impress::Notes;
use dioxus::prelude::*;
use indoc::indoc;

pub fn Conclusion(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Conclusion" }
        ul {
            li { span { class: "hide", "💔 Mocks are brittle and messy" } }
            li { span { class: "hide", "🤝 DDD paradigms can extend to testing" } }
            li { span { class: "hide", "❤️ Daniel Loves Tests" } }
        }
        Notes { p { "What did we learn?" } }
    ))
}

pub fn ConclusionOne(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Conclusion" }
        ul {
            li { "💔 Mocks are brittle and messy" }
            li { span { class: "hide", "🤝 DDD paradigms can extend to testing" } }
            li { span { class: "hide", "❤️ Daniel Loves Tests" } }
        }
        Notes { p { "Mocks are brittle and messy " } }
    ))
}

pub fn ConclusionTwo(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Conclusion" }
        ul {
            li { "💔 Mocks are brittle and messy" }
            li { "🤝 DDD paradigms can extend to testing" }
            li { span { class: "hide", "❤️ Daniel Loves Tests" } }
        }
        Notes { p { "Domain Driven Development paradigms can extend to testing" } }
    ))
}

pub fn ConclusionThree(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Conclusion" }
        ul {
            li { "💔 Mocks are brittle and messy" }
            li { "🤝 DDD paradigms can extend to testing" }
            li { "❤️ Daniel Loves Tests" }
        }
        Notes {
            p { "I love testing" }
            p { "But wait... that's not all these little scoundrels can be used for" }
        }
    ))
}

pub fn BonusLocalDev(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Bonus Usage 1: Local Dev" }
        img {
            src: "images/local-dev.png",
            alt: "Local dev with microservices and database",
            style: "width: 800px"
        }
        Notes {
            p { "First and perhaps most obviously relates to local development" }
            p { "In a microservice ecosystem we try to build many small systems" }
            p { "However because these systems depend on each other what do you do locally?" }
            p { "We could run each service we depend on locally" }
            p { "And each service they depend on and so on and so on... costly" }
            p { "Or we could connect to staging environments... risky" }
            p { "Or..." }
        }
    ))
}

pub fn BonusLocalDevStub(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Bonus Usage 1: Local Dev" }
        img {
            src: "images/local-dev-stub.png",
            alt: "Local dev with microservices and database",
            style: "width: 800px"
        }
        Notes {
            p {
                p { "Stub Adaptors!" }
                p { "We tested them against the real adaptors" }
                p { "So we know they behave the same way" }
                p { "So why not swap them out?" }
                p {
                    indoc! { "
                        One way to do this is to look for the connection settings and if they're not
                        there use the Stub Adaptor.
                    " }
                }
                p {
                    indoc! { "
                        A word of warning though, you should also check to make sure this is only
                        happening in a development environment because, as I said, running these in
                        production would be bad    
                    " }
                }
            }
        }
    ))
}

pub fn BonusProduction(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Bonus Usage 2: Production" }
        img {
            class: "hide",
            src: "images/cronjob.png",
            alt: "Local dev with microservices and database",
            style: "width: 600px"
        }
        Notes {
            p { "I lied!" }
            p { "You can, and I have, totally used these in production" }
            p { "But maybe not how you'd expect" }
            p { "I found a use for these in cronjobs" }
        }
    ))
}

pub fn BonusProductionReveal(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Bonus Usage 2: Production" }
        img {
            src: "images/cronjob.png",
            alt: "Local dev with microservices and database",
            style: "width: 600px"
        }
        Notes {
            p { "In this case we were reading csvs from AWS S3" }
            p { "We needed to report on the data in the CSV" }
            p { "And once complete move the CSV to another bucket" }
            p {
                indoc! { "
                    The data was of a type we did have a database store for but for the purpose of
                    these reports we didn't need to store the CSV data
                " }
            }
            p { "We did however, need to process it in a way our stores already provided" }
            p { "So we just used the stub adaptors we already had!" }
        }
    ))
}

pub fn ThankYou(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Thank you" }
        p { "Any Questions?" }
        Notes { p { "Any Questions?" } }
    ))
}

pub fn WhereToFind(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
            "Find these slides at "
            a { href: "https://danielmason.com/why-i-dont-mock", "danielmason.com/why-i-dont-mock" }
        }
        Notes {
            p { "Thank you for your time, I've been Daniel" }
            p { "If you want to look through these slides later, you can find them on" }
            p { "daniel mason dot com slash why i dont mock" }
        }
    ))
}
