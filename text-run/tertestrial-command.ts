import * as tr from "text-runner"
import * as childProcess from "child_process"
import * as os from "os"
import * as util from "util"
import * as path from "path"
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
  const { stdout, stderr } = await asyncExec(
    path.join(__dirname, "..", "target", "debug", "tertestrial") + " help"
  )
  const output = stdout.trim() + stderr.trim()
  let inSubcommandsSection = false
  const result = []
  const firstWordRE = /^\s*(\w+)/
  for (const line in output.split(os.EOL)) {
    if (line.startsWith("SUBCOMMANDS:")) {
      inSubcommandsSection = true
      continue
    }
    if (inSubcommandsSection) {
      const match = line.match(firstWordRE)
      result.push(match[1])
    }
  }
  console.log(result)
  return result
}
