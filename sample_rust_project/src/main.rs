mod root;

fn main() {
    // ローカルホスト3000番ポートでサーバーを起動する
    rouille::start_server("localhost:3000", move |request| {
        // リクエストを受け取ったら、レスポンスを返す
        rouille::Response::text("Hello, world!")
    });
}
