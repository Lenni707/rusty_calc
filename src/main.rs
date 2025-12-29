// i will be building and reading out a simple parse tree for a calc
// the enum grammarItem should represent/represents the type of Item, the inpit can be
// the struct ParseNode represents one enitity of the parse tree (all enitites have the type parsenode). A Entity has a value 
// and possible children, which are stored in a vec and also have possible children and so on
// 
// TODO ->
// - with my extremely tuff whiteboard, visualize how a parse tree works
// code the actual logic so:
// a lexer, which turns the input componets (+, - etc) into a GrammarItem
// a parser which takes the output of the lexer and turns it into a parse tree
// the actual code that reads out the parse tree and outputs the result

enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Parenthese
}
struct ParseNode {
    children: Vec<ParseNode>,
    value: GrammarItem
}

impl ParseNode {
    fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            value: GrammarItem::Parenthese
        }
    }
}



fn main() {
    println!("Hello, world!");
}
