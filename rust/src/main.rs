/// अ॒ध्या॒यः
#[derive(Debug)]
struct Adyayh<'a> {
    /// पाठः॑
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

fn main() {
    println!("pāṭhó gṛhyate.");
    let krsnarjunsmvadh =
        regex::Regex::new(r"(श्री ?कृ ?ष्णा ?र्जु ?न ?सं ?वा ?दे(.*?)[ोऽ] ?ध?्?या ?य?यः?)").unwrap();
    let path = (1..15434)
        .map(|i| {
            if i % 100 == 0 {
                print!("{} ", i);
            }
            krsnarjunsmvadh
                .replace_all(
                    &std::fs::read_to_string(format!("../txt/{:05}.txt", i)).unwrap(),
                    r"${1}ಗ",
                )
                .replace("इति", "ಇ")
        })
        .collect::<Vec<String>>()
        .join("\n");
    std::fs::write("path", &path);
    println!("\npāṭhó gṛhītáḥ.");
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
