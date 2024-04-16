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

    //shunting yard algorithm: converts to Reverse Polish Notation (RPN) and then evaluates it

    //clean out the whitespace
    let mut post: String = msg.content.chars().skip(10).collect();
    
    //need spaces because otherwise sepax
    post = post.trim().to_string();
    let res = parse_expression(&post);

    //standard way of setting up the problem
    let mut output_queue: VecDeque<Token> = VecDeque::new();

    //operators not added to the output queue
    let mut operator_stack: VecDeque<Token> = VecDeque::new();
    let mut iter = 0;
    for element in res {
        match element {

            //add numbers to the back
            Token::Number(_) => output_queue.push_back(element),
            Token::Operator(temp_element) => { //this operator is O2
                while let Some(Token::Operator(top_element)) = operator_stack.back() {

                    //check precedence and associativity following the rule for O2 and O1
                    //there is an operator o2 at the top of the operator stack which is not a left parenthesis, 
                    //and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))

                    if get_precedence(*top_element) > get_precedence(temp_element) ||
                       (get_precedence(*top_element) == get_precedence(temp_element) && !get_associativity(temp_element)) {

                        //pop o2 from the operator stack into the output queue
                        output_queue.push_back(operator_stack.pop_back().unwrap());
                    } else {
                        break;
                        //end condition reached
                    }
                }

                //push O1 onto the operator stack
                operator_stack.push_back(element);
            },
            Token::LeftParen => operator_stack.push_back(element),
            Token::RightParen => {

                //goal: find the associative left parenthesis
                let mut found_paren = false;
                while let Some(top) = operator_stack.pop_back() {
                    if let Token::LeftParen = top {
                        found_paren = true;
                        break;
                    } else {
                        //pop the operator from the operator stack into the output queue
                        output_queue.push_back(top);
                    }
                }
                if !found_paren {
                    return "Mismatched parenthesis".to_string();
                }
            }
        }
    }
    
    //make sure that the operator on the top of the stack is not a left parenthesis
    while let Some(element) = operator_stack.pop_back() {
        match element {
            Token::LeftParen => return "Mismatched parenthesis".to_string(),
            _ => output_queue.push_back(element),
        }
    }

    let mut eval_stack: VecDeque<f64> = VecDeque::new();
    for element in output_queue {
        match element {
            Token::Number(n) => eval_stack.push_back(n),
            Token::Operator(element) => {

                //this means there are too many operators
                if eval_stack.len() < 2 {
                    return "Invalid expression, not enough operands".to_string();
                }
                let right = eval_stack.pop_back().unwrap();
                let left = eval_stack.pop_back().unwrap();
                let result = match element {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    '^' => left.powf(right),
                    _ => return "Invalid Expression".to_string(),
                };
                eval_stack.push_back(result);
            },
            _ => (),
        }
    }
    let final_result: String = match eval_stack.pop_back() {
        Some(value) => value.to_string(),
        None => "Invalid expression, no result computed".to_string(),
    };
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