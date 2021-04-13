# Podstakannik

[![pipeline](
https://gitlab.com/grauwoelfchen/podstakannik/badges/trunk/pipeline.svg)](
https://gitlab.com/grauwoelfchen/podstakannik/commits/trunk) [![coverage](
https://gitlab.com/grauwoelfchen/podstakannik/badges/trunk/coverage.svg)](
https://gitlab.com/grauwoelfchen/podstakannik/commits/trunk) [![crate::podstakannik](
https://img.shields.io/crates/v/podstakannik?label=crates&style=flat)](
https://crates.io/crates/podstakannik) [![doc::podstakannik](
https://docs.rs/podstakannik/badge.svg)](https://docs.rs/crate/podstakannik)

Podstakannik (`Подстака́нник`) is a friend for your tea time ;)

This is a file/directory daemon watches personal diary posts on a local
filesystem.


## Repositories

This is mainly developed on [grauwoelfchen/podstakannik](
https://gitlab.com/grauwoelfchen/podstakannik) on GitLab.com, but the source
code is hosted also on several following repositories.

Any merge/pull requests or issues on any repository are welcomed.

* https://gitlab.com/grauwoelfchen/podstakannik
* https://github.com/grauwoelfchen/podstakannik
* https://git.sr.ht/~grauwoelfchen/podstakannik

```zsh
# the main branch is "trunk"
% git clone git@gitlab.com:grauwoelfchen/podstakannik.git
% git --no-pager branch -v
* trunk xxxxxxx XXX
```


## Install

```zsh
% cargo install podstakannik
```


## Usage

```zsh
% podstakannik

# or, please make an alias if you want since the name is a bit long...
% alias pod=podstakannik
% pod
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
publish        Publish package
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
Copyright 2020-2021 Yasuhiro Яша Asaka

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
