import { execFile } from "node:child_process"
import * as textRunner from "text-runner"

export function subcommand(action: textRunner.actions.Args, done: textRunner.exports.DoneFunction) {
  action.name("verify subcommand")
  const args = action.region.text().split(" ")
  switch (args.length) {
    case 0:
      done(new Error("empty block"))
      break
    case 1:
      done(new Error("no subcommand"))
      break
    case 2:
      action.name(`verify "${args[1]}"`)
      validate_subcommand(args[0], args[1], done)
      break
    default:
      done(new Error("too many args: " + args.length))
  }
}

const cucumberSortPath = "../target/debug/contest"

function validate_subcommand(executable: string, subcommand: string, done: textRunner.exports.DoneFunction) {
  execFile(cucumberSortPath, [subcommand, "--help"], (err: Error | null, stdout: string, stderr: string) => {
    if (err == null) {
      done()
    } else {
      console.log(stdout)
      console.log(stderr)
      done(new Error(`${subcommand} seems not a valid subcommand for ${executable}`))
    }
  })
}
