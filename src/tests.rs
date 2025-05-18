#[cfg(test)]
mod tests {
    use crate::ActionKV;
    use byteorder::{LittleEndian, WriteBytesExt};
    use crc32fast::Hasher;
    use std::io::Cursor;

    #[test]
    fn test_process_record() -> std::io::Result<()> {
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
    fn test_process_record_invalid_checksum() -> std::io::Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};
        use crc32fast::Hasher;
        use std::io::Cursor;

        let key = b"key1";
        let value = b"value1";
        let key_len = key.len() as u32;
        let val_len = value.len() as u32;
        let data: Vec<u8> = key.iter().chain(value.iter()).cloned().collect();

        let mut hasher = Hasher::new();
        hasher.update(&data);
        let mut checksum = hasher.finalize();
        checksum += 1; // Invalidate the checksum

        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_u32::<LittleEndian>(checksum)?;
        buffer.write_u32::<LittleEndian>(key_len)?;
        buffer.write_u32::<LittleEndian>(val_len)?;
        buffer.extend_from_slice(&data);

        let mut data = Cursor::new(buffer);
        let result = ActionKV::process_record(&mut data);

        match result {
            Ok(_) => panic!("Expected an error due to invalid checksum"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
        }

        Ok(())
    }
    #[test]
    fn test_open() -> std::io::Result<()> {
        use std::fs;
        use std::path::Path;

        let path = Path::new("test_open.akv");
        // Clean up the file if it exists from a previous run
        if path.exists() {
            fs::remove_file(path)?;
        }

        let akv = ActionKV::open(path)?;
        assert!(akv.index.is_empty());

        // Clean up the created file
        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_insert_and_get() -> std::io::Result<()> {
        use std::fs;
        use std::path::Path;

        let path = Path::new("test_insert_and_get.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key1 = b"key1";
        let value1 = b"value1";
        let key2 = b"key2";
        let value2 = b"value2";

        akv.insert(key1, value1)?;
        akv.insert(key2, value2)?;

        let retrieved_value1 = akv.get(key1)?;
        assert!(retrieved_value1.is_some());
        assert_eq!(retrieved_value1.unwrap(), value1.to_vec());

        let retrieved_value2 = akv.get(key2)?;
        assert!(retrieved_value2.is_some());
        assert_eq!(retrieved_value2.unwrap(), value2.to_vec());

        let non_existent_key = b"non_existent";
        let retrieved_non_existent = akv.get(non_existent_key)?;
        assert!(retrieved_non_existent.is_none());

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_load() -> std::io::Result<()> {
        use std::fs;
        use std::path::Path;

        let path = Path::new("test_load.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key1 = b"key1";
        let value1 = b"value1";
        let key2 = b"key2";
        let value2 = b"value2";

        akv.insert(key1, value1)?;
        akv.insert(key2, value2)?;

        // Close and reopen the database to test loading
        drop(akv);

        let mut akv_loaded = ActionKV::open(path)?;
        akv_loaded.load()?;

        let retrieved_value1 = akv_loaded.get(key1)?;
        assert!(retrieved_value1.is_some());
        assert_eq!(retrieved_value1.unwrap(), value1.to_vec());

        let retrieved_value2 = akv_loaded.get(key2)?;
        assert!(retrieved_value2.is_some());
        assert_eq!(retrieved_value2.unwrap(), value2.to_vec());

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_delete() -> std::io::Result<()> {
        use std::fs;
        use std::path::Path;

        let path = Path::new("test_delete.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key = b"key_to_delete";
        let value = b"value_to_delete";

        akv.insert(key, value)?;
        let retrieved_value = akv.get(key)?;
        assert!(retrieved_value.is_some());
        assert_eq!(retrieved_value.unwrap(), value.to_vec());

        akv.delete(key)?;
        let retrieved_value_after_delete = akv.get(key)?;
        assert!(retrieved_value_after_delete.is_none());

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_update() -> std::io::Result<()> {
        use std::fs;
        use std::path::Path;

        let path = Path::new("test_update.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key = b"key_to_update";
        let initial_value = b"initial_value";
        let updated_value = b"updated_value";

        akv.insert(key, initial_value)?;
        let retrieved_initial_value = akv.get(key)?;
        assert!(retrieved_initial_value.is_some());
        assert_eq!(retrieved_initial_value.unwrap(), initial_value.to_vec());

        akv.update(key, updated_value)?;
        let retrieved_updated_value = akv.get(key)?;
        assert!(retrieved_updated_value.is_some());
        assert_eq!(retrieved_updated_value.unwrap(), updated_value.to_vec());

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_seek_to_end() -> std::io::Result<()> {
        use std::fs;
        use std::io::Seek;
        use std::path::Path;

        let path = Path::new("test_seek_to_end.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key = b"key";
        let value = b"value";

        akv.insert(key, value)?;
        let pos1 = akv.seek_to_end()?;

        akv.insert(key, value)?;
        let pos2 = akv.seek_to_end()?;

        assert!(pos2 > pos1);

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn test_get_at() -> std::io::Result<()> {
        use std::fs;
        use std::io::Seek;
        use std::path::Path;

        let path = Path::new("test_get_at.akv");
        if path.exists() {
            fs::remove_file(path)?;
        }

        let mut akv = ActionKV::open(path)?;
        let key1 = b"key1";
        let value1 = b"value1";
        let key2 = b"key2";
        let value2 = b"value2";

        akv.insert(key1, value1)?;
        akv.insert(key2, value2)?;

        let pos1;
        let pos2;
        {
            let index = &akv.index;
            pos1 = *index.get(&key1[..]).unwrap();
            pos2 = *index.get(&key2[..]).unwrap();
        }

        let retrieved_kv1 = akv.get_at(pos1)?;
        assert_eq!(retrieved_kv1.key, key1.to_vec());
        assert_eq!(retrieved_kv1.value, value1.to_vec());

        let retrieved_kv2 = akv.get_at(pos2)?;
        assert_eq!(retrieved_kv2.key, key2.to_vec());
        assert_eq!(retrieved_kv2.value, value2.to_vec());

        fs::remove_file(path)?;
        Ok(())
    }
}
