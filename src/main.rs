#![allow(non_snake_case)]

mod impress;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::impress::ImpressGroup::ImpressGroup;
use crate::impress::ImpressInit::ImpressInit;
use crate::impress::Step::Step;
use crate::impress::Notes::Notes;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let x_step = 1000;
    let y_step = 800;
    cx.render(rsx! {
        ImpressGroup {
            Step { name: "intro", x: 0 * x_step, y: 0 * y_step,
                h1 { "Why I don't mock 💩" }
                h2 { "Daniel Mason" }
                Notes {
                    p { "Hi, in case you don't know I'm Daniel and I love testing" }
                    p { "But testing interconnected things gets complex" }
                    p { "So I want to show you how I manage that complexity" }
                }
            }

            Step { name: "unit-tests-are-awesome", x: 0 * x_step, y: 1 * y_step,
                h2 { "Unit Tests are Awesome" }
                Notes {
                    p { "Those that know me, know that I _love_ tests" }
                    p {
                        "I don't do well with compliments so when a human tells me my code is ",
                        "good, I don't necessarily believe them, but when a computer tells me my ",
                        "code is good, that feels good."
                    }
                }
            }

            Step { name: "unit-test-example", x: 1 * x_step, y: 1 * y_step,
                h3 { "Example Code:" }
                code { "" }
                h3 { "Example Unit Test:" }
            }

            Step { name: "integration-tests-are-awesome", x: 0 * x_step, y: 2 * y_step,
                h2 { "Integration Tests are Awesome" }
            }

            Step { name: "integration-test-example", x: 1 * x_step, y: 2 * y_step,
                h3 { "Example Code:" }
                h3 { "Example Integration Test:" }
            }

            Step { name: "dependency-injection-is-awesome", x: 0 * x_step, y: 3 * y_step,
                h2 {"Dependency Injection is Awesome"}
            }

            Step { name: "dependency-injection-example", x: 1 * x_step, y: 3 * y_step,
                h3 { "Example Code:" }
                h3 { "Example Test" }
                p { "Wait... does this now need to be an integration test" }
            }

            Step { name: "mocking", x: 0 * x_step, y: 4 * y_step,
                h2 {"Mocking is... well... it exists"}
            }

            Step { name: "mocking-example", x: 1 * x_step, y: 4 * y_step,
                h3 { "Example Code:" }
                h3 { "Example Mock" }
            }

            Step { name: "mocking-problem", x: 2 * x_step, y: 4 * y_step,
                h3 { "The problem with mocks" }
            }

            Step { name: "ddd-is-awesome", x: 0 * x_step, y: 5 * y_step,
                h2 { "Domain Driven Development to the rescue!" }
            }

            Step { name: "ddd-explainer", x: 1 * x_step, y: 5 * y_step,
                h3 { "What is DDD" }
                p { "Ports and Adaptors" }
            }

            Step { name: "ddd-test-adaptor", x: 2 * x_step, y: 5 * y_step,
                h3 { "Test Adaptors" }
            }

            Step { name: "ddd-integration-tests", x: 3 * x_step, y: 5 * y_step,
                h3 { "DDD Integration tests" }
            }

            Step { name: "ddd-in-tests", x: 4 * x_step, y: 5 * y_step,
                h3 { "Test adaptor usage" }
            }

            Step { name: "ddd-unleashed", x: 0 * x_step, y: 6 * y_step,
                h2 { "Why stop there" }
            }

            Step { name: "ddd-adaptors-in-the-wild", x: 1 * x_step, y: 6 * y_step,
                h3 { "Expanded use cases" }
            }
        }
        ImpressInit {}
    })
}
