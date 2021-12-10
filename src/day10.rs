pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    println!("Day 10 part 1: {}", Chunk::get_corrupted_high_score(&input));
    println!(
        "Day 10 part 2: {}",
        Chunk::get_incomplete_high_score(&input)
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParenType {
    Round,
    Square,
    Curly,
    Pointy,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Opening,
    Closing,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Token {
    paren_type: ParenType,
    direction: Direction,
}

impl Token {
    fn parse_tokens(input: &str) -> Vec<Token> {
        input.chars().map(|c| c.into()).collect()
    }
}

impl From<char> for Token {
    fn from(s: char) -> Self {
        use Direction::*;
        use ParenType::*;
        match s {
            '(' => Token {
                direction: Opening,
                paren_type: Round,
            },
            ')' => Token {
                direction: Closing,
                paren_type: Round,
            },
            '[' => Token {
                direction: Opening,
                paren_type: Square,
            },
            ']' => Token {
                direction: Closing,
                paren_type: Square,
            },
            '{' => Token {
                direction: Opening,
                paren_type: Curly,
            },
            '}' => Token {
                direction: Closing,
                paren_type: Curly,
            },
            '<' => Token {
                direction: Opening,
                paren_type: Pointy,
            },
            '>' => Token {
                direction: Closing,
                paren_type: Pointy,
            },
            e => panic!("Illegal character: '{}'", e),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ChunkError {
    Corrupted(Token),
    Incomplete(Vec<Token>),
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    paren_type: ParenType,
    children: Vec<Chunk>,
    start: usize,
    end: usize,
}

impl Chunk {
    fn parse(input: &[Token], position: &mut usize) -> Result<Chunk, ChunkError> {
        let opening_token = input[*position];
        if opening_token.direction != Direction::Opening {
            return Err(ChunkError::Corrupted(opening_token));
        }

        let expected_closing_token = Token {
            paren_type: opening_token.paren_type,
            direction: Direction::Closing,
        };
        let start: usize = *position;
        let mut children: Vec<Chunk> = Vec::new();

        *position += 1;
        if *position >= input.len() {
            return Err(ChunkError::Incomplete(vec![expected_closing_token]));
        }
        while input[*position] != expected_closing_token {
            match Chunk::parse(input, position) {
                Ok(child) => children.push(child),
                Err(e) => {
                    return match e {
                        ChunkError::Corrupted(_) => Err(e),
                        ChunkError::Incomplete(mut incomplete) => {
                            incomplete.push(expected_closing_token);
                            Err(ChunkError::Incomplete(incomplete))
                        }
                    };
                }
            }
            *position += 1;
            if *position >= input.len() {
                return Err(ChunkError::Incomplete(vec![expected_closing_token]));
            }
        }
        Ok(Chunk {
            paren_type: opening_token.paren_type,
            children,
            start,
            end: *position,
        })
    }

    fn rate_error(error: ChunkError) -> usize {
        match error {
            ChunkError::Corrupted(corrupted_token) => match corrupted_token.paren_type {
                ParenType::Round => 3,
                ParenType::Square => 57,
                ParenType::Curly => 1_197,
                ParenType::Pointy => 25_137,
            },
            ChunkError::Incomplete(missing_tokens) => missing_tokens
                .iter()
                .map(|token| match token.paren_type {
                    ParenType::Round => 1,
                    ParenType::Square => 2,
                    ParenType::Curly => 3,
                    ParenType::Pointy => 4,
                })
                .fold(0, |acc, v| acc * 5 + v),
        }
    }

    fn get_corrupted_high_score(input_lines: &str) -> usize {
        input_lines
            .trim()
            .lines()
            .map(|line| Token::parse_tokens(line.trim()))
            .map(|tokens| Chunk::parse(&tokens, &mut 0))
            .filter_map(|chunk| match chunk {
                Ok(_) => None,
                Err(e) => match e {
                    ChunkError::Corrupted(_) => Some(Chunk::rate_error(e)),
                    ChunkError::Incomplete(_) => None,
                },
            })
            .sum()
    }

    fn get_incomplete_high_score(input_lines: &str) -> usize {
        let mut scores: Vec<usize> = input_lines
            .trim()
            .lines()
            .map(|line| Token::parse_tokens(line.trim()))
            .map(|tokens| Chunk::parse(&tokens, &mut 0))
            .filter_map(|chunk| match chunk {
                Ok(_) => None,
                Err(e) => match e {
                    ChunkError::Corrupted(_) => None,
                    ChunkError::Incomplete(_) => Some(Chunk::rate_error(e)),
                },
            })
            .collect();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_tokens() {
        let input = "()";
        let expected: Vec<Token> = vec![
            Token {
                direction: Direction::Opening,
                paren_type: ParenType::Round,
            },
            Token {
                direction: Direction::Closing,
                paren_type: ParenType::Round,
            },
        ];
        assert_eq!(expected, Token::parse_tokens(input));
    }

    #[test]
    fn it_parses_simple_chunks() {
        let input = Token::parse_tokens("()");
        let chunk = Chunk::parse(&input, &mut 0);
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();
        let mut expected = Chunk {
            paren_type: ParenType::Round,
            children: vec![],
            start: 0,
            end: 1,
        };
        assert_eq!(expected, chunk);

        let input = Token::parse_tokens("({})");
        let chunk = Chunk::parse(&input, &mut 0).unwrap();
        expected.children.push(Chunk {
            paren_type: ParenType::Curly,
            children: vec![],
            start: 1,
            end: 2,
        });
        expected.end = 3;
        assert_eq!(expected, chunk);

        let input = Token::parse_tokens("({}<>)");
        let chunk = Chunk::parse(&input, &mut 0).unwrap();
        assert_eq!(2, chunk.children.len());
    }

    #[test]
    fn it_finds_corrupted_chunks() {
        let input = Token::parse_tokens("{([(<{}[<>[]}>{[]{[(<()>");
        let chunk = Chunk::parse(&input, &mut 0);
        assert!(chunk.is_err());
        let err = chunk.unwrap_err();
        assert_eq!(
            ChunkError::Corrupted(Token {
                paren_type: ParenType::Curly,
                direction: Direction::Closing
            }),
            err
        );
    }

    #[test]
    fn it_finds_incomplete_chunks() {
        let input = Token::parse_tokens("[({(<(())[]>[[{[]{<()<>>");
        let chunk = Chunk::parse(&input, &mut 0);
        assert!(chunk.is_err());
        let err = chunk.unwrap_err();
        assert_eq!(
            std::mem::discriminant(&ChunkError::Incomplete(vec![])),
            std::mem::discriminant(&err)
        );
    }

    #[test]
    fn it_rates_example1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(26_397, Chunk::get_corrupted_high_score(input));
    }

    #[test]
    fn it_finds_missing_token() {
        let input = Token::parse_tokens("[");
        let chunk = Chunk::parse(&input, &mut 0);
        assert!(chunk.is_err());
        let err = chunk.unwrap_err();
        match err {
            ChunkError::Corrupted(_) => assert!(false),
            ChunkError::Incomplete(missing) => assert_eq!(
                vec![Token {
                    paren_type: ParenType::Square,
                    direction: Direction::Closing
                }],
                missing
            ),
        };
    }

    #[test]
    fn it_finds_missing_tokens_simple() {
        let input = Token::parse_tokens("[(");
        let chunk = Chunk::parse(&input, &mut 0);
        assert!(chunk.is_err());
        let err = chunk.unwrap_err();
        let expected = Token::parse_tokens(")]");
        match err {
            ChunkError::Corrupted(_) => assert!(false, "it's corrupted instead of incomplete"),
            ChunkError::Incomplete(missing) => assert_eq!(expected, missing),
        };
    }

    #[test]
    fn it_rates_missing_tokens() {
        assert_eq!(
            288957,
            Chunk::rate_error(ChunkError::Incomplete(Token::parse_tokens("}}]])})]")))
        );
        assert_eq!(
            5566,
            Chunk::rate_error(ChunkError::Incomplete(Token::parse_tokens(")}>]})")))
        );
        assert_eq!(
            1480781,
            Chunk::rate_error(ChunkError::Incomplete(Token::parse_tokens("}}>}>))))")))
        );
        assert_eq!(
            995444,
            Chunk::rate_error(ChunkError::Incomplete(Token::parse_tokens("]]}}]}]}>")))
        );
        assert_eq!(
            294,
            Chunk::rate_error(ChunkError::Incomplete(Token::parse_tokens("])}>")))
        );
    }

    #[test]
    fn it_finds_missing_tokens() {
        let input = Token::parse_tokens("[({(<(())[]>[[{[]{<()<>>");
        let chunk = Chunk::parse(&input, &mut 0);
        let err = chunk.unwrap_err();
        match err {
            ChunkError::Corrupted(_) => assert!(false, "it's corrupted instead of incomplete"),
            ChunkError::Incomplete(inc) => assert_eq!(Token::parse_tokens("}}]])})]"), inc),
        };
    }

    #[test]
    fn it_rates_example2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(288957, Chunk::get_incomplete_high_score(input));
    }
}
