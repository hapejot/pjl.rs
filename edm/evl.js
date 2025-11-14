#!/usr/bin/env node 
const yaml = require('js-yaml')
let data = "";
async function main() {
    for await (const chunk of process.stdin) data += chunk;
    const f = eval("(" + data + ")")
    process.stdout.write(yaml.dump(f))
}

main()
