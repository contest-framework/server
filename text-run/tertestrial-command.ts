import * as tr from "text-runner"
import * as childProcess from "child_process"
import * as os from "os"
import * as util from "util"
const asyncExec = util.promisify(childProcess.exec)

export async function tertestrialCommand(action: tr.actions.Args) {
  const documented = action.region
    .text()
    .trim()
    .replace(/^tertestrial /, "")
  action.name(`Valid Tertestrial command: ${documented}`)
  const existing = await getExistingCommands()
  if (!existing.includes(documented)) {
    throw new Error(
      `Tertestrial has no command "${documented}"\n\
Known commands: ${existing.join(" | ")}`
    )
  }
}

async function getExistingCommands(): Promise<string[]> {
  const { stdout, stderr } = await asyncExec("tertestrial help")
  return (stdout.trim() + stderr.trim())
    .split(os.EOL)
    .map(line => line.match(/^- (\w+):/))
    .filter(match => match != null)
    .map(match => match[1])
}
