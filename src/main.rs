use rand::{thread_rng,Rng};

fn main() {
    let modes = [1,2,3];
    println!("The following modes are available:\n  - All letters (1)\n  - One random letter (2)\n  - Random letters sequence (3)");
    loop{
        println!("Please select the mode (1, 2 or 3)");
        let mut temp_num = String::new();
        let _ = std::io::stdin().read_line(&mut temp_num);
        let mode: u32 = temp_num.trim().parse().unwrap();
        if !modes.contains(&mode){
            println!("No such mode, please start again");
        }
        match mode{
            1=>all_letters(),
            2=>random_letters(),
            3=>random_sequence(),
            _=>{}
        }
        println!("\nTo select new mode and continue press Enter. To exit press 'e' and Enter");
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        match user_input.trim().to_lowercase().as_str(){
            "e"=> break,
            _ => continue,
        }
    }
}

fn all_letters(){
    let alphabet = Alphabet::new();
    println!("All letters mode. Press Enter to get the mkhedruli writing.");
    println!("To continue to the next letter press Enter. To exit press 'e' and Enter");
    for letter in &alphabet.alphabet{
        print_description(letter);
        let _ = std::io::stdin().read_line(&mut String::new());
        print_mkhedruli_writing(letter);
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        match user_input.trim().to_lowercase().as_str(){
            "e"=> break,
            _ => continue,
        }
    }
}

fn random_letters(){
    println!("Random letter mode. Press Enter to get the mkhedruli writing.");
    println!("To pick new letter press Enter. To exit press 'e' and Enter");
    let alphabet = Alphabet::new();
    loop{
        let letter = get_random_letter(&alphabet);
        print_description(letter);
        let _ = std::io::stdin().read_line(&mut String::new());
        print_mkhedruli_writing(letter);
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        match user_input.trim().to_lowercase().as_str(){
            "e"=> break,
            _ => continue,
        }
    }
}

fn random_sequence(){
    let alphabet = Alphabet::new();
    println!("Sequence mode.");
    loop{
        println!("Please enter the sequence length and press Enter.\nTo get the mkhedruli writings press Enter.");
        let mut temp_num = String::new();
        let _ = std::io::stdin().read_line(&mut temp_num);
        let sequence_len: u32 = temp_num.trim().parse().unwrap();
        let mut letters_refs:Vec<&Letter> = vec![];
        let mut i = 0;
        while i < sequence_len{
            let letter = get_random_letter(&alphabet);
            if !letters_refs.contains(&letter){
                print_description(letter);
                letters_refs.push(letter);
                i+=1;
            } 
        }
        let _ = std::io::stdin().read_line(&mut String::new());
        for letter in letters_refs{
            print_mkhedruli_writing(letter);
        }
        println!("To continue press Enter. To exit press 'e' andEenter");
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        match user_input.trim().to_lowercase().as_str(){
            "e"=> break,
            _ => continue,
        }
    }
}

fn print_description(letter: &Letter){
    println!("===================\nPosition: {}\nName: {}\nRussian equivalent: {}",letter.position,letter.name,letter.russian_equivalent);
}
fn print_mkhedruli_writing(letter: &Letter){
    println!("Mkhedruli letter: {}",letter.georgian_writing);
}
fn get_random_letter(alphabet: &Alphabet) -> &Letter{
    let mut thr = thread_rng();
    let rand_number = thr.gen_range(0..33);
    &alphabet.alphabet[rand_number]
}

struct Alphabet{
    alphabet: Vec<Letter>,
}
impl Alphabet{
    fn new()->Self{
        let alphabet = Alphabet::create_vec();
        Alphabet{
            alphabet
        }
    }
    fn create_vec() -> Vec<Letter>{
        let mut alphabet = vec![];
        alphabet.push(Letter{position:1,name:"AN",russian_equivalent:"А",georgian_writing:'ა',});
        alphabet.push(Letter{position:2,name:"BAN",russian_equivalent:"Б",georgian_writing:'ბ',});
        alphabet.push(Letter{position:3,name:"GAN",russian_equivalent:"Г",georgian_writing:'გ',});
        alphabet.push(Letter{position:4,name:"DON",russian_equivalent:"Д",georgian_writing:'დ',});
        alphabet.push(Letter{position:5,name:"EN",russian_equivalent:"Э",georgian_writing:'ე',});
        alphabet.push(Letter{position:6,name:"VIN",russian_equivalent:"В",georgian_writing:'ვ',});
        alphabet.push(Letter{position:7,name:"ZEN",russian_equivalent:"З",georgian_writing:'ზ',});
        alphabet.push(Letter{position:8,name:"TAN",russian_equivalent:"Т",georgian_writing:'თ',});
        alphabet.push(Letter{position:9,name:"IN",russian_equivalent:"И",georgian_writing:'ი',});
        alphabet.push(Letter{position:10,name:"KAN",russian_equivalent:"К",georgian_writing:'კ',});
        alphabet.push(Letter{position:11,name:"LAS",russian_equivalent:"Л",georgian_writing:'ლ',});
        alphabet.push(Letter{position:12,name:"MAN",russian_equivalent:"М",georgian_writing:'მ',});
        alphabet.push(Letter{position:13,name:"NAR",russian_equivalent:"Н",georgian_writing:'ნ',});
        alphabet.push(Letter{position:14,name:"ON",russian_equivalent:"О",georgian_writing:'ო',});
        alphabet.push(Letter{position:15,name:"PAR",russian_equivalent:"П",georgian_writing:'პ',});
        alphabet.push(Letter{position:16,name:"ZHAR",russian_equivalent:"Ж",georgian_writing:'ჟ',});
        alphabet.push(Letter{position:17,name:"RAE",russian_equivalent:"Р",georgian_writing:'რ',});
        alphabet.push(Letter{position:18,name:"SAN",russian_equivalent:"C",georgian_writing:'ს',});
        alphabet.push(Letter{position:19,name:"TAR",russian_equivalent:"T",georgian_writing:'ტ',});
        alphabet.push(Letter{position:20,name:"UN",russian_equivalent:"У",georgian_writing:'უ',});
        alphabet.push(Letter{position:21,name:"PHAR",russian_equivalent:"П",georgian_writing:'ფ',});
        alphabet.push(Letter{position:22,name:"KHAR",russian_equivalent:"К",georgian_writing:'ქ',});
        alphabet.push(Letter{position:23,name:"GHAN",russian_equivalent:"Гх",georgian_writing:'ღ',});
        alphabet.push(Letter{position:24,name:"QAR",russian_equivalent:"Кх",georgian_writing:'ყ',});
        alphabet.push(Letter{position:25,name:"SHIN",russian_equivalent:"Ш",georgian_writing:'შ',});
        alphabet.push(Letter{position:26,name:"CHIN",russian_equivalent:"Ч",georgian_writing:'ჩ',});
        alphabet.push(Letter{position:27,name:"CAN",russian_equivalent:"Ц",georgian_writing:'ც',});
        alphabet.push(Letter{position:28,name:"JIL",russian_equivalent:"Дз",georgian_writing:'ძ',});
        alphabet.push(Letter{position:29,name:"CIL",russian_equivalent:"Ц'",georgian_writing:'წ',});
        alphabet.push(Letter{position:30,name:"CHAR",russian_equivalent:"Ч'",georgian_writing:'ჭ',});
        alphabet.push(Letter{position:31,name:"XAN",russian_equivalent:"Кх",georgian_writing:'ხ',});
        alphabet.push(Letter{position:32,name:"JHAN",russian_equivalent:"Дж",georgian_writing:'ჯ',});
        alphabet.push(Letter{position:33,name:"HAE",russian_equivalent:"Х",georgian_writing:'ჰ',});
        alphabet
    }
}
#[derive(Debug, PartialEq)]
struct Letter{
    position: u8,
    name: &'static str,
    russian_equivalent: &'static str,
    georgian_writing: char,
}
impl std::fmt::Display for Letter{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f,"Position: {}\nName: {}\nRussian equivalent: {}\nMkhedruli letter: {}", self.position, self.name, self.russian_equivalent, self.georgian_writing)
    }
}
