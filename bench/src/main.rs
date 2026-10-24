use hound::WavReader;

fn main() -> Result<(), hound::Error> {
    let mut reader = WavReader::open("clips/voice.wav")?;
    let spec = reader.spec();

    println!("channels:          {}", spec.channels);
    println!("sample rate:       {} Hz", spec.sample_rate);
    println!("bits/sample:       {}", spec.bits_per_sample);
    println!("sample format:     {:?}", spec.sample_format);

    let frames = reader.duration();
    let seconds = frames as f32 / spec.sample_rate as f32;
    println!("length:             {} frames ({:.2}, s", frames, seconds);

    let full_scale = (1_i32 << (spec.bits_per_sample - 1)) as f32;

    let mut samples: Vec<f32> = Vec::new();
    for sample in reader.samples::<i32>() {
        let sample = sample?;
        samples.push(sample as f32 / full_scale);
    }

    let mut peak: f32 = 0.0;
    for s in &samples {
        peak = peak.max(s.abs());
    }

    println!("samples read:  {}", samples.len());
    println!("peak:          {:.3}", peak);
    println!("first 5:       {:?}", &samples[..5]);

    Ok(())
}
