const fs = require("fs");
const path = require("path");

console.time('operation complete in');

const query = process.argv[2];
const file = process.argv[3];

const filePath = path.resolve('../', file);

const data = fs.readFileSync(filePath).toString();

function search(query, data) {
    const matches = [];
    let lines = data.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].split(" ").includes(query)) {
            matches.push(lines[i]);
        }
    }
    return matches;
}

let results = search(query, data);

console.log("matches: ", results.length);
console.timeEnd('operation complete in');
