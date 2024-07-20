# git-multiaccount (gma)
> This is a placeholder name, will most likely change

gma is a cli tool for managing multiple git profiles, basic goal is to have it manage your ssh keys, usernames and emails correctly, in future I'm planning to add gpg key management

# Installation
TODO after first major version release

# Usage
> Disclaimer: this is just initial design, might change

I don't see much point in explaining each command and subcommand with flags as the help is pretty comprehensive (at least I hope so).
Here's the overview of top level commands:
```
Usage: git-multiaccount [COMMAND]

Commands:
  su       Switch profiles
  profile  Manage profiles
  clone    Clone a git repository
  help     Print this message or the help of the given subcommand(s)
```

# How does it work?
gma stores your profiles as `.json` files in `~/.config/gma-profiles/`, containing the following data:
```json
{
  "name": [name of git user],
  "email": [email of git user]
}
```
Name of the profile itself is determined by name of the file, for example `john_smith.json` corresponds to `john_smith` profile.
Upon creation of profile via gma, such config file is created, there is also `ssh-ed25519` key pair generated (so far `ed25519` is the only supported algorithm).
