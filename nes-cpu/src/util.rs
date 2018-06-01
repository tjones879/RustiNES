use std::io::{self, Read};

pub fn read_to_buf(mut buf: &mut [u8], rd: &mut Read) -> io::Result<()> {
    let mut total = 0;
    while total < buf.len() {
        let count = try!(rd.read(&mut buf[total..]));
        if count == 0 {
            // Buffer not yet filled, but EOF reached
            return Err(io::Error::new(io::ErrorKind::Other, "eof reached prematurely"))
        }
        total += count;
    }

    Ok(())
}