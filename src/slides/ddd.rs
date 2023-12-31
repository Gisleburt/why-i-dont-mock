use crate::code::Typescript;
use crate::impress::Notes;

use dioxus::prelude::*;
use indoc::indoc;

pub fn DddIsAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "DDD is Awesome" }
        Notes {
            p { "Domain Drive Development is awesome" }
            p {
                indoc! { "
                    DDD obviously encompasses many things but I want to talk about one specific
                    thing
                " }
            }
        }
    ))
}

pub fn HexagonalArchitecture(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Hexagonal Architecture" }
        img {
            src: "images/how-to-communicate.png",
            alt: "How does a web server talk to a database?",
            style: "width: 400px"
        }
        Notes {
            p { "Hexagonal architecture is... a terrible name for a brilliant idea" }
            p {
                indoc! {"
                    It was named after honey comb structures but otherwise has nothing to do with
                    hexagons or the number 6
                " }
            }
            p {
                indoc! { "
                    Instead, think about it as a loose description of how we talk to external
                    systems
                " }
            }
            p {
                indoc! { "
                    We have two separate systems that we want to talk to each other. Ideally we
                    don't want to tightly couple these systems, we want to be able to change the
                    external system without a ton of work. For example, what happens if your
                    business, formally tied to a large Mongo dataset, decides to break it up into
                    many Postgres databases?
                " }
            }
        }
    ))
}

pub fn PortsAndAdaptors(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Ports and Adaptors" }
        img {
            src: "images/ports-and-adaptors.png",
            alt: "Using ports and adaptors to separate coupled code",
            style: "width: 400px"
        }
        Notes {
            p { "The two key parts of hexagonal architecture are ports and adaptors" }
            p {
                indoc! {"
                    On the left we have our application, on the right some external service like a
                    database or API.
                " }
            }
            p { "In between we have two elements, a port and an adaptor" }
            p {
                indoc! {"
                    The port is a pure abstraction, without any logic, that describes how our
                    application would like to communicate its needs. This can usually be
                    represented by what many languages would call an interface.
                " }
            }
            p {
                indoc! {"
                    The adaptor is the implementation of that interface. Lets go back to our code
                    examples.
                " }
            }
        }
    ))
}

pub fn PortExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Port Example" }
        Typescript {
            indoc! { "
                interface UserStore {
                    async create(user: User) => Promise<void>;
            
                    async readByEmail(email: String) => Promise<User>;
                }
            " }
        }

        Notes {
            p { "Here we've made User Store just an interface" }
            p { "It still has the same methods and signatures as before" }
        }
    ))
}
pub fn AdaptorExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Adaptor Example" }
        Typescript {
            indoc! { "
                class PostgresUserStore implements UserStore {
                    constructor(private db: Database) {}
            
                    async create(user: User): Promise<void> {
                        const dbUser = userToDb(user);
                        await this.db.insert(dbUser);
                    }
            
                    async readByEmail(email: String): Promise<User> {
                        const dbUser = await this.db.select('email', email);
                        return userFromDb(dbUser);
                    }
                }
            " }
        }
        Notes {
            p { "We've moved the implementation to a class that implements that interface." }
            p { "Apart from the name, and the 'implements interface' bit, its otherwise identical" }
            p { "Great, we've got hexagonal architecture... how the hell does that help" }
        }
    ))
}
