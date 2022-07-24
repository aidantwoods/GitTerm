# GitTerm

![usage](https://github.com/aidantwoods/GitTerm/raw/master/static/usage.png)

## Installation

Run:

```bash
make install
```

Place the following in `.zshrc` to enable:

```
precmd() { eval $(git-term) }
```
