use chrono::{Datelike, Local, NaiveDate};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Application, Element, Length, Settings, Theme};

struct CalendarApp {
    current_date: NaiveDate,
    selected_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
enum Message {
    PreviousMonth,
    NextMonth,
    DateSelected(NaiveDate),
    BackToCalendar,
}

impl Application for CalendarApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            CalendarApp {
                current_date: Local::now().date_naive(),
                selected_date: None,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calendar")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::PreviousMonth => {
                self.current_date =
                    self.current_date.with_day(1).unwrap() - chrono::Duration::days(1);
                self.current_date = self.current_date.with_day(1).unwrap();
            }
            Message::NextMonth => {
                self.current_date =
                    self.current_date.with_day(1).unwrap() + chrono::Duration::days(32);
                self.current_date = self.current_date.with_day(1).unwrap();
            }
            Message::DateSelected(date) => {
                self.selected_date = Some(date);
            }
            Message::BackToCalendar => {
                self.selected_date = None;
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.selected_date {
            Some(date) => self.detail_view(date),
            None => self.calendar_view(),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}

impl CalendarApp {
    fn calendar_view(&self) -> Element<Message> {
        let mut content = column![].spacing(20);

        // Month and year header
        let header = row![
            button("<").on_press(Message::PreviousMonth),
            text(format!(
                "{} {}",
                self.current_date.format("%B"),
                self.current_date.year()
            ))
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center),
            button(">").on_press(Message::NextMonth),
        ];

        content = content.push(header);

        // Days of the week
        let days = row![
            text("Sun"),
            text("Mon"),
            text("Tue"),
            text("Wed"),
            text("Thu"),
            text("Fri"),
            text("Sat")
        ];

        content = content.push(days);

        // Calendar grid
        let first_day = self.current_date.with_day(1).unwrap();
        let last_day = (self.current_date + chrono::Duration::days(32))
            .with_day(1)
            .unwrap()
            - chrono::Duration::days(1);

        let mut day =
            first_day - chrono::Duration::days(first_day.weekday().num_days_from_sunday() as i64);

        while day <= last_day {
            let mut week = row![];
            for _ in 0..7 {
                let button_style = if day.month() == self.current_date.month() {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                };

                week = week.push(
                    button(text(day.day().to_string()))
                        .style(button_style)
                        .on_press(Message::DateSelected(day)),
                );

                day = day + chrono::Duration::days(1);
            }
            content = content.push(week);
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn detail_view(&self, date: NaiveDate) -> Element<Message> {
        let content = column![
            button("Back to Calendar").on_press(Message::BackToCalendar),
            text(format!("Date: {}", date.format("%B %d, %Y"))).size(24),
            text(format!("Day of the year: {}", date.ordinal())),
            text(format!("Week number: {}", date.iso_week().week())),
            text(format!("Zodiac sign: {}", self.get_zodiac_sign(date)))
        ]
        .spacing(20);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn get_zodiac_sign(&self, date: NaiveDate) -> &'static str {
        let month = date.month();
        let day = date.day();

        match (month, day) {
            (1, 1..=19) | (12, 22..=31) => "Capricorn",
            (1, 20..=31) | (2, 1..=18) => "Aquarius",
            (2, 19..=29) | (3, 1..=20) => "Pisces",
            (3, 21..=31) | (4, 1..=19) => "Aries",
            (4, 20..=30) | (5, 1..=20) => "Taurus",
            (5, 21..=31) | (6, 1..=20) => "Gemini",
            (6, 21..=30) | (7, 1..=22) => "Cancer",
            (7, 23..=31) | (8, 1..=22) => "Leo",
            (8, 23..=31) | (9, 1..=22) => "Virgo",
            (9, 23..=30) | (10, 1..=22) => "Libra",
            (10, 23..=31) | (11, 1..=21) => "Scorpio",
            (11, 22..=30) | (12, 1..=21) => "Sagittarius",
            _ => "Unknown",
        }
    }
}

fn main() -> iced::Result {
    CalendarApp::run(Settings::default())
}
