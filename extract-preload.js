const fs = require('fs');
const path = require('path');
const prettier = require('prettier');

// Use file name from command line argument
const inputFile = process.argv[2];
if (!inputFile) {
    console.error('Usage: node extract-preload.js <Component-preload.js>');
    process.exit(1);
}

// Read the given file
const objStr = fs.readFileSync(inputFile, 'utf8');

// Find the start of the sap.ui.require.preload call (last occurrence)
// const callStart = preload.lastIndexOf('sap.ui.require.preload');
// if (callStart === -1) {
//     console.error('Could not find sap.ui.require.preload call.');
//     process.exit(1);
// }

// // Cut from the start of the call to the end of file
// let callStr = preload.slice(callStart);
// // Remove the function call wrapper: sap.ui.require.preload(...);
// callStr = callStr.replace(/^sap\.ui\.require\.preload\s*\(/, '');
// callStr = callStr.replace(/\);?\s*$/, '');

// // // The first argument is the namespace, remove it (and the comma)
// // let firstComma = callStr.indexOf(',');
// // if (firstComma === -1) {
// //     console.error('Could not parse preload call arguments.');
// //     process.exit(1);
// // }
// // let objStr = callStr.slice(firstComma + 1).trim();
// objStr = "((a,b) => {return a;})("+ callStr.trim() +")";
// // Evaluate the object
// let files;
function writeout(filename, content) {
        const outPath = path.join('extracted', filename);
        fs.mkdirSync(path.dirname(outPath), { recursive: true });
        fs.writeFileSync(outPath, typeof content === 'string' ? content : String(content));
}

(async () => {
    try {
        const jQuery = {
            sap: {
                registerPreloadedModules: (s) => {
                    console.log(`Registering preloaded modules: ${s.name}`);
                    Object.entries(s.modules).forEach(([filename, content]) => {
                        writeout(filename, content);
                    });
                }
            }
        };
        const sap = {
            ui: {
                require: {
                    preload: (files, namespace) => {
                        console.log(`Preloading namespace: ${namespace}`);
                        Object.entries(files).forEach(([filename, content]) => {
                            writeout(filename, content);
                        });
                    }
                },
                predefine: async (name, dependencies, f) => {
                    console.log(`Defining module: ${name}`);
                    let code = "f = " + f.toString() + ";";
                    try {
                        code = await prettier.format(code, { parser: "babel" });
                    } catch (e) {
                        console.warn("Could not pretty-print code for", name, e);
                    }
                    writeout(name+".js", code);
                }
            }
        };
        files = await eval(objStr);
    } catch (e) {
        console.error('Could not eval preload object:', e);
        process.exit(1);
    }
})();

