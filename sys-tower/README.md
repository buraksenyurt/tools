# System Tower

Bu program basit sistem bilgilerini terminal ekranına yazdırır. **Rust** programlama dili kullanılarak, **Windows 11** platformunda geliştirilmiştir. Amaç, **Rust** dilinde pratik yapmak ve sistem bilgilerini renkli formatta gösterebilen hafif bir araç oluşturmaktır. Sistem bilgilerini çekmek için **`sysinfo`** kütüphanesi kullanılmıştır. Terminal çıktısını renklendirmek için ise **`colored`** kütüphanesi tercih edilmiştir. **`sysinfo`** küfesi, sistem bilgilerini erişmek için iyi bir soyutlama *(abstraction)* sunar. Program, CPU, bellek kullanımı, disk durumu ve en çok işlem yapan process bilgilerinden beşini göstermektedir, daha fazlası değil. Kullanımı oldukça basittir; programı çalıştırmak yeterlidir :D

```bash
# Program kaynak kodundan aşağıdaki çalıştırılabilir:
cargo run
cargo run -- --help # Yardım bilgilerini ekrana yazdıracaktır

# Release modunda derleyip çalıştırmak için:
cargo build --release
./target/release/sys-tower
```

## Örnek Ekran Çıktıları

```bash
cargo run -- --help
```

![sys_tower_00](../images/sys_tower_00.png)

```bash
cargo run
```

![sys_tower_01](../images/sys_tower_01.png)
