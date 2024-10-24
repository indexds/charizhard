use base64::Engine;
use base64::prelude::BASE64_STANDARD;

pub fn index_html() -> String {

    let favicon_data = include_bytes!("../../favicon.ico");
    let favicon = BASE64_STANDARD.encode(favicon_data);

    format!(
        r###"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>Charizhard</title>
                <link rel="icon" type="image/png" href="data:image/png;base64,{favicon}">
            </head>
            <body>
                <form action="#" method="post">
                    <label for="user-input"></label>
                    <input type="text" id="user-input" name="user-input">
                    <label for="
                    <button type="submit">Submit</button>
                </form>
            </body>
        </html>
        "###
    )
}

pub fn submit() -> String {

    let favicon_data = include_bytes!("../../favicon.ico");
    let favicon = BASE64_STANDARD.encode(favicon_data);

    format!(
        r###"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>Charizhard</title>
                <link rel="icon" type="image/png" href="data:image/png;base64,{favicon}">
            </head>
            <body>
                Issou
            </body>
        </html>
        "###
    )
}