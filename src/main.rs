#![allow(non_snake_case)]

mod code;
mod impress;
mod mermaid;
mod pos;
mod slides;

use crate::{
    code::HighlightInit,
    impress::{ImpressGroup, ImpressInit, Step},
    mermaid::MermaidInit,
    pos::AutoReposition,
    slides::{
        di::{
            DependencyInjection, DependencyInjectionExample, MockingExample, MockingIsNotAwesome,
        },
        external_systems::{
            Communicating, ExternalSystems, IntegrationTestExample, IntegrationTestsAreAwesome,
        },
        intro::Intro,
        unit_tests::{AreAwesome, AreLoved, UnitTestExample},
    },
};
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let height = 720;
    let width = 1280;
    let margin = 100;
    let height_buffer = (margin / 10) * 9 * 3;
    let width_buffer = (margin / 10) * 16 * 3;
    let y_step = height + margin;
    let x_step = width + margin;

    let auto_pos = RefCell::new(AutoReposition::new());

    let row = || auto_pos.borrow_mut().row() * y_step;
    let new_row = || auto_pos.borrow_mut().new_row() * y_step;
    let col = || auto_pos.borrow_mut().col() * x_step;

    cx.render(rsx! {
        ImpressGroup { width: width + width_buffer, height: height + height_buffer,
            Step { Intro{}, name: "intro", y: new_row(), x: col() }

            Step { AreAwesome{}, name: "unit-tests-are-awesome", y: new_row(), x: col() }

            Step { AreLoved{}, name: "unit-tests-are-love", y: row(), x: col(),}

            Step { UnitTestExample{} name: "unit-test-example", y: row(), x: col() }

            Step { ExternalSystems{}, name: "external-systems", y: new_row(), x: col() }

            Step { Communicating{}, name: "external-systems-interface", y: row(), x: col() }

            Step { IntegrationTestsAreAwesome{}, name: "int-tests-are-awesome", y: row(), x: col() }

            Step { IntegrationTestExample{}, name: "integration-test-example", y: row(), x: col() }

            Step { DependencyInjection{}, name: "di", y: new_row(), x: col() }

            Step { DependencyInjectionExample{} , name: "di-example", y: row(), x: col() }

            Step { MockingIsNotAwesome{}, name: "mocking", y: row(), x: col() }

            Step { MockingExample{}, name: "mocking-example", y: row(), x: col() }

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
        MermaidInit {}
    })
}
