mod chaos;
use chaos::chaos_game;

fn main() {
    let p = chaos_game(
        (500, 500),
        100000,
        None, //Some(vec![(250, 0), (0, 220), (500, 220), (170, 500), (340, 500)]),
        true,
        2f32,
    );
    p.unwrap().save("fractals.png").unwrap();
}
