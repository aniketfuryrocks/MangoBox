use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, BarChart},
    Frame, style::{Style, Color, Modifier},
};
use mango_api::fetch_price;

use crate::app::{AppBackend, AppState};

pub fn draw(frame: &mut Frame<AppBackend>, _state: &AppState) {
    let prices=fetch_price();
    let mut chatdata:[(&str,u64);30]=[("",0);30];
    let mut counter=0;
    for i in prices.as_array().unwrap(){
        chatdata[counter]=(i["baseSymbol"].as_str().unwrap_or(""),i["midPrice"].as_f64().unwrap() as u64);
        counter+=1;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
        .split(frame.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);

    let barchart=BarChart::default()
    .block(Block::default().title("BarChart").borders(Borders::ALL))
    .bar_width(10)
    .bar_gap(1)
    .bar_style(Style::default().fg(Color::Green))
    .value_style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::White))
    .data(&chatdata);
    frame.render_widget(barchart, chunks[1]);
}
