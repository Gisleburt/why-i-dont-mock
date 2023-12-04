use crate::code::Typescript;
use crate::impress::Notes;
use dioxus::prelude::*;
use indoc::indoc;

pub fn DependencyInjection(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 {
            "Dependency Injection "
            span { class: "hide", "is Awesome" }
        }
        Notes { p { "Now lets combined our two contrived examples to a new contrived example" } }
    ))
}

pub fn DependencyInjectionIsAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Dependency Injection is Awesome" }
        Notes { p { "Now lets combined our two contrived examples to a new contrived example" } }
    ))
}

pub fn DependencyInjectionExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Contrived Example" }
        Typescript { 
            indoc! { "
                const firstTimeUser = async (userStore: UserStore, user: User): Promise<string> => {
                    await userStore.create(user);
                    return `Welcome ${user.casualName}`;
                }
            " }
        }
        Notes { 
            p { "So now we're using our DB code in our original code... how do we test it" }
            p { "We can't unit test unit test it with a database connection" }
            p {
                indoc! { "
                    We _could_ move it to an integration test... but then its turtles all the way
                    down
                " }
            }
            p { "What if we injected something that doesn't talk to the database" }
        }
    ))
}

pub fn MockingIs(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 {
            "Mocking is "
            span { class: "hide", "... erm" }
        }
        Notes { p { "Mocking is... lets come back to my opinion on that" } }
    ))
}

pub fn MockingIsNotAwesome(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Mocking is ... erm" }
        Notes { p { "Mocking is... lets come back to that" } }
    ))
}

pub fn MockingExample(cx: Scope) -> Element {
    cx.render(rsx!(
        h3 { "Unit Test with Mock Example" }
        Typescript { 
            indoc! { "
                it('should save the user and greet them', () => {
                    const user = createUser({ casualName: Daniel });
            
                    const userStore = {
                        create: jest.fn(() => Promise.resolve()),
                    } as UserStore;
            
                    await expect(firstTimeUser(userStore, user)).resolves.toBe('Welcome Daniel');
                    expect(userStore.create.mock.calls[0][0].toBe(user);
                    expect(userStore.create.mock.instances.length).toBe(1);
                });
            " }
        }
        Notes { 
            p { "Mocks are a way we can 'simulate' real behaviour" }
            p { "Here we've got what I'd call a typical mock." }
            p {
                indoc! { "
                    We've only mocked what we need, we return a value that makes the test pass, and
                    we've cast the mock so that we can inject it into function, we've tested the
                    function returns the right value... and for 'reasons' we've tested that user was
                    passed to the mock and that the mock was only called once.
                " }
            }
            p { "You can probably tell that... I do not think this is awesome" }
            p {
                indoc! { "
                    First, we've written a behavioural abstraction into the test. We're going to
                    need to repeat this every single time we test code that uses this store.
                " }
            }
            p {
                indoc! { "
                    Second, there's no guarantee this behavioural abstraction will actually behave
                    correctly. It might even change in the future and then we have to hope our tests
                    correctly start failing and then go change everywhere our mocks are now wrong.
                    This might seem unlikely but I've seen test doubles return things in the wrong
                    order before, and that's enough to cause a problem. 
                " }
            }
            p { "Lets take a break" }
        }
    ))
}
