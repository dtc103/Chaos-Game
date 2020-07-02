use rand;
use rand::prelude::*;
use rand::seq::SliceRandom;
/*
returns an image with the fractasl contained

params:
    picture_size: the dimension of the picture
    iteration_depth: the amount of dots to draw
    input_point: the pivot points for our fractal generation
    colored: if the picture should be coloured
    relative jump: the relative distance from one point to another, the point should jump
*/
pub fn chaos_game(
    picture_size: (u32, u32),
    iteration_depth: u32,
    input_points: Option<Vec<(u32, u32)>>,
    colored: bool,
    mut relative_jump: f32, //should be bigger than 1
) -> Option<image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let mut image = image::RgbImage::new(picture_size.0, picture_size.1);

    if relative_jump <= 1f32 {
        relative_jump = 1f32;
    }

    let steps = iteration_depth;

    //only use these 3 points, if the user doesnt provide own points
    let a: (u32, u32);
    let b: (u32, u32);
    let c: (u32, u32);
    let mut point_array = Vec::new();
    match input_points {
        Some(mut input_list) => {
            for point in &input_list {
                //wont check for < 0, because the type checker will throw an exception if that happens.
                if point.0 > image.width() || point.1 > image.height() {
                    return None;
                }
            }
            point_array.append(&mut input_list);
        }
        None => {
            a = (image.width() / 2, (image.height() as f32 * 0.02) as u32);
            b = (
                (image.width() as f32 * 0.02) as u32,
                (image.height() as f32 * 0.98) as u32,
            );
            c = (
                (image.width() as f32 * 0.98) as u32,
                (image.height() as f32 * 0.98) as u32,
            );
            point_array.append(&mut vec![a, b, c]);
        }
    };
    //first point is in the middle of the picture(Could be at random position, but this doesnt rly matter).
    let mut point = (image.width() / 2, image.height() / 2);

    let mut rnd = thread_rng();

    for _ in 0..steps {
        let corner = point_array.as_slice().choose(&mut rnd).unwrap();
        point = (
            (point.0 as i32 + ((corner.0 as i32 - point.0 as i32) as f32 / relative_jump) as i32)
                as u32,
            (point.1 as i32 + ((corner.1 as i32 - point.1 as i32) as f32 / relative_jump) as i32)
                as u32,
        );

        if colored {
            let color: (u8, u8, u8) = hsv_to_rgb(
                360f32 * ((image.height() - point.1) as f32 / image.height() as f32),
                1f32,
                1f32,
            );
            image.put_pixel(point.0, point.1, image::Rgb([color.0, color.1, color.2]));
        } else {
            image.put_pixel(point.0, point.1, image::Rgb([255, 255, 255]));
        }
        //println!("{:?}", color);
    }

    Some(image)
}

fn hsv_to_rgb(mut h: f32, mut s: f32, mut v: f32) -> (u8, u8, u8) {
    while h >= 360f32 {
        h -= 360f32;
    }
    if h < 0f32 {
        h = 0f32;
    }

    if s < 0f32 {
        s = 0f32;
    }
    if s > 1f32 {
        s = 1f32;
    }

    if v < 0f32 {
        v = 0f32;
    }
    if v > 1f32 {
        v = 1f32;
    }

    let c = v * s;
    let x = c * (1f32 - ((h / 60f32) % 2f32 - 1f32).abs());
    let m = v - c;

    //h has to be in the interval [0, 360) (because of the first while loop in this function)
    // the conversion to u32 also leads to the fact, that we cant use all the values on the circle, but only 360
    match h as u32 {
        0..=59 => (
            ((c + m) * 255f32) as u8,
            ((x + m) * 255f32) as u8,
            (m * 255f32) as u8,
        ),
        60..=119 => (
            ((x + m) * 255f32) as u8,
            ((c + m) * 255f32) as u8,
            (m * 255f32) as u8,
        ),
        120..=179 => (
            (m * 255f32) as u8,
            ((c + m) * 255f32) as u8,
            ((x + m) * 255f32) as u8,
        ),
        180..=239 => (
            (m * 255f32) as u8,
            ((x + m) * 255f32) as u8,
            ((c + m) * 255f32) as u8,
        ),
        240..=299 => (
            ((x + m) * 255f32) as u8,
            (m * 255f32) as u8,
            ((c + m) * 255f32) as u8,
        ),
        300..=359 => (
            ((c + m) * 255f32) as u8,
            (m * 255f32) as u8,
            ((x + m) * 255f32) as u8,
        ),
        _ => panic!("This case shouldnt be happen"),
    }
}
