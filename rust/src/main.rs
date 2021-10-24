/// अ॒ध्या॒यः
#[derive(Debug)]
struct Adyayh<'a> {
    /// पा॒ठः
    path: &'a str,
    /// अ॒ध्या॒यस्य॒ क्रम॑स्य॒ संस्कृ॑ते॒ नाम॑
    krmnam: String,
    /// क्रमः॑
    krmh: usize,
    /// पर्व॑णो॒ नाम॑
    prvnam: &'a str,
}

/// पर्व॑
struct Prv<'a> {
    adyayah: Vec<Adyayh<'a>>,
}

/// पर्व॑णां॒ ग्रह॑णम्
fn prvgrhnm<'a>(path: &'a str) -> Vec<Prv<'a>> {
    let mut prvani: Vec<Prv> = vec![];
    let re = regex::Regex::new(r"(?ms)^([ं-् <b>]+)[ोऽ] ?ध्या ?यः *[ \*</b>]*$(.*?)ಇ([^ಇ]*)[ोऽ] ?ध?्?या ?य?यः?२?[ ्।</b>\n]*([०-९]+)[ ।</b>]*$").unwrap();
    let ankkrmh = "०१२३४५६७८९";
    let snkya = |path: &str| -> usize {
        path.chars().fold(0, |s, c| {
            s * 10 + ankkrmh.chars().position(|a| a == c).unwrap()
        })
    };
    for adyaypath in re.captures_iter(path) {
        let pathbagh = |krmh| {
            let bagh = adyaypath.get(krmh).unwrap();
            &path[bagh.start()..bagh.end()]
        };
        let adyayh = Adyayh {
            path: pathbagh(2),
            krmnam: pathbagh(1).replace(" ", "").replace("<b>", "").replace("</b>", ""),
            krmh: snkya(pathbagh(4)),
            prvnam: pathbagh(3),
        };
        let krmnam = adyayh.krmnam.to_string();
        let krmh = adyayh.krmh;
        let prvnam = adyayh.prvnam.to_string();
        if adyayh.krmnam == "प्रथमो" {
            println!("prathamáḥ.");
            prvani.push(Prv {
                adyayah: vec![adyayh],
            });
        } else {
            let prvsnkya = prvani.len();
            assert!(prvsnkya > 0);
            prvani[prvsnkya - 1].adyayah.push(adyayh);
        }
        let adyaysnkya = prvani.last().unwrap().adyayah.len();
        println!(
            "nā́ma: {}, párva: {}, kramanāmá: {}, krámaḥ: {}",
            prvnam,
            prvani.len(),
            krmnam,
            krmh
        );
        assert!(krmh == adyaysnkya || (krmh / 10 == 1 && adyaysnkya / 10 == 9 && krmh % 10 == adyaysnkya % 10));
    }
    prvani
}

fn pathsodh(path: String) -> String {
    let mut p = path;
    let r = [
        (r"\[sl\]", "<i>"),
        (r"\[/sl\]", "</i>"),
        (r"\[/[^\[\]]*-Bold\]", "</b>"),
        (r"\[[^\[\]]*-Bold\]", "<b>"),
        (r"\[[!-Z\^-~]+\]", ""),
        (r"</b>([ \n]*)<b>", "$1"),
        (r"</i>([ \n]*)<i>", "$1"),
    ];
    for x in r {
        p = regex::Regex::new(x.0).unwrap().replace_all(&p, x.1).to_string();
    }
    let mut q = "".to_string();
    let succ = regex::Regex::new(r"(.)<CCsucc>(([क-हक़-य़]़?्)*[क-हक़-य़]़?)").unwrap();
    let prec = regex::Regex::new(r"(([क-हक़-य़]़?्)*[क-हक़-य़ऋ][^क-हक़-य़ऋ]*)र्<CCprec>").unwrap();
    while q.ne(&p) {
        q = p.to_string();
        p = succ.replace_all(&p, "$2$1").to_string();
        p = prec.replace_all(&p, "र्$1").to_string();
    }
    p.replace("र्ऋ", "रृ")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    let krsnarjunsmvadh =
        regex::Regex::new(r"(श्री ?कृ ?ष्णा ?र्जु ?न ?सं ?वा ?दे(.*?)[ोऽ] ?ध?्?या ?य?यः?)").unwrap();
    println!("pāṭhó gṛhyate.");
    let path = krsnarjunsmvadh
                .replace_all(
                    &pathsodh(std::fs::read_to_string(&args[1]).unwrap()),
                    r"${1}ಗ",
                )
                .replace("इति", "ಇ")
                .replace("\u{0c}", "");
    for (i, prv) in prvgrhnm(&path).iter().enumerate() {
        std::fs::create_dir_all(format!("../pages/mbh/{}", i + 1));
        for (ai, adyayh) in prv.adyayah.iter().enumerate() {
            println!("adhyāyó likhyate: {}.{}", i + 1, ai + 1);
            std::fs::write(
                format!("../pages/mbh/{}/{}.html", i + 1, ai + 1),
                format!(
                    "<title>{}ोऽध्यायः</title><b>{}ोऽध्यायः</b><br>{}<br><b>इति {}ोऽध्यायः॥</b>",
                    adyayh.prvnam,
                    adyayh.krmnam,
                    adyayh.path.replace("\n", "<br>").replace(" ", "&nbsp;"),
                    adyayh.prvnam.replace("<b>", "").replace("</b>", "")
                )
                .replace("ोो", "ो")
                .replace("ಇ", "इति")
                .replace("ಗ", ""),
            );
        }
    }
}
