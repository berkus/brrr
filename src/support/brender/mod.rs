use {super::Error, culpa::throws, std::io::BufRead};

pub mod actor;
pub mod material;
pub mod model;
pub mod pixelmap;
pub mod resource;

// Load a C-style 0-terminated string from the file and return it
#[throws(Error)]
pub fn read_c_string<R: BufRead>(reader: &mut R) -> String {
    let mut buf = vec![];
    /*let num_bytes =*/
    reader.read_until(0, &mut buf)?;
    buf.pop();
    let s = String::from_utf8_lossy(&buf);
    s.to_string()
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        byteorder::ReadBytesExt,
        std::io::{BufReader, Cursor},
    };

    #[test]
    fn test_read_c_string() {
        let data = Cursor::new(b"hello world\0abc");
        let mut reader = BufReader::new(data);

        let s = read_c_string(&mut reader).unwrap();
        let t = reader.read_u8().unwrap();
        let u = reader.read_u8().unwrap();
        let v = reader.read_u8().unwrap();
        assert_eq!("hello world", s);
        assert_eq!(b"a"[0], t);
        assert_eq!(b"b"[0], u);
        assert_eq!(b"c"[0], v);
    }
}
