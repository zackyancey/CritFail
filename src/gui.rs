use iced::{
    button, scrollable, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Row, Sandbox, Scrollable, Settings, Space, Text,
};

mod expression_box;
mod style;
use expression_box::*;
mod result_box;
use result_box::*;

pub fn run() {
    Window::run(Settings::default());
}

#[derive(Default)]
struct Window {
    /// Entries for roll expressions
    expressions: Vec<ExpressionBox>,
    expressions_scroll: scrollable::State,
    /// Button to add an expression box
    add_button: button::State,
    /// The result of the last roll
    result_box: ResultBox,
}

#[derive(Debug, Clone)]
enum Message {
    ExpressionMessage(usize, ExpressionMessage),
    AddPressed,
}

impl Sandbox for Window {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Critfail")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExpressionMessage(i, ExpressionMessage::RollPressed) => {
                let expression = &self.expressions[i];
                let result = expression.roll();

                self.result_box
                    .update(ResultMessage::from_roll(&expression, result))
            }
            Message::ExpressionMessage(i, ExpressionMessage::DeletePressed) => {
                self.expressions.remove(i);
            }
            Message::ExpressionMessage(i, msg) => self.expressions[i].update(msg),
            Message::AddPressed => self.expressions.push(ExpressionBox::new()),
        }
    }

    fn view(&mut self) -> Element<Message> {
        if self.expressions.is_empty() {
            self.expressions.push(ExpressionBox::new())
        }

        let expressions = self.expressions.iter_mut().enumerate().fold(
            Column::new().spacing(20),
            |col, (i, exp)| {
                col.push(
                    exp.view()
                        .map(move |msg| Message::ExpressionMessage(i, msg)),
                )
            },
        );

        let add_button = Button::new(
            &mut self.add_button,
            Text::new("+")
                .size(35)
                .horizontal_alignment(HorizontalAlignment::Center)
                .width(Length::Fill),
        )
        .on_press(Message::AddPressed)
        .style(style::Button::Secondary);

        Container::new(
            Column::new()
                .max_width(1000)
                .padding(20)
                .spacing(20)
                .align_items(Align::Center)
                .push(self.result_box.view())
                .push(
                    Scrollable::new(&mut self.expressions_scroll)
                        .spacing(20)
                        .push(expressions)
                        .push(
                            Row::new()
                                .push(Space::with_width(Length::Fill))
                                .push(add_button.width(Length::FillPortion(3)))
                                .push(Space::with_width(Length::Fill)),
                        ),
                ),
        )
        .center_x()
        .into()
    }
}
