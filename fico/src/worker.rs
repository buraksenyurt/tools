use std::path::PathBuf;

pub fn copy_file(source: &PathBuf, destination: &PathBuf, force: bool) -> std::io::Result<()> {
    if !source.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source file `{}` does not exist", source.display()),
        ));
    }

    if destination.exists() && !force {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "Destination file `{}` already exists. Use --force to overwrite.",
                destination.display()
            ),
        ));
    }

    //todo@buraksenyurt: Implement progress bar functionality here
    std::fs::copy(source, destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::copy_file;
    use std::path::PathBuf;

    #[test]
    fn test_source_not_found() {
        let source = PathBuf::from("non_existent_file.txt");
        let destination = PathBuf::from("destination.txt");
        let result = copy_file(&source, &destination, false);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), std::io::ErrorKind::NotFound);
    }
}
