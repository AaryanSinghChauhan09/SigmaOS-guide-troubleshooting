pub struct TroubleshootingGuideBook;
impl TroubleshootingGuideBook {
    pub fn get_manual_name() -> &'static str { "SigmaOS Troubleshooting & System Recovery Guide" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_troubleshooting_guide_title() {
        assert_eq!(TroubleshootingGuideBook::get_manual_name(), "SigmaOS Troubleshooting & System Recovery Guide");
    }
}
