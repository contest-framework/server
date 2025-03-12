# Contest Server Configuration

Run <code type="contest-command">contest setup</code> in the root directory of
your codebase to generate Contest's configuration file
(<code type="repo/existing-file">contest.json5</code>). This file tells the
Contest server which actions it should perform when it receives messages from a
Contest client.

The `trigger` block of an action describes the command sent by the Contest
client. Contest performs pattern matching on this block. Attributes can contain
globs as placeholders.

The `run` block defines the console command to run. You can insert values
received in the `trigger` block via placeholders in the
[mustache](https://mustache.github.io) syntax.

Here is an example **contest.json** file for JavaScript developers who use
[Mocha](https://mochajs.org) for unit testing:

```json
{
  "actions": [
    {
      "type": "testAll",
      "run": "mocha"
    },
    {
      "type": "testFileLine",
      "filename": "**/*.js",
      "line": "*",
      "run": "mocha {{filename}}:{{line}}"
    }
  ]
}
```

You can also use the filename `contest.json5` if you want to use comments in the
JSON.
