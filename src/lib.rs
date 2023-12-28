// this panic macro just calls the panic! macro from the standard library
#[macro_export]
macro_rules! panic {
    ($($x:expr),*) => {{
        use rodio::{Decoder, OutputStream, Sink};
        let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
            println!("Failed to create output stream");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            $(
                  std::panic!($x);
            )*
        };
        let Ok(sink) = Sink::try_new(&stream_handle) else {
            println!("Failed to create sink");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            $(
                  std::panic!($x);
            )*
        };
        let mp3_audio = include_bytes!("../assets/hey-look-ma-i-made-it.mp3");
        let reader = std::io::Cursor::new(mp3_audio);
        let Ok(source) = Decoder::new(reader) else {
            println!("Failed to create decoder");
            println!("🎩 Hey Look Ma, I Made It! 🕺");
            $(
                  std::panic!($x);
            )*
        };
        sink.append(source);
        println!("🎩 Hey Look Ma, I Made It! 🕺");
        sink.sleep_until_end();
        $(
              std::panic!($x);
        )*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn it_works() {
        panic!("This is a test");
    }
}
