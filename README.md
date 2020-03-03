# actions-cli - GitHub actions without JavaScript

Some weird people (like me) enjoy writing Bash more than JavaScript.

This little tool wraps common tasks that one would do with GitHub actions and
is currently only supported by https://github.com/actions/toolkit/

This first version wraps all of the logging commands from https://help.github.com/en/actions/reference/development-tools-for-github-actions#logging-commands
## Usage

TODO: add one-liner installation

`$ actions-cli --help`
```
actions-cli 0.1.0

USAGE:
    actions-cli <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    add-mask         Mask a value in log
    add-path         Add a system path
    debug            Set a debug message
    end-group        End an output group
    error            Set an error message
    export           Like set-env but exports an existing environment variable
    get-input        Gets the value of an input. The value is also trimmed
    get-state        Gets the value of an state set by this action's main execution
    help             Prints this message or the help of the given subcommand(s)
    is-debug         Gets whether Actions Step Debug is on or not
    issue-command    The generic version of the other commands
    save-state       Saves state for current action, the state can only be retrieved by this action's post job
                     execution
    set-env          Set and environment variable for future actions in the job
    set-output       Set an output parameter
    start-group      Begin an output group
    warning          Set a warning message
```

## Example

`$ actions-cli add-path ~root`
```
::add-path::/root
```

## Missing features

* pull-request post-comment <comment>: a command that posts a comment to the
    current pull-request.

