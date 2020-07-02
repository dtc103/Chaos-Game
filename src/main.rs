mod chaos;
use chaos::chaos_game;

fn main() {
    let p = chaos_game(
        (10000, 10000),
        10000000,
        None, //Some(vec![(250, 0), (0, 220), (500, 220), (170, 500), (340, 500)]),
        true,
        5f32,
    );
    p.unwrap().save("fractals.png").unwrap();
}