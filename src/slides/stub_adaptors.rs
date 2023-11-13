use crate::{code::Typescript, impress::Notes, mermaid::Mermaid};
use dioxus::prelude::*;
use indoc::indoc;

pub fn StubAdaptors(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Stub Adaptors" }
        Mermaid { indoc! { "
            flowchart LR
                app{{ Application }}
                port[[ Port ]]
                apt> Adaptor ]
                ext[( External 'Stuff' )]
                stub>Stub Adaptor]
                
                app --- port
                port --> apt
                port --> stub
                apt --> ext
        " } }
        Notes {
            p { "Stub Adaptors" }
            p { "What if we made an adaptor that doesn't talk to another service" }
            p { indoc! {"
                What we had an adaptor that matched the port but encapsulated all of the behaviour
            " } }
            p { "What might that look like?" }
        }
    ))
}

pub fn StubAdaptorExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example" }
        Typescript { indoc! { "
            class StubUserStore implements UserStore {
                constructor(private users: User[] = []) {}

                async create(user: User): Promise<void> {
                    this.users.push(user);
                }

                async readByEmail(email: String): Promise<User> {
                    return this.users.find(user => user.email === email)
                }
            }
        " } }
        Notes {
            p { "Here we simply store the user data in an array" }
            p { indoc! { "
                For a production app, this would be terrible, but for testing, this is very useful
            " } }
            p { "We've also allowed you to set the array to something you control, handy later" }
        }
    ))
}

pub fn StubAdaptorInTest(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Usage in unit tests" }
        Typescript { indoc! {"
            it('should greet the user', async () => {
                const userStore = new StubUserStore();

                const user = createUser({ casualName: Daniel, email: randomEmail() });
                expect(await firstTimeUser(user)).toBe('Welcome Daniel');

                const storedUser = await userStore.readByEmail(user.email);
                expect(storedUser).toEqual(user);
            });
        " } }
        Notes {
            p { "Now our test just uses the stub adaptor" }
            p { "We create a StubUserStore that will only last the length of the test" }
            p { "We test that our function returns the right thing" }
            p { "We then use the store itself to check the right thing was stored" }
            p { "As we control the array in the store, we could check that instead if we wanted" }
        }
    ))
}

pub fn MocksReview(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {" Did we fix mocks" }
        ol {
            li {" Behavioural abstraction removed from the test ✅" }
            li {" Behavioural abstraction matches the other store ❌" }
        }
        Notes {
            p { "So, sid we fix mocks?" }
            p { "We did manage to remove our behavioural abstraction from our tests" }
            p { "But we haven't confirmed that our behavioural abstraction matches the real thing" }
        }
    ))
}

pub fn TestAllTheThings(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {" Test all the things" }
        Notes {
            p {" It's probably not a surprise that I think we can solve this with more tests" }
            p {" But this step is way less work than you might think" }
            p {" Lets go back to our integration tests" }
        }
    ))
}

pub fn IntegrationTestsForStubAdaptors(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Back to integration tests" }
        Typescript { indoc! { "
            const pgStore = new PostgresUserStore(dbConnection);
            const stubStore = new StubUserStore();

            describe.each([pgStore, stubStore])('UserStore', () => {
                const email = randomEmail();
    
                it('should store the user', async () => {
                    const user = createUser({ email });
                    await expect(userStore.create(user)).resolves.not.toThrow()
                });
    
                it('should recall the user the user', async () => {
                    const user = await userStore.readByEmail(email);
                    expect(user.email).toBe(email);
                });
            });
        " } }
        Notes {
            p { "These are almost identical to our earlier integration steps" }
            p { indoc! { "
                The only thing I've really changed is that now the integration tests run on both the
                postgres store _and_ the stub store.
            " } }
            p { indoc! { "
                These are the exact same tests so any difference in behaviour between our adaptors
                will be immediately detected
            " } }
            p { indoc! { "
                If you happen to work with a system that already has multiple adaptors connected to
                the same ports, you can even extend your integration tests to include those too!
            " } }
            p { indoc! { "
                So now everything is tested, the computer tells Daniel he is good, and Daniel is
                happy
            " } }
        }
    ))
}
