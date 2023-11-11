#![allow(arithmetic_overflow)]
use std::{fs::{self, File}, env::args, path::Path, fmt::Display};

use json::JsonValue;
use rand::{seq::SliceRandom, thread_rng};
use wav::{BitDepth, Header};
use stopwatch::Stopwatch;

struct Input {
    down: bool,
    frame: i32
}

fn to_vec_i16(bd: &BitDepth) -> Option<Vec<i16>> {
    let mut v: Vec<i16> = vec![];
    match bd {
        BitDepth::Empty => None,
        BitDepth::Eight(u) => {
            for i in u {
                v.push((*i) as i16 * 256);
            }
            Some(v)
        }
        BitDepth::Sixteen(u) => {
            for i in u {
                v.push(*i);
            }
            Some(v)
        }
        BitDepth::TwentyFour(u) => {
            for i in u {
                v.push((*i) as i16 / 256);
            }
            Some(v)
        }
        BitDepth::ThirtyTwoFloat(u) => {
            for i in u {
                v.push((*i * 32768.0) as i16);
            }
            Some(v)
        }
    }
}

fn expand(v: &mut Vec<i16>, len: usize) {
    while v.len() < len {
        v.push(0);
    }
}

fn clamp<'a, T: PartialOrd>(a: &'a T, min: &'a T, max: &'a T) -> &'a T {
    if a < min {min} else {if a <= max {a} else {max}}
}

fn insert_or_push(v: &mut Vec<i16>, add: &Vec<i16>, ind: usize) {
    expand(v, ind + add.len());
    for i in (ind)..(ind + add.len()) {
        // For lifetime purposes.
        let mut zero: i16 = 0;
        let val = add.get(i - ind).unwrap_or(&0);
        let modif: &mut i16 = v.get_mut(i).unwrap_or(&mut zero);
        let result: i32 = *modif as i32 + *val as i32;
        *modif = (*clamp(&result, &(i16::MIN as i32), &(i16::MAX as i32))) as i16;
    }
}

fn load_wav<P: Display>(path: P) -> (Header, Vec<i16>) {
    let (header, bd) = wav::read(
        &mut File::open(format!("assets/{path}"))
            .expect("Couldn't locate assets/down.wav!")
        ).expect("Couldn't parse assets/down.wav!");
    (header, to_vec_i16(&bd).expect("down sound was empty!"))
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let mut argv = args().skip(1);
    println!("Opening macro...");
    let jval: JsonValue = json::parse(&fs::read_to_string(argv.next().unwrap_or("macro.json".to_owned())).expect("File could not be read!")).expect("Invalid JSON!");
    let mut events: Vec<Input> = vec![];
    let fps: f32;

    println!("Opening click audio...");
    let down_names = fs::read_to_string("downs.txt")
        .unwrap_or(format!("down.wav"));
    let mut iter = down_names
        .split(['\n', '\r'])
        .map(|s|s.trim())
        .filter(|s|s.len() != 0);

    let (downh, data) = load_wav(iter.next().expect("`downs.txt` was empty!"));
    let mut downs = vec![data];
    downs.extend(iter.map(|path|load_wav(path).1)
        .collect::<Vec<Vec<i16>>>());

    let ups = fs::read_to_string("ups.txt")
        .unwrap_or(format!("up.wav"))
        .split(['\n', '\r'])
        .map(|s|s.trim())
        .filter(|s|s.len() != 0)
        .map(|path|load_wav(path).1)
        .collect::<Vec<Vec<i16>>>();
    if ups.len() == 0 { panic!("`ups.txt` was empty!") }

    println!("Getting clicks...");
    match jval["events"].clone() {
        JsonValue::Array(arr) => {
            println!("Parsing {} clicks...", arr.len());
            for i in arr {
                let down = &i["down"];
                events.push(Input {
                    down: match down {JsonValue::Boolean(b) => *b, _ => {continue;}},
                    frame: match &i["frame"] {JsonValue::Number(n) => f32::from(*n) as i32, v => {panic!("Invalid macro (expected number value, got {})!", v)}}
                })
            }
        }
        _ => {panic!("The `events` parameter was not an array!")}
    }
    println!("Getting fps...");
    match jval["meta"].clone() {
        JsonValue::Object(ob) => {
            match ob["fps"].clone() {
                JsonValue::Number(f) => {
                    fps = f.into();
                }
                _ => {panic!("The `meta` parameter was not an array!")}
            }
        }
        _ => {panic!("The `meta` parameter was not an array!")}
    }
    let mut output: Vec<i16> = vec![];
    println!("Adding clicks to audio file...");
    let mut i: i32 = 0;
    let mut counter: i32 = 1;
    for inp in events {
        insert_or_push(&mut output, if inp.down {&downs} else {&ups}.choose(&mut thread_rng()).unwrap(), /* debug bottleneck for many clicks --> */downh.sampling_rate as usize * downh.channel_count as usize * inp.frame as usize / fps as usize);
        if i >= counter {
            println!("{} clicks finished...", i);
            counter += counter;
        }
        i += 1;
    }
    println!("Saving...");
    wav::write(downh, &BitDepth::Sixteen(output), &mut File::create("output.wav").expect("Could not create output file!")).expect("Could not write to wav file!");
    sw.stop();
    let el = sw.elapsed_ms();
    println!("Clickbot succeeded in {}ms ({}s, {}s with decimal) on {} profile!", el, el / 1000, el as f32 / 1000.0, if cfg!(debug_assertions){"debug"}else{"release"});
}
