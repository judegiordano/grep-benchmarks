file length 4,036,000 lines

---

`NODE`

```sh
node . frog ../poem.txt
```

matches: 403600 operation complete in `471.737ms`

matches: 403600 operation complete in `468.603ms`

matches: 403600 operation complete in `482.273ms`

matches: 403600 operation complete in `473.751ms`

matches: 403600 operation complete in `469.377ms`

---

`RUST`

```sh
./target/release/grep.exe frog ../poem.txt
```

matches: 403600 operation complete in `78.4785ms`

matches: 403600 operation complete in `78.9137ms`

matches: 403600 operation complete in `78.8427ms`

matches: 403600 operation complete in `91.945ms`

matches: 403600 operation complete in `78.504ms`

---

`PYTHON`

```sh
python main.py frog ../poem.txt
```

matches: 403600 operation complete in `445.0 ms`

matches: 403600 operation complete in `445.63 ms`

matches: 403600 operation complete in `443.1 ms`

matches: 403600 operation complete in `443.04 ms`

matches: 403600 operation complete in `454.29 ms`

---

`CSHARP`

```sh
.\bin\Debug\net6.0\publish\chsarp.exe frog ../poem.txt
```

matches: 403600 operation complete in `295 ms`

matches: 403600 operation complete in `295 ms`

matches: 403600 operation complete in `296 ms`

matches: 403600 operation complete in `307 ms`

matches: 403600 operation complete in `287 ms`

---

`DENO`

```sh
deno run --allow-read src/index.ts frog ../poem.txt
```

matches: 403600 operation complete in `1.684s`

matches: 403600 operation complete in `1.646s`

matches: 403600 operation complete in `1.666s`

matches: 403600 operation complete in `1.654s`

matches: 403600 operation complete in `1.634s`

---
`GO`
```sh
.\grep.exe frog ../poem.txt
```
matches: 403600
operation complete in: `124ms`

matches: 403600
operation complete in: `126ms`

matches: 403600
operation complete in: `124ms`

matches: 403600
operation complete in: `122ms`

matches: 403600
operation complete in: `127ms`