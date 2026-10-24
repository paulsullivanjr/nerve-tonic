use hound::WavReader;

fn main() -> Result<(), hound::Error> {
    let reader = WavReader::open("clips/voice.wav")?;
    let spec = reader.spec();

    println!("channels:          {}", spec.channels);
    println!("sample rate:       {} Hz", spec.sample_rate);
    println!("bits/sample:       {}", spec.bits_per_sample);
    println!("sample format:     {:?}", spec.sample_format);

    let frames = reader.duration();
    let seconds = frames as f32 / spec.sample_rate as f32;
    println!("length:             {} framdes ({:.2}, s", frames, seconds);

    Ok(())
}
