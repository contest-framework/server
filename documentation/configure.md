# Contest Server Configuration

Run <code type="subcommand">contest init</code> in the root of your codebase to
create Contest's configuration file,
<code type="repo/existing-file">.contest.json</code>. This file tells the
Contest server what to do when it receives messages from a Contest client.

The configuration file has two main sections:

- `actions` defines what Contest should execute
- `options` contains general configuration settings

Let's start at the `actions` section.

## "test-all" action

The simplest action type is `test-all`. It runs the given command regardless of
which file is currently open in your editor.

Example:

```json
actions: [
  {
    "type": "test-all",
    "run": "make test"
  }
]
```

With this setup, when you trigger `Contest: Test everything` in your editor, the
Contest server runs `make test`.

## "test-file" action

The `test-file` action lets you define how to test individual files, depending
on their type.

Imagine a project that includes both Go and JavaScript code. In Go, test files
end with `_test.go` and are run with `go test <file path>`. In JavaScript, test
files end with `.test.js` and are run with: `mocha <file path>`

Here how that configuration might look:

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

A few notes:

- `comment` is an optional human-readable note, Contest ignores it.
- `type: "test-file"` indicates that this action applies to specific files.
- `files` defines a
  [glob pattern](https://en.wikipedia.org/wiki/Glob_(programming)) to match file
  paths. If the file you have open in your editor matches this pattern, Contest
  runs the corresponding action.
- `run` specifies the command to execute. You can use the `{{file}}`
  placeholder, which Contest replaces with the path of the file you have
  currently open in your editor.

With this setup:

- Opening `src/flux_test.go` in your editor and running
  `Contest: test this file` executes `go test src/flux_test.go`.
- Opening `scripts/flim.test.js` and running the same command executes
  `mocha scripts/flim.test.js`.

## "test-file-line" action

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

If the built-in variables like `file` and `line` are not enough, you can create
your own variables.

### Refining existing variables

To run all unit tests in a Rust file, we would need to execute
`cargo test <module name>` where `<module name>` is the name of the Rust module,
which is the filename without extension. Contest doesn't provide a variable
containing the filename without extension, so let's create one ourselves and
call it `file_without_ext`:

```json
actions: [
  {
    "comment": "all unit tests in a Rust file",
    "type": "test-file",
    "files": "**/*.rs",
    "run": "cargo test {{file_without_ext}}"
    "vars": [
      {
        "name": "file_without_ext",
        "source": "file",
        "filter": "([^/]+)\\.rs$"
      }
    ],
  },
]
```

The `vars` block defines new variables. In this case a variable with the name
`file_without_ext`. The `source` field describes where the content for the new
variable comes from. In this case from of the existing variable `file`. The
`filter` field contains a regex that captures the part of the source value that
will be used as the value for the new variable. In this case, we take anything
after the last forward slash until extension (`.rs`), i.e. the equivalent of
running `basename {{file}} .rs`.

Now, when the client sends the command `test-file` with `src/parser/lexer.rs`,
Contest executes `cargo test lexer`.

### extracting parts of the source code file

To run a single Rust unit test, we need to execute
`cargo test <test function name>` where `<test function name>` is the name of
the Rust function that implements the test. Contest doesn't provide the name of
the current Rust function as a variable, so let's create one ourselves and call
it `fn_name`:

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
the value from the given source code file. Contest follows these steps:

1. read the text of the line `{{line}}` in file `{{file}}`
1. apply the regular expression from the `filter` field to that text line
1. If the regex captures something, use that capture as the variable content. If
   it captures nothing, move to the line above and go to step 1.

Assume our Rust source code looks like this:

```rs
#[test]
fn with_flux() {
  // test code here
}
```

If in our editor the cursor is somewhere inside that function body, and we
trigger `Contest: test this line in this file`, Contest will execute
`cargo test with_flux`.
