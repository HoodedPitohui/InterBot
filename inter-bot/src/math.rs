use serenity::model::channel::Message;
use std::collections::VecDeque;
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

    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: VecDeque<Token> = VecDeque::new();

    let mut iter = 0;

    for element in res {
        match element {
            Token::Number(_) => output_queue.push_front(element),
            Token::Operator(temp_element) => {
                let mut iter2 = 0;
                while let Some(Token::Operator(top_element)) = operator_stack.back() {
                    if (get_precedence(*top_element) > get_precedence(temp_element) || 
                        (get_precedence(*top_element) == get_precedence(temp_element) && get_associativity(temp_element))) {
                            output_queue.push_front(operator_stack.pop_back().unwrap());
                        }
                    else {
                        break;
                    }   
                }
                operator_stack.push_back(element);
            }
            Token::LeftParen => operator_stack.push_back(element),
            Token::RightParen => operator_stack
        }
    }
    post
}
fn get_precedence(element: char) -> u8 {
    match element {
        '^' => 4,
        '*' | '/' => 3,
        '+' | '-' => 3,
    }
}

fn get_associativity(element: char) -> bool {
    match element {
        '^' => false,
        '*' | '/' => true,
        '+' | '-' => true,
    }
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

    //iterate through all of the characters
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
            _ => (), //just do nothing
        }
    }

    flush_num_buffer(&mut num_buffer, &mut tokens);
    tokens
}