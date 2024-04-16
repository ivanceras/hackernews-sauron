use crate::sauron;
use crate::sauron::prelude::*;
use client::{App, Msg};

/// We are creating an index page.
/// From the `App` supplied, we can derive the view by calling `App.view` function.
/// we extract the state and serialize it.
pub fn index(app: &App) -> Node<Msg> {
    println!("app: {:#?}", app);
    let serialized_state = serde_json::to_string(&app).unwrap();
    let serialized_state = serialized_state.replace('`', r#"${"`"}"#);
    node! {
        <!doctype html>
        <html lang="en">
            <head>
               <meta http-equiv="Content-type" content="text/html; charset=utf-8"/>
               <meta name="referrer" content="origin"/>
               <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
               <link rel="stylesheet" type="text/css" href="/style.css"/>
               <link rel="shortcut icon" href="/favicon.ico"/>
               <link rel="shortcut icon" href="/favicon.svg" type="image/x-icon"/>
               <title>"Hacker News"</title>
                <script type="module">
                    {text!("
                          import init, {{ main }} from '/pkg/client.js';
                          async function start() {{
                            await init();
                            let app_state = String.raw`{}`;
                            await main(app_state);
                          }}
                          start();
                    ",serialized_state)}
                </script>
            </head>
            { app.view() }
        </html>
    }
}
