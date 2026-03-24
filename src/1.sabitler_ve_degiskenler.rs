/******\
SABITLER
\*    */

const PI: f32 = 3.14;

/*
Bu bir "sabit"tir. Sabit, programin kalani boyunca asla degistirilmeyecek olan degerler icin kullanilan anahtar kelimelerdir.

Her programlama dilinde oldugu gibi Rust da program boyunca kullanilacak, duruma gore azalip cogalacak veya hep sabit kalacak degerler icin
sabitler ve degiskenler sistemini kullanir. Programci programda kullanacagi tamamen (sabitler) veya kismen sabit (unmutable yani degismez degiskenler)
ya da dinamik degerler (mutable yani degistirilebilir degiskenler) icin isim, tur ve deger belirleyerek aciklanacak kurallara gore tanimlayabilir,
tekrar tekrar ayni degeri yazmak yerine programin devaminda amacina uygun bicimde kullanabilir.

Mesela yukarida kullandigimiz PI sabiti genelde kabul gormus pi degeri ne zaman kullanilmak istense, her seferinde 3.14 yazmak yerine PI yazilarak kullanilabilir.
Bu sekilde eger bir sebeple sonradan piyi 3.14 degil de daha uzun versiyonlariyla almak gerekirse teker teker her 3.14 degeri degistirilmek yerine
bastaki PI sabitinin degerinin degistirilmesi yeterli olacaktir. Ayni sey degiskenler icin de gecerlidir falat degiskenler adlarindan da anlasilabilecegi gibi
gerektiginde degistirilebilir olarak tanimlanabilirler. Asagida ornekleri verilecektir.

Sabitleri atamak icin "const" anahtar sozcugu kullanilir. const'tan sonra kisinin takdirine gore sabit ismi, sonra (:), sonra turu, sonra (=), sonra degeri, sonra da (;) yazilir.

Sabitler asagida gelecek degiskenlerden farkli olarak asla degisebilir olarak ayarlanamaz, yine farkli olarak global kapsamda veya alt kapsamlarda belirlenebilirler.
Degiskenlerse global kapsamda (global scope) belirlenemezler. Global kapsam herhangi bir fonksiyon baslamadan onceki ve fonksiyonlar bittiktensonraki kapsamdir.
Kapsamlar ({}) ile belirtilir. Asagida ornegi gorulecektir.

Ayni zamanda sabitler degiskenlerin aksine IDE'nizdeki otomatik tur belirleme ozelligine tabi degildir, dolayisiyla tur programci tarafindan manuel yazilmalidir.
Otomatik tur belirleme sisteminin nasil calistigi asagida gosterilecektir.

Ayrica sabitlerde degerin derleme esnasinda bilinmesi de zorunludur, programin calismasi sirasinda belirlenemezler.
Kisaca programci her sabit icin degeri de turu de yazmak zorundadir.
*/

fn main()
{
    /*********\
    DEGISKENLER
    \*       */

    /*
    Asagidaki tum degisken degerlerinde alabilecekleri maksimum deger esas alinmistir. Minimum deger ayrica belirtilmistir. "||" "ya da" anlamindadir.

    Sayilari her uc basamakta bir "_" ile ayirmak mumkundur, okunakligi arttirmada kullanilan bir yontemdir.

    Rust'ta degiskenler default olarak unmutable yani degistirilemezdir. Degistirilebilir yapmak icin asagida aciklanacak arti bir komut kullanilir.
    */

    // Unmutable signed integer (hem negatif hem pozitif deger alabilen, degistirilemez tamsayi) turundeki degiskenlerin tanimlanma sekli:

    let q: i8 = 127; // || -128;
    let w: i16 = 32_767; // || -32_768;
    let e = 2_147_483_647; // || -2_147_483_648;
    let r: i64 = 9_223_372_036_854_775_807; // || -9_223_372_036_854_775_808;
    let t: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // || -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let y: isize = 0; // Bu degisken turunde size (boyut) uyarlanabilirdir. Kullanilan bilgisayarin mimarisine gore belirlenir (32-64).

    /*
    Goruldugu uzere once "let" islemi kullanilarak degisken atanacagi bildirildi, (:) atilip tur yazildi, (=) sonrasina da ture ozgu deger girildi.

    Sabitlerin aksine degiskenlerde degerin derleme esnasinda bilinmesi gerekmez, programin calismasi sirasinda belirlenebilir:
    */

    let x; // Goruldugu uzere herhangi bir deger girilmedi. Derlemede "deger atanmamis" olarak derlenecek.
    x = 0; // Deger burada belirlenmis oldu, yani degisken ilk atandiginda degil program calisirken.

    /*
    Rust'da degiskenlerin turunu tanimlamak gereklidir ancak bu, programci yapmazsa rust_analyzer yuklu IDE tarafindan otomatik yapilir.
    Manuel tanimlamanin farki, otomatik yapildiginda muhtemel olan fazla bellek kullanimi gibi sorunlari bulundurmamasidir.

    Otomatik tur tanimlamada genelde turlerin default (onceden belirlenmis, varsayilan) bellekleri esas alinir. Mesela belli araliktaki sayilarda bu,
    yukaridaki (e) degiskeninde gorulecegi uzere 32 bittir.
    Degiskenin otomatik tanimlandigini renginin diger tur belirteclerinden farkli olarak gri olmasindan anlayabiliriz.

    Rust'da satirlarin cogu (;) ile biter. Bazi satirlar bitmez ve bunun ozel anlamlari vardir. Mesela "return" komutuna gerek kalmadan deger dondurmek gibi.
    Ileride bunlara da deginilecektir.
    */

    // Unmutable unsigned integer (sifir veya pozitif sayilari deger alabilen, degistirilemez tamsayi -minimum deger tamaminda 0 olmak uzere-) turundeki degiskenlerin tanimlanma sekli:

    let u: u8 = 255;
    let i: u16 = 65_535;
    let o: u32 = 4_294_967_295;
    let p: u64 = 18_446_744_073_709_551_615;
    let a: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let s: usize = 0; // Bu degisken turunde size uyarlanabilirdir. Kullanilan bilgisayarin mimarisine gore belirlenir (32-64).

    // su ana kadarki tum degiskenler unmutable yani degistirilemezdir. Mutable degisken tanimlama icin "let"ten sonra "mut" kullanilir, boylece sonrasinda deger degistirilebilir:

    let mut d = 0;
    d = 1;

    /*
    Goruldugu uzere once degisken mutable olarak ayarlanarak 0 degeri verildi, sonrasinda bu deger sadece degisken ismi ve (=) isareti kullanilarak 1 ile degistirildi.
    Eger mut komutu yazilmasaydi program derlenmezdi.

    Derleme Zamani (Compile Time): Kodlarin islemcinin anlayacagi yegane dil olan binary'ye cevrilme surecidir. Sonunda calistirilabilir (executable) bir dosya olusur.
    calisma Zamani (Runtime)     : Derlenmis kodlarin calistirildigi surectir. Rust'ta zor da olsa derleme zamaninda hata almayip calisma zamaninda hata almak mumkundur.
    */

    // DEGISKENLERDE GOLGELEME (SHADOWING) \\

    /*
    Rust'ta ayni kapsam (scope) icinde ayni isimle iki degisken tanimlamak mumkundur. Ikinci degisken birincinin degerini iptal eder ve bu deger kapsam bitip deger yok sayilana
    veya ikinci degisken de ucuncu bir degisken tarafindan golgelenene dek gecerli olur. Asagida ikisine de ornek verilmistir:
    */

    let f: i32 = 10; // (f) isminde bir degisken tanimladik, turunu i32 olarak sectik ve 10 degerini atadik.
    println!("Block scope ve shadowing denemeleri:\nSayinin ilk degeri: {f}"); // Ekrana (f)'in o siradaki yani ilk tanimlanan degeri yazdirilir.

    let f: i32 = f + 1; // Ilk (f) degiskenini golgeleyen ikinci bir degisken tanimladik ve degerini f/10 + 1 = 11 olarak atadik.
    println!("Sayinin ikinci degeri: {f}"); // Ekrana (f)'in o siradaki yani ilkini golgeleyip atanan 11 degeri yazdirilir.

    {
        // Rust dilinde oncesinde herhangi bir fonksiyon veya ifade olmasa da alt kapsam acilabilir. Buna "block scope" denir. Kapsami actik.
        let f: i32 = f * 2; // Kapsam icinde onceki 11 degerli (f) degiskenini de golgeleyen ucuncu bir degisken tanimlayarak ona f/11 * 2 = 22 degerini atadik.
        println!("Sayinin ucuncu degeri: {f}"); // Ekrana (f)'in o siradaki yani ikinciyi de golgeleyip atanan 22 degeri yazdirilir.
    } // Kapsam sona erdiginden bu suslu parantezden sonra (f) degiskeni son gecerli degerine yani 11'e geri dondu.

    println!("Sayinin son degeri: {f}"); // Sayinin kapsamdan onceki son gecerli degeri olan 11 yazdirildi.

    //  YER TUTUCULARI  \\

    /*
    Burada gormus oldugumuz diger bir sey de yer tutucularidir. ({}) simgesi bu tarz bir string dizesi icerisindeyse dize icine onceden belirlenmis olan degiskenin
    degerini yazdirmak icin kullanilir. Bunu yukaridaki gibi degiskenin ismini suslu parantezlerin icine yazarak yapabilecegimiz gibi asagidaki sekilde, parantezleri
    bos birakip string dizesi ("") ile sonlandirildiktan sonra (,) koyup degisken ismini yazarak da yapabiliriz.
    (Birden cok degisken bulunmasi halinde virgulden sonraki yazilis sirasina gore yerlestirilir.):
    */

    println!("\nSayinin son degeri (virgullu yer tutucu denemesi): {}", f);

    // Mut ile farklari: 1. Golgeleme yaparken golgelenenin degerini degistirmek mumkundur:

    let g: i32 = 0;
    let g: &str = "Merhaba";

    // 2. Degisken mut ile degisebilir yapildiginda onu degistirmek mumkun olur ancak golgeleme degismez degiskeni tek basina degisebilir yapmaz.
    
    //  KAPSAM TURLERI  \\

    /*
    1. Blok Kapsami: Icerisinde tanimlanan degiskenler disinda kullanilamaz. Bu tum kapsamlar icin gecerlidir. Deger dondurebilir. ornegi yukaridadir.
    2. Fonksiyon kapsami: su an icinde bulunulan main kapsami gibi.
    3. Dongu Kapsami (Loop scope):
    */

    for i in 0..3 {
        let h = i * 2;
        println!("h'nin degeri: {h}");
    }

    // MIN-MAX KULLANIMI \\

    let j: i32 = i32::MAX; //i32 turunde alinabilecek en buyuk degeri j'ye atar.
    let k: i32 = i32::MIN; //i32 turunde alinabilecek en kucuk degeri j'ye atar.

    /*
    Diger sayi sistemlerinde de deger atamak mumkundur:
    - Hex (16'li sayi sistemi): onaltilik sayinin onune "0x" yazilir.
    - Octal (8'li sayi sistemi): "0o"
    - Binary (2'li sayi sistemi): "0b"
    - Byte (sadece u8): "b'"A"'"(?)
    */

    //  TAMSAYI TAsMASI (INTEGER OVERFLOW)  \\

    /*
    - Debug modda derleme yaparken, degisken icin tanimlanan tur katsayisi yazilan rakami karsilayamayacak bir sekilde degistirilirse derleme engellenir.
    - Ancak release modda engellenmez ve integer overflow yasanir: sayi durumuna gore basa doner, asagidaki ornekte sayi negatif deger alamayacagindan 255 degerini alir.
    - Eger 255 degeri atamis olsaydik ve bir ekleseydik bu kez de deger en ust siniri asip basa, 0'a donecektir. Yani katsayilar dairesel calisir.
    */

    let mut z: u8 = 0;
    println!("\nOverflow denemeleri:\n\nBaslangic degeri: {z}");

    z = z - 1;
    println!("Tasma sonrasi deger: {z}");

    let mut c: u8 = 255;
    println!("\nOverflow denemeleri - 2:\n\nBaslangic degeri: {c}");

    c = c + 1;
    println!("Tasma sonrasi deger: {c}");

    // SATURATING METODU \\

    /*
    Bu metot yapilacak islemde eger tasma varsa bu tasmayi engeller ve islemi tasma olmadan yapilabilecek son halinde yazdirir.
    (.) isareti: belirli ayarlari cagirmak icin kullanilmaktadir.

    Saturating ayarinin 4 farkli sekli bulunur:
    - Toplama icin saturating_add
    - cikarma icin saturating_sub
    - carpma icin saturating_mul
    - Bolme icin saturating_div ve
    - ussunu almak icin saturating_pow kullanilir.
    */

    let mut l: u8 = 1; // (l) degiskenine (0) degeri atiliyor ve u8 turunde belirleniyor. Alabilecegi minimum deger zaten 0.
    println!("\nSaturating baslangic degeri: {l}");

    l = l.saturating_sub(2); // Tasma engelleyici saturating komutuyla (l)'den 2 cikariliyor. Normalde tasmasi lazim ama izin vermiyor. 1 cikardiktan sonra 0 kaliyor ve onu yazdiriyor.

    println!("Saturating tasma sonrasi deger: {l}");

    // FLOAT TuRu \\

    // Float ondalik sayilar icin kullanilir ve iki alt turu vardir: f32 ve f64. Varsayilan olarak f64 kullanilir. Ototmatik type atamasinda da bu sekilde olur.

    let v = 1.0; // Otomatik olarak daha fazla hassasiyet saglayan f64 turunu secti.
    let b: f32 = 1.0; // f32 yapmak istersek manuel olarak yazariz.

    /*
    Rust'ta aritmetik islemler [toplama (+), cikarma (-), carpma (*), bolme (/) ve kalan (%)] seklinde temsil edilir.
    Integer turunde islem sonucu float cikarsa en yakin integer'a yuvarlanir.
    */

    let qw: i32 = 3 / 2;
    println!("\nOndalikli sonuca sahip tamsayi turunde degiskenin islem sonucu: {qw}"); //Sonuc 1 olarak yazdirilir.

    let add = 2 + 1;
    let sub = 2 - 1;
    let mul = 2 * 1;
    let div = 2 / 1;
    let rem = 3 % 2;

    println!(
        "\nIslem turlerinin sonuclari:\n\nToplama: {}\ncikarma: {}\ncarpma: {}\nBolme: {}\nKalan: {}",
        add, sub, mul, div, rem
    ); //Sonuc 1 olarak yazdirilir.

    // BOOLEAN TuRu \\

    // Sadece true (dogru) ve false (yanlis) degeri alabilen, hafizada 0 byte yer kaplayan, sadece if kosulu icinde anlamli olan bir turdur.

    let dgr: bool = true;
    let yls: bool = false;

    // CHAR TuRu \\

    /*
    Alfabedeki karakterleri temsil etmek icin kullanilir. UTF-8 destekler. Iki (') karakteri arasina yazilir. ("") arasina yazilirsa hata alinir ve derlenmez.
    4 byte yer kaplar.
    Bunlarin tamami "Skaler Turler" olarak isimlendirilir. Asagida compound (birlesik) iki rust turu anlatilacaktir.
    */

    //** COMPOUND TuRLER **\\

    // TUPLE TuRu \\

    /*
     * Birden fazla degiskenin tek bir veri yapisi, diger bir deyisle diger baska bir degiskende bir arada tutulmasidir.
     * Boyutlari sabittir ve degismez (Unmutable).
     */

    let tuple: (i32, f64, char, bool) = (1, 2.5, 'a', true); // Ilk parantez icierisinde eleman sayisinca tur, ikincisine de sirasiyla degerleri girilir.
    println!("\nTuple: {:?}", tuple);

    /*
     * Goruldugu gibi once let ile degisken tanimlayacagimizi bildirdik.
     * Sonra degiskenimize isim verdik.
     * Sonra (:)'den sonrasinda (=)'e kadar () icinde elemanlarin sirasiyla tur belirttik.
     * (=)'den sonraki () icindeyse sirasiyla degerleri girdik.
     */

    // tuple icindeki elemanlara erismek icin su indis girmeli "degisken atama" yontemi kullanilabilir:

    let first = tuple.0;
    let second = tuple.1;
    let third = tuple.2;
    let fourth = tuple.3;

    /*
    tuple'daki elemanlar, tuple isminden sonra yazilacak bir (.) ve sonrasinda yazilacak indis numaralariyla eleman sayisinca yeni degiskene atanabilir.
    Boylece tuple icindeki elemanlara teker teker ulasmak da mumkun olur.
    Indis numaralari 0'dan baslar ve eleman sayisindan bir eksiktir.
    */

    println!("\nTuple icindeki elemanlar: {first}, {second}, {third}, {fourth}");

    /*
    Bu degerlere ulasmanin diger bir yolu da oruntu eslemedir (pattern matching). Diger bir adi da ogelere ayirmadir (deconstruction).
    Bu islem de tuple basta belirlendikten sonra yapilir.
    */

    let (itg, flt, chr, bol) = tuple;
    println!("{itg} {flt} {chr} {bol}");

    // ARRAY TuRu \\

    /*
     * ozellikleri:
     * Tuple'dan farkli olarak tum degiskenlerin turu ayni olmalidir.
     * Tuple gibi sabittir ve degistirilemez (Unmutable).
     * Tuple gibi indisler 0'dan baslar fakat atama yapilacagi zaman (.)'dan sonra degil isimden sonraki ([]) icinde yazilir. deger atama = iledir.
     * Array'in tum degiskenlerne deger atanmasi zorunludur.
     */

    let ary: [i32; 5] = [1, 2, 3, 4, 5];

    /*
     * Goruldugu uzere once let ile degisken yazacagimizi belirttik,
     * Sonra isim verdk.
     * (:)'dan sonra ([]) icerisine array icindeki elemanlarin turunu ve (;) ile ayirarak kac tane eleman olacagini girdik.
     * (=)'den sonra da yine ([]) icine degiskenlerin degerlerini (,) ile ayirarak yazdik.
     */

    // ARRAY'LERIN DEgIsKENLERINE TEKER TEKER DEgER ATAMA:
    // Bunu su sekilde yapariz:

    let mut ary: [i32; 5] = [0; 5]; // mut kullanarak array'in degiskenlerini sonradan tanimlanabilir yaptik. Sonra tum degiskenlere 0 atadik ([0; 5]).

    // 0'dan baska bir deger de atanabilirdi.
    // Fark ettigimiz diger nokta arrayin degiskenlerini de degerini de atarken sadece once tur veya deger (;)'dan sonra da adet belirterek atama yapilabildigi.

    ary[0] = 1;
    ary[1] = 2;
    ary[2] = 3;
    ary[3] = 4;
    ary[4] = 5;

    println!("\nArray sifirinci ve birinci eleman indis deger sorgu:\n1: {}\n2: {}", ary[0], ary[1]);

    // Sinirlarin ustune cikilirsa compiler error (panic) verecektir.
}