use prettytable::{row, Cell, Row, Table};

pub(crate) trait ToTable {
    fn title() -> Row;
    fn to_table_row(&self) -> Row;
}

pub(crate) fn display<T>(resources: &Vec<T>)
where
    T: ToTable,
{
    let mut table = Table::new();
    table.set_titles(T::title());
    for res in resources {
        table.add_row(res.to_table_row());
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

impl ToTable for crate::model::RequestTable {
    fn to_table_row(&self) -> Row {
        row![
            Cell::new(&self.id.to_string()),
            Cell::new(&self.method),
            Cell::new(&self.url),
            Cell::new(&self.namespace),
        ]
    }

    fn title() -> Row {
        row![
            Cell::new("ID"),
            Cell::new("Method"),
            Cell::new("Url"),
            Cell::new("Namespace"),
        ]
    }
}

impl ToTable for crate::model::ResponseTable {
    fn title() -> Row {
        row![
            Cell::new("ID"),
            Cell::new("Status"),
            Cell::new("Response Body"),
            Cell::new("Namespace")
        ]
    }

    fn to_table_row(&self) -> Row {
        row![
            Cell::new(&self.id.to_string()),
            Cell::new(&self.status_code.to_string()),
            Cell::new(&self.body),
            Cell::new(&self.namespace),
        ]
    }
}
