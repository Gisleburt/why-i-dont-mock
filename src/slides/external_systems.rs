use crate::{code::Typescript, impress::Notes};
use dioxus::prelude::*;
use indoc::indoc;

pub fn ExternalSystems(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "External Systems" }
        img {
            src: "images/external-systems.png",
            alt: "Web Server talking to Database",
            style: "width: 400px"
        }
        Notes {
            p {
                indoc! { "
                    Our systems rarely exist in isolation and often have to talk to external
                    systems. Commonly, for example, we want to store information away from the
                    application, particularly if we're going to run many applications that need
                    access to the same data.
                " }
            }
        }
    ))
}

pub fn Communicating(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Database CRUD" }
        Typescript {
            indoc! { "
                class UserStore {
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
            p {
                indoc! { "
                    That user object from before, lets say we want to be able to create and read
                    back that user. We might write a class something like this.
                " }
            }
            p {
                indoc! { "
                    We instantiate our class with some kind of database object that lets us talk to
                    the database. 
                " }
            }
            p {
                indoc! { "
                    When we create a new user, we map our user type to something the database
                    understands, then store it.
                " }
            }
            p {
                indoc! { "
                    When we recall the user from the database we map it back to our User type and
                    return it.
                " }
            }
        }
    ))
}

pub fn IntegrationTests(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {
            "Integration Tests "
            span { class: "hide", "are Awesome" }
        }
        Notes { p { "Integration tests ..." } }
    ))
}

pub fn IntegrationTestsAreAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Integration Tests are Awesome" }
        Notes {
            p { "...are awesome" }
            p { "I'm predictable" }
            p {
                indoc! { "
                    Because our User Store class needs to talk to the database we can't unit test it
                    but we can write an integration test under the assumption the test will talk to
                    some sort of database
                " }
            }
        }
    ))
}

pub fn IntegrationTestExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Integration Test" }
        Typescript {
            indoc! { "
                const email = randomEmail();
                const userStore = new UserStore(dbConnection);
            
                it('should store the user', async () => {
                    const user = createUser({ email });
                    await expect(userStore.create(user)).resolves.not.toThrow()
                });
            
                it('should recall the user the user', async () => {
                    const user = await userStore.readByEmail(email);
                    expect(user.email).toBe(email);
                });
            " }
        }
        Notes {
            p { "Ok, I know, these are a little contrived" }
            p {
                indoc! { "
                    But here we've got a test that will store data in an actual database and then
                    recall it again. Great.
                " }
            }
        }
    ))
}
