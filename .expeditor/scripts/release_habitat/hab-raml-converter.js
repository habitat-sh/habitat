//  This script will convert web api raml files for
//  habitat service sup and builder service builder-api.
//  Models are imported from RAML format and converted to
//  OAS30 JSON format.
const wap = require('webapi-parser').WebApiParser

async function main() {
  var argv = require('minimist')(process.argv.slice(2));

  const input_path = argv['i'];
  const output_path = argv['o'];

  try {
    const model = await wap.raml10.parse(`file://${input_path}`)
    await wap.oas30.generateFile(model, `file://${output_path}`)
  }
  catch(e) {
    console.log("ERROR: ", e)
  }
}

main()
