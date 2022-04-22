//! Utilities for reading and writing for the Minecraft protocol

mod read;
mod write;

pub use read::{McBufReadable, McBufVarintReadable, Readable};
pub use write::{McBufVarintWritable, McBufWritable, Writable};

// const DEFAULT_NBT_QUOTA: u32 = 2097152;
const MAX_STRING_LENGTH: u16 = 32767;
// const MAX_COMPONENT_STRING_LENGTH: u32 = 262144;

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_core::resource_location::ResourceLocation;
    use std::{collections::HashMap, io::Cursor};
    use tokio::io::BufReader;

    #[test]
    fn test_write_varint() {
        let mut buf = Vec::new();
        buf.write_varint(123456).unwrap();
        assert_eq!(buf, vec![192, 196, 7]);

        let mut buf = Vec::new();
        buf.write_varint(0).unwrap();
        assert_eq!(buf, vec![0]);
    }

    #[tokio::test]
    async fn test_read_varint() {
        let mut buf = BufReader::new(Cursor::new(vec![192, 196, 7]));
        assert_eq!(buf.read_varint().await.unwrap(), 123456);
        assert_eq!(buf.get_varint_size(123456), 3);

        let mut buf = BufReader::new(Cursor::new(vec![0]));
        assert_eq!(buf.read_varint().await.unwrap(), 0);
        assert_eq!(buf.get_varint_size(0), 1);

        let mut buf = BufReader::new(Cursor::new(vec![1]));
        assert_eq!(buf.read_varint().await.unwrap(), 1);
        assert_eq!(buf.get_varint_size(1), 1);
    }

    #[tokio::test]
    async fn test_read_varint_longer() {
        let mut buf = BufReader::new(Cursor::new(vec![138, 56, 0, 135, 56, 123]));
        assert_eq!(buf.read_varint().await.unwrap(), 7178);
    }

    #[tokio::test]
    async fn test_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec!["a", "bc", "def"], |buf, s| buf.write_utf(s))
            .unwrap();

        // there's no read_list because idk how to do it in rust
        let mut buf = BufReader::new(Cursor::new(buf));

        let mut result = Vec::new();
        let length = buf.read_varint().await.unwrap();
        for _ in 0..length {
            result.push(buf.read_utf().await.unwrap());
        }

        assert_eq!(result, vec!["a", "bc", "def"]);
    }

    #[tokio::test]
    async fn test_int_id_list() {
        let mut buf = Vec::new();
        buf.write_list(&vec![1, 2, 3], |buf, i| buf.write_varint(*i))
            .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let result = buf.read_int_id_list().await.unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_map() {
        let mut buf = Vec::new();
        buf.write_map(
            vec![("a", 1), ("bc", 23), ("def", 456)],
            Vec::write_utf,
            Vec::write_varint,
        )
        .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let mut result = Vec::new();
        let length = buf.read_varint().await.unwrap();
        for _ in 0..length {
            result.push((
                buf.read_utf().await.unwrap(),
                buf.read_varint().await.unwrap(),
            ));
        }

        assert_eq!(
            result,
            vec![
                ("a".to_string(), 1),
                ("bc".to_string(), 23),
                ("def".to_string(), 456)
            ]
        );
    }

    #[tokio::test]
    async fn test_nbt() {
        let mut buf = Vec::new();
        buf.write_nbt(&azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
            "hello world".to_string(),
            azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                "name".to_string(),
                azalea_nbt::Tag::String("Bananrama".to_string()),
            )])),
        )])))
        .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        let result = buf.read_nbt().await.unwrap();
        assert_eq!(
            result,
            azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                "hello world".to_string(),
                azalea_nbt::Tag::Compound(HashMap::from_iter(vec![(
                    "name".to_string(),
                    azalea_nbt::Tag::String("Bananrama".to_string()),
                )])),
            )]))
        );
    }

    #[tokio::test]
    async fn test_long() {
        let mut buf = Vec::new();
        buf.write_long(123456).unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        assert_eq!(buf.read_long().await.unwrap(), 123456);
    }

    #[tokio::test]
    async fn test_resource_location() {
        let mut buf = Vec::new();
        buf.write_resource_location(&ResourceLocation::new("minecraft:dirt").unwrap())
            .unwrap();

        let mut buf = BufReader::new(Cursor::new(buf));

        assert_eq!(
            buf.read_resource_location().await.unwrap(),
            ResourceLocation::new("minecraft:dirt").unwrap()
        );
    }
}
