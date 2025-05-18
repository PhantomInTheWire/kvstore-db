#[cfg(test)]
mod tests {

    use std::path::Path;
    use std::io::ErrorKind;
    use crate::ActionKV;

    #[test]
    fn test_insert_and_get() -> std::io::Result<()> {
        let path = Path::new("test.akv");
        let mut akv = ActionKV::open(&path)?;

        akv.insert(b"key1", b"value1")?;
        let value = akv.get(b"key1")?;
        assert_eq!(value, Some(b"value1".to_vec()));

        std::fs::remove_file(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                // Ignore file not found error
                println!("Test file not found, ignoring error");
            }
            e
        })?;

        Ok(())
    }
}