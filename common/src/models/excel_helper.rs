use xlsxwriter::{format::FormatColor, Format, Workbook};

pub struct XlsxHelper {
    pub wb: Box<Workbook>,
    pub file_name: String,
}

impl XlsxHelper {
    pub fn new() -> Self {
        let file_name = get_file_name_with_timestamp();
        let wb = Workbook::new(file_name.as_str()).unwrap();
        Self {
            wb: Box::new(wb),
            file_name,
        }
    }

    pub fn headers_format() -> Format {
        let mut format = Format::new();
        let format = format
            .set_font_name("微软雅黑")
            .set_font_size(12.0)
            .set_bold()
            .set_font_color(FormatColor::Blue);
        format.clone()
    }
    pub fn format() -> Format {
        let mut format = Format::new();
        let format = format.set_font_name("微软雅黑").set_font_size(12.0);
        format.clone()
    }
}

pub fn get_file_name_with_timestamp() -> String {
    format!(
        "db_dump_data_{}.xlsx",
        chrono::Local::now().format("%Y-%m-%d-%H%M%S")
    )
}
