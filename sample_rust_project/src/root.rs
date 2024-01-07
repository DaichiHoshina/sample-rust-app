use rouille::Response;

pub fn index() -> Response {
    Response::text("Hello, world!")
}

pub fn hello(name: &str) -> Response {
    Response::text(format!("Hello, {}!", name))
}

pub fn goodbye(name: &str) -> Response {
    Response::text(format!("Goodbye, {}!", name))
}