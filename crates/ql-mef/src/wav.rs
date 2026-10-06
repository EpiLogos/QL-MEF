//! The plain WAV container the offline renderer writes: canonical 44-byte
//! header, 16-bit mono PCM — the file a host OS audio player opens without any
//! project context.

/// Canonical 44-byte-header 16-bit mono PCM WAV of produced frames; the plain
/// container a host OS audio player opens without any project context.
pub fn wav16(samples: &[f64], sample_rate: u32) -> Vec<u8> {
    let mut wav = Vec::with_capacity(44 + samples.len() * 2);
    let data_len = (samples.len() * 2) as u32;
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for x in samples {
        let scaled = (x * 32767.0).round().clamp(-32768.0, 32767.0) as i16;
        wav.extend_from_slice(&scaled.to_le_bytes());
    }
    wav
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canonical header, parsed field by field.
    struct Header {
        riff: [u8; 4],
        riff_size: u32,
        wave: [u8; 4],
        fmt: [u8; 4],
        fmt_size: u32,
        format: u16,
        channels: u16,
        sample_rate: u32,
        byte_rate: u32,
        block_align: u16,
        bits: u16,
        data: [u8; 4],
        data_size: u32,
    }

    fn parse(wav: &[u8]) -> Header {
        let word = |i: usize| u32::from_le_bytes(wav[i..i + 4].try_into().unwrap());
        let half = |i: usize| u16::from_le_bytes(wav[i..i + 2].try_into().unwrap());
        Header {
            riff: wav[0..4].try_into().unwrap(),
            riff_size: word(4),
            wave: wav[8..12].try_into().unwrap(),
            fmt: wav[12..16].try_into().unwrap(),
            fmt_size: word(16),
            format: half(20),
            channels: half(22),
            sample_rate: word(24),
            byte_rate: word(28),
            block_align: half(32),
            bits: half(34),
            data: wav[36..40].try_into().unwrap(),
            data_size: word(40),
        }
    }

    #[test]
    fn the_header_names_the_canonical_mono_sixteen_bit_container() {
        let wav = wav16(&[0.0; 5], 48_000);
        let h = parse(&wav);
        assert_eq!(&h.riff, b"RIFF");
        assert_eq!(&h.wave, b"WAVE");
        assert_eq!(&h.fmt, b"fmt ");
        assert_eq!(&h.data, b"data");
        assert_eq!(h.format, 1, "PCM");
        assert_eq!(h.channels, 1, "mono");
        assert_eq!(h.bits, 16);
        assert_eq!(h.sample_rate, 48_000);
        assert_eq!(h.fmt_size, 16);
        assert_eq!(h.byte_rate, 96_000, "one frame is two bytes a second");
        assert_eq!(h.block_align, 2);
    }

    #[test]
    fn riff_and_data_sizes_count_the_exact_sample_bytes() {
        let samples = [0.25, -0.5, 0.75];
        let wav = wav16(&samples, 22_050);
        let h = parse(&wav);
        assert_eq!(h.data_size, (samples.len() * 2) as u32);
        assert_eq!(h.riff_size, 36 + h.data_size);
        assert_eq!(wav.len(), 44 + samples.len() * 2);
        assert_eq!((wav.len() - 44) / 2, samples.len(), "one i16 per sample");
    }

    #[test]
    fn samples_are_scaled_and_clamped_into_sixteen_bits() {
        let wav = wav16(&[0.0, 1.0, -1.0, 2.0, -2.0], 48_000);
        let pcm = &wav[44..];
        let at = |i: usize| i16::from_le_bytes([pcm[2 * i], pcm[2 * i + 1]]);
        assert_eq!(at(0), 0);
        assert_eq!(at(1), 32_767, "full scale positive");
        assert_eq!(at(2), -32_767, "full scale negative");
        assert_eq!(at(3), 32_767, "out-of-range values clamp");
        assert_eq!(at(4), -32_768, "out-of-range values clamp");
    }

    #[test]
    fn a_parsed_header_round_trips_back_to_its_inputs() {
        for rate in [8_000_u32, 22_050, 44_100, 48_000, 96_000] {
            let samples = [0.5; 9];
            let wav = wav16(&samples, rate);
            let h = parse(&wav);
            assert_eq!(h.sample_rate, rate);
            assert_eq!(
                h.byte_rate,
                u32::from(h.channels) * u32::from(h.bits) * rate / 8
            );
            assert_eq!(h.block_align, h.channels * h.bits / 8);
            assert_eq!(h.riff_size as usize, wav.len() - 8);
            assert_eq!(h.data_size as usize, wav.len() - 44);
            assert_eq!(h.data_size as usize / 2, samples.len());
        }
    }
}
