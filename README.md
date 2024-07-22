# g

Don't you hate it when you're interacting with multiple git accounts, forget about it and end up making commits with wrong name?
g is here to solve that issue, without cumbersome management of custom hosts in you `~/.ssh/config` and switching your credentials manually each time.

# Usage
## Creating a profile
To get started, you'll need to create your first profile.
```
g profile add --name johnsmith --username "John Smith" --email john.smith@example.com
```
By default, it looks if the profile file exists - skipping this stage if it doesn't.
> Warning: Data inside existing profile is not validated against your cli arguments in this case, to edit it you'll need to use `g profile edit`

Then it attempts to generate ssh key pair for this profile - if private key already exists, public key is generated from it, otherwise both are generated from scratch.

You can also run this command with `--force` flag to re-generate everything without warning.

## Inspecting your profiles
(TODO add description about `whoami` command once added)
You can list all existing g profiles with `g profile list` - it'll show you all the names. \
To specific settings of a profile, use `g profile show <PROFILE_NAME>`.

The core feature of g is quickly jumping between your profiles. You can do it with the `su` command: `g su johnsmith`.
This configures username, email and ssh key to use for the git repository you are currently in, it fails outside repositories unless you
use `--global` flag, which tells g to modify global git config instead.

Even though `su` is also related to profile management, I've decided to put it as a separate command rather than subcommand of `profile`, because
of how often it is used.

This is just basic overview of commands, for more info run the built-in `g help`, or help for a specific command/subcommand.

# How does it work?

g stores your profiles as `.json` files in `~/.config/g-profiles/`, for example:
```json
{
  "name": "John Smith",
  "email": "john.smith@example.com"
}
```
This file is always named `<PROFILE_NAME>.json`.
Ssh keys related to the profile are stored as `~/.ssh/id_<PROFILE_NAME>` and `~/.ssh/id_<PROFILE_NAME>.pub`.
> Warning: File names matter, if you manually alter them, g won't be able to find them
