use zepl::*;

fn main() {
    App::new(
        include_str!("../Zepl.project"),
        include_str!("../target/systems.list"),
    )
    .run();
}
