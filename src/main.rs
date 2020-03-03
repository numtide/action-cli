use std::error::Error;
use structopt::StructOpt;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "action")]
struct Opt {
    // /// Used by post-comment
    // #[structopt(long, env = "GITHUB_TOKEN")]
    // github_token: Option<String>,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// The generic version of the other commands
    IssueCommand {
        #[structopt(long = "prop", parse(try_from_str = parse_key_val), name = "key=value")]
        properties: Vec<(String, String)>,
        command: String,
        message: String,
    },
    /// Exports an existing env variable for future actions in the job
    Export {
        /// The name of the variable to set
        key: String,
    },
    /// Registers a secret which will get masked from logs
    SetSecret {
        /// Value of the secret
        secret: String,
    },
    /// Prepends input_path to the PATH (for future actions)
    AddPath {
        input_path: String,
    },
    /// Gets the value of an input. The value is also trimmed.
    GetInput {
        name: String,
        #[structopt(long)]
        required: bool,
    },
    /// Sets the value of an output.
    SetOutput {
        /// Name of the output to set
        name: String,
        /// Value to store
        value: String,
    },
    /// Gets whether Actions Step Debug is on or not
    IsDebug,
    /// Writes debug message to user log
    Debug {
        /// Debug message
        message: String,
    },
    /// Adds an error issue
    Error {
        /// Issue message
        message: String,
    },
    /// Adds a warning issue
    Warning {
        /// Issue message
        message: String,
    },
    /// Writes info to log. This is a bit useless, it's just prints to stdout.
    Info {
        /// Info message
        message: String,
    },
    /// Begin an output group.
    ///
    /// Output until the next `groupEnd` will be foldable in this group
    StartGroup {
        /// The name of the output group
        name: String,
    },
    /// End an output group.
    EndGroup,
    /// Saves state for current action, the state can only be retrieved by this action's post job execution.
    SaveState {
        /// Name of the state to store
        name: String,
        /// Value to store
        value: String,
    },
    /// Gets the value of an state set by this action's main execution.
    GetState {
        /// Name of the state to get
        name: String,
    },
}

fn escape_data<T: AsRef<str>>(s: T) -> String {
    s.as_ref().replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
}

fn escape_property<T: AsRef<str>>(s: T) -> String {
    escape_data(s).replace(":", "%3A").replace(",", "%2C")
}

// https://github.com/actions/toolkit/blob/3261dd988308cb227481dc6580026034e5780160/packages/core/src/command.ts
fn issue_command<T, U>(command: T, message: U, properties: Vec<(String, String)>) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let mut cmd_str = format!("::{}", command.as_ref());

    if properties.len() > 0 {
        let joined_props = properties.iter().map(|(key, value)|
            format!("{}={}", key, escape_property(value))
        ).collect::<Vec<String>>().join(",");

        cmd_str = format!("{} {}", cmd_str, joined_props);
    }

    cmd_str = format!("{}::{}", cmd_str, escape_data(message.as_ref()));

    cmd_str
}

fn issue<T, U>(command: T, message: U) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    issue_command(command, message, vec!())
}

fn main() {
    let opt = Opt::from_args();

    let out = match opt.command {
        Command::IssueCommand { command, message, properties } => issue_command(&command[..], message, properties),
        Command::Export { key } => {
            match std::env::var(key.clone()) {
                Ok(val) => issue_command("set-env", val, vec!(("name".to_owned(), key))),
                Err(e) => panic!(e)
            }
        },
        Command::SetSecret { secret } => {
            issue("add-mask", secret)
        },
        Command::AddPath { input_path } => {
            match std::fs::canonicalize(input_path) {
                Ok(path) => issue("add-path", path.to_string_lossy().into_owned()),
                Err(e) => panic!(e),
            }
        },
        Command::GetInput { name, required } => {
            let key = format!("INPUT_{}", name.replace(" ", "_").to_ascii_uppercase());
            match std::env::var(key) {
                Ok(val) => val,
                Err(e) => if required { panic!(e) } else { "".to_owned() }
            }
        },
        Command::SetOutput { name, value } => {
            issue_command("set-output", value, vec!(("name".to_owned(), name)))
        },
        Command::IsDebug => {
            match std::env::var("RUNNER_DEBUG") {
                Ok(val) => val,
                Err(e) => panic!(e)
            }
        },
        Command::Debug { message } => {
            issue("debug", message)
        },
        Command::Error { message } => {
            issue("error", message)
        },
        Command::Warning { message } => {
            issue("warning", message)
        },
        Command::Info { message } => message,
        Command::StartGroup { name } => {
            issue("group", name)
        },
        Command::EndGroup => {
            issue("endgroup", "".to_owned())
        },
        Command::SaveState { name, value } => {
            issue_command("save-state", value, vec!(("name".to_owned(), name)))
        },
        Command::GetState { name } => {
            let key = format!("STATE_{}", name);
            match std::env::var(key) {
                Ok(val) => val,
                Err(_) => "".to_owned(),
            }
        },
    };

    println!("{}", out)
}
