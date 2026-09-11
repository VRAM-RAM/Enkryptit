pub mod argument;

use comfy_table::{Table};
use comfy_table::{
    presets::UTF8_FULL,
    Cell, ContentArrangement,
};

use crate::diagnostic::report::argument::ReportArgument;

pub struct Report {
    title: ReportArgument,
    fields: Vec<(ReportArgument, ReportArgument)>
}


impl Report {
    pub fn new(title: ReportArgument) -> Self {
        Self { title: title, fields: vec![] }
    }

    pub fn field(mut self, name: ReportArgument, value: ReportArgument) -> Self {
        self.fields.push((name, value));
        self
    }

    pub fn display(&self) {
        let mut table = Table::new();

        table
            .load_style(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            Cell::new(&self.title.value)
                .fg(self.title.style.color())
                .add_attribute(self.title.style.attribute()),
            Cell::new(""),
        ]);

        for (label, value) in &self.fields {
            table.add_row(vec![
                Cell::new(&label.value)
                    .fg(label.style.color())
                    .add_attribute(label.style.attribute()),

                Cell::new(&value.value)
                    .fg(value.style.color())
                    .add_attribute(value.style.attribute()),
            ]);
        }

        println!("{table}");
    }
}
