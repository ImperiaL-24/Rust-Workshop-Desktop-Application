use slint::include_modules;

include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.on_handle_click(move |button: slint::SharedString| {
        let ui = ui_handle.unwrap();
        match button.as_str() {
            "C" | "c" => ui.set_textarea("".into()),
            "=" | "\n" => {
                ui.set_textarea(match compute(ui.get_textarea().as_str()) {
                    Ok(val) => val.to_string().into(),
                    Err(str) => str.into()
                });
            },
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "*" | "/" | "+" | "-" | "(" | ")" | "." => {
                let mut new_str = ui.get_textarea();
                new_str.push_str(&button.as_str());
                ui.set_textarea(new_str);
            },
            _ => ()
        };

        // TASK: Adaugă logica pentru cazurile:
        // - "C": Golirea zonei de text
        // - "=": Calcularea rezultatului și afișarea acestuia
        // - Altele: Adăugarea input-ului curent la zona de text
        // HINT: Folosește un `match` pentru a verifica valoarea `new_input`.
    });

    ui.run()
}

// TASK: Completează funcția `evaluate`, care verifică validitatea expresiei și returnează rezultatul sau un mesaj de eroare
// HINT: Folosește regex-ul `VALID_EXPRESSION` pentru a verifica dacă `input` este o expresie validă.
// Dacă expresia este validă, apelează funcția `compute`. Dacă nu, returnează un mesaj de eroare, cum ar fi "Invalid Expression".
// fn evaluate(input: &str) -> String {
//     todo!() // <-- Înlocuiește `todo!()` cu implementarea funcției
// }

#[derive(Clone, Copy, PartialEq)]
enum Token {
    ADD,
    SUB,
    MUL,
    DIV,
    INT(f64),
    LPAR,
    RPAR,
    EOS
}

struct Parser {
    token_idx: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            token_idx: 0,
            tokens
        }
    }

    fn next(&mut self) {
        self.token_idx += 1;
    }

    fn get(&self) -> Token {
        if self.token_idx >= self.tokens.len() {
            Token::EOS
        } else {
            self.tokens[self.token_idx]
        }
    }

    fn expect(&mut self, tok:Token) -> Result<(), &'static str> {
        if self.get() != tok {
            Err("Unexpected Token")
        } else {
            Ok(())
        }

        
    }
}

struct Tokenizer {
    tok: Option<String>,
    tokens: Vec<Token>
}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            tok: None,
            tokens: Vec::new()
        }
    }

    fn finish(&mut self) {
        if let Some(token) = &self.tok {
            let val = token.as_str().parse::<f64>().unwrap();
            self.tokens.push(Token::INT(val));
        }
        self.tok = None;
    }
    fn add(&mut self, chr: char) {
        self.tok = Some(if let Some(token) = self.tok.as_mut() {
            token.push(chr);
            token.to_string()
        } else {
            chr.to_string()
        })
        
    }
    fn add_token(&mut self, tok: Token) {
        self.finish();
        self.tokens.push(tok);
    }

    fn get(self) -> Vec<Token>{
        self.tokens
    }
}

fn literal(parser: &mut Parser) -> Result<f64, &'static str> {
    match parser.get() {
        Token::LPAR => {
            parser.next();
            let val = expr(parser)?;
            parser.expect(Token::RPAR)?;
            Ok(val)
        }
        Token::INT(val) => {
            parser.next();
            Ok(val)
        }
        _ => Err("Invalid Expression!")
    }
}

fn factor(parser: &mut Parser) -> Result<f64, &'static str> {
    let mut val = literal(parser)?;
    while parser.get() == Token::ADD || parser.get() == Token::SUB {
        let op = parser.get();
        parser.next();
        let rhs = literal(parser)?;
        if op == Token::ADD {
            val += rhs;
        } else {
            val -= rhs;
        }
    }
    Ok(val)
}

fn expr(parser: &mut Parser) -> Result<f64, &'static str> {
    let mut val = factor(parser)?;
    while parser.get() == Token::MUL || parser.get() == Token::DIV {
        let op = parser.get();
        parser.next();
        let rhs = factor(parser)?;
        if op == Token::MUL {
            val *= rhs;
        } else {
            val /= rhs;
        }
    }
    Ok(val)
}

// TASK: Implementează funcția `compute` pentru a realiza operațiile de bază (+, -, *, /) și a returna rezultatul
// HINT: Parcurge simbolurile de operare și folosește `.split()` pentru a împărți `input` în două părți: înainte și după simbol.
// Convertește fiecare parte în `f64` și returnează rezultatul în funcție de simbol.
fn compute(input: &str) -> Result<f64, &'static str> {
    
    let mut tokenizer = Tokenizer::new();
    for char in input.chars() {
        match char {
            '+' => tokenizer.add_token(Token::ADD),
            '-' => tokenizer.add_token(Token::SUB),
            '*' => tokenizer.add_token(Token::MUL),
            '/' => tokenizer.add_token(Token::DIV),
            '(' => tokenizer.add_token(Token::LPAR),
            ')' => tokenizer.add_token(Token::RPAR),
            num => tokenizer.add(num),
        }
    }
    tokenizer.finish();
    let mut parser = Parser::new(tokenizer.get());
    expr(&mut parser)

    // TASK: Inițializează simbolurile de operare (+, -, *, /)
    // HINT: Creează o listă `let symbols = ["+", "-", "*", "/"];`

     // <-- Returnează None dacă nu găsește niciun simbol valid
}
