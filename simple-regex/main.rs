
#[derive(Debug)]
enum Commands{
  MatchOne(char),
  MatchMany(char),
  MatchAnyOne,
  MatchAnyMany
}


//struct TestCase {
//  matched : bool,
//  index : usiz
//}

struct Context {
  commands_vec : Vec<Commands>,
  current_command_index: usize,
  current_string_index : usize
}
impl Context {
  fn new(pattern : String) -> Context {
    let mut context = Context {
      commands_vec : Vec::new(),
      current_command_index : 0,
      current_string_index : 0
    };
    let mut split_input : Vec<char> = pattern.chars().collect();
    split_input.remove(split_input.len()-1);
    let mut i = 0;
    while i < split_input.len() {
      println!("index: {}, char: {}", i, split_input[i]);
      if i != split_input.len()-1 && split_input[i+1] == '*' {
        if split_input[i] == '.' {
          context.commands_vec.push(Commands::MatchAnyMany);
          i += 2;
          continue;
        }
        context.commands_vec.push(Commands::MatchMany(split_input[i]));
        i += 2;
      } else {
        if split_input[i] == '.' {
          context.commands_vec.push(Commands::MatchAnyOne);
          i += 1;
          continue;
        }
        context.commands_vec.push(Commands::MatchOne(split_input[i]));
        i += 1;
      }
    }
    println!("Commands: {:?}", context.commands_vec);
    return context;
  }

  fn static_apply_command(input : &Vec<char>, current_string_index : &mut usize, current_command : &Commands, next_command : Option<&Commands>) -> bool {
    match current_command {
      Commands::MatchOne(c) => {
        println!("index: {:?}", current_string_index);
        if input[*current_string_index] == *c {
          *current_string_index += 1;
          println!("true");
          return true;
        } else {
           println!("false");
          return false;
        }
      },
      Commands::MatchMany(c) => { // issue is this matches repititions of c, so if there is MatchOne(c) then it matches that sets the index to greater than it and so it doesn't match.
        println!("Matching {:?}", *c);
        let mut i = *current_string_index;
        while input[i] == *c {
          if i == input.len()-1 {
            break;
          }
          i+=1;
        }
        println!("i: {:?}, index: {:?}", i, *current_string_index);
        if i > *current_string_index {
          *current_string_index = i;
          return true;
        } else {
          return false;
        }
      },
      Commands::MatchAnyOne => {
        if *current_string_index == input.len()-1 {
          return false;
        }
        *current_string_index += 1;
        return true;
      },
      Commands::MatchAnyMany => { // fix this instead of messing with above, this is the real issue.
        if next_command.is_none() {
          *current_string_index = input.len();
          return true;
        }
        let mut current_string_index_copy = *current_string_index;
        //let mut next_command_out = Context::static_apply_command(&input, &mut current_string_index_copy, current_command, None);
        loop {
          let next_command_out = Context::static_apply_command(&input, &mut current_string_index_copy, next_command.unwrap(), None);
          if next_command_out || *current_string_index == input.len() {break;}
          *current_string_index += 1;
          current_string_index_copy += 1;
          println!("out: {:?}", next_command_out);
        }
        println!("MatchAnyMany Index: {:?}", *current_string_index);
        return true;
      }
    } 
  }

  fn apply_command(&mut self, input : &Vec<char>) -> bool {
    
    return Context::static_apply_command(input, &mut self.current_string_index, &self.commands_vec[self.current_command_index], if self.current_command_index == self.commands_vec.len()-1 {None} else {Some(&&self.commands_vec[self.current_command_index+1])})
  }

  fn next(&mut self) -> Option<&Commands> { // increments counter and returns next, called by while loop
    if self.has_next() {
      self.current_command_index += 1;
      return Some(&self.commands_vec[self.current_command_index]); // fuck this
    } else {
      return None;
    }
  }

  fn has_next(&self) -> bool {
    if self.current_command_index == self.commands_vec.len()-1 {
      return false;
    }
    return true;
  }
}

fn parse(input : String, pattern : String) -> bool {
  let mut input_chars : Vec<char> = input.chars().collect();
  input_chars.remove(input_chars.len()-1);
  println!("{:?}", input_chars);
  let mut context = Context::new(pattern);
  let mut case = context.apply_command(&input_chars);
  while !context.next().is_none() {
    println!("matched : {:?}", case);
    if !case {
      return false;
    }
    case = context.apply_command(&input_chars);
  }
  if !case {
    return false;
  }
  println!("String Index: {:?}, input length: {:?}",context.current_string_index, input_chars.len());
  return if context.current_string_index == input_chars.len() {true} else {false};
}

fn main() {
  let mut pattern = String::new();
  let mut input = String::new();
  println!("Pattern: ");
  std::io::stdin().read_line(&mut pattern).unwrap_or_else(|_| panic!("Error reading pattern"));
  println!("Input: ");
  std::io::stdin().read_line(&mut input).unwrap_or_else(|_| panic!("Error reading input"));
  assert!(parse(input, pattern));
}