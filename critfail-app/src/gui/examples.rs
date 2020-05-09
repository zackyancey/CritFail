use crate::gui::style;
use crate::gui::{Message, ResultBox, ResultMessage};
use critfail::{AdvState, Roll, RollExpression};
use iced::{button, Button, Color, Column, Element, Length, Row, Text};

#[derive(Debug, Clone, Copy)]
pub enum SectionId {
    Main,
    Check,
    Damage,
    Attack,
}

pub(super) struct ExampleGroup {
    section: SectionId,
    examples: Vec<Example>,
    result: ResultBox,
}

pub(super) struct Example {
    description: &'static str,
    expression: &'static str,
    roll: Roll,
    roll_button: button::State,
    adv_button: button::State,
    dis_button: button::State,
}

#[derive(Clone)]
pub enum ExampleGroupMessage {
    Roll(usize, Option<AdvState>),
}

#[derive(Clone)]
pub enum ExampleMessage {
    Roll(Option<AdvState>),
}

impl ExampleGroup {
    pub fn new(section: SectionId) -> Self {
        Self {
            section,
            examples: Default::default(),
            result: ResultBox::with_title("Click one of the buttons above to see results here"),
        }
    }

    pub fn push(mut self, example: Example) -> Self {
        self.examples.push(example);
        self
    }

    pub fn view(&mut self) -> Element<Message> {
        let section = self.section;

        self.examples
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(20), |col, (i, ex)| {
                col.push(
                    ex.view().map(move |ExampleMessage::Roll(adv)| {
                        Message::ExampleRolled(section, i, adv)
                    }),
                )
            })
            .push(self.result.view())
            .into()
    }

    pub fn update(&mut self, msg: ExampleGroupMessage) {
        match msg {
            ExampleGroupMessage::Roll(i, adv) => self.result.update(self.examples[i].roll(adv)),
        }
    }
}

impl Example {
    pub fn new(description: &'static str, expression: &'static str) -> Self {
        Self {
            description,
            expression,
            roll: expression.parse().unwrap(),
            roll_button: Default::default(),
            adv_button: Default::default(),
            dis_button: Default::default(),
        }
    }

    pub fn view(&mut self) -> Element<ExampleMessage> {
        let mut buttons = Row::new()
            .spacing(10)
            .push(Text::new(self.expression).width(Length::Units(300)))
            .push(
                Button::new(&mut self.roll_button, Text::new("Roll"))
                    .style(style::Button::Primary)
                    .on_press(ExampleMessage::Roll(None)),
            );

        match self.roll {
            Roll::Check(_) | Roll::Attack(_) => {
                buttons = buttons
                    .push(
                        Button::new(&mut self.adv_button, Text::new("Adv."))
                            .style(style::Button::Primary)
                            .on_press(ExampleMessage::Roll(Some(AdvState::Advantage))),
                    )
                    .push(
                        Button::new(&mut self.dis_button, Text::new("Dis."))
                            .style(style::Button::Primary)
                            .on_press(ExampleMessage::Roll(Some(AdvState::Disadvantage))),
                    );
            }
            _ => (),
        };

        Column::new()
            .spacing(10)
            .push(Text::new(self.description).color(Color::from_rgb(0.6, 0.6, 0.6)))
            .push(buttons)
            .into()
    }

    pub fn roll(&self, adv: Option<AdvState>) -> ResultMessage {
        let outcome = match adv {
            Some(adv) => match &self.roll {
                Roll::Check(check) => check.roll_with_advantage(adv).into(),
                Roll::Attack(attack) => attack.roll_with_advantage(adv).into(),
                Roll::Damage(damage) => {
                    debug_assert!(false, "The roll with advantage/disadvantage buttons shouldn't ever be visible for a damage roll.");
                    damage.roll().into()
                }
            },
            None => self.roll.roll(),
        };

        ResultMessage::RollSucceeded {
            name: self.expression.into(),
            expression: "".into(),
            roll: outcome,
        }
    }
}
