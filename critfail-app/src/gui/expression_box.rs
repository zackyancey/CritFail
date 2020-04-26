//! This module defines the element that is used for entering attacks.
use crate::gui::style;
use critfail::{AdvState, ParseError, Roll, RollExpression, RollOutcome};
use iced::{
    button, text_input, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Text,
    TextInput, VerticalAlignment,
};

#[derive(Debug, Clone)]
/// Messages that can be sent by an expression box.
pub(super) enum ExpressionMsg {
    /// The Roll button has been pressed.
    RollPressed(Option<AdvState>),
    /// The delete button has been pressed.
    DeletePressed,
    /// The value of the name box has been changed.
    NameChanged(String),
    /// The value of the roll expression box has been changed.
    RollChanged(String),
}

pub(super) struct ExpressionBox {
    /// A text input that holds the name of the attack
    name_box: text_input::State,
    name: String,
    /// A text input that holds the attack string
    roll_box: text_input::State,
    expression: String,
    roll: Result<Roll, ParseError>,
    has_adv: bool,
    /// A button to delete this expression box
    delete_button: button::State,
    /// A button to roll the attack
    roll_button: button::State,
    adv_button: button::State,
    dis_button: button::State,
}

impl Default for ExpressionBox {
    fn default() -> Self {
        Self {
            name_box: Default::default(),
            name: Default::default(),
            roll_box: Default::default(),
            expression: Default::default(),
            roll: Err(Default::default()),
            has_adv: false,
            delete_button: Default::default(),
            roll_button: Default::default(),
            adv_button: Default::default(),
            dis_button: Default::default(),
        }
    }
}

// The buttons and text entries need to be a bit smaller on web to be
// usable on mobile.
#[cfg(target_arch = "wasm32")]
const TEXTENTRY_SIZE: u16 = 30;
#[cfg(not(target_arch = "wasm32"))]
const TEXTENTRY_SIZE: u16 = 40;
const ADV_TEXT_SIZE: u16 = TEXTENTRY_SIZE / 2;

impl ExpressionBox {
    pub(super) fn new() -> Self {
        ExpressionBox::default()
    }

    pub(super) fn update(&mut self, message: ExpressionMsg) {
        match message {
            ExpressionMsg::RollPressed(_) => panic!("RollPressed should be handled upstream"),
            ExpressionMsg::DeletePressed => panic!("DeletePressed should be handled upstream"),
            ExpressionMsg::NameChanged(name) => self.name = name,
            ExpressionMsg::RollChanged(expression) => {
                self.roll = Roll::new(&expression);
                self.has_adv = self
                    .roll
                    .as_ref()
                    .map(|r| r.is_check() || r.is_attack())
                    .unwrap_or(self.has_adv);
                self.expression = expression;
            }
        }
    }

    pub(super) fn view(&mut self) -> Element<ExpressionMsg> {
        let roll_box = TextInput::new(
            &mut self.roll_box,
            "Dice",
            &self.expression,
            ExpressionMsg::RollChanged,
        )
        .size(TEXTENTRY_SIZE)
        .padding(0)
        .on_submit(ExpressionMsg::RollPressed(None));

        let name_box = TextInput::new(
            &mut self.name_box,
            "Description",
            &self.name,
            ExpressionMsg::NameChanged,
        )
        .padding(0)
        .size(TEXTENTRY_SIZE);

        let roll_button = Button::new(
            &mut self.roll_button,
            Text::new("Roll")
                .size(TEXTENTRY_SIZE)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .height(Length::Units(TEXTENTRY_SIZE * 2))
        .padding(20)
        .style(style::Button::Primary)
        .on_press(ExpressionMsg::RollPressed(None));

        let roll_button: Element<_> = if !self.has_adv {
            roll_button.into()
        } else {
            Row::new()
                .push(roll_button)
                .push(
                    Column::new()
                        .push(
                            Button::new(
                                &mut self.adv_button,
                                Text::new("Adv.")
                                    .size(ADV_TEXT_SIZE)
                                    .width(Length::Units(TEXTENTRY_SIZE))
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .padding(10)
                            .style(style::Button::Primary)
                            .on_press(ExpressionMsg::RollPressed(Some(AdvState::Advantage))),
                        )
                        .push(
                            Button::new(
                                &mut self.dis_button,
                                Text::new("Dis.")
                                    .size(ADV_TEXT_SIZE)
                                    .width(Length::Units(TEXTENTRY_SIZE))
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .padding(10)
                            .style(style::Button::Primary)
                            .on_press(ExpressionMsg::RollPressed(Some(AdvState::Disadvantage))),
                        ),
                )
                .width(Length::Shrink)
                .into()
        };

        let delete_button = Button::new(
            &mut self.delete_button,
            Text::new("x")
                .vertical_alignment(VerticalAlignment::Center)
                .size(25),
        )
        .style(style::Button::Secondary)
        .on_press(ExpressionMsg::DeletePressed);

        Row::new()
            .spacing(20)
            .align_items(Align::Start)
            .push(delete_button)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .push(name_box)
                    .push(roll_box),
            )
            .push(roll_button)
            .height(Length::Shrink)
            .into()
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn expression(&self) -> &str {
        &self.expression
    }

    pub(super) fn roll(&self, adv: Option<AdvState>) -> Result<RollOutcome, String> {
        let roll = Roll::new(&self.expression).map_err(|err| format!("{}", err))?;

        let outcome = match adv {
            Some(adv) => match roll {
                Roll::Check(check) => RollOutcome::Check(check.roll_with_advantage(adv)),
                Roll::Attack(attack) => RollOutcome::Attack(attack.roll_with_advantage(adv)),
                Roll::Damage(damage) => {
                    debug_assert!(false, "The roll with advantage/disadvantage buttons shouldn't ever be visible for a damage roll.");
                    RollOutcome::Damage(damage.roll())
                }
            },
            None => roll.roll(),
        };
        Ok(outcome)
    }
}
