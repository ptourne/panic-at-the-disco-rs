// // this panic macro just calls the panic! macro from the standard library
// #[macro_export]
// macro_rules! panic {

//     ($($x:expr),*) => {{
//         let mp3_audio = include_bytes!("../assets/hey-look-ma-i-made-it.mp3");
//         let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
//         let source = rodio::Decoder::new(mp3_audio).unwrap();
//         stream_handle.play_raw(source.convert_samples());
//         std::thread::sleep(std::time::Duration::from_secs(5));
//         println!("🎩 Hey Look Ma, I Made It! 🕺");
//         $(
//            // call std panic!
//               std::panic!($x);
//         )*
//     }};
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use rodio::{Decoder, OutputStream, Sink};
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let mp3_audio = include_bytes!("../assets/hey-look-ma-i-made-it.mp3");
        let reader = std::io::Cursor::new(mp3_audio);
        let source = Decoder::new(reader).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}
