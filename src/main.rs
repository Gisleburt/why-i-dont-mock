#![allow(non_snake_case)]

mod impress;
mod code;
mod slides;

use dioxus::prelude::*;
use crate::{
    code::HighlightInit,
    impress::{ImpressGroup, ImpressInit, Step}
};
use crate::slides::intro::Intro;
use crate::slides::slide::Slide;
use crate::slides::unit_tests::{AreAwesome, AreLoved, UnitTestExample};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let y_step = 720 + 100;
    let x_step = 1280 + 100;

    let row = |row: i32| row * y_step;
    let col = |col: i32| col * x_step;

    cx.render(rsx! {
        ImpressGroup {
            Slide { Intro{}, name: "intro", y: row(0), x: col(0) }

            Slide { AreAwesome{}, name: "unit-tests-are-awesome", y: row(1), x: col(0) }

            Slide { AreLoved{}, name: "unit-tests-are-love", y: row(1), x: col(1),}

            Slide { UnitTestExample{} name: "unit-test-example", y: row(1), x: col(2) }

            Step { name: "external-systems", x: 0* x_step, y: 2 * y_step,
                h2 { "Talking to External Systems" }
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
        HighlightInit {}
    })
}
