# Tertestrial Server Configuration

Run <code type="tertestrial-command">tertestrial setup</code> in the root
directory of your codebase to generate Tertestrial's configuration file
(<code type="repo/existing-file">.testconfig.json</code>). This file tells the
Tertestrial server which actions it should perform when it receives messages
from a Tertestrial client.

The `trigger` block of an action describes the command sent by the Tertestrial
client. Tertestrial performs pattern matching on this block. Attributes can
contain globs as placeholders.

The `run` block defines the console command to run. You can insert values
received in the `trigger` block via placeholders in the
[mustache](https://mustache.github.io) syntax.

Here is an example **.testconfig.json** file for JavaScript developers who use
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

You can also use the filename `.testconfig.json5` if you want to use comments in
the JSON.
