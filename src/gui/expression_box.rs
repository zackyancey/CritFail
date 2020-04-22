//! This module defines the element that is used for entering attacks.
use critfail::{Roll, RollExp, Rollable};
use iced::{
    button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput,
    VerticalAlignment,
};

#[derive(Debug, Clone)]
/// Messages that can be sent by an expression box.
pub(super) enum ExpressionMessage {
    /// The Roll button has been pressed.
    RollPressed,
    /// The delete button has been pressed.
    DeletePressed,
    /// The value of the name box has been changed.
    NameChanged(String),
    /// The value of the roll expression box has been changed.
    RollChanged(String),
}

#[derive(Default)]
pub(super) struct ExpressionBox {
    /// A text input that holds the name of the attack
    name_box: text_input::State,
    name: String,
    /// A text input that holds the attack string
    roll_box: text_input::State,
    roll: String,
    /// A button to delete this expression box
    delete_button: button::State,
    /// A button to roll the attack
    roll_button: button::State,
}

impl ExpressionBox {
    pub(super) fn new() -> Self {
        ExpressionBox::default()
    }

    pub(super) fn update(&mut self, message: ExpressionMessage) {
        match message {
            ExpressionMessage::RollPressed => panic!("RollPressed should be handled upstream"),
            ExpressionMessage::DeletePressed => panic!("DeletePressed should be handled upstream"),
            ExpressionMessage::NameChanged(name) => self.name = name,
            ExpressionMessage::RollChanged(roll) => self.roll = roll,
        }
    }

    pub(super) fn view(&mut self) -> Element<ExpressionMessage> {
        let roll_box = TextInput::new(
            &mut self.roll_box,
            "Attack roll",
            &self.roll,
            ExpressionMessage::RollChanged,
        )
        .size(50)
        .on_submit(ExpressionMessage::RollPressed);

        let name_box = TextInput::new(
            &mut self.name_box,
            "Attack name",
            &self.name,
            ExpressionMessage::NameChanged,
        )
        .size(50);

        let roll_button = Button::new(
            &mut self.roll_button,
            Text::new("Roll")
                .size(50)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .on_press(ExpressionMessage::RollPressed)
        .height(Length::Units(100))
        .padding(20);

        let delete_button = Button::new(
            &mut self.delete_button,
            Text::new("x").vertical_alignment(VerticalAlignment::Center),
        )
        .on_press(ExpressionMessage::DeletePressed);

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
            .into()
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn expression(&self) -> &str {
        &self.roll
    }

    pub(super) fn roll(&self) -> Result<Roll, String> {
        let rollexp: RollExp = self.roll.parse().map_err(|err| format!("{}", err))?;
        Ok(rollexp.roll())
    }
}
