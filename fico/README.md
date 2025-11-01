# Fico - Basit Dosya Kopyalama Aracı

Bu terminal uygulaması basit bir dosya koplayama aracıdır. Indicatif isimli crate kullanılarak kopyalama işlemi sırasında terminalde progress bar gösterimi sağlanır. Özellikle büyük boyutlu dosyaların kopyalanması sırasında bu tip bir görsel bildirim aracı oldukça kullanışlıdır.

## Kullanım

Komut satırı argümanları ile aşağıdaki gibi kullanılabilir.

```bash
# Klasik bir dosya kopyalama işlemi
cargo run -- -s .\games.dat -d .\games_copy.dat

# Var olan dosyanın üzerine yazmak için -f (force) argümanı eklenir
cargo run -- -s .\games.dat -d .\games_copy.dat -f
```

> Not: Örnekler devam edecek...

## Kullanılan Rust Özellikleri

- **Dosya okuma ve yazma**: `std::fs::File` kütüphanesinden `BufReader`, `BufWriter` kullanılarak okuma ve yazma işlemleri gerçekleştirilir.
- **Komut satırı argümanları**: `clap` crate'i kullanılarak komut satırı argümanlarının daha yapısal bir şekilde işlenmesi sağlanır.
- **Progress Bar**: `indicatif` crate'i kullanılarak terminalde kopyalama işlemi sırasında görsel bir progress bar oluşturulması sağlanır.

## Örnek Ekran Çıktıları

Yaklaşık 700 MB boyutundaki bir dosyanın kopyalanması sırasında terminalde aşağıdaki gibi bir çıktı elde edilir:

![fico_00.png](../images/fico_00.png)
