#![allow(non_snake_case)]

mod impress;
mod code;
mod slides;
mod pos;

use dioxus::prelude::*;
use crate::{
    pos::AutoReposition,
    code::HighlightInit,
    impress::{ImpressGroup, ImpressInit, Step},
    slides::intro::Intro,
    slides::unit_tests::{AreAwesome, AreLoved, UnitTestExample}
};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let y_step = 720 + 100;
    let x_step = 1280 + 100;

    let auto_pos = RefCell::new(AutoReposition::new());

    let row = || auto_pos.borrow_mut().row() * y_step;
    let new_row = || auto_pos.borrow_mut().new_row() * y_step;
    let col = || auto_pos.borrow_mut().col() * x_step;

    cx.render(rsx! {
        ImpressGroup {
            Step { Intro{}, name: "intro", y: new_row(), x: col() }

            Step { AreAwesome{}, name: "unit-tests-are-awesome", y: new_row(), x: col() }

            Step { AreLoved{}, name: "unit-tests-are-love", y: row(), x: col(),}

            Step { UnitTestExample{} name: "unit-test-example", y: row(), x: col() }

            Step { name: "external-systems", y: new_row(), x: col(),
                h2 { "External Systems" }
            }

            Step { name: "integration-tests-are-awesome", y: new_row(), x: col(),
                h2 { "Integration Tests are Awesome" }
            }

            Step { name: "integration-test-example", y: row(), x: col(),
                h3 { "Example Code:" }
                h3 { "Example Integration Test:" }
            }

            Step { name: "dependency-injection-is-awesome", y: new_row(), x: col(),
                h2 {"Dependency Injection is Awesome"}
            }

            Step { name: "dependency-injection-example", y: row(), x: col(),
                h3 { "Example Code:" }
                h3 { "Example Test" }
                p { "Wait... does this now need to be an integration test" }
            }

            Step { name: "mocking", y: new_row(), x: col(),
                h2 {"Mocking is... well... it exists"}
            }

            Step { name: "mocking-example", y: row(), x: col(),
                h3 { "Example Code:" }
                h3 { "Example Mock" }
            }

            Step { name: "mocking-problem", y: row(), x: col(),
                h3 { "The problem with mocks" }
            }

            Step { name: "ddd-is-awesome", y: new_row(), x: col(),
                h2 { "Domain Driven Development to the rescue!" }
            }

            Step { name: "ddd-explainer", y: row(), x: col(),
                h3 { "What is DDD" }
                p { "Ports and Adaptors" }
            }

            Step { name: "ddd-test-adaptor", y: row(), x: col(),
                h3 { "Test Adaptors" }
            }

            Step { name: "ddd-integration-tests", y: row(), x: col(),
                h3 { "DDD Integration tests" }
            }

            Step { name: "ddd-in-tests", y: row(), x: col(),
                h3 { "Test adaptor usage" }
            }

            Step { name: "ddd-unleashed", y: row(), x: col(),
                h2 { "Why stop there" }
            }

            Step { name: "ddd-adaptors-in-the-wild", y: row(), x: col(),
                h3 { "Expanded use cases" }
            }
        }
        ImpressInit {}
        HighlightInit {}
    })
}
