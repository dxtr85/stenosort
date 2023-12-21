use std::collections::{HashMap, HashSet};
// use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let długość = args.len();
    if długość != 4 {
        eprintln!(
            "Użycie: {} <porządek sortowania> <plik do sortowania> <plik wyjściowy>",
            &args[0]
        );
        exit(255)
    }
    let porządek = args.get(1).unwrap();
    let plik = args.get(2).unwrap();
    let plik_wyjściowy = args.get(3).unwrap();
    println!(
        "Kolejność: {}\nWejście:   {}\nWejście:   {}",
        porządek, plik, plik_wyjściowy
    );
    // if !bez_powtórzeń(porządek.chars()) {
    //     eprintln!("\x1b[97;41;5mERR\x1b[m Porządek zawiera powtórzenia: {}", porządek);
    //     exit(254)
    // }
    let mut indeksy = HashMap::new();
    let mut odwrócone_indeksy = HashMap::new();
    let mut indeks: u8 = 0;
    for znak in porządek.chars() {
        if indeksy.contains_key(&znak) {
            eprintln!(
                "\x1b[97;41;5mERR\x1b[m Znak {} już jest zaindeksowany",
                znak
            );
            continue;
        }
        indeksy.insert(znak, indeks);
        odwrócone_indeksy.insert(indeks, znak);
        // println!("{} - {}", znak, indeks);
        indeks += 1;
    }
    // let zawartość = fs::read_to_string(&plik).expect("Coś nie bangla z tym plikiem");
    println!("Uzupełniam kombinacje...");
    let mut kombinacje_indeksowane = Vec::<Kombinacja>::new();
    uzupełnij_kombinacje(&mut kombinacje_indeksowane, plik, &indeksy);
    println!("Sortuję...");
    kombinacje_indeksowane.sort_by(|a, b| a.znaki.cmp(&b.znaki));
    println!("Zapisuję...");
    let f = File::create(plik_wyjściowy).expect("Nie udało się utworzyć pliku wyjściowego");
    let mut f = BufWriter::new(f);
    f.write_all("{\n".as_bytes()).expect("Unable to write data");
    for kombi in kombinacje_indeksowane.drain(..) {
        let tekst = deindeksuj(&kombi.znaki, &odwrócone_indeksy);
        let do_zapisu = format!(" \"{}\": \"{}\",\n", tekst, kombi.tekst);
        f.write_all(do_zapisu.as_bytes());
    }
    f.write_all("}\n".as_bytes()).expect("Unable to write data");
}

fn bez_powtórzeń<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn uzupełnij_kombinacje(
    kombinacje: &mut Vec<Kombinacja>,
    plik: &String,
    indeksy: &HashMap<char, u8>,
) {
    let plik = File::open(plik).expect("Unable to open file");
    let zawartość = BufReader::new(plik);
    for linia in zawartość.lines() {
        let mut akord_zaczęty = false;
        let mut akord_kompletny = false;
        let mut tekst_zaczęty = false;
        // println!("Uzupełniam z {}", linia);
        let mut akord = String::new();
        let mut tekst = String::new();
        for znak in linia.unwrap().trim().chars() {
            match znak {
                ' ' => {
                    if tekst_zaczęty {
                        tekst.push(' ')
                    }
                }
                '"' => {
                    if !akord_kompletny {
                        akord_zaczęty = !akord_zaczęty;
                        // println!("{:?}", akord_zaczęty)
                    } else {
                        tekst_zaczęty = !tekst_zaczęty;
                        if !tekst_zaczęty {
                            // println!("koniec");
                            break;
                        }
                    }
                }
                ':' => {
                    if !tekst_zaczęty {
                        akord_kompletny = true
                    } else {
                        tekst.push(znak);
                    }
                }
                _ => {
                    if akord_zaczęty {
                        akord.push(znak);
                    } else if tekst_zaczęty {
                        tekst.push(znak);
                    }
                }
            }
        }
        // println!("dodaję akord:{}, tekst: {}", akord, tekst);
        if akord_kompletny {
            let znaki = indeksuj(&akord, &indeksy);
            kombinacje.push(Kombinacja { znaki, tekst });
            // kombinacje.insert(akord, tekst);
        }
    }
}

fn indeksuj(tekst: &str, indeksy: &HashMap<char, u8>) -> Vec<u8> {
    let mut wektor = Vec::new();
    for znak in tekst.chars() {
        if let Some(ind) = indeksy.get(&znak) {
            wektor.push(ind.clone())
        } else {
            eprintln!("Nie znalazłem indeksu dla {}, zakładam indeks 0", &znak);
            wektor.push(0)
        }
    }
    wektor
}

fn deindeksuj(indeksy: &Vec<u8>, deindeksy: &HashMap<u8, char>) -> String {
    let mut tekst = String::new();
    for indeks in indeksy.iter() {
        tekst.push(deindeksy.get(&indeks).unwrap().clone())
    }
    tekst
}

struct Kombinacja {
    znaki: Vec<u8>,
    tekst: String,
}
