use super::ExpressionBox;
use critfail::RollOutcome;
use iced::{Align, Color, Column, Element, HorizontalAlignment, Length, Row, Space, Text};

#[derive(Clone)]
pub(super) struct ResultBox {
    name: String,
    expression: String,
    result: Result<RollOutcome, String>,
}

impl Default for ResultBox {
    fn default() -> Self {
        Self {
            name: "".into(),
            expression: Default::default(),
            result: Err(Default::default()),
        }
    }
}

pub(super) enum ResultMessage {
    RollSucceeded {
        name: String,
        expression: String,
        roll: RollOutcome,
    },
    RollError {
        name: String,
        error: String,
    },
}

impl ResultBox {
    pub(super) fn with_title(title: &str) -> Self {
        Self {
            name: title.into(),
            ..Default::default()
        }
    }

    pub(super) fn update(&mut self, message: ResultMessage) {
        match message {
            ResultMessage::RollSucceeded {
                name,
                expression,
                roll,
            } => {
                self.name = name;
                self.expression = expression;
                self.result = Ok(roll);
            }
            ResultMessage::RollError { name, error } => {
                self.name = name;
                self.result = Err(error);
            }
        }
    }

    pub(super) fn view(&mut self) -> Element<super::Message> {
        let name_text = Text::new(&self.name).horizontal_alignment(HorizontalAlignment::Left);

        let title_bar = Row::new()
            .push(Space::with_width(Length::Units(40)))
            .push(name_text.width(Length::Fill));
        let title_bar = match &self.result {
            Ok(_) => {
                let expression_text =
                    Text::new(&self.expression).horizontal_alignment(HorizontalAlignment::Right);

                title_bar
                    .push(expression_text.width(Length::Fill))
                    .push(Space::with_width(Length::Units(40)))
            }
            Err(_) => title_bar,
        };

        let result_section = match &self.result {
            Ok(roll) => Column::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(Text::new(format!("{}", roll)).size(50))
                .push(
                    Text::new(format!("{:?}", roll))
                        .size(25)
                        .color(Color::from_rgb(0.6, 0.6, 0.6)),
                ),
            Err(err) => Column::new().push(Text::new(err).size(50)),
        };

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(title_bar)
            .push(result_section)
            .height(Length::Units(160))
            .into()
    }
}

impl ResultMessage {
    pub(super) fn from_roll(expression: &ExpressionBox, roll: Result<RollOutcome, String>) -> Self {
        let name = expression.name().into();

        if expression.expression().is_empty() {
            return ResultMessage::RollError {
                name,
                error: "".into(),
            };
        };

        match roll {
            Ok(roll) => ResultMessage::RollSucceeded {
                name,
                expression: expression.expression().into(),
                roll,
            },
            Err(error) => ResultMessage::RollError { name, error },
        }
    }
}
