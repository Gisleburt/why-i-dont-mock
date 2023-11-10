#![allow(non_snake_case)]

mod code;
mod impress;
mod mermaid;
mod pos;
mod slides;

use crate::{
    code::HighlightInit,
    code::Typescript,
    impress::{ImpressGroup, ImpressInit, Step},
    mermaid::{Mermaid, MermaidInit},
    pos::AutoReposition,
    slides::intro::Intro,
    slides::unit_tests::{AreAwesome, AreLoved, UnitTestExample},
};
use dioxus::prelude::*;
use indoc::indoc;

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

            Step { name: "external-systems", y: new_row(), x: col(),
                h2 { "External Systems" }
                Mermaid {
                    """
                    graph LR
                        A[App] --> B[Data Store]
                    """
                }
            }

            Step { name: "external-systems-interface", y: row(), x: col(),
                h3 { "Example Database CRUD" }
                Typescript { indoc! {"
                    class UserStore {{
                        constructor(private db: Database) {{}}

                        async create(user: User): Promise<void> {{
                            await this.db.insert(user);
                        }}

                        async readByEmail(email: String): Promise<User> {{
                            const dbUser = await this.db.select('email', email);
                            return userFromDb(dbUser);
                        }}
                    }}
                "}}
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
        MermaidInit {}
    })
}
