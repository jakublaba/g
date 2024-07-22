Don't you hate it when you're interacting with multiple git accounts, forget about it and end up making commits with
wrong name? \
g is here to solve that issue, without cumbersome management of custom hosts in you `~/.ssh/config` and switching your
credentials manually each time.

# Prerequisites

- git 2.10 or newer
- cargo 1.79 or newer

# Usage

## Creating a profile

To get started, you'll need to create your first profile.

```
g profile add --name johnsmith --username "John Smith" --email john.smith@example.com
```

By default, it looks if the profile file exists - skipping this stage if it doesn't.
> Warning: Data inside existing profile is not validated against your cli arguments in this case, to edit it you'll need
> to use `g profile edit`

Then it attempts to generate ssh key pair for this profile - if private key already exists, public key is generated from
it, otherwise both are generated from scratch.

You can also run this command with `--force` flag to re-generate everything without warning.

## Inspecting your profiles

You can list all existing g profiles with `g profile list` - it'll show you all the names. \
To specific settings of a profile, use `g profile show <PROFILE_NAME>`.

## Switching profiles

The core feature of g is quickly jumping between your profiles. You can do it with the `su` command: `g su johnsmith`.
This configures your credentials for current git repository if you run g from inside a repo, or globally otherwise.
You can still set profile globally from inside a repo by using the `--global` flag.

Even though `su` is also related to profile management, I've decided to put it as a separate command rather than
subcommand of `profile`, because of how often it is used.

You can see currently active profile with `g whoami`, it also supports `--global` flag to check the globally configured
profile.

This is just basic overview of commands, for more info run the built-in `g help`, or help for a specific
command/subcommand.

# How does it work?

g stores your profiles in `~/.config/g-profiles/`, for example:

```json
{
  "name": "John Smith",
  "email": "john.smith@example.com"
}
```

This file is always named `<PROFILE_NAME>.json`.
Ssh keys related to the profile are stored as `~/.ssh/id_<PROFILE_NAME>` and `~/.ssh/id_<PROFILE_NAME>.pub`.
Currently only ssh-ed25519 keys are supported.
> Warning: File names matter, if you manually alter them, g won't be able to find them

When switching profiles, g sets `user.name` and `user.email`.
Authorization is handled by using `core.sshCommand` setting, and because of that git is required.
