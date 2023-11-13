use crate::code::Typescript;
use crate::impress::Notes;
use dioxus::prelude::*;
use indoc::indoc;

pub fn DependencyInjection(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Dependency Injection is Awesome" }
        Notes {
            p {"Now lets combined our two contrived examples to a new contrived example"}
        }
    ))
}

pub fn DependencyInjectionExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Contrived Example" }
        Typescript { indoc! {"
            const firstTimeUser = async (userStore: UserStore, user: User): Promise<string> => {
                await userStore.create(user);
                return `Welcome ${user.casualName}`;
            }
        "}}
        Notes {
            p {"So now we're using our DB code in our original code... how do we test it"}
            p {"We can't unit test unit test it with a database connection"}
            p {"We _could_ move it to an integration test... but then its turtles all the way down"}
            p {"What if we injected something that doesn't talk to the database"}
        }
    ))
}

pub fn MockingIsNotAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {"Mocking is ..."}
        Notes {
            p {"Mocking is... lets come back to that"}
        }
    ))
}

pub fn MockingExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {"Unit Test with Mock Example"}
        Typescript { indoc! {"
            it('should save the user and greet them', () => {
                const user = createUser({ casualName: Daniel });

                const userStore = {
                    create: jest.fn(() => Promise.resolve()),
                } as UserStore;

                await expect(() => firstTimeUser(userStore, user)).resolves.toBe('Welcome Daniel');
                expect(userStore.create.mock.calls[0][0].toBe(user);
                expect(userStore.create.mock.instances.length).toBe(1);
            });
        "}}
        Notes {
            p {"Here we've got what I'd call a typical mock."}
            p { indoc! {"
                We've only mocked what we need, we return a value that makes the test pass, and
                we've cast the mock so that we can inject it into function, we've tested the
                function returns the right value... and for 'reasons' we've tested that user was
                passed to the mock and that the mock was only called once.
            "}}
            p {"You can probably tell that... I do not think this is awesome"}
            p { indoc! {"
                First, we've written a behavioural abstraction into the test. If the underlying
                behaviour test changes, our test might still pass, even though we need to update
                our code.
            "}}
            p {{ indoc! {"
                A quick language specific aside here, any time in TypeScript you see `as`,
                something has gone horribly wrong. If you want to hear more about that come fined
                me later.
            "}}}
            p { indoc! {"
                Second, we are, for some reason, testing the mock, is this helpful, is it
                meaningful?
            "}}
        }
    ))
}
