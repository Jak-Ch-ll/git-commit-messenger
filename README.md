# GCM - Git Commit Messenger

There are many different ways out there how a Git commit message should look, like [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) or [Gitmoji](https://gitmoji.dev). This tool aims to make it easier to stick to per team or per project rules via flexible configuration files.

## Next Todos

Before I work on any additional features I want to work on tests and error handling.

## Features

- [ ] Configuration
  - [x] Via config file in ~/.config/gcm/config.yml
  - [ ] Via config file in project root
  - [ ] Finalize configuration schema and add it to schemastore.org
  - [ ] allow CLI-option to use custom config path
- [ ] Message
  - [ ] Gitmoji
    - [x] Input as emoji
    - [ ] Input as :text:
  - [x] Conventional commit types
  - [x] Configurable select option
  - [x] Text input
  - [x] Fixed text (literal)
  - [ ] Multi-line text
  - [ ] General pattern replacement (currently only works for Select and Text)
  - [ ] Allow editing final message before commit

## Resources

- Gitmoji from https://github.com/carloscuesta/gitmoji/blob/master/src/data/gitmojis.json
