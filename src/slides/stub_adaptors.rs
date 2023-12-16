use crate::{code::Typescript, impress::Notes};
use dioxus::prelude::*;
use indoc::indoc;

pub fn StubAdaptors(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Stub Adaptors" }
        img {
            src: "images/stub-adaptors.png",
            alt: "Create an adaptor for a port that doesn't talk to anything",
            style: "width: 400px"
        }
        Notes {
            p { "Stub Adaptors" }
            p { "What if we made an adaptor that doesn't talk to another service" }
            p {
                indoc! {"
                    We could have an adaptor that matched the port but encapsulated all of the
                    behaviour
                " }
            }
            p { "Lets try it with our User Store" }
        }
    ))
}

pub fn StubAdaptorExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Example" }
        Typescript {
            indoc! { "
                class StubUserStore implements UserStore {
                    constructor(private users: User[] = []) {}
            
                    async create(user: User): Promise<void> {
                        this.users.push(user);
                    }
            
                    async readByEmail(email: String): Promise<User> {
                        return this.users.find(user => user.email === email)
                    }
                }
            " }
        }
        Notes {
            p {
                indoc! { "
                    In this implementation of our User Store, instead of passing in a database
                    connection, we instead pass in an array, empty by default, but allowing some
                    pre-configuration
                " }
            }
            p { "Our create user method simply stores the user in the array" }
            p { "And our read by email method finds them in the array" }
            p {
                indoc! { "
                    For a production app, this would be terrible, but for testing, this is very
                    useful
                " }
            }
        }
    ))
}

pub fn StubAdaptorInTest(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Usage in unit tests" }
        Typescript {
            indoc! {"
                it('should greet the user', async () => {
                    const userStore = new StubUserStore();
            
                    const user = createUser({ casualName: Daniel, email: randomEmail() });
                    expect(await firstTimeUser(userStore, user)).toBe('Welcome Daniel');
            
                    const storedUser = await userStore.readByEmail(user.email);
                    expect(storedUser).toEqual(user);
                });
            " }
        }
        Notes {
            p { "We can now use the stub adaptor instead of the mock in our unit test" }
            p { "We create a StubUserStore that will only last the length of the test" }
            p { "We test that our function returns the right thing" }
            p { "We then use the store itself to check the right thing was stored" }
            p { "As we control the array in the store, if we wanted, we could check that instead" }
        }
    ))
}

pub fn StubAdaptorsReview(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Did we fix mocks" }
        ol {
            li { span { class: "hide", "✅ Behavioural abstraction removed from the test" } }
            li { span { class: "hide", "❌ Behavioural abstraction might be wrong" } }
        }
        Notes { p { "So, sid we fix mocks?" } }
    ))
}

pub fn StubAdaptorsReviewOne(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Did we fix mocks" }
        ol {
            li { "✅ Behavioural abstraction removed from the test" }
            li { span { class: "hide", "❌ Behavioural abstraction might be wrong" } }
        }
        Notes { p { "We did manage to remove our behavioural abstraction from our tests" } }
    ))
}

pub fn StubAdaptorsReviewTwo(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Did we fix mocks" }
        ol {
            li { "✅ Behavioural abstraction removed from the test" }
            li { "❌ Behavioural abstraction might be wrong" }
        }
        Notes {
            p { "But we haven't confirmed that our behavioural abstraction matches the real thing" }
        }
    ))
}

pub fn TestAllTheThings(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { " Test all the things" }
        img {
            src: "images/all-the-things.png",
            alt: "All The Things (by Colleen Powers)",
            style: "width: 400px"
        }
        Notes {
            p { " It's probably not a surprise that I think we can solve this with more tests" }
            p { " But this step is way less work than you might think" }
            p { " Lets go back to our integration tests" }
        }
    ))
}

pub fn IntegrationTestsForStubAdaptors(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Back to integration tests" }
        Typescript {
            indoc! { "
                const pgStore = new PostgresUserStore(dbConnection);
                const stubStore = new StubUserStore();
                const email = randomEmail();
            
                describe.each([pgStore, stubStore])('UserStore', (userStore) => {
                    it('should store the user', async () => {
                        const user = createUser({ email });
                        await expect(userStore.create(user)).resolves.not.toThrow()
                    });
            
                    it('should recall the user the user', async () => {
                        const user = await userStore.readByEmail(email);
                        expect(user.email).toBe(email);
                    });
                });
            " }
        }
        Notes {
            p { "These are almost identical to our earlier integration steps" }
            p {
                indoc! { "
                    The only thing I've really changed is that now the integration tests run on both
                    the postgres store _and_ the stub store.
                " }
            }
            p {
                indoc! { "
                    These are the exact same tests so any difference in behaviour between our
                    adaptors will be immediately detected
                " }
            }
            p {
                indoc! { "
                    If you happen to work with a system that already has multiple adaptors connected
                    to the same ports, you can even extend your integration tests to include those
                    too!
                " }
            }
        }
    ))
}

pub fn StubAdaptorsReviewAfterTests(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Did we fix mocks" }
        ol {
            li { "✅ Behavioural abstraction removed from the test" }
            li { "✅ Behavioural abstraction is \"correct\"" }
        }
        Notes {
            p {
                indoc! { "
                    And with that, we're now confident that our abstraction behaves the same way as
                    our real world adaptor, everything is tested, the computer tells Daniel is good
                    and Daniel is happy!
                " }
            }
        }
    ))
}
