const { spawnSync } = require('child_process');
const { platform } = require('os');

function getHelp(command, sub) {
  const proc = runCommand(command, ['-h']);

  function render(data) {
    parsed = parseOutput(command, data.replace(/`/g, ''));
    console.log(markdownForCommand(parsed, sub));

    parsed.subcommands.forEach(item => {
      getHelp(`${item.parent} ${item.command}`, item.parent !== 'hab');
    });
  }

  render(proc.stdout.toString());
}

function runCommand(command, args) {
  return spawnSync(command, args, { shell: true });
}

function parseOutput(command, output) {
  const sectionToken = '\^/--IMATOKEN--\^/';

  const lines = output.split('\n');
  // TODO: Fix spacing issues when options include a line break between the option name and description.
  const sections = output.replace(/\n\n(.+):\n/g, sectionToken + '$1:').split(sectionToken);

  let result = {
    name: lines[0].trim(),
    description: lines[1].trim()
  };

  sections.forEach(s => {
    const c = s.indexOf(':');
    const heading = s.slice(0, c).trim();
    const body = s.slice(c + 1);
    result[heading.trim()] = body
      .trim()
      .replace(/(\n[ ]{9,})[\W]/gm, ' ')
      .split('\n')
      .map(line => line.trim().replace(/^--/, '    --'));
  });

  return {
    command: command,
    name: result.name,
    description: result.description,
    aliases: result.ALIASES || [],
    args: result.ARGS || [],
    flags: result.FLAGS || [],
    options: result.OPTIONS || [],
    subcommands_body: result.SUBCOMMANDS || [],
    subcommands: (result.SUBCOMMANDS || [])
      .filter(line => !line.match(/help/))
      .map(line => {
        const matched = line.match(/^(\w+) (.+)$/);
        return {
          parent: `${command}`,
          command: matched ? matched[1].trim() : '',
          description: matched ? matched[2].trim() : ''
        };
    }),
    usage: result.USAGE || [],
  };
}

function anchor(str) {
  return str.replace(/ /g, '-');
}

function markdownForHeader() {
  const now = new Date();
  const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  const formatted = `${now.getDate()} ${months[now.getMonth()]} ${now.getFullYear()}`
  return `<!-- This is a generated file, do not edit it directly. See https://github.com/habitat-sh/habitat/blob/master/www/scripts/generate-cli-docs.js -->
 ---
title: Habitat Docs - hab CLI Reference
draft: false
---

# Habitat Command-Line Interface (CLI) Reference

The commands for the Habitat CLI (\`hab\`) are listed below.

| Applies to Version | Last Updated |
| ------- | ------------ |
| ${runCommand('hab', ['--version']).stdout.toString().trim()} (${platform()}) | ${formatted} |
`;
}

function markdownForCommand(parsed, sub) {
  return `##${sub ? '#' : ''} ${parsed.command}

${parsed.description}

${markdownForSubsection('Usage', parsed.usage.join('\n').replace(/^hab-/, 'hab ').replace(/hab butterfly/, 'hab').trim())}
${markdownForSubsection('Flags', parsed.flags.join('\n').trim())}
${markdownForSubsection('Options', parsed.options.join('\n').trim())}
${markdownForSubsection('Args', parsed.args.join('\n').trim())}
${markdownForSubsection('Aliases', parsed.aliases.join('\n').trim())}
${markdownForSubcommands(parsed.subcommands)}
---
`;
}

function markdownForSubcommands(subcommands) {
  if (!subcommands.length) {
    return '';
  }

  return `**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
${subcommands.map(item => `| [${item.parent} ${item.command}](#${anchor(`${item.parent} ${item.command}`)}) | ${item.description} |`).join('\n')}`;
}

function markdownForSubsection(title, data) {
  if (data) {
    return `**${title.toUpperCase()}**

\`\`\`
${data}
\`\`\`
`
  }

  return '';
}

if (platform() !== 'linux') {
  console.error('This script is intended to run only on Linux-based platforms.');
  process.exit(1);
}

console.log(markdownForHeader());
getHelp('hab');
