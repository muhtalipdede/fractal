# Fraktal Uygulaması

Bu projede Rust programlama dilini kullanarak ekrana fraktal örnekleri çizdirmeyi amaçlamaktayız.

## Fraktal Nedir?

Fraktal, belirli bir matematiksel deseni veya yapıyı incelediğinizde, parçalarının bütün olarak aynı veya benzer özelliklere sahip olduğu, tekrarlanabilir bir şekilde yapılanmış geometrik veya matematiksel bir desendir. Fraktallar, kendilerini benzer parçalara (fraktal parçaları veya kendine benzerlikler) sahip olan yapılardır.

## Mandelbrot Seti

Mandelbrot Seti, karmaşık sayı düzlemindeki belirli bir bölgenin karmaşık sayılar üzerinde yaptığı bir dizi tekrarlamalı işlemin sonucunda oluşan bir fraktal kümedir. Bu fraktal küme, 20. yüzyılın sonlarına doğru matematikçi Benoît Mandelbrot tarafından tanımlandığı için onun adını almıştır.

![Mandelbrot Seti Örneği](/images/mandelbrot/30.png)
![Mandelbrot 4th Power Seti Örneği](/images/mandelbrot_4th/30.png)
![Mandelbrot 5rd Power Seti Örneği](/images/mandelbrot_5rd/30.png)

## Julia Seti

Julia Seti, kompleks sayı düzleminde belirli bir parametre değeri için tekrarlayan bir formül kullanılarak oluşturulan bir fraktal kümedir. Julia Seti, Mandelbrot Seti gibi matematiksel bir fraktal olup, karmaşık sayılar teorisinde önemli bir yere sahiptir. Adını Fransız matematikçi Gaston Julia'dan alır.

![Julia Seti Örneği](/images/julia/30.png)

## Burning Ship

![Burning Ship Örneği](/images/burning_ship/30.png)

## Tricorn

![Tricorn Örneği](/images/tricorn/30.png)

## Dede

![Dede Örneği](/images/dede/30.png)

## Projeyi Çalıştırma

```bash
cargo run mandelbrot 10
cargo run mandelbrot 20
cargo run mandelbrot 30

cargo run julia 10
cargo run julia 20
cargo run julia 30

cargo run burning_ship 10
cargo run burning_ship 20
cargo run burning_ship 30

cargo run tricorn 10
cargo run tricorn 20
cargo run tricorn 30
```
