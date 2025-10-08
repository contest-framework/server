# Contest Server Configuration

Run <code type="subcommand">contest init</code> in the root directory of your
codebase to generate Contest's configuration file
(<code type="repo/existing-file">.contest.json</code>). This file tells the
Contest server which actions it should perform when it receives messages from a
Contest client.

It consists of two main blocks:

- `actions` defines the activities that Contest executes for you
- `options` contains configuration settings for Contest

Let's look at the `actions` block first.

## test-all action

The most simple action type is `test-all`. It executes the given command no
matter which file is open in your editor. Here is an example:

```json
actions: [
  {
    "type": "test-all",
    "run": "make test"
  }
]
```

When configured this way, when you choose `Contest: Test everything`, the
Contest server executes `make test`.

## test-file action

The `test-file` action allows you to execute file-type specific tests.

Assume we have a code base that contains Go and JavaScript. In Go, unit tests
exist in files whose name ends in `_test.go`. We run them by executing
`go test <file path>`. JavaScript unit tests exist in files whose name ends in
`.test.js` and in our hypothetical codebase are run like this:
`mocha <file path>`

Here is the corresponding Contest configuration for this setup:

```json
actions: [
  {
    "comment": "run all tests in the currently open Go test file",
    "type": "test-file",
    "files": "**/*_test.go",
    "run": "go test {{file}}"
  },

  {
    "comment": "run all tests in the currently open JavaScript test file",
    "type": "test-file",
    "files": "**/*.test.js",
    "run": "mocha {{file}}"
  }
]
```

- The `comment` field allows adding a human-friendly description of a block.
  Contest ignores it.
- The action `type` is now `test-file`, indicating that this action tests
  individual files. This action type requires that you specify the files for
  which this action applies in the `files` field.
- The `files` field contains a glob pattern. If this glob pattern matches the
  name of the file you have open in your editor, this action triggers.
- The `run` field, as usual, contains the command to run. In this case, the
  commend contains a placeholder`{{file}}` using
  [mustache](https://mustache.github.io) syntax. Contest replaces it with the
  built-in `file` variable, which contains the path of the file you have
  currently open in your editor.

With this configuration, if you have file `src/flux_test.go` open in your editor
and trigger the `Contest: test this file` action in your editor, the Contest
server will execute `go test src/flux_test.go`. But if you have file
`scripts/flim.test.js` open and trigger the same action, the Contest server will
execute `mocha scripts/flim.test.js`.

## test-file-line action

The `test-file-line` action works similar to the `test-file` action, but the
Contest client also sends the line that your cursor is currently at. This allows
you to execute a single specific test.

Here is how you would use this:

```json
actions: [
  {
    "comment": "run the test at the given line in the currently open JavaScript file",
    "type": "test-file-line",
    "files": "**/*.test.js",
    "run": "mocha {{file}}:{{line}}"
  }
]
```

So if you have file `scripts/flim.test.js` open in your editor at line 7, and
trigger the `Contest: test this line in this file` action, it executes
`mocha scripts/flim.js:7`.

You don't need to use the `{{line}}` variable. For example, the built-in test
runner for Node.js currently cannot look up tests by line numbers. To make it
run a single test, you need to add `{ only: true }` to that test and then
execute `node --test-only <file>`. You can still use the `test-file-line` action
here:

```json
actions: [
  {
    "comment": "run the test at the given line in the currently open JavaScript file",
    "type": "test-file-line",
    "files": "**/*.test.js",
    "run": "node --test --test-only {{file}}"
  }
]
```

Now, when you add `{ only: true}` to a test and trigger the
`Contest: test this line in this file` action in your editor, it executes only
the marked test.

## Custom variables

If the built-in variables like `file` and `line` don't work, you can create your
own variables.

### refining existing variables

To run all unit tests in a Rust file, we would need to execute
`cargo test <module name>` where `<module name>` is the name of the Rust module,
which is the same as the filename without extension. Contest doesn't provide a
variable containing the filename without extension, so let's create one
ourselves and call it `file_without_ext`:

```json
actions: [
  {
    "comment": "Rust unit tests",
    "type": "test-file",
    "files": "**/*.rs",
    "vars": [
      {
        "name": "file_without_ext",
        "source": "file",
        "filter": "([^/]+)\\.rs$"
      }
    ],
    "run": "cargo test {{file_without_ext}}"
  },
]
```

The `vars` block allows us to define new variables. Here we define one with the
name `file_without_ext`. The `source` field describes where the value for the
new variable comes from, in this case from of the existing variable `file`. The
`filter` field contains a regex that captures the part of the source value that
will be used as the value for the new variable. In this case, we take the last
part of the filename (before the extension `.rs`) that isn't a forward slash,
i.e. the basename of the path without the extension.

Now, when the client sends the command `test-file` with `src/parser/lexer.rs`,
Contest executes `cargo test lexer`.

### extracting parts of the source code file content

To run a single Rust unit test, we need to execute `cargo test {{test name}}`
where `<test name>` is the name of the test function to execute. Contest doesn't
provide the name of the current Rust function as a variable, so let's create one
ourselves and call it `fn_name`:

```json
{
  "type": "test-file-line",
  "files": "**/*.rs",
  "vars": [
    {
      "name": "fn_name",
      "source": "currentOrAboveLineContent",
      "filter": "\\bfn (\\w+)\\("
    }
  ],
  "run": "cargo test {{fn_name}}"
},
```

The `vars` block defines a new variable with name `fn_name`. As always, the
`source` field describes where the value for the new variable comes from. In
this case it says `currentOrAboveLineContent`, which means Contest will extract
the value from the given source code file! Contest follows these steps:

1. apply the regular expression from the `filter` field to the current line
2. If that captures something, it uses that capture as the variable content.
3. If it captures nothing, move to the line above and go to step 1

Assume our Rust source code looks like this:

```rs
#[test]
fn with_flux() {
  // test code here
}
```

Contest woulnd execute `cargo test with_flux`.
