const start = performance.now();

const [query, file] = Deno.args;

const text = await Deno.readTextFile(file);

function search(query: string, data: string) {
    const matches = [];
    const lines = data.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].split(" ").includes(query)) {
            matches.push(lines[i]);
        }
    }
    return matches;
}

const results = search(query, text);

const end = performance.now();
console.log("matches: ", results.length);
console.log(`operation complete in ${(end - start)/1000}s`);
