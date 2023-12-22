use manager_core::app_entry::{Access, AppEntry};
use ratatui::{prelude::*, widgets::*};
// use ratatui::{
//     style::{Color, Modifier, Style},
//     text::Text,
//     widgets::{Block, Borders, List, ListDirection, ListItem},
// };

// replace list and listitems with Tables
pub fn app_entry_list<'a>(entries: &Vec<AppEntry>, list_type: Access) -> List<'a> {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|ent| {
            // let mut text = Text::default();
            let icon = ent.icon.clone().unwrap_or(String::new());
            // text.extend([
            //     ent.name.clone().blue(),
            //     Span::raw(" "),
            //     format!("{:?}", ent.app_type).yellow(),
            //     Span::raw(" "),
            //     icon.clone().blue(),
            // ]);

            let text = format!(
                "{} ----- {:?} ----- {}",
                ent.name.clone(),
                ent.app_type,
                icon.clone()
            )
            .cyan();
            ListItem::new(text)
        })
        .collect();
    List::new(items)
        .block(
            Block::default()
                .title(match list_type {
                    Access::User => "User entries",
                    Access::Shared => "Shared entries",
                })
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}
