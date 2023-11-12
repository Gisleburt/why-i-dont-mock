use crate::{code::Typescript, impress::Notes, mermaid::Mermaid};
use dioxus::prelude::*;
use indoc::indoc;

pub fn ExternalSystems(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "External Systems" }
        Mermaid { "
            graph LR
                A[App] --> B[Data Store]
        " }
        Notes {
            p { indoc! { "
                Our systems rarely exist in isolation and often have to talk to external
                systems. Commonly, for example, we want to store information away from the
                application, particularly if we're going to run many applications that need
                access to the same data.
            " }}
        }
    ))
}

pub fn Communicating(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example Database CRUD" }
        Typescript { indoc! {"
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
        "}}
        Notes {
            p{ indoc! {"
                That user object from before, lets say we want to be able to create and read back
                that user. We might write a class something like this.
            " }}
            p{ indoc! {"
                We instantiate our class with some kind of database object that lets us talk to the
                database. When we create a new user, we map our user type to the  
            " }}
        }
    ))
}

pub fn IntTestAreAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Integration Tests are Awesome" }
        Notes {
            p {"Integration tests are awesome"}
            p {"I'm predictable"}
            p { indoc! {"
                As this is the part of how we communicate with the database (and assuming those 
                other functions were unit tested), this makes sense to test against a real database
            "}}
        }
    ))
}

pub fn IntegrationTestExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Integration Tests are Awesome" }
        Typescript { indoc! {"
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
        "}}
        Notes {
            p { indoc! {"Ok, I know, these are a little contrived"}}
            p { indoc! {"
                But here we've got a test that will store data in an actual database and then
                recall it again. Great.
            "}}
        }
    ))
}
