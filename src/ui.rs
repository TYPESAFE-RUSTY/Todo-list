use crate::app::App;
use crate::state::Screen;
use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(2),
            Constraint::Min(2),
            Constraint::Length(2),
        ],
    )
    .split(frame.size());

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .cyan()
            .title("TODO LIST")
            .bold(),
        main_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ],
    )
    .split(main_layout[1]);

    app.update_len(inner_layout[0].x, inner_layout[1].x, inner_layout[2].y);

    if app.state.active == Screen::COMPLETED {
        frame.render_widget(
            Paragraph::new(app.show_completed_todos()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .white()
                    .title("COMPLETED")
                    .bold(),
            ),
            inner_layout[0],
        );
    } else {
        frame.render_widget(
            Paragraph::new(app.show_completed_todos()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .cyan()
                    .title("COMPLETED")
                    .bold(),
            ),
            inner_layout[0],
        );
    }

    // render white widget if it is active else cyan
    if app.state.active == Screen::ACTIVE {
        frame.render_widget(
            Paragraph::new(app.show_active_todos()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .white()
                    .title("TODOS")
                    .bold(),
            ),
            inner_layout[1],
        );
    } else {
        frame.render_widget(
            Paragraph::new(app.show_active_todos()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .cyan()
                    .title("TODOS")
                    .bold(),
            ),
            inner_layout[1],
        );
    }

    // display different tips for different screens
    if app.state.active == Screen::ACTIVE {
        frame.render_widget(
            Paragraph::new(app.info_texts[0].to_string()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .cyan()
                    .title("KEY BINDING")
                    .bold(),
            ),
            inner_layout[2],
        );
    } else {
        frame.render_widget(
            Paragraph::new(app.info_texts[1].to_string()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .cyan()
                    .title("KEY BINDING")
                    .bold(),
            ),
            inner_layout[2],
        );
    }

    frame.render_widget(
        Paragraph::new(
            "CURRENTLY ACTIVE : ".to_owned()
                + app.state.active.get()
                + " -> "
                + app.state.mode.get(),
        )
        .block(
            Block::new()
                .borders(Borders::TOP)
                .cyan()
                .title("MISC")
                .bold(),
        )
        .bold(),
        main_layout[2],
    );
}
