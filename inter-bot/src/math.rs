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
            Token::Number(_) => output_queue.push_back(element),
            Token::Operator(temp_element) => {
                while let Some(Token::Operator(top_element)) = operator_stack.back() {
                    if get_precedence(*top_element) > get_precedence(temp_element) ||
                       (get_precedence(*top_element) == get_precedence(temp_element) && !get_associativity(temp_element)) {
                        output_queue.push_back(operator_stack.pop_back().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push_back(element);
            },
            Token::LeftParen => operator_stack.push_back(element),
            Token::RightParen => {
                while let Some(top) = operator_stack.pop_back() {
                    if let Token::LeftParen = top {
                        break;
                    } else {
                        output_queue.push_back(top);
                    }
                }
            }
        }
    }
    
    while let Some(element) = operator_stack.pop_back() {
        output_queue.push_back(element);
    }

    let mut eval_stack: VecDeque<f64> = VecDeque::new();
    for element in output_queue {
        match element {
            Token::Number(n) => eval_stack.push_back(n),
            Token::Operator(element) => {
                let right = eval_stack.pop_back().expect("Wrong syntax");
                let left = eval_stack.pop_back().expect("Wrong syntax");
                let result = match element {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    '^' => left.powf(right),
                    _ => 0.0,
                };
                eval_stack.push_back(result);
            },
            _ => (),
        }
    }
    // eval_stack.pop_back().expect("Invalid expression");

    //there is probably an issue with this line of code
    // return String::from(format!("length of eval stack: {}", eval_stack.len()));
    let final_result = String::from(eval_stack.pop_back().expect("Invalid expression").to_string());
    final_result
    // final_result
}
fn get_precedence(element: char) -> u8 {
    let var_name = match element {
        '^' => 4,
        '*' | '/' => 3,
        '+' | '-' => 2,
        _ => 0,
    };
    var_name
}

fn get_associativity(element: char) -> bool {
    let var_name = match element {
        '^' => false,
        '*' | '/' => true,
        '+' | '-' => true,
        _ => false,
    };
    var_name
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
            '+' | '-' | '*' | '/' | '^' => {
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