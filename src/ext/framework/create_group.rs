pub use ext::framework::command::{Command, CommandType, CommandGroup};
pub use ext::framework::create_command::CreateCommand;

use std::default::Default;
use std::sync::Arc;
use ::client::Context;
use ::model::Message;

#[derive(Default)]
pub struct CreateGroup(pub CommandGroup);

/// Used to create command groups
///
/// # Examples
///
/// Create group named Information where all commands are prefixed with info,
/// and add one command named "name". For example, if prefix is "~", we say "~info name"
/// to call the "name" command.
///
/// ```rust,ignore
/// framework.group("Information", |g| g
///     .prefix("info")
///     .command("name", |c| c
///         .exec_str("meew0")))
/// ```
impl CreateGroup {
    /// Adds a command to group.
    pub fn command<F>(mut self, command_name: &str, f: F) -> Self
        where F: FnOnce(CreateCommand) -> CreateCommand {
        let cmd = f(CreateCommand(Command::default())).0;

        self.0.commands.insert(command_name.to_owned(), Arc::new(cmd));

        self
    }

    /// Adds a command to group with simplified API.
    /// You can return Err(string) if there's an error.
    pub fn on<F>(mut self, command_name: &str, f: F) -> Self
        where F: Fn(&Context, &Message, Vec<String>) -> Result<(), String> + Send + Sync + 'static {
        let cmd = Arc::new(Command::new(f));

        self.0.commands.insert(command_name.to_owned(), cmd);

        self
    }

    /// If prefix is set, it will be required before all command names.
    /// For example, if bot prefix is "~" and group prefix is "image"
    /// we'd call a subcommand named "hibiki" by sending "~image hibiki".
    ///
    /// **Note**: serenity automatically puts a space after group prefix.
    pub fn prefix(mut self, desc: &str) -> Self {
        self.0.prefix = Some(desc.to_owned());

        self
    }
}
