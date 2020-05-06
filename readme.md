# hex_table

Format a byte slice into a hex table.

```rust
let x = "Lorem ipsum cupidatat voluptate exercitation aliquip exercitation \
         do exercitation est? Consectetur est non commodo exercitation.";

let table = HexTable {
    columns: 16,
    offset: 0,
    header: false,
    ascii: true,
    zeros: true,
};

table.format(x.as_bytes(), &mut std::io::stdout()).unwrap();
```

```
00: 4C 6F 72 65 6D 20 69 70 73 75 6D 20 63 75 70 69 | Lorem.ipsum.cupi
10: 64 61 74 61 74 20 76 6F 6C 75 70 74 61 74 65 20 | datat.voluptate.
20: 65 78 65 72 63 69 74 61 74 69 6F 6E 20 61 6C 69 | exercitation.ali
30: 71 75 69 70 20 65 78 65 72 63 69 74 61 74 69 6F | quip.exercitatio
40: 6E 20 64 6F 20 65 78 65 72 63 69 74 61 74 69 6F | n.do.exercitatio
50: 6E 20 65 73 74 3F 20 43 6F 6E 73 65 63 74 65 74 | n.est..Consectet
60: 75 72 20 65 73 74 20 6E 6F 6E 20 63 6F 6D 6D 6F | ur.est.non.commo
70: 64 6F 20 65 78 65 72 63 69 74 61 74 69 6F 6E 2E | do.exercitation.
```

[Documentation](https://docs.rs/hex_table)