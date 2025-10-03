#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::scanner::Scanner;

    #[test]
    #[ignore = "Requires network access on CI/CD"]
    fn should_port_scan_works_for_localhost() {
        let scanner = Scanner::new("127.0.0.1".parse().unwrap(), Duration::from_secs(1));
        let actual = scanner.scan_port(135);
        assert_eq!(actual.port, 135);
        assert_eq!(actual.service, Some("RPC Endpoint Mapper".into()));
    }
}
