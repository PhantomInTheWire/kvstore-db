#[cfg(test)]
mod integration_tests {

    use std::io::ErrorKind;
    use std::path::Path;
    use byteorder::{LittleEndian, WriteBytesExt};
    use crc32fast::Hasher;
    use libactionkv::ActionKV;

    #[test]
    fn test_insert_and_get() -> std::io::Result<()> {
        let path = Path::new("test.akv");
        let mut akv = ActionKV::open(&path)?;

        // Test insert and get
        akv.insert(b"key1", b"value1")?;
        let value = akv.get(b"key1")?;
        assert_eq!(value, Some(b"value1".to_vec()));

        // Test update
        akv.insert(b"key1", b"value2")?;
        let value = akv.get(b"key1")?;
        assert_eq!(value, Some(b"value2".to_vec()));

        // Test delete
        akv.delete(b"key1")?;
        let value = akv.get(b"key1")?;
        assert_eq!(value, None);

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                // Ignore file not found error
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_multiple_inserts() -> std::io::Result<()> {
        let path = Path::new("test_multiple.akv");
        std::fs::remove_file(&path).ok();

        let mut akv = ActionKV::open(&path)?;

        // Insert multiple values
        akv.insert(b"key1", b"value1")?;
        akv.insert(b"key2", b"value2")?;
        akv.insert(b"key3", b"value3")?;

        // Check retrieval
        let value1 = akv.get(b"key1")?;
        assert_eq!(value1, Some(b"value1".to_vec()));
        let value2 = akv.get(b"key2")?;
        assert_eq!(value2, Some(b"value2".to_vec()));
        let value3 = akv.get(b"key3")?;
        assert_eq!(value3, Some(b"value3".to_vec()));

        // Check non-existent key
        let value4 = akv.get(b"key4")?;
        assert_eq!(value4, None);

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                // Ignore file not found error
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_process_record() -> std::io::Result<()> {
        use std::io::Cursor;
        use byteorder::{LittleEndian, WriteBytesExt};
        use crc32fast::Hasher;

        let key = b"key1";
        let value = b"value1";
        let key_len = key.len() as u32;
        let val_len = value.len() as u32;
        let data: Vec<u8> = key.iter().chain(value.iter()).cloned().collect();

        let mut hasher = Hasher::new();
        hasher.update(&data);
        let checksum = hasher.finalize();

        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_u32::<LittleEndian>(checksum)?;
        buffer.write_u32::<LittleEndian>(key_len)?;
        buffer.write_u32::<LittleEndian>(val_len)?;
        buffer.extend_from_slice(&data);

        let mut data = Cursor::new(buffer);
        let kv = ActionKV::process_record(&mut data)?;
        assert_eq!(kv.key, key.to_vec());
        assert_eq!(kv.value, value.to_vec());

        Ok(())
    }

    #[test]
    fn test_seek_to_end() -> std::io::Result<()> {
        let path = Path::new("test_seek.akv");
        std::fs::remove_file(&path).ok();

        let mut akv = ActionKV::open(&path)?;

        akv.insert(b"key1", b"value1")?;
        let end_position = akv.seek_to_end()?;

        assert!(end_position > 0);

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_get_at() -> std::io::Result<()> {
        let path = Path::new("test_get_at.akv");
        std::fs::remove_file(&path).ok();

        let mut akv = ActionKV::open(&path)?;

        akv.insert(b"key1", b"value1")?;
        let position = akv.insert_but_ignore_index(b"key2", b"value2")?;

        let kv = akv.get_at(position)?;
        assert_eq!(kv.key, b"key2".to_vec());
        assert_eq!(kv.value, b"value2".to_vec());

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_insert_but_ignore_index() -> std::io::Result<()> {
        let path = Path::new("test_ignore_index.akv");
        std::fs::remove_file(&path).ok(); // Ensure file is removed before test

        let mut akv = ActionKV::open(&path)?;

        let _position = akv.insert_but_ignore_index(b"key1", b"value1")?;

        // Key should not be in the index
        let value = akv.get(b"key1")?;
        assert_eq!(value, None);

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_find() -> std::io::Result<()> {
        let path = Path::new("test_find.akv");
        std::fs::remove_file(&path).ok(); // Ensure file is removed before test

        let mut akv = ActionKV::open(&path)?;

        akv.insert_but_ignore_index(b"key1", b"value1")?;
        akv.insert_but_ignore_index(b"key2", b"value2")?;
        akv.insert_but_ignore_index(b"key3", b"value3")?;

        let found = akv.find(b"key3")?;
        assert_eq!(found, None);

        let not_found = akv.find(b"key4")?;
        assert_eq!(not_found, None);

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }

    #[test]
    fn test_load() -> std::io::Result<()> {
        let path = Path::new("test_load.akv");
        std::fs::remove_file(&path).ok(); // Ensure file is removed before test

        let mut akv = ActionKV::open(&path)?;

        akv.insert(b"key1", b"value1")?;
        akv.insert(b"key2", b"value2")?;

        // Re-open the database to trigger loading from disk
        let mut akv2 = ActionKV::open(&path)?;
        akv2.load()?;

        let value1 = akv2.get(b"key1")?;
        assert_eq!(value1, Some(b"value1".to_vec()));
        let value2 = akv2.get(b"key2")?;
        assert_eq!(value2, Some(b"value2".to_vec()));

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }
}
