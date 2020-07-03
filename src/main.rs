mod chaos;
use chaos::chaos_game;
use chaos::barnsley_fern;

use rand;
use rand::prelude::*;

fn main() {
    // let p = chaos_game(
    //     (500, 500),
    //     100000,
    //     None, //Some(vec![(250, 0), (0, 220), (500, 220), (170, 500), (340, 500)]),
    //     true,
    //     2f32,
    // );


    let p = barnsley_fern((600, 600), 100000, false);
    p.unwrap().save("fractals.png").unwrap();
}
