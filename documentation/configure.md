# Contest Server Configuration

Run <code type="subcommand">contest init</code> in the root directory of your
codebase to generate Contest's configuration file
(<code type="repo/existing-file">.contest.json</code>). This file tells the
Contest server which actions it should perform when it receives messages from a
Contest client.

It consists of two main blocks:

- `actions` defines the activities that Contest executes for you
- `options` contains configuration settings for Contest

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

When configured this way, if the Contest server receives the `test-all` command
from a Contest client, it executes `make test`.

## test-file action

The `test-file` action allows you to execute tests specific to file types.
Assuming we have a code base that contains Go and JavaScript.

In Go, unit tests exist in files whose name ends in `_test.go`. We run them by
executing `go test <file path>`. JavaScript unit tests exist in files whose name
ends in `.test.js` and are run like this: `mocha <file path>`

Here is the corresponding Contest configuration for this setup:

```json
actions: [
  {
    "comment": "run the tests in the currently open Go test file",
    "type": "test-file",
    "files": "**/*_test.go",
    "run": "go test {{file}}"
  },

  {
    "comment": "run the tests in the currently open JavaScript file",
    "type": "test-file",
    "files": "**/*.test.js",
    "run": "mocha {{file}}"
  }
]
```

Let's go through each field.

- The `comment` field allows adding a human-friendly description of a block.
  Contest ignores it.
- The action `type` is now `test-file`, indicating that this action tests
  individual files. This action type requires that you specify the files for
  which this action applies in the `files` field.
- The `files` field contains a glob pattern that describes the files for which
  this action triggers.
- The `run` field, as usual, contains the command to run. In this case, the
  commend contains a placeholder`{{file}}` using
  [mustache](https://mustache.github.io) syntax. Contest replaces it with the
  built-in `file` variable, which contains the file path received from the
  Contest client.

With this configuration, if you have file `src/flux_test.go` open in your editor
and trigger the `Contest: test this file` action in your editor, the Contest
server will execute `go test src/flux_test.go`. But if you have file
`scripts/flim.test.js` open and trigger the same action in your editor, the
Contest server will execute `mocha scripts/flim.test.js`.

## test-file-line action

The `test-file-line` action works similar to the `test-file` action, but the
Contest client also sends the line that your cursor is currently at. This allows
you to execute one specific unit test.

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

You don't need to use the `{{line}}` variable. For example, Node's built-in test
runner currently doesn't support line numbers. You could use the
`test-file-line` action to run it with the `--test-only` flag. Here is an
example:

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

Now, to run a single test, you add `{ only: true }` to a test, trigger the
`Contest: test this line in this file` and it executes only the marked test.

## Custom variables

---

Here is an example **.contest.json** file:

```json
{
  "actions": [
    {
      "comment": "run all tests",
      "type": "test-all",
      "run": "make test"
    },
    {
      "comment": "JavaScript unit tests",
      "type": "test-file",
      "filename": "**/*.test.js",
      "line": "*",
      "run": "mocha {{filename}}"
    }
  ]
}
```

You can use the `comment` field for human-readable comments to organize your
tests. Contest also allows JavaScript comments (starting with `//`) in the JSON
file.
