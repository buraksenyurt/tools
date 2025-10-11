# Loggy - A Simple Log File Analyzer

Loggy, Rust programlama dili ile yazılmış, log dosyalarını analiz etmek ve belirli desenlere göre filtrelemek için kullanılan basit bir terminal uygulamasıdır. Bu araç, log dosyalarındaki belirli kalıpları aramak, gerçek zamanlı log izleme yapmak ve log seviyelerine göre filtreleme gibi işlevleri destekler.

## Özellikler

Komut satırı argümanları ile aşağıdaki gibi kullanılabilir.

```bash
# Log dosyasını oku ve "ERROR" içeren satırları filtrele
cargo run -- --file .\api-runtime.log --pattern "ERROR"

# Log dosyasını okur ve "burak" isimli kullanıcıya ait logları filtreler
cargo run -- --file .\api-runtime.log --pattern "burak"

# Logları belirli bir zaman aralığına göre filtrelemek içinse örneğin:
cargo run -- --file .\api-runtime.log --start "2025-10-12 10:01:00" --end "2025-10-12 10:04:00"
```

> Not: Örnekler devam edecek...

## Kullanılan Rust Özellikleri

- **Regex Kullanımı**: `regex` kütüphanesi ile log dosyasında belirli desenleri aramak için kullanılır.
- **Dosya Okuma**: `std::fs::File` ve `std::io::{BufRead, BufReader}` kullanılarak log dosyası satır satır okunur.  
- **Error Handling**: `Result` tipi ile hata yönetimi sağlanır. Fonksiyonlar hata durumunda uygun mesajlar döner.
- **Command Line Argument Parsing**: `clap` kütüphanesi ile komut satırı argümanları kolayca işlenir.

> Not: Proje geliştirme aşamasında olup, ilerleyen zamanlarda daha fazla özellik eklenmesi planlanmaktadır.

## Örnek Ekran Çıktıları

> Not: Ekran çıktıları eklenecek...
