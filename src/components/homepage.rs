use dioxus::prelude::*;

const HeroContent: [(&'static str, &'static str); 3] = [
    ("Declarative", 
    "React makes it painless to create interactive UIs. Design simple views for each state in your application, and React will efficiently update and render just the right components when your data changes.\nDeclarative views make your code more predictable and easier to debug."),

    ("Component-Based", "Build encapsulated components that manage their own state, then compose them to make complex UIs.\nSince component logic is written in JavaScript instead of templates, you can easily pass rich data through your app and keep state out of the DOM."),

    ("Learn Once, Write Anywhere", "We don’t make assumptions about the rest of your technology stack, so you can develop new features in React without rewriting existing code.\nReact can also render on the server using Node and power mobile apps using React Native."),
];

pub static Home: FC<()> = |cx| {
    let hero = HeroContent.iter().map(|(title, body)| {
        rsx! {
            div {
                h3 { "{title}" }
                div { {body.split("\n").map(|paragraph| rsx!( p{"{paragraph}"} ))} }
            }
        }
    });

    rsx! { in cx,
        div {
            header {
                // Hero
                section {
                    div { {hero} }
                }
                hr {}
                // Highlighted Snippets
                section {
                    // {snippets}
                }
            }
            div {}
            section {}
        }
    }
};