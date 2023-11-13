#![allow(non_snake_case)]

mod code;
mod impress;
mod mermaid;
mod pos;
mod slides;

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

            Step { DddIsAwesome{}, name: "ddd-is-awesome", y: new_row(), x: (x_step * 5) - col(), rotate_z: 180 }

            Step { HexagonalArchitecture{}, name: "hex-arch", y: row(), x: (x_step * 5) - col(), rotate_z: 180 }

            Step { PortsAndAdaptors{}, name: "ports-adaptors", y: row(), x: (x_step * 5) - col(), rotate_z: 180 }

            Step { OurPortAndAdaptor{}, name: "ddd-example", y: row(), x: (x_step * 5) - col(), rotate_z: 180 }

            Step { StubAdaptors{}, name: "stub-adaptors", y: new_row(), x: col() }

            Step { StubAdaptorExample{}, name: "stub-adaptor-example", y: row(), x: col() }

            Step { StubAdaptorInTest{}, name: "stub-adaptors-in-tests", y: row(), x: col() }

            Step { MocksReview{}, name: "stub-adaptors-vs-mocks", y: row(), x: col() }

            Step { TestAllTheThings{}, name: "test-all-the-things", y: row(), x: col() }

            Step { IntegrationTestsForStubAdaptors{}, name: "int-tests-for-stubs", y: row(), x: col() }
        }
        ImpressInit {}
        HighlightInit {}
        MermaidInit {}
    })
}
