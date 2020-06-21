# rust-todo-cli
Show me my todos in CLI.

# Setup
`cargo build --release` and put the binary wherever you want it.

Add the following to your `~/.bashrc` or `~/.zshrc`

Replace the alias path with the location of the binary
```bash
alias todo=path/to/rust-todo-app
todo
```

# Usage
```
todo
todo add <priority> <"Description">
todo delete <id>
```
