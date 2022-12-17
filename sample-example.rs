extern crate ffmpeg;

use ffmpeg::codec::{self, Codec, Context};
use ffmpeg::format::{self, Format};
use ffmpeg::media::{Type, Media};
use ffmpeg::util::frame::{self, Audio, Video};

fn main() {
    // Open the input file
    let input_format = Format::input("input.mp4").unwrap();
    let input_stream = input_format.streams().best(Type::Video).unwrap();
    let input_codec = input_stream.codec().decoder().unwrap();
    let mut input_context = input_codec.create().unwrap();

    // Open the output file
    let output_format = Format::output("output.mp4").unwrap();
    let output_stream = output_format.add_stream(input_codec).unwrap();
    let output_codec = output_stream.codec().encoder().unwrap();
    let mut output_context = output_codec.create().unwrap();

    // Set up the decoding and encoding contexts
    input_context.set_width(input_stream.width().unwrap());
    input_context.set_height(input_stream.height().unwrap());
    output_context.set_width(input_stream.width().unwrap());
    output_context.set_height(input_stream.height().unwrap());

    // Decode and encode the frames
    let mut packet = codec::packet::Packet::empty();
    let mut frame = frame::Video::empty();
    while let Ok(true) = input_format.read(&mut packet) {
        if let Ok(true) = input_context.send(&packet) {
            while let Ok(true) = input_context.receive(&mut frame) {
                output_context.send(frame.clone()).unwrap();
                while let Ok(true) = output_context.receive(&mut packet) {
                    output_format.write(&packet, &mut output_stream).unwrap();
                }
            }
        }
    }
}
