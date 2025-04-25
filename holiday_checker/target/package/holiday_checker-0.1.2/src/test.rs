#[cfg(test)]
mod tests {
    use super::super::holiday::*;
    
    fn sample_data() -> HolidaysData {
        HolidaysData {
            year: 2025,
            holidays: vec![
                Holiday {
                    date: "2025-01-01".into(),
                    name: "New Year".into(),
                    r#type: "religious".into(),
                    categories: vec!["public".into(), "bank".into()],
                },
            ],
        }
    }

    #[test]
    fn test_is_holiday_true() {
        let data = sample_data();
        assert!(data.is_holiday("2025-01-01"));
    }

    #[test]
    fn test_is_holiday_false() {
        let data = sample_data();
        assert!(!data.is_holiday("2025-01-02"));
    }
}
