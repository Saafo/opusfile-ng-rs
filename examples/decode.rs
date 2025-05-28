use std::{fs::read, io::Write};

use opusfile_ng::OggOpusFile;

fn main() {
    // let file = OggOpusFile::open_file(
    //     "/Users/saafo/Documents/Coding/rust-sdk/lib-oggopus-codec/tests/gs-16b-1c-44100hz.opus",
    // )
    // .unwrap();
    let mut file = OggOpusFile::open_file("/Users/saafo/Downloads/rust.opus").unwrap();
    println!("seekable: {:?}", file.seekable());
    println!("link_count: {:?}", file.link_count());
    let link_index = file.current_link().unwrap();
    println!("channel_count: {:?}", file.channel_count(link_index));
    let bitrate = file.bitrate(link_index).unwrap();
    println!("bitrate: {:?}", bitrate);
    println!("serialno: {:?}", file.serial_number_of_link(link_index));
    let pcm_total = file.pcm_total(link_index).unwrap();
    println!(
        "raw_total: {:?}, pcm_total: {:?}",
        file.raw_total(link_index),
        pcm_total
    );
    fn print_progress(file: &OggOpusFile) {
        println!(
            "raw_tell: {:?}, pcm_tell: {:?}",
            file.raw_tell(),
            file.pcm_tell()
        );
    }
    print_progress(&file);
    // file.pcm_seek((pcm_total as i64)/ 2).unwrap();
    // println!("after seek");
    // print_progress(&file);
    let mut pcm_data: Vec<u8> = Vec::new();
    let wav_header = generate_wav_header(
        file.channel_count(link_index) as u16,
        48000,
        16,
        (pcm_total * 2) as u32, // fixme
    );
    pcm_data.extend_from_slice(&wav_header);
    // // test
    // let mut buffer: Vec<i16> = vec![0; 6];
    // let read_len = file.read_stereo(&mut buffer).unwrap();
    // println!("read_len: {read_len}, buffer: {:?}", buffer);
    // let u8_data = buffer[0..read_len * 2].bitwise_to_u8();
    // println!("{:?}", u8_data);
    let mut buffer: Vec<i16> = vec![0; 1024];
    let mut total_len = 0;
    loop {
        let read_len = file.read(&mut buffer, None).unwrap();
        // println!("read_len: {read_len}, buffer: {:?}", buffer.len());
        if read_len == 0 {
            break;
        }
        let u8_data = buffer[0..read_len].bitwise_to_u8();
        pcm_data.extend_from_slice(&u8_data);
        buffer = vec![0; 1024];
        total_len += read_len;
    }
    println!("pcm_total: {pcm_total}, total_len: {total_len}");
    // write pcm_data to file
    let mut file = std::fs::File::create("/Users/saafo/Downloads/rust.wav").unwrap();
    // let mut file = std::fs::File::create("/Users/saafo/Downloads/rust.pcm").unwrap();
    file.write_all(&pcm_data).unwrap();
    // println!("read result: {:?}", ret);
    // println!("read last buffer: {:?}", buffer.last());
    // print_progress(&file);
}

trait BitwiseToU8 {
    fn bitwise_to_u8(&self) -> Vec<u8>;
}
impl BitwiseToU8 for [i16] {
    fn bitwise_to_u8(&self) -> Vec<u8> {
        let len = self.len();
        let mut vec_u8 = Vec::with_capacity(len * 2);
        for item in self {
            vec_u8.extend_from_slice(&i16::to_le_bytes(*item));
        }
        vec_u8
    }
}

fn generate_wav_header(
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    data_size: u32,
) -> Vec<u8> {
    let mut header = Vec::with_capacity(44);

    // RIFF chunk descriptor
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&(36 + data_size).to_le_bytes()); // ChunkSize
    header.extend_from_slice(b"WAVE");

    // Format sub-chunk
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes()); // SubChunk1Size
    header.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (1 = PCM)
    header.extend_from_slice(&channels.to_le_bytes()); // NumChannels
    header.extend_from_slice(&sample_rate.to_le_bytes()); // SampleRate
    header.extend_from_slice(
        &(sample_rate * (channels as u32) * (bits_per_sample as u32) / 8).to_le_bytes(),
    ); // ByteRate
    header.extend_from_slice(&((channels * bits_per_sample) / 8).to_le_bytes()); // BlockAlign
    header.extend_from_slice(&bits_per_sample.to_le_bytes()); // BitsPerSample

    // Data sub-chunk
    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_size.to_le_bytes()); // SubChunk2Size

    header
}
