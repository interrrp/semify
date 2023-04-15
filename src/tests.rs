#[cfg(test)]
mod tests {
    use super::super::add_semicolon;

    #[test]
    fn test_add_semicolon() {
        assert_eq!(add_semicolon("x = 2"), "x = 2;");
        assert_eq!(add_semicolon("def main():"), "def main():");
        assert_eq!(add_semicolon("    "), "    ");
        assert_eq!(add_semicolon(""), "");
    }
}
