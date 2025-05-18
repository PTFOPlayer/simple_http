use std::{io::Write, net::TcpStream};

use super::status::Status;

pub fn err404(stream: &mut TcpStream) {
    let site404 = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Simple HTTP</h1>
    <p>404</p>
</body>
</html>"#;
    let response = format!(
        "{}\nContent-Length: {}\n\n{}",
        Status::NOT_FOUND,
        site404.len(),
        site404
    );

    stream.write_all(response.as_bytes()).unwrap();
}
