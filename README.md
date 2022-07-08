file length 4,036,000 lines

---

`NODE`

```sh
node . frog ../poem.txt
```

matches: 403600 operation complete in `1.541s`

matches: 403600 operation complete in `1.546s`

matches: 403600 operation complete in `1.542s`

matches: 403600 operation complete in `1.547s`

matches: 403600 operation complete in `1.539s`

---

`RUST`

```sh
./target/release/grep.exe frog ../poem.txt
```

matches: 403600 operation complete in `120.9365ms`

matches: 403600 operation complete in `120.8149ms`

matches: 403600 operation complete in `121.6225ms`

matches: 403600 operation complete in `121.0709ms`

matches: 403600 operation complete in `127.6297ms`

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
