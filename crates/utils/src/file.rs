use file_format::FileFormat;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileKind {
    Jpeg,
    Png,
}

#[inline]
pub fn check_ext(filename: &str, valid: &[&str]) -> bool {
    std::path::Path::new(&filename)
        .extension()
        .and_then(|s| s.to_str())
        .map_or(false, |e| valid.iter().any(|&v| e.eq_ignore_ascii_case(v)))
}

#[inline]
pub fn check_kind(header: impl AsRef<[u8]>, valid: &[FileKind]) -> bool {
    let format = FileFormat::from_bytes(header.as_ref());
    valid.iter().any(|kind| match kind {
        FileKind::Jpeg => format == FileFormat::JointPhotographicExpertsGroup,
        FileKind::Png => format == FileFormat::PortableNetworkGraphics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ext() {
        assert!(check_ext("test.jpg", &["jpg", "jpeg"]));
        assert!(check_ext("test.PNG", &["jpg", "jpeg", "png"]));
        assert!(check_ext("test.JPeg", &["jpg", "jpeg", "png"]));
        assert!(check_ext("test.txt", &["txt"]));
        assert!(!check_ext("test.txtt", &["txt"]));
        assert!(!check_ext("test.txt", &["jpg", "jpeg"]));
        assert!(!check_ext("test", &["jpg", "jpeg"]));
    }

    #[test]
    fn test_check_kind() {
        assert!(check_kind(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], &[FileKind::Png]));
        assert!(check_kind(
            &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01],
            &[FileKind::Jpeg]
        ));
        assert!(!check_kind(
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x3A, 0x1A, 0x0A],
            &[FileKind::Jpeg, FileKind::Png]
        ));
        assert!(!check_kind(
            &[0x2F, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01],
            &[FileKind::Jpeg, FileKind::Png]
        ));
    }
}
