import Ajv from "ajv"
import { readFile } from "node:fs/promises"
import * as textRunner from "text-runner"

export async function verifyJSON(action: textRunner.actions.Args) {
  action.name("verify JSON to conform to JSON-Schema")

  // Read and parse the schema file
  var schemaText = await readFile("../documentation/schema.json", "utf-8")
  const schema = JSON.parse(schemaText)

  // Parse the JSON text from the action region
  const jsonText = action.region.text()
  const json = JSON.parse(jsonText)

  // Validate the JSON against the schema
  const ajv = new Ajv()
  const validate = ajv.compile(schema)
  const valid = validate(json)

  if (!valid) {
    throw new Error(`JSON validation failed: ${ajv.errorsText(validate.errors)}`)
  }
}
