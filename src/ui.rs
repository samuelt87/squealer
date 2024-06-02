use crate::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Cell, Row as TableRow, Table, TableState};
use ratatui::Frame;
use sqlx::Column;
use sqlx::Row;
use std::error::Error;

pub fn render_ui(
    f: &mut Frame,
    app: &App,
    table_state: &mut TableState,
) -> Result<(), Box<dyn Error>> {
    let (headers, data) = extract_row_data(&app).unwrap();
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(12), Constraint::Min(5)].as_ref())
        .split(size);

    // Render input field
    f.render_widget(app.query_input.widget(), chunks[0]);

    // Render table
    render_table(f, chunks[1], &headers, &data, table_state);
    Ok(())
}

/// Execute a query from the input field and update the results.
fn render_table(
    frame: &mut Frame,
    area: Rect,
    headers: &[String],
    data: &[Vec<String>],
    state: &mut TableState,
) {
    let rows = data.iter().map(|row| {
        TableRow::new(
            row.iter()
                .map(|cell| Cell::from(cell.as_str()))
                .collect::<Vec<_>>(),
        )
    });

    let table = Table::new(rows, [15, 20, 30])
        .header(TableRow::new(
            headers
                .iter()
                .map(|h| Cell::from(h.as_str()))
                .collect::<Vec<_>>(),
        ))
        .block(Block::default().title("SQL Results").borders(Borders::ALL))
        .highlight_symbol(">>");

    frame.render_stateful_widget(table, area, state);
}

fn extract_row_data(app: &App) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    match app.results {
        None => return Err("No results to display".into()),
        Some(ref rows) => {
            let headers = rows
                .first()
                .map(|row| {
                    row.columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let data = rows
                .iter()
                .map(|row| {
                    (0..row.len())
                        .map(|i| row.try_get::<&str, _>(i).unwrap_or_default().to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Ok((headers, data))
        }
    }
}
