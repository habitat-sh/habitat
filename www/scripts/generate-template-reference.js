let $RefParser = require('json-schema-ref-parser');
const stdout = process.stdout;
let lines = [];

function writeHeader() {
  lines.push(`<!-- This is a generated file, do not edit it directly. See https://github.com/habitat-sh/habitat/blob/master/www/scripts/generate-template-reference.js -->`)
  lines.push(`# Template Data`);
  lines.push('');
  lines.push(`The following settings can be used during a Chef Habitat service's lifecycle. This means that you can use these settings in any of the plan hooks, such as \`init\`, or \`run\`, and also in any templatized configuration file for your application or service.`)
  lines.push('');
  lines.push(`These configuration settings are referenced using the [Handlebars.js](https://github.com/wycats/handlebars.js/) version of [Mustache-style](https://mustache.github.io/mustache.5.html) tags.`)
  lines.push('');
}

function writeDefinitions() {
  lines.push(`### Reference Objects`);
  lines.push('');
  lines.push(`Some of the template expressions referenced above return objects of a specific shape; for example, the \`svc.me\` and \`svc.first\` expressions return "service member" objects, and the \`pkg\` property of a service member returns a "package identifier" object. These are defined below.`);
  lines.push('');

  Object.keys(schema.definitions)
    .map(key => {
      const p = schema.definitions[key];

      lines.push(`### ${key}`);
      lines.push('');
      lines.push(p.description);
      lines.push('');

      props(p.properties);
    });
}

function writeProperties() {
  Object.keys(schema.properties)
    .map(key => {
      const p = schema.properties[key];
      const properties = p.properties;
      const additional = p.additionalProperties;

      lines.push(`### ${key}`);
      lines.push('');
      lines.push(p.description);
      lines.push('');

      if (properties) {
        props(properties);
      }
      else if (additional && additional.properties) {
        props(additional.properties);
      }
    });
}

function props(collection) {
  lines.push(`| Property | Type | Description |`);
  lines.push(`| -------- | ---- | ----------- |`);

  Object.keys(collection).map(key => {
    lines.push(`| ${key} | ${getType(collection[key])} | ${collection[key].description} |`)
  });

  lines.push('');
}

function getType(prop) {
  const type = prop.type;
  const oneOf = prop.oneOf;
  const ref = prop.$ref;

  if (type) {
    return type;
  }

  if (oneOf && oneOf.length) {
    if (oneOf[0].type) {
      return oneOf[0].type;
    }

    if (oneOf[0].$ref) {
      const name = oneOf[0].$ref.split('/').pop();
      return `[${name}](#${name})`;
    }
  }

  if (ref) {
    const name = ref.split('/').pop();
    return `[${name}](#${name})`;
  }

  return '--';
}

$RefParser.bundle(process.argv[2]).then(function(deref_schema) {
    console.log("ARGV", process.argv[2]);
    schema = deref_schema;
    writeHeader();
    writeProperties();
    writeDefinitions();
    stdout.write(lines.join('\n'));
});
