# gitdentity v1.0.1
You ever have multiple Git identities and you need to switch between them. This program stores git identities in a database and lets you switch between them with a short ID name so you don't have to repeat `git config --global` commands every time you want to commit with another git identity.

## Installation
This program will be published to [crates.io](https://crates.io/crates/gitdentity), so to install the program, run
```
cargo install gitdentity
```
## Usage
### CLI
This program comes with a CLI manager for managing your identities. You can run it by not giving the program any arguments. Here is what the CLI looks like.
```
Gitdentity
Current Identities:
------------------
1 | example | Example Name | example@example.net
2 | example2 | Example Person | example.person@example.net
1) Add an Identity
2) Delete an Identity
3) Exit this program
> : 
```
You can add an identity with `1` and it will write the identity to the SQLite database. You can delete an identity with `2` and it will delete the identity out of the database. You can exit the program with `3`.

### Arguments
When you give the command an argument, it querys the database and writes the identity name and email to the .gitconfig file located in your Home directory.
```
gitdentity example
```
And it prints out this message
```
I have wrote the example identity to /home/example/.gitconfig
```
## Requirements
[rustqlite](https://github.com/rusqlite/rusqlite) requires you to have SQLite installed on your computer by default. If you don't want to use your local copy and want to use a bundled copy, run this command when installing gitdentity.
```
cargo install gitdentity --features=bundled
```
