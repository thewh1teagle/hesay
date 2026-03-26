/*
Prepare models:
    wget https://huggingface.co/thewh1teagle/renikud/resolve/main/model.onnx -O renikud.onnx
    wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/michael.onnx -O michael.onnx
    wget https://huggingface.co/thewh1teagle/phonikud-tts-checkpoints/resolve/main/model.config.json -O michael.onnx.json

Usage:
    cargo run --release -- <text> [output.wav]

    Both G2P and TTS models are embedded in the binary.
    If output.wav is provided, saves to file instead of playing.

Example:
    cargo run --release -- "שלום עולם" out.wav
    cargo run --release -- "אני אוהב machine learning" out.wav
*/

use anyhow::Context;
use espeak_rs::text_to_phonemes;
use ort::session::Session;
use piper_rs::{ModelConfig, Piper};
use regex::Regex;
use renikud_rs::G2P;
use std::sync::LazyLock;

static G2P_MODEL: &[u8] = include_bytes!("../renikud.onnx");
static TTS_MODEL: &[u8] = include_bytes!("../michael.onnx");
static TTS_CONFIG: &[u8] = include_bytes!("../michael.onnx.json");

static LATIN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-zA-Z]+").unwrap());

fn to_phonemes(text: &str, g2p: &mut G2P) -> anyhow::Result<String> {
    let mut result = String::new();
    let mut last = 0;
    for m in LATIN_RE.find_iter(text) {
        let hebrew = &text[last..m.start()];
        if !hebrew.is_empty() {
            result += &g2p.phonemize(hebrew)?;
        }
        let ipa = text_to_phonemes(m.as_str(), "en-us", None)
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .join(" ");
        result += &ipa;
        last = m.end();
    }
    let rest = &text[last..];
    if !rest.is_empty() {
        result += &g2p.phonemize(rest)?;
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let text = args.next().context("Usage: hebrew-speaker <text> [output.wav]")?;
    let output = args.next();

    // G2P: Hebrew+English → IPA
    let session = Session::builder()?.commit_from_memory(G2P_MODEL)?;
    let mut g2p = G2P::from_session(session)?;
    let ipa = to_phonemes(&text, &mut g2p)?;
    eprintln!("IPA: {ipa}");

    // TTS: IPA → audio
    let config: ModelConfig = serde_json::from_slice(TTS_CONFIG)?;
    let session = Session::builder()?.commit_from_memory(TTS_MODEL)?;
    let mut piper = Piper::from_session(session, config);
    let (samples, sample_rate) = piper.create(&ipa, true, None, None, None, None)?;

    match output {
        Some(path) => {
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::create(&path, spec)?;
            for s in &samples {
                writer.write_sample(*s)?;
            }
            writer.finalize()?;
            eprintln!("Saved to {path}");
        }
        None => {
            use rodio::buffer::SamplesBuffer;
            let (_stream, handle) = rodio::OutputStream::try_default()?;
            let sink = rodio::Sink::try_new(&handle)?;
            sink.append(SamplesBuffer::new(1, sample_rate, samples));
            sink.sleep_until_end();
        }
    }

    Ok(())
}
