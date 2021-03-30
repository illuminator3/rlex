mod rlex {
    use regex::{Regex, escape};

    #[derive(Debug)]
    pub struct Line {
        content: String,
        line: usize,
        file: String
    }

    #[derive(Debug)]
    pub struct LexedToken {
        content: String,
        line: usize,
        index: usize,
        line_content: String,
        token_type: Token
    }

    #[derive(Debug)]
    pub struct LexerData {
        tokens: Vec<Token>
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Token {
        id: &'static str,
        regex: &'static str,
        is_regex: bool
    }

    impl Line {
        pub fn content(&self) -> &String {
            &self.content
        }

        pub fn line(&self) -> &usize {
            &self.line
        }

        pub fn file(&self) -> &String {
            &self.file
        }
    }

    impl LexedToken {
        pub fn check_type(self, expected: Token, message: String) -> LexedToken {
            if self.token_type != expected {
                panic!("{} at ({}:{}):\n{}\n{}", message, self.line, self.index, self.line_content, " ".repeat((self.index + 1) as usize) + "^");
            }

            self
        }

        pub fn content(&self) -> &String {
            &self.content
        }

        pub fn line(&self) -> &usize {
            &self.line
        }

        pub fn index(&self) -> &usize {
            &self.index
        }

        pub fn line_content(&self) -> &String {
            &self.line_content
        }

        pub fn token_type(&self) -> &Token {
            &self.token_type
        }
    }

    impl LexerData {
        pub fn tokens(&self) -> &Vec<Token> {
            &self.tokens
        }
    }

    impl Token {
        pub fn id(&self) -> &'static str {
            &self.id
        }

        pub fn regex(&self) -> &'static str {
            &self.regex
        }

        pub fn is_regex(&self) -> &bool {
            &self.is_regex
        }
    }

    pub fn read_lines(comment: String, content: String, file: String) -> Vec<Line> {
        content.lines().enumerate().map(|(i, s)| {
            Line {
                content: s.split(&comment).next().unwrap().to_owned(),
                line: i,
                file: file.clone()
            }
        }).collect()
    }

    pub fn data(tokens: Vec<Token>) -> LexerData {
        LexerData {
            tokens
        }
    }

    pub fn token(id: &'static str, regex: &'static str, is_regex: bool) -> Token {
        Token {
            id,
            regex,
            is_regex
        }
    }

    pub fn full_lex(content: String, file: String, comment: String, data: LexerData) -> Vec<LexedToken> {
        lex(read_lines(comment, content, file), data)
    }

    pub fn lex(lines: Vec<Line>, data: LexerData) -> Vec<LexedToken> {
        let mut tokens = Vec::new();

        lines.iter().enumerate().for_each(|(i, l)| {
            let mut index = 0;

            while !l.content[index..].is_empty() {
                let mut found_token = false;

                data.tokens.iter().for_each(|p| {
                    let content = &l.content[index..];
                    let regex = Regex::new(&format!("^{}", if p.is_regex {
                        p.regex.to_owned()
                    } else {
                        escape(p.regex)
                    })).unwrap(); // escape regex if p.is_regex == false
                    let option = regex.find(content);

                    if option.is_none() {
                        return;
                    }

                    let found = option.unwrap();

                    tokens.push(LexedToken {
                        content: found.as_str().to_owned(),
                        line: i,
                        index,
                        line_content: l.content.clone(),
                        token_type: *p
                    });
                    index += found.as_str().len();
                    found_token = true;
                });

                if !found_token {
                    panic!("Unrecognized token at ({}:{}):\n{}\n", l.line, index, l.content); // TODO change this to Result stuff
                }
            }

            tokens.push(LexedToken {
                content: "\n".to_owned(),
                line: l.line,
                index,
                line_content: "?".to_owned(),
                token_type: token(
                    "NEW LINE",
                    "\n",
                    false
                )
            });
        });

        tokens
    }
}