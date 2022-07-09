const fs = require("fs");
const es = require("event-stream");

console.time("operation complete in");

const query = process.argv[2];
const file = process.argv[3];

const matches = []
fs.createReadStream(file).pipe(es.split()).pipe(
    es.mapSync((line) => {
        if (line.indexOf(query) >= 0) {
            matches.push(line);
        }
    })
        .on("end", () => {
            console.log("matches: ", matches.length);
            console.timeEnd("operation complete in");
        })
)
