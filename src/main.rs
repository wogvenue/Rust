/*
Buradaki amacim aldiğim eğitimi satiri satirina uygulamaktir.
Kaynak kitabim Dr. Aydin Erden'in "Rust ile Programlama" isimli eseridir.
*/

/******\
SABİTLER
\*    */

const PI: f32 = 3.14;

/*
Bu bir "sabit"tir. Sabit, programin kalani boyunca asla değiştirilmeyecek olan değerler için kullanilan anahtar kelimelerdir.

Her programlama dilinde olduğu gibi Rust da program boyunca kullanilacak, duruma göre azalip çoğalacak veya hep sabit kalacak değerler için
sabitler ve değişkenler sistemini kullanir. Programci programda kullanacaği tamamen (sabitler) veya kismen sabit (unmutable yani değişmez değişkenler)
ya da dinamik değerler (mutable yani değiştirilebilir değişkenler) için isim, tür ve değer belirleyerek açiklanacak kurallara göre tanimlayabilir,
tekrartekrar ayni değeri yazmak yerine programin devaminda amacina uygun biçimde kullanabilir.

Mesela yukarida kullandiğimiz PI sabiti genelde kabul görmüş pi değeri ne zaman kullanilmak istense, her seferinde 3.14 yazmak yerine PI yazilarak kullanilabilir.
Bu şekilde eğer bir sebeple sonradan piyi 3.14 değil de daha uzun versiyonlariyla almak gerekirse teker teker her 3.14 değeri değiştirilmek yerine
baştaki PI sabitinin değerinin değiştirilmesi yeterli olacaktir. Ayni şey değişkenler için de geçerlidir falat değişkenler adlarindan da anlaşilabileceği gibi
gerektiğinde değiştirilebilir olarak tanimlanabilirler. Aşağida örnekleri verilecektir.

Sabitleri atamak için "const" anahtar sözcüğü kullanilir. const'tan sonra kişinin takdirine göre sabit ismi, sonra (:), sonra türü, sonra (=), sonra değeri, sonra da (;) yazilir.

Sabitler aşağida gelecek değişkenlerden farkli olarak asla değişebilir olarak ayarlanamaz, yine farkli olarak global kapsamda veya alt kapsamlarda belirlenebilirler.
Değişkenlerse global kapsamda (global scope) belirlenemezler. Global kapsam herhangi bir fonksiyon başlamadan önceki ve fonksiyonlar bittiktensonraki kapsamdir.
Kapsamlar ({}) ile belirtilir. Aşağida örneği görülecektir.

Ayni zamanda sabitler değişkenlerin aksine IDE'nizdeki otomatik tür belirleme özelliğine tabi değildir, dolayisiyla tür programci tarafindan manuel yazilmalidir.
Otomatik tür belirleme sisteminin nasil çaliştiği aşağida gösterilecektir.

Ayrica sabitlerde değerin derleme esnasinda bilinmesi de zorunludur, programin çalişmasi sirasinda belirlenemezler.
Kisaca programci her sabit için değeri de türü de yazmak zorundadir.

Genel anlamda isimlendirmelerde uyulacak kurallar şunlardir:

- ASCII karakterleri kullanilmalidir: Yani Amerikan alfabesinde bulunmayan harf, rakam ve semboller kullanilamaz.
- İsimler (_) veya harfle başlamalidir, rakamlarla başlayamazlar.
- Büyük-küçük harf duyarliliği mevcuttur, yani "degisken" kelimesiyle "Degisken" kelimesi iki farkli değişkeni atamak için kullanilabilir.

- Sabitler isimlendirilirken tavsiye edilen yöntem "SCREAMING_SNAKE_CASE"dir. Yani tamami büyük harfli yazilir, kelimelerin arasini ayirmak için (_) kullanilir.
- Değişkenler ve fonksiyonlar isimlendirilirken tavsiye edilen yöntem "snake_case"dir. Yani tamami küçük harf olup boşluklar için (_) kullanilir.
*/

fn main() {
    /*
    fn: Yazilacak şeyin bir fonksiyon olduğunu bildirir. "main", yazilan fonksiyonun ismidir.
    Main fonksiyonu diğer fonksiyonlardan ayri olarak programin asil çaliştirdiği fonksiyondur. Derleyici sadece burayi dikkate alir.
    Parametre parantezleri yazilirken parantezlerle fonksiyon ismi arasina boşluk birakilmaz. "()" içine, varsa parametreler eklenir. Yoksa boş birakilir.
    ({): Fonksiyon başlangici için gerekli süslü parantezdir.
    Rust da diğer pekçok dil gibi okunabilirlik için girinti (indentation) kullanir.
    Girinti 4 boşluk miktarindadir, TAB değil.
    */

    println!("\nHello, world!\n");

    /*
    Evrensel ilk programdir. Komut satirina (CLI) istenen bir yazinin yazdirilmasini sağlayan bir makro yani karmaşik fonksiyondur.
    "println": makronun adi, (!): bunun normal bir fonksiyon değil makro olduğunun göstergesi, (()): parametre alicilari,
    parantez içindeki (""): yazdirilacak şeyin string (çok karakterli yazi) olduğunun belirtecidir. Onlarin içinde de yazdirilacak metin bulunur.
    (;) ise aşağida da söyleneceği üzere komutun bittiğinin göstergesidir ve pekçok komutun sonuna koyulur. Bazilarina koyulmaz.
    */

    /*********\
    DEĞİŞKENLER
    \*       */

    /*
    Aşağidaki tüm değişken değerlerinde alabilecekleri maksimum değer esas alinmiştir. Minimum değer ayrica belirtilmiştir. "||" "ya da" anlamindadir.

    Sayilari her üç basamakta bir "_" ile ayirmak mümkündür, okunakliği arttirmada kullanilan bir yöntemdir.

    Rust'ta değişkenler default olarak unmutable yani değiştirilemezdir. Değiştirilebilir yapmak için aşağida açiklanacak arti bir komut kullanilir.
    */

    // Unmutable signed integer (hem negatif hem pozitif değer alabilen, değiştirilemez tamsayi) türündeki değişkenlerin tanimlanma şekli:

    let q: i8 = 127; // || -128;
    let w: i16 = 32_767; // || -32_768;
    let e = 2_147_483_647; // || -2_147_483_648;
    let r: i64 = 9_223_372_036_854_775_807; // || -9_223_372_036_854_775_808;
    let t: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // || -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let y: isize = 0; // Bu değişken türünde size (boyut) uyarlanabilirdir. Kullanılan bilgisayarın mimarisine göre belirlenir (32-64).

    /*
    Görüldüğü üzere önce "let" işlemi kullanilarak değişken atanacaği bildirildi, (:) atilip tür yazildi, (=) sonrasina da türe özgü değer girildi.

    Sabitlerin aksine değişkenlerde değerin derleme esnasinda bilinmesi gerekmez, programin çalişmasi sirasinda belirlenebilir:
    */

    let x; // Görüldüğü üzere herhangi bir değer girilmedi. Derlemede "değer atanmamiş" olarak derlenecek.
    x = 0; // Değer burada belirlenmiş oldu, yani değişken ilk atandiğinda değil program çalişirken.

    /*
    Rust'da değişkenlerin türünü tanimlamak gereklidir ancak bu, programci yapmazsa rust_analyzer yüklü IDE tarafindan otomatik yapilir.
    Manuel tanimlamanin farki, otomatik yapildiğinda muhtemel olan fazla bellek kullanimi gibi sorunlari bulundurmamasidir.

    Otomatik tür tanimlamada genelde türlerin default (önceden belirlenmiş, varsayilan) bellekleri esas alinir. Mesela belli araliktaki sayilarda bu,
    yukaridaki (e) değişkeninde görüleceği üzere 32 bittir.
    Değişkenin otomatik tanimlandiğini renginin diğer tür belirteçlerinden farkli olarak gri olmasindan anlayabiliriz.

    Rust'da satirlarin çoğu (;) ile biter. Bazi satirlar bitmez ve bunun özel anlamlari vardir. Mesela "return" komutuna gerek kalmadan değer döndürmek gibi.
    İleride bunlara da değinilecektir.
    */

    // Unmutable unsigned integer (sifir veya pozitif sayilari değer alabilen, değiştirilemez tamsayi -minimum değer tamaminda 0 olmak üzere-) türündeki değişkenlerin tanimlanma şekli:

    let u: u8 = 255;
    let i: u16 = 65_535;
    let o: u32 = 4_294_967_295;
    let p: u64 = 18_446_744_073_709_551_615;
    let a: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let s: usize = 0; // Bu değişken türünde size uyarlanabilirdir. Kullanılan bilgisayarın mimarisine göre belirlenir (32-64).

    // Şu ana kadarki tüm değişkenler unmutable yani değiştirilemezdir. Mutable değişken tanimlama için "let"ten sonra "mut" kullanilir, böylece sonrasinda değer değiştirilebilir:

    let mut d = 0;
    d = 1;

    /*
    Görüldüğü üzere önce değişken mutable olarak ayarlanarak 0 değeri verildi, sonrasinda bu değer sadece değişken ismi ve (=) işareti kullanilarak 1 ile değiştirildi.
    Eğer mut komutu yazilmasaydi program derlenmezdi.

    Derleme Zamani (Compile Time): Kodlarin işlemcinin anlayacaği yegane dil olan binary'ye çevrilme sürecidir. Sonunda çaliştirilabilir (executable) bir dosya oluşur.
    Çalişma Zamani (Runtime)     : Derlenmiş kodlarin çaliştirildiği süreçtir. Rust'ta zor da olsa derleme zamaninda hata almayip çalişma zamaninda hata almak mümkündür.
    */

    // DEGISKENLERDE GOLGELEME (SHADOWING) \\

    /*
    Rust'ta ayni kapsam (scope) içinde ayni isimle iki değişken tanimlamak mümkündür. İkinci değişken birincinin değerini iptal eder ve bu değer kapsam bitip değer yok sayilana
    veya ikinci değişken de üçüncü bir değişken tarafindan gölgelenene dek geçerli olur. Aşağida ikisine de örnek verilmiştir:
    */

    let f: i32 = 10; // (f) isminde bir değişken tanimladik, türünü i32 olarak seçtik ve 10 değerini atadik.
    println!("Block scope ve shadowing denemeleri:\nSayinin ilk değeri: {f}"); // Ekrana (f)'in o siradaki yani ilk tanimlanan değeri yazdirilir.

    let f: i32 = f + 1; // İlk (f) değişkenini gölgeleyen ikinci bir değişken tanimladik ve değerini f/10 + 1 = 11 olarak atadik.
    println!("Sayinin ikinci değeri: {f}"); // Ekrana (f)'in o siradaki yani ilkini gölgeleyip atanan 11 değeri yazdirilir.

    {
        // Rust dilinde öncesinde herhangi bir fonksiyon veya ifade olmasa da alt kapsam açilabilir. Buna "block scope" denir. Kapsami açtik.
        let f: i32 = f * 2; // Kapsam içinde önceki 11 değerli (f) değişkenini de gölgeleyen üçüncü bir değişken tanimlayarak ona f/11 * 2 = 22 değerini atadik.
        println!("Sayinin üçüncü değeri: {f}"); // Ekrana (f)'in o siradaki yani ikinciyi de gölgeleyip atanan 22 değeri yazdirilir.
    } // Kapsam sona erdiğinden bu süslü parantezden sonra (f) değişkeni son geçerli değerine yani 11'e geri döndü.

    println!("Sayinin son değeri: {f}"); // Sayinin kapsamdan önceki son geçerli değeri olan 11 yazdirildi.

    //  YER TUTUCULARI  \\

    /*
    Burada görmüş olduğumuz diğer bir şey de yer tutucularidir. ({}) simgesi bu tarz bir string dizesi içerisindeyse dize içine önceden belirlenmiş olan değişkenin
    değerini yazdirmak için kullanilir. Bunu yukaridaki gibi değişkenin ismini süslü parantezlerin içine yazarak yapabileceğimiz gibi aşağidaki şekilde, parantezleri
    boş birakip string dizesi ("") ile sonlandirildiktan sonra (,) koyup değişken ismini yazarak da yapabiliriz.
    (Birden çok değişken bulunmasi halinde virgülden sonraki yaziliş sirasina göre yerleştirilir.):
    */

    println!("\nSayinin son değeri (virgüllü yer tutucu denemesi): {}", f);

    // Mut ile farkları: 1. Gölgeleme yaparken gölgelenenin değerini değiştirmek mümkündür:

    let g: i32 = 0;
    let g: &str = "Merhaba";

    // 2. Değişken mut ile değişebilir yapıldığında onu değiştirmek mümkün olur ancak gölgeleme değişmez değişkeni tek başına değişebilir yapmaz.

    //  KAPSAM TURLERI  \\

    /*
    1. Blok Kapsamı: İçerisinde tanımlanan değişkenler dışında kullanılamaz. Bu tüm kapsamlar için geçerlidir. Değer döndürebilir. Örneği yukarıdadır.
    2. Fonksiyon kapsamı: Şu an içinde bulunulan main kapsamı gibi.
    3. Döngü Kapsamı (Loop scope):
    */

    for i in 0..3 {
        let h = i * 2;
        println!("h'nin değeri: {h}");
    }

    // MIN-MAX KULLANIMI \\

    let j: i32 = i32::MAX; //i32 türünde alınabilecek en büyük değeri j'ye atar.
    let k: i32 = i32::MIN; //i32 türünde alınabilecek en küçük değeri j'ye atar.

    /*
    Diğer sayı sistemlerinde de değer atamak mümkündür:
    - Hex (16'lı sayı sistemi): onaltılık sayının önüne "0x" yazılır.
    - Octal (8'li sayı sistemi): "0o"
    - Binary (2'li sayı sistemi): "0b"
    - Byte (sadece u8): "b'"A"'"(?)
    */

    //  TAMSAYI TAŞMASI (INTEGER OVERFLOW)  \\

    /*
    - Debug modda derleme yaparken, değişken için tanımlanan tür katsayısı yazılan rakamı karşılayamayacak bir şekilde değiştirilirse derleme engellenir.
    - Ancak release modda engellenmez ve integer overflow yaşanır: sayı durumuna göre başa döner, aşağıdaki örnekte sayı negatif değer alamayacağından 255 değerini alır.
    - Eğer 255 değeri atamış olsaydık ve bir ekleseydik bu kez de değer en üst sınırı aşıp başa, 0'a dönecektir. Yani katsayılar dairesel çalışır.
    */

    let mut z: u8 = 0;
    println!("\nOverflow denemeleri:\n\nBaşlangıç değeri: {z}");

    z = z - 1;
    println!("Taşma sonrası değer: {z}");

    let mut c: u8 = 255;
    println!("\nOverflow denemeleri - 2:\n\nBaşlangıç değeri: {c}");

    c = c + 1;
    println!("Taşma sonrası değer: {c}");

    // SATURATING METODU \\

    /*
    Bu metot yapılacak işlemde eğer taşma varsa bu taşmayı engeller ve işlemi taşma olmadan yapılabilecek son halinde yazdırır.
    (.) işareti: belirli ayarları çağırmak için kullanılmaktadır.

    Saturating ayarının 4 farklı şekli bulunur:
    - Toplama için saturating_add
    - Çıkarma için saturating_sub
    - Çarpma için saturating_mul
    - Bölme için saturating_div ve
    - Üssünü almak için saturating_pow kullanılır.
    */

    let mut l: u8 = 1; // (l) değişkenine (0) değeri atılıyor ve u8 türünde belirleniyor. Alabileceği minimum değer zaten 0.
    println!("\nSaturating başlangıç değeri: {l}");

    l = l.saturating_sub(2); // Taşma engelleyici saturating komutuyla (l)'den 2 çıkarılıyor. Normalde taşması lazım ama izin vermiyor. 1 çıkardıktan sonra 0 kalıyor ve onu yazdırıyor.

    println!("Saturating taşma sonrası değer: {l}");

    // FLOAT TÜRÜ \\

    // Float ondalık sayılar için kullanılır ve iki alt türü vardır: f32 ve f64. Varsayılan olarak f64 kullanılır. Ototmatik type atamasında da bu şekilde olur.

    let v = 1.0; // Otomatik olarak daha fazla hassasiyet sağlayan f64 türünü seçti.
    let b: f32 = 1.0; // f32 yapmak istersek manuel olarak yazarız.

    /*
    Rust'ta aritmetik işlemler [toplama (+), çıkarma (-), çarpma (*), bölme (/) ve kalan (%)] şeklinde temsil edilir.
    Integer türünde işlem sonucu float çıkarsa en yakın integer'a yuvarlanır.
    */

    let qw: i32 = 3 / 2;
    println!("\nOndalıklı sonuca sahip tamsayı türünde değişkenin işlem sonucu: {qw}"); //Sonuç 1 olarak yazdırılır.

    let add = 2 + 1;
    let sub = 2 - 1;
    let mul = 2 * 1;
    let div = 2 / 1;
    let rem = 3 % 2;

    println!(
        "\nİşlem türlerinin sonuçları:\n\nToplama: {}\nÇıkarma: {}\nÇarpma: {}\nBölme: {}\nKalan: {}",
        add, sub, mul, div, rem
    ); //Sonuç 1 olarak yazdırılır.

    // BOOLEAN TÜRÜ \\

    // Sadece true (doğru) ve false (yanlış) değeri alabilen, hafızada 0 byte yer kaplayan, sadece if koşulu içinde anlamlı olan bir türdür.

    let dgr: bool = true;
    let yls: bool = false;

    // CHAR TÜRÜ \\

    /*
    Alfabedeki karakterleri temsil etmek için kullanılır. UTF-8 destekler. İki (') karakteri arasına yazılır. ("") arasına yazılırsa hata alınır ve derlenmez.
    4 byte yer kaplar.
    Bunların tamamı "Skaler Türler" olarak isimlendirilir. Aşağıda compound (birleşik) iki rust türü anlatılacaktır.
    */

    //** COMPOUND TÜRLER **\\

    // TUPLE TÜRÜ \\

    /*
     * Birden fazla değişkenin tek bir veri yapısı, diğer bir deyişle diğer başka bir değişkende bir arada tutulmasıdır.
     * Boyutları sabittir ve değişmez (Unmutable).
     */

    let tuple: (i32, f64, char, bool) = (1, 2.5, 'a', true); // İlk parantez içierisinde eleman sayısınca tür, ikincisine de sırasıyla değerleri girilir.
    println!("\nTuple: {:?}", tuple);

    /*
     * Görüldüğü gibi önce let ile değişken tanımlayacağımızı bildirdik.
     * Sonra değişkenimize isim verdik.
     * Sonra (:)'den sonrasında (=)'e kadar () içinde elemanların sırasıyla tür belirttik.
     * (=)'den sonraki () içindeyse sırasıyla değerleri girdik.
     */

    // tuple içindeki elemanlara erişmek için şu indis girmeli "değişken atama" yöntemi kullanılabilir:

    let first = tuple.0;
    let second = tuple.1;
    let third = tuple.2;
    let fourth = tuple.3;

    /*
    tuple'daki elemanlar, tuple isminden sonra yazılacak bir (.) ve sonrasında yazılacak indis numaralarıyla eleman sayısınca yeni değişkene atanabilir.
    Böylece tuple içindeki elemanlara teker teker ulaşmak da mümkün olur.
    İndis numaraları 0'dan başlar ve eleman sayısından bir eksiktir.
    */

    println!("\nTuple içindeki elemanlar: {first}, {second}, {third}, {fourth}");

    /*
    Bu değerlere ulaşmanın diğer bir yolu da örüntü eşlemedir (pattern matching). Diğer bir adı da ögelere ayırmadır (deconstruction).
    Bu işlem de tuple başta belirlendikten sonra yapılır.
    */

    let (itg, flt, chr, bol) = tuple;
    println!("{itg} {flt} {chr} {bol}");

    // ARRAY TÜRÜ \\

    /*
     * Özellikleri:
     * Tuple'dan farklı olarak tüm değişkenlerin türü aynı olmalıdır.
     * Tuple gibi sabittir ve değiştirilemez (Unmutable).
     * Tuple gibi indisler 0'dan başlar fakat atama yapılacağı zaman (.)'dan sonra değil isimden sonraki ([]) içinde yazılır. değer atama = iledir.
     * Array'ın tüm değişkenlerne değer atanması zorunludur.
     */

    let ary: [i32; 5] = [1, 2, 3, 4, 5];

    /*
     * Görüldüğü üzere önce let ile değişken yazacağımızı belirttik,
     * Sonra isim verdk.
     * (:)'dan sonra ([]) içerisine array içindeki elemanların türünü ve (;) ile ayırarak kaç tane eleman olacağını girdik.
     * (=)'den sonra da yine ([]) içine değişkenlerin değerlerini (,) ile ayırarak yazdık.
     */

    // ARRAY'LERİN DEĞİŞKENLERİNE TEKER TEKER DEĞER ATAMA:
    // Bunu şu şekilde yaparız:

    let mut ary: [i32; 5] = [0; 5]; // mut kullanarak array'ın değişkenlerini sonradan tanımlanabilir yaptık. Sonra tüm değişkenlere 0 atadık ([0; 5]).

    // 0'dan başka bir değer de atanabilirdi.
    // Fark ettiğimiz diğer nokta arrayın değişkenlerini de değerini de atarken sadece önce tür veya değer (;)'dan sonra da adet belirterek atama yapılabildiği.

    ary[0] = 1;
    ary[1] = 2;
    ary[2] = 3;
    ary[3] = 4;
    ary[4] = 5;

    println!(
        "\nArray sıfırıncı ve birinci eleman indis değer sorgu:\n1: {}\n2: {}",
        ary[0], ary[1]
    );

    // Sınırların üstüne çıkılırsa compiler error (panic) verecektir.

    /**********\
    FONKSİYONLAR
    \*        */

    /*
     * Function: Belirli bir işlemi gerçekleştiren ve sonuç dmndürebilen kod bloğu.
     * fn: Fonksiyon belirten primitive type (ilkel tür).
     * Sonra ismi verilir ve boşluksuz () girilir. İçine döndürülecek şeyin türü yazılır. Değer döndürmüyorsa boş bırakılır (unit).
     * Sonra main'de aşina olduğumuz şekilde {} kullanılır.
     */

    merhaba(); // Fonksiyon (merhaba) için aşağıya bakınız.
    println!("i32 döndüren toplama fonksiyonu denemesi: {}", top(5, 5)); // Fonksiyon (top) için aşağıya bakınız.
    println!("i32 döndüren çıkarma fonksiyonu denemesi: {}", cık(5, 5)); // Fonksiyon (cık) için aşağıya bakınız.
    println!(
        "i32 döndüren komleks çıkarma fonksiyonu denemesi: {}",
        complex(5, 5)
    ); // Fonksiyon (complex) için aşağıya bakınız.

    // EXPRESSION (İFADE) VE STATEMENT (İŞLEM) \\

    /*
     * Expression: Değer üreten kod parçalarına expression denir. İşlemlerin parçalarıdır.
     * - Aritmetik işlemler
     * - Değişkenler (let x: i32 = 5'te 5 bir ifadedir.)
     * - Fonksiyon çağrıları (hiçbie değer döndürmese de unit yani boş değer döndürmüş olurlar.)
     * - Değer döndüren kontrol yapıları
     * - Block scope'lar.
     * Statement: Değer üretmeyen kod parçalarıdır.
     * - Değişken atamaları (let'le başlayan atama kodunun tamamı. Kısımlarıysa ifadedir.)
     * - Değer döndürmeyen kontrol yapıları
     * Diğer bazı dillerde değişken atamaları statement değil expression'dur. Dolayısıyla değişken atanırken değer kısmına başka bir değişken ataması girilebilir.
     * Örnek:
     * Rust'ta şu yapılamaz (kod derlenmez): let a: i32 = let b: i32 = 5 fakat C'de int a = int b = 5 yazılabilir.
     * !: Fonksiyon çağrıları (yürütme olayının yapıldığı sıra yazılan) expression, fonksiyon tanımları (fonksiyonun ne yaptığını belilediğimiz kısım) statement'tir.
     */
}

fn merhaba() -> () {
    // Fonksiyonumuz hiçbir parametre almadığından isme bitişik parantezlere parametre girmedik. Değer döndürmediğinden (-> ()) ekledik, unit yani void döndürecek.
    println!("\nUnit fonksiyon denemesi: Merhaba!");
}

fn top(s1: i32, s2: i32) -> i32 {
    /*
     * Parantez içerisine fonksiyonumuz kaç parametre alacaksa türüyle beraber yazıyoruz.
     * Döndüreceği değerin türünü (dönüş türü) (->)'tan sonra yazıp {} içine fonksiyonumuzu yazıyoruz.
     */
    let sonuc = s1 + s2;
    sonuc
    // Fonksiyonların döndüreceği değerleri taşıyan değişkenler, Rust dilinde return kullanılmadan ve sonuna (;) koyulmadan bu şekilde sonuna yazılır.
}

fn cık(s1: i32, s2: i32) -> i32 {
    let sonuc = s1 - s2;
    return sonuc;
    // Klasik (return;) komutuyla da değer döndürmek mümkündür. Hatta eğer fonksiyonun tamamının sonu değilse zorunluluktur. Sonsa ilk yöntem tercih edilir.
}

fn complex(s1: i32, s2: i32) -> i32 {
    if s1 == 0 {
        return s2; // Mecburi return kullanımı: fonksiyon tamamlanmadı.
    }
    if s2 == 0 {
        return s1;
    }
    let sonuc = s1 + s2;
    sonuc // Fonksiyon sonu, yalın kullanım tercihi.
}
