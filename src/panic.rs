/// Plays part of "Hey Look Ma, I Made It!" by Panic! at the Disco, prints
/// "🎩 Hey Look Ma, I Made It! 🕺" to the console, and then panics.
#[macro_export]
macro_rules! panic {
    ($($x:expr),*) => {{
        use $crate::audio_bytes::MP3_AUDIO;
        use $crate::rodio::{Decoder, OutputStream, Sink};
        let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
            println!("Failed to create output stream");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            std::panic!($($x)*);
        };
        let Ok(sink) = Sink::try_new(&stream_handle) else {
            println!("Failed to create sink");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            std::panic!($($x)*);
        };
        sink.set_volume(0.05);
        let reader = std::io::Cursor::new(MP3_AUDIO);
        let Ok(source) = Decoder::new(reader) else {
            println!("Failed to create decoder");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            std::panic!($($x)*);
        };
        sink.append(source);
        println!("🎩 Hey Look Ma, I Made It! 🕺");
        sink.sleep_until_end();
        std::panic!($($x)*);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn it_works() {
        crate::panic!("This is a test panic!");
    }
}
