use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{window, Color, Element, Length, Size, Task, Theme};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Debug, Clone)]
pub enum AppTheme {
    Light,
    Dark,
}

impl AppTheme {
    fn to_iced_theme(&self) -> Theme {
        match self {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
        }
    }
}

#[derive(Debug)]
pub struct TodoApp {
    todos: HashMap<usize, Todo>,
    input_value: String,
    next_id: usize,
    current_theme: AppTheme,
}

impl Default for TodoApp {
    fn default() -> Self {
        TodoApp {
            todos: HashMap::new(),
            input_value: String::new(),
            next_id: 1,
            current_theme: AppTheme::Light,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    AddTodo,
    ToggleTodo(usize),
    DeleteTodo(usize),
    ToggleTheme,
    ClearCompleted,
}

impl TodoApp {
    fn update(state: &mut TodoApp, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                state.input_value = value;
            }
            Message::AddTodo => {
                if !state.input_value.trim().is_empty() {
                    let todo = Todo {
                        id: state.next_id,
                        text: state.input_value.trim().to_string(),
                        completed: false,
                    };
                    state.todos.insert(state.next_id, todo);
                    state.next_id += 1;
                    state.input_value.clear();
                }
            }
            Message::ToggleTodo(id) => {
                if let Some(todo) = state.todos.get_mut(&id) {
                    todo.completed = !todo.completed;
                }
            }
            Message::DeleteTodo(id) => {
                state.todos.remove(&id);
            }
            Message::ToggleTheme => {
                state.current_theme = match state.current_theme {
                    AppTheme::Light => AppTheme::Dark,
                    AppTheme::Dark => AppTheme::Light,
                };
            }
            Message::ClearCompleted => {
                state.todos.retain(|_, todo| !todo.completed);
            }
        }
        Task::none()
    }

    fn view(state: &TodoApp) -> Element<Message> {
        let title = text("Todo List").size(36).width(Length::Fill);

        let subtitle = text("Organize your tasks efficiently")
            .size(16)
            .width(Length::Fill)
            .style(|_theme: &Theme| text::Style {
                color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
            });

        let header = column![title, subtitle]
            .spacing(5)
            .align_x(iced::Alignment::Center);

        let input = text_input("What needs to be done?", &state.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::AddTodo)
            .padding(12)
            .size(16)
            .width(Length::FillPortion(3));

        let add_button = button("Add")
            .on_press(Message::AddTodo)
            .padding([12, 20])
            .width(Length::FillPortion(1));

        let input_section = row![input, add_button]
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let theme_button = button(match state.current_theme {
            AppTheme::Light => "Dark Mode",
            AppTheme::Dark => "Light Mode",
        })
        .on_press(Message::ToggleTheme)
        .padding([8, 16]);

        let clear_button = button("Clear Completed")
            .on_press(Message::ClearCompleted)
            .padding([8, 16]);

        let controls = row![theme_button, clear_button]
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let mut todo_list = column![].spacing(8);

        if state.todos.is_empty() {
            let empty_state = container(
                column![
                    text("No todos yet!").size(24),
                    text("Add your first task above to get started")
                        .size(14)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                        }),
                ]
                .spacing(10)
                .align_x(iced::Alignment::Center),
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill);

            todo_list = todo_list.push(empty_state);
        } else {
            // Sort todos: incomplete first, then by id
            let mut todos: Vec<&Todo> = state.todos.values().collect();
            todos.sort_by_key(|todo| (todo.completed, todo.id));

            for todo in todos {
                let checkbox = checkbox("", todo.completed)
                    .on_toggle(move |_| Message::ToggleTodo(todo.id))
                    .size(20);

                let todo_text = if todo.completed {
                    text(&todo.text)
                        .size(16)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                        })
                } else {
                    text(&todo.text).size(16)
                };

                let delete_button = button("Delete")
                    .on_press(Message::DeleteTodo(todo.id))
                    .padding([6, 12]);

                let todo_row = row![checkbox, todo_text, delete_button]
                    .spacing(12)
                    .align_y(iced::Alignment::Center)
                    .width(Length::Fill);

                let todo_item = container(todo_row).padding(12).width(Length::Fill);

                todo_list = todo_list.push(todo_item);
            }
        }

        let stats = {
            let total = state.todos.len();
            let completed = state.todos.values().filter(|todo| todo.completed).count();
            let remaining = total - completed;

            let stats_text = if total == 0 {
                "Ready to add your first task!".to_string()
            } else {
                format!(
                    "Total: {} | Completed: {} | Remaining: {}",
                    total, completed, remaining
                )
            };

            container(
                text(stats_text)
                    .size(14)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
            .width(Length::Fill)
            .padding([10, 0])
            .center_x(Length::Fill)
        };

        let content = column![
            header,
            input_section,
            controls,
            container(scrollable(todo_list))
                .padding([15, 0])
                .height(Length::Fill),
            stats,
        ]
        .spacing(20)
        .padding(30)
        .max_width(700);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }

    fn theme(state: &TodoApp) -> Theme {
        state.current_theme.to_iced_theme()
    }
}

fn main() -> iced::Result {
    iced::application("Todo List", TodoApp::update, TodoApp::view)
        .theme(TodoApp::theme)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(400.0, 500.0)),
            ..Default::default()
        })
        .run()
}
