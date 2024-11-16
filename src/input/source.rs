use super::{
    speaker_notes::SpeakerNotesCommand,
    user::{CommandKeyBindings, KeyBindingsValidationError, UserInput},
};
use crate::custom::KeyBindingsConfig;
use iceoryx2::{port::subscriber::Subscriber, service::ipc::Service};
use serde::Deserialize;
use std::{io, time::Duration};
use strum::EnumDiscriminants;

/// The source of commands.
///
/// This expects user commands as well as watches over the presentation file to reload if it that
/// happens.
pub struct CommandSource {
    user_input: UserInput,
    speaker_notes_event_receiver: Option<Subscriber<Service, SpeakerNotesCommand, ()>>,
}

impl CommandSource {
    /// Create a new command source over the given presentation path.
    pub fn new(
        config: KeyBindingsConfig,
        speaker_notes_event_receiver: Option<Subscriber<Service, SpeakerNotesCommand, ()>>,
    ) -> Result<Self, KeyBindingsValidationError> {
        let bindings = CommandKeyBindings::try_from(config)?;
        Ok(Self { user_input: UserInput::new(bindings), speaker_notes_event_receiver })
    }

    /// Try to get the next command.
    ///
    /// This attempts to get a command and returns `Ok(None)` on timeout.
    pub(crate) fn try_next_command(&mut self) -> io::Result<Option<Command>> {
        if let Some(receiver) = self.speaker_notes_event_receiver.as_mut() {
            // TODO: Handle Err instead of unwrap.
            if let Some(msg) = receiver.receive().unwrap() {
                match msg.payload() {
                    SpeakerNotesCommand::GoToSlide(idx) => {
                        return Ok(Some(Command::GoToSlide(*idx)));
                    }
                }
            }
        }
        match self.user_input.poll_next_command(Duration::from_millis(250))? {
            Some(command) => Ok(Some(command)),
            None => Ok(None),
        }
    }
}

/// A command.
#[derive(Clone, Debug, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(derive(Deserialize))]
pub(crate) enum Command {
    /// Redraw the presentation.
    ///
    /// This can happen on terminal resize.
    Redraw,

    /// Move forward in the presentation.
    Next,

    /// Move to the next slide fast.
    NextFast,

    /// Move backwards in the presentation.
    Previous,

    /// Move to the previous slide fast.
    PreviousFast,

    /// Go to the first slide.
    FirstSlide,

    /// Go to the last slide.
    LastSlide,

    /// Go to one particular slide.
    GoToSlide(u32),

    /// Render any async render operations in the current slide.
    RenderAsyncOperations,

    /// Exit the presentation.
    Exit,

    /// Suspend the presentation.
    Suspend,

    /// The presentation has changed and needs to be reloaded.
    Reload,

    /// Hard reload the presentation.
    ///
    /// Like [Command::Reload] but also reloads any external resources like images and themes.
    HardReload,

    /// Toggle the slide index view.
    ToggleSlideIndex,

    /// Toggle the key bindings config view.
    ToggleKeyBindingsConfig,

    /// Hide the currently open modal, if any.
    CloseModal,
}
