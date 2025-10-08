# Contest Server Changelog

#### 0.5.0

- rename config file from `.contest.json` to `contest.json` (no longer hidden)

#### 0.4.0

- the config file is proper JSON now
- renames the "setup" command to "init"
- improve JSON-Schema for config file
- better defaults for config settings
- improve config file scaffold
- improve printing of results

#### 0.3.1

- the generated example configuration now uses globs instead of regular
  expressions

#### 0.3.0

- The new "quit" command ends the server remotely. This is useful if the
  terminal locks up.

#### 0.2.0

- expects the `line` field to be a number now
- support for comments in the config file and JSON5 file extension
- setup assistant now scaffolds options
- textual output now doesn't output a bang and can be disabled via the new
  `printResult` option

#### 0.1.0

- streamlined config file syntax
- support for running custom commands
- remove all possibilities to crash
- can now run without config file
- end-to-end tests
