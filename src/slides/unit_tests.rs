use dioxus::prelude::*;
use crate::code::Typescript;
use crate::impress::Notes;

pub fn AreAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Unit Tests are Awesome" }
        Notes {
            p { "Unit Tests are awesome" }
            p { "Those that know me, know that"}
        }
    ))
}

pub fn AreLoved(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {"I love unit tests"}
        p { "<insert meme image>" }
        Notes {
            p { "I _love_ tests" }
            p {
                "I don't do well with compliments so when a human tells me my code is ",
                "good, I don't necessarily believe them, but when a computer tells me my ",
                "code is good, that feels good."
            }
            p { "Lets write a test." }
        }
    ))
}

pub fn UnitTestExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Code:" }
        Typescript {
            "const greet = (user: User): Promise<string> => {{\n",
            "  return `Hello {{user.casualName}}`;\n",
            "}}"
        }
        h3 { "Example Unit Test:" }
        Typescript {
            "it('should greet the user', () => {{\n",
            "  const user = createUser({{ casualName: Daniel }});\n",
            "  expect(greet(user)).toBe('Hello Daniel');\n",
            "}});"
        }
        Notes {
            p {
                "Here's an example of some code that takes a user, and writes a custom ",
                "greeting for them"
            }
            p { "And we use a unit test to check that it behaves the way we expect" }
        }
    ))
}
