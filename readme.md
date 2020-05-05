# hex_table

Format a byte slice into a hex table.

```rust
let contents = read("lorem_ipsum.txt").unwrap();

let output = HexTable::default().format_with_ascii(&contents);

println!("{}", output);
```

```
    00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f
00: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 65 78 63 65 Lorem.ipsum.exce
10: 70 74 65 75 72 20 63 6f 6e 73 65 63 74 65 74 75 pteur.consectetu
20: 72 20 65 78 65 72 63 69 74 61 74 69 6f 6e 20 65 r.exercitation.e
30: 78 20 6e 6f 6e 20 73 65 64 20 63 75 70 69 64 61 x.non.sed.cupida
40: 74 61 74 20 65 78 63 65 70 74 65 75 72 2c 20 64 tat.excepteur..d
50: 6f 6c 6f 72 20 65 78 65 72 63 69 74 61 74 69 6f olor.exercitatio
60: 6e 20 63 6f 6e 73 65 71 75 61 74 20 63 75 6c 70 n.consequat.culp
70: 61 20 72 65 70 72 65 68 65 6e 64 65 72 69 74 2e a.reprehenderit.
```

[Documentation](https://docs.rs/hex_table)