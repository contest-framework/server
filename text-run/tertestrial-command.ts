import * as childProcess from "child_process";
import * as os from "os";
import * as path from "path";
import * as tr from "text-runner";
import * as util from "util";
const asyncExec = util.promisify(
  childProcess
    .exec,
);

export async function tertestrialCommand(
  action: tr.actions.Args,
) {
  const documented = action
    .region
    .text()
    .trim()
    .replace(
      /^tertestrial /,
      "",
    );
  action
    .name(
      `Valid Tertestrial command: ${documented}`,
    );
  const existing = await getExistingCommands();
  if (
    !existing
      .includes(
        documented,
      )
  ) {
    throw new Error(
      `Tertestrial has no command "${documented}"\n\
Known commands: ${
        existing
          .join(
            " | ",
          )
      }`,
    );
  }
}

async function getExistingCommands(): Promise<string[]> {
  const { stdout, stderr } = await asyncExec(
    path.join(__dirname, "..", "target", "debug", "tertestrial") + " help",
  );
  const output = stdout.trim() + stderr.trim();
  let inSubcommandsSection = false;
  const result = [];
  const firstWordRE = /^\s*(\w+)/;
  const lines = output.split(os.EOL);
  for (const line of lines) {
    if (line.startsWith("SUBCOMMANDS:")) {
      inSubcommandsSection = true;
      continue;
    }
    if (inSubcommandsSection) {
      const matches = line.match(firstWordRE);
      result.push(matches[1]);
    }
  }
  return result;
}
