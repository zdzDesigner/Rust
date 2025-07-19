mod pipe;
use pipe::Router;
use pipe::Event;
fn main() {

    let mut router = Router::new();
    router.on(String::from("probe"), |val| {
        eprintln!("on :{val}");
        pipe::send("probe send\n");
    });
    router.on(String::from("peq"), |val| {
        eprintln!("on :{val}");
        pipe::send("peq send\n");
    });

    pipe::listen(&router);
}
