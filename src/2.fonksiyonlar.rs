fn main()
{
    /**********\
    FONKSIYONLAR
    \*        */

    /*
     * Function: Belirli bir islemi gerceklestiren ve sonuc dmndurebilen kod blogu.
     * fn: Fonksiyon belirten primitive type (ilkel tur).
     * Sonra ismi verilir ve bosluksuz () girilir. Icine dondurulecek seyin turu yazilir. Deger dondurmuyorsa bos birakilir (unit).
     * Sonra main'de asina oldugumuz sekilde {} kullanilir.
     */

    merhaba(); // Fonksiyon (merhaba) icin asagiya bakiniz.
    println!("i32 donduren toplama fonksiyonu denemesi: {}", top(5, 5)); // Fonksiyon (top) icin asagiya bakiniz.
    println!("i32 donduren cikarma fonksiyonu denemesi: {}", cik(5, 5)); // Fonksiyon (cik) icin asagiya bakiniz.
    println!("i32 donduren komleks cikarma fonksiyonu denemesi: {}", complex(5, 5)); // Fonksiyon (complex) icin asagiya bakiniz.

    // EXPRESSION (IFADE) VE STATEMENT (IŞLEM) \\

    /*
     * Expression: Deger ureten kod parcalarina expression denir. Islemlerin parcalaridir.
     * - Aritmetik islemler
     * - Degiskenler (let x: i32 = 5'te 5 bir ifadedir.)
     * - Fonksiyon cagrilari (hicbie deger dondurmese de unit yani bos deger dondurmus olurlar.)
     * - Deger donduren kontrol yapilari
     * - Block scope'lar.
     * Statement: Deger uretmeyen kod parcalaridir.
     * - Degisken atamalari (let'le baslayan atama kodunun tamami. Kisimlariysa ifadedir.)
     * - Deger dondurmeyen kontrol yapilari
     * Diger bazi dillerde degisken atamalari statement degil expression'dur. Dolayisiyla degisken atanirken deger kismina baska bir degisken atamasi girilebilir.
     * Ornek:
     * Rust'ta su yapilamaz (kod derlenmez): let a: i32 = let b: i32 = 5 fakat C'de int a = int b = 5 yazilabilir.
     * !: Fonksiyon cagrilari (yurutme olayinin yapildigi sira yazilan) expression, fonksiyon tanimlari (fonksiyonun ne yaptigini beliledigimiz kisim) statement'tir.
     */
}

fn merhaba() -> () {
    // Fonksiyonumuz hicbir parametre almadigindan isme bitisik parantezlere parametre girmedik. Deger dondurmediginden (-> ()) ekledik, unit yani void dondurecek.
    println!("\nUnit fonksiyon denemesi: Merhaba!");
}

fn top(s1: i32, s2: i32) -> i32 {
    /*
     * Parantez icerisine fonksiyonumuz kac parametre alacaksa turuyle beraber yaziyoruz.
     * Dondurecegi degerin turunu (donus turu) (->)'tan sonra yazip {} icine fonksiyonumuzu yaziyoruz.
     */
    let sonuc = s1 + s2;
    sonuc
    // Fonksiyonlarin dondurecegi degerleri tasiyan degiskenler, Rust dilinde return kullanilmadan ve sonuna (;) koyulmadan bu sekilde sonuna yazilir.
}

fn cik(s1: i32, s2: i32) -> i32 {
    let sonuc = s1 - s2;
    return sonuc;
    // Klasik (return;) komutuyla da deger dondurmek mumkundur. Hatta eger fonksiyonun tamaminin sonu degilse zorunluluktur. Sonsa ilk yontem tercih edilir.
}

fn complex(s1: i32, s2: i32) -> i32 {
    if s1 == 0 {
        return s2; // Mecburi return kullanimi: fonksiyon tamamlanmadi.
    }
    if s2 == 0 {
        return s1;
    }
    let sonuc = s1 + s2;
    sonuc // Fonksiyon sonu, yalin kullanim tercihi.
}