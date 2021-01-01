# Podstakannik

Podstakannik (`Подстака́нник`) is a friend for your tea time ;)

This is a file/directory daemon watches personal diary posts on a local
filesystem.


## Setup

```zsh
% cargo install podstakannik
```

## Usage

```zsh
% podstakannik

# or, please make an alias if you want since the name is a bit long...
% alias post=podstakannik
% post
```

## Development

```zsh
% git clone https://gitlab.com/grauwoelfchen/podstakannik.git
% cd podstakannik
% make && make install

# debug build in .cargo/bin
% podstakannik
```

For other development targets, see the output by ``make help``.

```zsh
% make help
build:debug    Build in debug mode [alias: build]
build:release  Create release build
clean          Clean up
coverage       Generate coverage report of tests [alias: cov]
help           Display this message
install        Install a debug target into the directory same with cargo
package        Create package
test:all       Run all unit tests [alias: test]
verify:all     Check code using all verify:xxx targets [alias: verify]
verify:check   Verify code syntax [alias: check]
verify:format  Verify format without changes [alias: verify:fmt, format, fmt]
verify:lint    Verify coding style using clippy [alias: lint]
```

## License

`GPL-3.0-or-later`

```txt
Podstakannik
Copyright 2020 Yasuhiro Яша Asaka

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
