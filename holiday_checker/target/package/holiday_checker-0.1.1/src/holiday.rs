use serde::{ Serialize, Deserialize};
use chrono:: {Datelike, NaiveDate};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Holiday {
    pub date: String,
    pub name: String,
    pub r#type: String,
    pub categories: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HolidayData {
    pub year: u16,
    pub holidays: Vec<Holiday>
}

impl HolidayData {
    ///get all holidays
    pub fn get_all_holidays(&self) -> &Vec<Holiday> {
        &self.holidays
    }

    ///check the date whether holiday or not
    pub fn is_holiday(&self, date: &str) -> bool {
        self.holidays.iter().any(|holiday_date| holiday_date.date == date)
    }

    ///get all holidays by month
    pub fn get_holidays_by_month(&self, month: u8) -> Vec<&Holiday> {
        self.holidays.iter().filter(|holiday_date| {
            NaiveDate::parse_from_str(&holiday_date.date, "%Y-%m-%d")
            .map(|holiday_month| holiday_month.month() == month as u32)
            .unwrap_or(false)
        }).collect()
    }

    ///get all holidays by type
    pub fn get_holidays_by_type(&self, holiday_type: &str) -> Vec<&Holiday> {
        self.holidays.iter().filter(|h| h.r#type == holiday_type).collect()
    }

    ///get holidays by category
    pub fn get_holidays_by_category(&self, category: &str) -> Vec<&Holiday> {
        self.holidays.iter().filter(|h| h.categories.contains(&category.to_string())).collect()
    }

    ///get holidays by date
    pub fn get_holiday_by_date(&self, date: &str) -> Option<&Holiday> {
        self.holidays.iter().find(|h| h.date == date)
    }

    //search holidays by name
    pub fn search_holiday_by_name(&self, keyword: &str) -> Vec<&Holiday> {
        self.holidays.iter().filter(|h| h.name.to_lowercase().contains(&keyword.to_lowercase())).collect()
    }

}

///function to read from a JSON file
pub fn load_holidays_from_file(path: &str) -> Result<HolidayData, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let data: HolidayData = serde_json::from_reader(reader)?;
    Ok(data)
}