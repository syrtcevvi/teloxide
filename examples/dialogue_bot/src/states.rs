use teloxide::prelude::*;
use teloxide_macros::Transition;

#[derive(Transition, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    Start(StartState),
    ReceiveDaysOfWeek(ReceiveDaysOfWeekState),
    Receive10x5Answer(Receive10x5AnswerState),
    ReceiveGandalfAlternativeName(ReceiveGandalfAlternativeNameState),
}

#[derive(Default)]
pub struct StartState;

pub struct ReceiveDaysOfWeekState {
    rest: StartState,
}

pub struct Receive10x5AnswerState {
    rest: ReceiveDaysOfWeekState,
    days_of_week: u8,
}

pub struct ReceiveGandalfAlternativeNameState {
    rest: Receive10x5AnswerState,
    _10x5_answer: u8,
}

up!(
    StartState -> ReceiveDaysOfWeekState,
    ReceiveDaysOfWeekState + [days_of_week: u8] -> Receive10x5AnswerState,
    Receive10x5AnswerState + [_10x5_answer: u8] -> ReceiveGandalfAlternativeNameState,
);
