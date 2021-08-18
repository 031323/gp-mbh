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
    prvnam: Option<&'a str>,
}

/// पर्व॑
struct Prv<'a> {
    adyayah: Vec<Adyayh<'a>>,
}

/// पर्व॑णां॒ ग्रह॑णम्
fn prvgrhnm<'a>(path: &'a str) -> Vec<Prv<'a>> {
    let mut prvani: Vec<Prv> = vec![];//             षट् पञ्चाशत्तमोऽध्यायः णि गोहरणपर्वणि दे वागमने षट्पञ्चाशत्तमोऽध्यायः  । । 
    let re = regex::Regex::new(r"(?ms)^ *([ं-् ]+)[ोऽ] ?ध्या ?यः[ \*]*$(.*?)इति (.*?)[ोऽ] ?ध?्?या ?य?यः?[ ।\n]*([०-९]+)[ ।]*$").unwrap();
    let ankkrmh = "०१२३४५६७८९";
    let snkya = |path: &str| -> usize {
        path.chars()
            .fold(0, |s, c| s * 10 + ankkrmh.chars().position(|a| a == c).unwrap())
    };
/*
    let adyaypdani: Vec<_> = path.match_indices("ऽध्यायः").collect();
    let adyaysbdaksrsnkya = "ऽध्यायः".len();
    assert_eq!(adyaypdani.len() % 2, 0);
    for i in 0..adyaypdani.len()/2 {
        let prtmm = adyaypdani[i*2].0;
        let dvitiym = adyaypdani[i*2+1].0;
        let adyayh = Adyayh {
            path: &path[prtmm + adyaysbdaksrsnkya..dvitiym + adyaysbdaksrsnkya],
            krmnam: path[prtmm - (0..).find(|i| *path.as_bytes()[prtmm-i..].iter().next().unwrap() == "\n".as_bytes()[0]).unwrap() + 1..prtmm].replace(" ", ""),
            krmh: 1,//snkya(&path[dvitiym + adyaysbdaksrsnkya..path[dvitiym + adyaysbdaksrsnkya..].chars().position(|c| c == '\n').unwrap() + dvitiym + adyaysbdaksrsnkya]).unwrap(),
            prvnam: None,
        };
        let krmnam = adyayh.krmnam.to_string();
        if adyayh.krmnam == "प्रथमो" {
            assert_eq!(adyayh.krmh, 1);
            prvani.push(Prv{adyayah: vec![adyayh]});
        } else {
            let prvsnkya = prvani.len();
            assert!(prvsnkya > 0);
            prvani[prvsnkya-1].adyayah.push(adyayh);
        }
        if prvani.len() == 1 {
            println!("parvasaṅkhyā́: {}, adhyāyasaṅkhyā́: {}, kramanāmá: {}", prvani.len(), prvani.last().unwrap().adyayah.len(), krmnam);
        }
    }
    */

    for adyaypath in re.captures_iter(path) {
        let pathbagh = |krmh| {
            let bagh = adyaypath.get(krmh).unwrap();
            &path[bagh.start()..bagh.end()]
        };
        let adyayh = Adyayh {
            path: pathbagh(0),
            krmnam: pathbagh(1).replace(" ", ""),
            krmh: snkya(pathbagh(4)),
            prvnam: Some(pathbagh(3)),
        };
        let krmnam = adyayh.krmnam.to_string();
        let krmh = adyayh.krmh;
        if adyayh.krmnam == "प्रथमो" {
            prvani.push(Prv{adyayah: vec![adyayh]});
        } else {
            let prvsnkya = prvani.len();
            assert!(prvsnkya > 0);
            prvani[prvsnkya-1].adyayah.push(adyayh);
        }
        let adyaysnkya = prvani.last().unwrap().adyayah.len();
        assert!(krmh == adyaysnkya || (krmh == 15 && adyaysnkya == 95));
        println!("párva: {}, kramanāmá: {}, krámaḥ: {}, nā́ma: {}", prvani.len(), krmnam, krmh, prvani.last().unwrap().adyayah.last().unwrap().prvnam.unwrap());
    }
   
    prvani
}

fn main() {
    println!("pā́ṭho gṛhyate.");
    let path = (1..15434/*4*/).map(|i| {
        if i % 100 == 0 {
            print!("{} ", i);
        }
        std::fs::read_to_string(format!("../txt/{:05}.txt", i)).unwrap()
    }).collect::<Vec<String>>().join("\n");
    println!("\npā́ṭho gṛhītáḥ.");
    for l in path.split("\n").filter(|l| l.contains("ऽध्यायः")).take(200) {
        println!("{}", l);
    }
    let prvani = prvgrhnm(&path);
    for adyayh in prvani[0].adyayah.iter() {
        //println!("{} {}", adyayh.krmnam, adyayh.path.len());
    }
}