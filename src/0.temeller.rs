/*
Buradaki amacim aldigim egitimi satiri satirina uygulamaktir.
Kaynak kitabim Dr. Aydin Erden'in "Rust ile Programlama" isimli eseridir.
*/

fn main()
{
    /*
    fn: Yazilacak seyin bir fonksiyon oldugunu bildirir. "main", yazilan fonksiyonun ismidir.
    Main fonksiyonu diger fonksiyonlardan ayri olarak programin asil calistirdigi fonksiyondur. Derleyici sadece burayi dikkate alir.
    Parametre parantezleri yazilirken parantezlerle fonksiyon ismi arasina bosluk birakilmaz. "()" icine, varsa parametreler eklenir. Yoksa bos birakilir.
    ({): Fonksiyon baslangici icin gerekli suslu parantezdir.
    Rust da diger pekcok dil gibi okunabilirlik icin girinti (indentation) kullanir.
    Girinti 4 bosluk miktarindadir, TAB degil.
    */

    println!("\nHello, world!\n");

    /*
    Evrensel ilk programdir. Komut satirina (CLI) istenen bir yazinin yazdirilmasini saglayan bir makro yani karmasik fonksiyondur.
    "println": makronun adi, (!): bunun normal bir fonksiyon degil makro oldugunun gostergesi, (()): parametre alicilari,
    parantez icindeki (""): yazdirilacak seyin string (cok karakterli yazi) oldugunun belirtecidir. Onlarin icinde de yazdirilacak metin bulunur.
    (;) ise asagida da soylenecegi uzere komutun bittiginin gostergesidir ve pekcok komutun sonuna koyulur. Bazilarina koyulmaz.

    Genel anlamda isimlendirmelerde uyulacak kurallar sunlardir:

    - ASCII karakterleri kullanilmalidir: Yani Amerikan alfabesinde bulunmayan harf, rakam ve semboller kullanilamaz.
    - Isimler (_) veya harfle baslamalidir, rakamlarla baslayamazlar.
    - Buyuk-kucuk harf duyarliligi mevcuttur, yani "degisken" kelimesiyle "Degisken" kelimesi iki farkli degiskeni atamak icin kullanilabilir.

    - Sabitler isimlendirilirken tavsiye edilen yontem "SCREAMING_SNAKE_CASE"dir. Yani tamami buyuk harfli yazilir, kelimelerin arasini ayirmak icin (_) kullanilir.
    - Degiskenler ve fonksiyonlar isimlendirilirken tavsiye edilen yontem "snake_case"dir. Yani tamami kucuk harf olup bosluklar icin (_) kullanilir.
    */
}