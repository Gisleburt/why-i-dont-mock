#![allow(non_snake_case)]

mod code;
mod impress;
mod pos;
mod slides;

use crate::slides::conclusion::{
    Bonus, Conclusion, ConclusionOne, ConclusionThree, ConclusionTwo, ThankYou, WhereToFind,
};
use crate::slides::ddd::{
    AdaptorExample, DddIsAwesome, HexagonalArchitecture, PortExample, PortsAndAdaptors,
};
use crate::slides::di::{DependencyInjectionIsAwesome, MockingIs};
use crate::slides::external_systems::IntegrationTests;
use crate::slides::stub_adaptors::{
    IntegrationTestsForStubAdaptors, MocksReview, MocksReviewOne, MocksReviewTwo,
    StubAdaptorExample, StubAdaptorInTest, StubAdaptors, TestAllTheThings,
};
use crate::slides::unit_tests::{UnitTestCode, UnitTestsAreAwesome};
use crate::{
    code::HighlightInit,
    impress::{ImpressGroup, ImpressInit, Step},
    pos::AutoReposition,
    slides::{
        di::{
            DependencyInjection, DependencyInjectionExample, MockingExample, MockingIsNotAwesome,
        },
        external_systems::{
            Communicating, ExternalSystems, IntegrationTestExample, IntegrationTestsAreAwesome,
        },
        intro::Intro,
        unit_tests::{AreLoved, UnitTestExample, UnitTests},
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

    let row = || auto_pos.borrow().row() * y_step;
    let next_row = || auto_pos.borrow_mut().next_row() * y_step;
    let col = || auto_pos.borrow().col() * x_step;
    let next_col = || auto_pos.borrow_mut().next_col() * x_step;
    let max_row = || auto_pos.borrow().max_row() * y_step;
    let max_col = || auto_pos.borrow().max_col() * x_step;

    cx.render(rsx! {
        ImpressGroup { width: width + width_buffer, height: height + height_buffer,
            Step { name: "intro", y: next_row(), x: {next_col(); next_col();next_col();next_col()}, Intro {} }

            Step { name: "unit-tests-are-awesome", y: next_row(), x: {next_col(); next_col(); next_col()}, UnitTests {} }

            Step {
                name: "unit-tests-are-awesome-reveal",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                UnitTestsAreAwesome {}
            }

            Step { name: "unit-tests-are-love", y: row(), x: next_col(), AreLoved {} }

            Step { name: "unit-test-code", y: row(), x: next_col(), UnitTestCode {} }

            Step {
                name: "unit-test-example",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                UnitTestExample {}
            }

            Step { name: "external-systems", y: next_row(), x: { next_col(); next_col() }, ExternalSystems {} }

            Step { name: "external-systems-interface", y: row(), x: next_col(), Communicating {} }

            Step { name: "int-tests-are-awesome", y: row(), x: next_col(), IntegrationTests {} }

            Step {
                name: "int-tests-are-awesome-reveal",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                IntegrationTestsAreAwesome {}
            }

            Step { name: "integration-test-example", y: row(), x: next_col(), IntegrationTestExample {} }

            Step { name: "di", y: next_row(), x: {next_col(); next_col(); next_col()}, DependencyInjection {} }

            Step {
                name: "di-is-awesome",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                DependencyInjectionIsAwesome {}
            }

            Step { name: "di-example", y: row(), x: next_col(), DependencyInjectionExample {} }

            Step { name: "mocking-is", y: row(), x: next_col(), MockingIs {} }

            Step {
                name: "mocking-is-umm",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                MockingIsNotAwesome {}
            }

            Step { name: "mocking-example", y: row(), x: next_col(), MockingExample {} }

            Step {
                name: "ddd-is-awesome",
                y: next_row(),
                x: (x_step * 7) - next_col(),
                rotate_z: 180,
                DddIsAwesome {}
            }

            Step {
                name: "hex-arch",
                y: row(),
                x: (x_step * 7) - next_col(),
                rotate_z: 180,
                HexagonalArchitecture {}
            }

            Step {
                name: "ports-adaptors",
                y: row(),
                x: (x_step * 7) - next_col(),
                rotate_z: 180,
                PortsAndAdaptors {}
            }

            Step {
                name: "port-example",
                y: row(),
                x: (x_step * 7) - next_col(),
                rotate_z: 180,
                PortExample {}
            }

            Step {
                name: "adaptor-example",
                y: row(),
                x: (x_step * 7) - next_col(),
                rotate_z: 180,
                AdaptorExample {}
            }

            Step { name: "stub-adaptors", y: next_row(), x: next_col(), StubAdaptors {} }

            Step { name: "stub-adaptor-example", y: row(), x: next_col(), StubAdaptorExample {} }

            Step { name: "stub-adaptors-in-tests", y: row(), x: next_col(), StubAdaptorInTest {} }

            Step { name: "stub-adaptors-vs-mocks", y: row(), x: next_col(), MocksReview {} }

            Step {
                name: "stub-adaptors-vs-mocks-1",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                MocksReviewOne {}
            }

            Step {
                name: "stub-adaptors-vs-mocks-2",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                MocksReviewTwo {}
            }

            Step { name: "test-all-the-things", y: row(), x: next_col(), TestAllTheThings {} }

            Step { name: "int-tests-for-stubs", y: row(), x: next_col(), IntegrationTestsForStubAdaptors {} }

            Step { name: "conclusion", y: next_row(), x: {next_col();next_col();next_col()}, Conclusion {} }

            Step {
                name: "conclusion-1",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                ConclusionOne {}
            }

            Step {
                name: "conclusion-2",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                ConclusionTwo {}
            }

            Step {
                name: "conclusion-3",
                class: "stack",
                y: row(),
                x: col(),
                transition_duration: 0,
                ConclusionThree {}
            }

            Step { name: "bonus-use", y: row(), x: next_col(), Bonus {} }

            Step { name: "thank-you", y: row(), x: next_col(), ThankYou {} }

            Step {
                name: "where-to-find",
                class: "final",
                y: max_row() / 2 + (2 * y_step),
                x: max_col() / 2 + (1 * x_step),
                scale: 8,
                WhereToFind {}
            }
        }
        ImpressInit {}
        HighlightInit {}
    })
}
