#![allow(non_snake_case)]

mod code;
mod impress;
mod mermaid;
mod pos;
mod slides;

use crate::slides::conclusion::{Bonus, Conclusion, ThankYou};
use crate::slides::ddd::{
    DddIsAwesome, HexagonalArchitecture, OurPortAndAdaptor, PortsAndAdaptors,
};
use crate::slides::stub_adaptors::{
    IntegrationTestsForStubAdaptors, MocksReview, StubAdaptorExample, StubAdaptorInTest,
    StubAdaptors, TestAllTheThings,
};
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
            Step { name: "intro", y: new_row(), x: col(), Intro {} }

            Step { name: "unit-tests-are-awesome", y: new_row(), x: col(), AreAwesome {} }

            Step { name: "unit-tests-are-love", y: row(), x: col(), AreLoved {} }

            Step { name: "unit-test-example", y: row(), x: col(), UnitTestExample {} }

            Step { name: "external-systems", y: new_row(), x: col(), ExternalSystems {} }

            Step { name: "external-systems-interface", y: row(), x: col(), Communicating {} }

            Step { name: "int-tests-are-awesome", y: row(), x: col(), IntegrationTestsAreAwesome {} }

            Step { name: "integration-test-example", y: row(), x: col(), IntegrationTestExample {} }

            Step { name: "di", y: new_row(), x: col(), DependencyInjection {} }

            Step { name: "di-example", y: row(), x: col(), DependencyInjectionExample {} }

            Step { name: "mocking", y: row(), x: col(), MockingIsNotAwesome {} }

            Step { name: "mocking-example", y: row(), x: col(), MockingExample {} }

            Step {
                name: "ddd-is-awesome",
                y: new_row(),
                x: (x_step * 5) - col(),
                rotate_z: 180,
                DddIsAwesome {}
            }

            Step { name: "hex-arch", y: row(), x: (x_step * 5) - col(), rotate_z: 180, HexagonalArchitecture {} }

            Step {
                name: "ports-adaptors",
                y: row(),
                x: (x_step * 5) - col(),
                rotate_z: 180,
                PortsAndAdaptors {}
            }

            Step {
                name: "ddd-example",
                y: row(),
                x: (x_step * 5) - col(),
                rotate_z: 180,
                OurPortAndAdaptor {}
            }

            Step { name: "stub-adaptors", y: new_row(), x: col(), StubAdaptors {} }

            Step { name: "stub-adaptor-example", y: row(), x: col(), StubAdaptorExample {} }

            Step { name: "stub-adaptors-in-tests", y: row(), x: col(), StubAdaptorInTest {} }

            Step { name: "stub-adaptors-vs-mocks", y: row(), x: col(), MocksReview {} }

            Step { name: "test-all-the-things", y: row(), x: col(), TestAllTheThings {} }

            Step { name: "int-tests-for-stubs", y: row(), x: col(), IntegrationTestsForStubAdaptors {} }

            Step { name: "conclusion", y: new_row(), x: col(), Conclusion {} }

            Step { name: "bonus-use", y: row(), x: col(), Bonus {} }

            Step { name: "thank-you", y: row(), x: col(), ThankYou {} }
        }
        ImpressInit {}
        HighlightInit {}
        MermaidInit {}
    })
}
