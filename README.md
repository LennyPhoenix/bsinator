# Bootstrapinator

> The one bootstrap program to rule them all.

Bootstrapinator aims to fulfil three primary functions:

- Firstly, dependency installation.
- Secondly, installation script execution.
- Thirdly, the above two in a modular form.

## Configuration

Bootstrapinator will recursively check for configuration in the following path: `$HOME/.config/bsinator/`

Configuration is divided into modules, each module must contain a name, and optionally a description, a pre-installation hook, list of dependencies and/or post-installation hook.

> **WARNING**: Modules may run arbitrary shell commands, it is *very important* to make sure you only run code that you trust. Look over any modules before installing them.

Here is an example module to install neovim with some dependencies:

```yaml
name: Neovim Setup
description: |
  Packages necessary for the neovim module.
dependencies:
- prompt: "Neovim module core packages"
  packages: [neovim, fzf, ripgrep, python-neovim]
- prompt: "Please select a terminal file browser:"
  packages: [nnn, ranger]
  requires: 1
pre: echo "Beginning neovim module installation..."
post: echo "Neovim module installation finished!"
```

A module structure will look something like this:

```
~/.config/bsinator
├── a-module
│   ├── config.yml
│   ├── script1.sh
│   └── script2.sh
└── another-module
    └── config.yml
```

The user will be asked to select from the given dependencies, the pre hook will be executed, dependencies installed, and finally the post hook executed.

### Options

- `name`: **Required** - String, given to the user in the module list.
- `description`: String, the longer text description given to the user before dependencies are listed. Not required, but recommended.
- `dependencies`: List, dependency groups to process.
- `pre`: String, pre-installation hook executed in shell before dependencies are installed.
- `post`: String, post-installation hook executed in shell after dependencies are installed.

### Dependency Groups

A dependency group may contain any of the following keys:

- `packages`: **Required** - List, must contain strings corresponding to package names.
- `prompt`: String, outputted to the user before they choose from the package list. Default: `""`
- `requires`: Int, the minimum number of packages the user must choose for the group, `-1` indicates all must to be installed. Default: `-1`
- `asdeps`: Bool, whether to install the packages with the `--asdeps` flag. Default: `False`

## License

Program is [MIT licensed](LICENSE).
