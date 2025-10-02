# Contest Server Configuration

Run <code type="subcommand">contest setup</code> in the root directory of your
codebase to generate Contest's configuration file
(<code type="repo/existing-file">.contest.json</code>). This file tells the
Contest server which actions it should perform when it receives messages from a
Contest client.

The `trigger` block of an action describes the command sent by the Contest
client. Contest performs pattern matching on this block. Attributes can contain
globs as placeholders.

The `run` block defines the console command to run. You can insert values
received in the `trigger` block via placeholders in the
[mustache](https://mustache.github.io) syntax.

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
      "run": "node --test {{filename}}"
    }
  ]
}
```

You can use the `comment` field for human-readable comments to organize your
tests. Contest also allows JavaScript comments (starting with `//`) in the JSON
file.
