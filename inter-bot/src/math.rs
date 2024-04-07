use serenity::model::channel::Message;

enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

//order of operations calculator
pub fn pemdas(msg: &Message) -> String {

    //clean out the whitespace
    let mut post: String = msg.content.chars().skip(10).collect();
    
    //need spaces because otherwise sepax
    post = post.trim().to_string();
    let res = parse_expression(&post);
    post
}

fn parse_expression(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buffer = String::new();

    //define a closure:
    let flush_num_buffer = |num_buffer: &mut String, tokens: &mut Vec<Token>| {
        if !num_buffer.is_empty() {
            if let Ok(num) = num_buffer.parse::<f64>() {
                tokens.push(Token::Number(num));
            }
            num_buffer.clear();
        }
    };
    for c in expr.chars() {
        match c {
            '0'..='9' | '.' => num_buffer.push(c),
            '+' | '-' | '*' | '/' => {
                flush_num_buffer(&mut num_buffer, &mut tokens);
                tokens.push(Token::Operator(c));
            },
            '(' => {
                flush_num_buffer(&mut num_buffer, &mut tokens);
                tokens.push(Token::LeftParen);
            },
            ')' => {
                flush_num_buffer(&mut num_buffer, &mut tokens);
                tokens.push(Token::RightParen);
            },
            ' ' => flush_num_buffer(&mut num_buffer, &mut tokens),
            _ => (),
        }
    }

    flush_num_buffer(&mut num_buffer, &mut tokens);
    tokens
}