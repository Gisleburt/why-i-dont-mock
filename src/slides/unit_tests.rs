use crate::code::Typescript;
use crate::impress::Notes;
use dioxus::prelude::*;
use indoc::indoc;

pub fn UnitTests(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 {
            "Unit Tests "
            span { class: "hide", "are Awesome" }
        }
        Notes { p { "Unit Tests..." } }
    ))
}
pub fn UnitTestsAreAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Unit Tests are Awesome" }
        Notes {
            p { "are Awesome" }
            p { "Those that know me, know that..." }
        }
    ))
}

pub fn AreLoved(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "I love unit tests" }
        img {
            src: "/images/glomp-unit-tests-2-short-and-sweet.gif",
            alt: "Daniel glomping Unit Tests",
            style: "width: 400px; padding-left: 70px; image-rendering: crisp-edges;"
        }
        Notes {
            p { "I _love_ tests" }
            p {
                indoc! { "
                    I think I don't do well with compliments so when a person tells me my code is
                    good, I don't necessarily believe them, but when a computer tells me my
                    code is good, that feels good.
                " }
            }
            p { "Lets write a test." }
        }
    ))
}

pub fn UnitTestCode(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Code:" }
        Typescript {
            indoc! { "
                const firstTimeUser = (user: User): string => {
                    return `Welcome ${user.casualName}`;
                }
            "}
        }

        div { class: "hide",
            h3 { "Example Unit Test:" }
            Typescript {
                indoc! { "
                    it('should greet the user', () => {
                        const user = createUser({ casualName: 'Daniel' });
                        expect(firstTimeUser(user)).toBe('Welcome Daniel');
                    });
                " }
            }
        }

        Notes {
            p {
                indoc! {"
                    Here's an example of some code that takes a user, and writes a custom greeting
                    for them
                " }
            }
        }
    ))
}

pub fn UnitTestExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Code:" }
        Typescript {
            indoc! { "
                const firstTimeUser = (user: User): string => {
                    return `Welcome ${user.casualName}`;
                }
            "}
        }
        h3 { "Example Unit Test:" }
        Typescript {
            indoc! { "
                it('should greet the user', () => {
                    const user = createUser({ casualName: 'Daniel' });
                    expect(firstTimeUser(user)).toBe('Welcome Daniel');
                });
            " }
        }
        Notes {
            p { "We use a unit test to check that it behaves the way we expect" }
            p {
                indoc!{ "
                    This test simply calls the function with some data and checks we get back what
                    we expect to get back. A little over simplified but hopefully gets the idea
                    across 
                " }
            }
        }
    ))
}
