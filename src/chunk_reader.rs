// source:
// https://internals.rust-lang.org/t/extend-io-bufread-to-read-multiple-lines-at-once/10196
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=615e0da07bf50e56ae41664dedd0c28c

use std::io;

pub trait ChunkReader: io::BufRead {
    /// Reads and copies the underlying buffer into the provided `Vec`; does additional reads
    /// until a newline (the 0xA byte) is reached.
    fn read_chunk(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        // Read one buffer worth of data as a whole
        let readbuf = self.fill_buf()?;
        if readbuf.is_empty() {
            return Ok(0);
        }
        let mut len = readbuf.len();
        // If a newline in reached within the first 128 bytes, we need only
        // one allocation
        buf.reserve(len + 128);
        buf.extend_from_slice(readbuf);
        self.consume(len);
        len += self.read_until(b'\n', buf)?;
        Ok(len)
    }

    /// Returns an iterator over chunks of the underlying reader, delimited
    /// by newlines.
    fn chunks(self) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks { reader: self }
    }
}

impl<T> ChunkReader for T where T: io::BufRead {}

pub struct Chunks<T> {
    reader: T,
}

impl<T> Chunks<T>
where
    T: ChunkReader,
{
    #[allow(dead_code)]
    pub fn lines(self) -> impl Iterator<Item = io::Result<String>> {
        self.map(|rv| {
            rv.and_then(|v| {
                String::from_utf8(v).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }
}

impl<T> Iterator for Chunks<T>
where
    T: ChunkReader,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        match self.reader.read_chunk(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() -> io::Result<()> {
        let inp: Vec<u8> = "Foo\nBar".bytes().collect();
        let mut r = io::BufReader::with_capacity(2, &inp[..]);
        let mut buf = Vec::new();
        r.read_chunk(&mut buf)?;
        assert_eq!(&buf, &[70, 111, 111, 10]);
        r.read_chunk(&mut buf)?;
        assert_eq!(&buf, &[70, 111, 111, 10, 66, 97, 114]);
        Ok(())
    }

    #[test]
    fn chunks() -> io::Result<()> {
        let inp: Vec<u8> = "Foo\nBar".bytes().collect();
        let r = io::BufReader::with_capacity(2, &inp[..]);
        let chunks: io::Result<Vec<Vec<u8>>> = r.chunks().collect();
        assert_eq!(chunks?, vec![vec![70, 111, 111, 10], vec![66, 97, 114]]);
        Ok(())
    }

    #[test]
    fn lines() -> io::Result<()> {
        let inp: Vec<u8> = "Foo\nBar".bytes().collect();
        let r = io::BufReader::with_capacity(2, &inp[..]);
        let lines: io::Result<Vec<String>> = r.chunks().lines().collect();
        assert_eq!(&lines?, &["Foo\n", "Bar"]);
        Ok(())
    }
}
