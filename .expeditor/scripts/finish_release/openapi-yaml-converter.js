import { writeFile, access, constants } from 'node:fs/promises';
import minimist from 'minimist';
import OASNormalize from 'oas-normalize';

const argv = minimist(process.argv.slice(2));
const input_path = argv['i'];
const output_path = argv['o'];

// Input validation
function validateArgs() {
  if (!input_path) {
    console.error('Error: Input path (-i) is required');
    console.error('Usage: node openapi-yaml-converter.js -i <input> -o <output>');
    process.exit(1);
  }

  if (!output_path) {
    console.error('Error: Output path (-o) is required');
    console.error('Usage: node openapi-yaml-converter.js -i <input> -o <output>');
    process.exit(1);
  }
}

async function checkFileExists(filePath) {
  try {
    await access(filePath, constants.F_OK);
  } catch (error) {
    console.error(`Error: Input file does not exist: ${filePath}`);
    process.exit(1);
  }
}

async function convertOpenAPI() {
  try {
    // Check if input file exists
    await checkFileExists(input_path);

    const oas = new OASNormalize(input_path, {
      colorizeErrors: true,
      enablePaths: true
    });

    // Validate the OpenAPI specification
    console.log('Validating OpenAPI specification...');
    await oas.validate();
    console.log('✓ Validation successful');

    // Convert the specification
    console.log('Converting specification...');
    const definition = await oas.convert();

    // Write the output file
    console.log(`Writing to ${output_path}...`);
    await writeFile(
      output_path,
      JSON.stringify(definition, null, 2),
      'utf8'
    );

    console.log(`✓ Successfully wrote to ${output_path}`);

  } catch (error) {
    console.error('Error during conversion:');
    console.error(error.message || error);
    process.exit(1);
  }
}

async function main() {
  validateArgs();
  await convertOpenAPI();
}

// Handle unhandled promise rejections
process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Promise Rejection:', reason);
  process.exit(1);
});

// Run the script
main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});
