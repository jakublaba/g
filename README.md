Don't you hate it when you're interacting with multiple git accounts, forget about it and end up making commits with
wrong name? \
g is here to solve that issue, without cumbersome management of custom hosts in your `~/.ssh/config` and switching your
credentials manually each time.

# Prerequisites

- git 2.10+
- openssl 3+
- cargo 1.79+

# Installation

```
cargo install g-rs
```

> Warning: Windows is currently not supported.

# Usage

## Creating a profile

To get started, you'll need to create your first profile.

```
g profile add johnsmith --username "John Smith" --email john.smith@example.com
```

By default, g looks if the profile file exists - skipping this stage if it does.
> Warning: Data inside existing profile is not validated against your cli arguments in this case, to edit it you'll need
> to use `g profile edit`

Duplicating profile names is not allowed, using the same username + email combination for 2 different profiles is also
not allowed.

Then g generates ssh keys - if none exist, they're both generated; if private exists, public is re-generated from it.
You can also run this command with `--force` flag to overwrite profile if it exists and re-generate ssh keys.

## Inspecting your profiles

You can list all existing g profiles with `g profile list`. \
To see properties of a specific profile, use `g profile show <PROFILE_NAME>`.

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

Switching profiles doesn't do anything fancy - it just finds the correct git config and sets `user.name`, `user.email`
and `core.sshCommand` there.

Your profiles are serialized to bytes and saved under `~/.config/g-profiles/`. \
Ssh keys are stored in the standard location - `~/.ssh`. \

When using `whoami` command, g infers your identity from `user.name` and `user.email` set in detected git config.
In order to avoid scanning all profiles for that, g caches a small key-value store in `~/.config/g-profiles/.cache`.
When you remove a profile, it's also wiped from this cache.
