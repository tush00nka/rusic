use core::{
    clone::Clone,
    f32::consts::PI,
    iter::{ExactSizeIterator, Iterator},
};
use std::process::exit;

const SAMPLE_RATE: u32 = 48_000;
const STANDARD_PITCH: f32 = 440.;
const BPM: usize = 120;
const BEAT_DURATION: f32 = 60. / BPM as f32;
const VOLUME: f32 = 0.2;

fn usage() {
    println!("USAGE: rusic <FILENAME> [OPTIONS]\nOPTIONS:\n\t-o <filename>\tset output filename");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        println!("ERROR: Filename not provided");
        exit(1);
    }

    if args.len() == 3 {
        usage();

        if args[2] == "-o" {
            println!("ERROR: output filename is not set");
        } else {
            println!("ERROR: unknown option");
        }
        exit(1);
    }

    let output_filename;
    if args.len() >= 4 {
        if args[2] == "-o" {
            output_filename = args[3].clone();
        } else {
            usage();
            println!("ERROR: unknown option");
            exit(1);
        }
    } else {
        output_filename = "output.wav".to_string();
    }

    let _ = std::fs::write("output.pcm", parse_melody("examples/sandstorm.txt", 1));
    let _ = std::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "f32le",
            "-ar",
            "48000",
            // "-ac",
            // "2",
            "-i",
            "output.pcm",
            &output_filename,
        ])
        .output();
    let _ = std::process::Command::new("rm").arg("output.pcm").output();


}

fn get_note(semitone: i32, beats: f32) -> Vec<u8> {
    let hz = STANDARD_PITCH * (2.0_f32).powf(semitone as f32 / 12.);
    let step = (hz * 2. * PI) / SAMPLE_RATE as f32;
    let duration = beats * BEAT_DURATION;
    let sample = (SAMPLE_RATE as f32 * duration) as u32;

    let output = (0..sample).map(|s| (s as f32 * step).sin());
    let attack = (0..).map(|a| (a as f32 / 1000.0).min(1.0));
    let release = attack
        .clone()
        .take(output.len())
        .collect::<Vec<f32>>()
        .into_iter()
        .rev();

    let output = output
        .zip(attack)
        .map(|(o, a)| o * a)
        .zip(release)
        .map(|(o, r)| o * r);

    let output = output
        .map(|o| (o * VOLUME).to_le_bytes())
        .flatten()
        .collect::<Vec<u8>>();

    output
}

fn parse_melody(filepath: &str, repeat: usize) -> Vec<u8> {
    let mut melody = vec![];

    let file = std::fs::read_to_string(filepath).unwrap();
    for line in file.lines() {
        let mut separated = line.split_whitespace();
        let semitone = separated.next().unwrap().parse::<i32>().unwrap();
        let beats = separated.next().unwrap().parse::<f32>().unwrap();
        melody.push(get_note(semitone, beats));
    }

    for _ in 0..repeat {
        melody.append(&mut melody.clone());
    }

    melody.concat()
}
