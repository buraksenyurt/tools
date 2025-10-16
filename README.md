# Tools

Bu repoda Rust programlama dili ile yazılmış çeşitli sistem araçlarına yer verilmesi planlanmaktadır. Amaç hem rust bilgilerimizi tazelemek hemde hafifsiklet işe yarar program parçaları geliştirmektir.

## Fikirler

- **sys-tower** : Çeşitli sistem verilerini _(CPU, bellek kullanım verileri, disk doluluk oranı gibi)_ renkli formatta terminal ekranına yazdıran bir konsol uygulamasıdır. [Detaylar için...](./sys-tower/README.md)
- **port-scan** : Belirtilen IP adresi üzerinde belli bir port, port listesi veya port değer aralığında tarama yapıp sonuçları renkli formatta terminal ekranına yazdıran bir konsol uygulamasıdır. [Detaylar için...](./port-scan/README.md)
- **d-dash**: Bir klasör içerisindeki dosyalar ve dizinlerle ilgili çeşitli bilgileri renkli formatta terminal ekranına yazdıran bir konsol uygulamasıdır. [Detaylar için...](./d-dash/README.md)
- **loggy**: Log dosyaları üzerinde çeşitli işlemleri terminalde yapmamızı sağlayan uygulama. Log dosyasında pattern bazlı arama yapma, real-time log izleme, level bazında filtreleme, JSON veya CSV formatında çıktı verme gibi görevleri gerçekleştirecek. [Detaylar için...](./loggy/README.md)

## Gerçekleştirilenler

- **sys-tower** : Temel işlevsellik tamamlandı. CPU, bellek ve disk kullanım verileri renkli formatta terminal ekranına yazdırılıyor.
- **port-scan** : Temel işlevsellik tamamlandı. Belirtilen IP adresi üzerinde belli bir port, port listesi veya port değer aralığında tarama yapıp sonuçları renkli formatta terminal ekranına yazdırıyor. Ayrıca belirli bir port aralığı için çoklu iş parçacığı ile tarama yapma desteği eklendi.
- **d-dash**: Temel işlevsellik tamamlandı. Bir klasör içerisindeki dosyalar ve dizinlerle ilgili çeşitli bilgileri renkli formatta terminal ekranına yazdırıyor.
