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

<a type="json-schema">

```json
{
  "actions": [
    {
      "type": "test-all",
      "run": "make test"
    }
  ]
}
```

</a>

With this setup, when you trigger `Contest: Test everything` in your editor, the
Contest server runs `make test`.

## "test-file" action

The `test-file` action lets you define how to test individual files, depending
on their type.

Imagine a project that includes both Go and JavaScript code. In Go, test files
end with `_test.go` and are run with `go test <file path>`. In JavaScript, test
files end with `.test.js` and are run with: `mocha <file path>`.

Here is how that configuration might look:

```json
{
  "actions": [
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
}
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

The `test-file-line` action works like `test-file`, but the client also sends
the cursor line number, allowing you to run a specific test within the file.

Example:

```json
{
  "actions": [
    {
      "comment": "run the JavaScript test at the cursor",
      "type": "test-file-line",
      "files": "**/*.test.js",
      "run": "mocha {{file}}:{{line}}"
    }
  ]
}
```

If your cursor is on line 7 of `scripts/flim.test.js` and you trigger
`Contest: test this line in this file`, Contest runs `mocha scripts/flim.js:7`.

You don't always need to use `{{line}}`. For example, Node's built-in test
runner doesn't support running tests by line number. To run a single test, you
typically mark it with `{ only: true }` and then run `node --test-only <file>`.

You can still use the `test-file-line` action here:

```json
{
  "actions": [
    {
      "comment": "run the JavaScript test at the cursor",
      "type": "test-file-line",
      "files": "**/*.test.js",
      "run": "node --test-only {{file}}"
    }
  ]
}
```

Now, when you add `{ only: true}` to a test and trigger
`Contest: test this line in this file`, Contest runs only the marked test.

## Custom variables

If the built-in variables like `{{file}}` and `{{line}}` aren't enough, you can
define your own.

### Refining existing variables

Running all unit tests in a Rust file is done via `cargo test <module name>`
where `<module name>` is the filename without extension. Since Contest doesn't
provide this value, we create it ourselves:

```json
{
  "actions": [
    {
      "comment": "all unit tests in a Rust file",
      "type": "test-file",
      "files": "**/*.rs",
      "run": "cargo test {{file_without_ext}}",
      "vars": [
        {
          "name": "file_without_ext",
          "source": "file",
          "filter": "([^/]+)\\.rs$"
        }
      ]
    }
  ]
}
```

Here is what happens:

- the `vars` block defines a new variable `file_without_ext`
- `source` tells Contest where the value comes from, in this case from the
  existing `{{file}}` variable
- `filter` is a regex that extracts part of the source value, here everything
  after the last `/` and before `.rs`

So, if you have `src/parser/lexer.rs` open and run `Contest: Test this file`,
Contest runs `cargo test lexer`.

### extracting code from source files

You can also extract data from the source file itself.

To run a single Rust test, you'd do `cargo test <function name>` where
`<function name>` is the name of the test function. Let's extract that name
dynamically and call it `fn_name`:

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

Here is how it works:

1. Contest reads the content of line `{{line}}` in `{{file}}`
1. it applies the regular expression from `filter`
1. If the regex captures something, that capture becomes the value of the
   variable. If not, Contest moves up one line and repeats.

Given this Rust source:

```rs
#[test]
fn my_test() {
  // test code here
}
```

If in our editor the cursor is somewhere inside that function body, and you
trigger `Contest: test this line in this file`, Contest will execute
`cargo test my_test`.
