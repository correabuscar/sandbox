// someone (mib_*) on irc had this example, now modified a bit
// fixed by needing a .clone()

struct Parser {
    lines: Vec<String>,
    current_index: usize,
}

impl Parser {
    pub fn get_cur_index(&self) -> usize {
        self.current_index
    }
    pub fn can_step(&self) -> bool {
        self.current_index < self.lines.len()
    }

    pub fn step(&mut self) {
        self.current_index += 1;
    }

    pub fn get_line<'b>(&'b self, which: usize) -> &'b String {
        &(self.lines[which])
    }
    pub fn get_cur_line<'b>(&'b self) -> &'b String {
        &(self.lines[self.get_cur_index()])
    }
}

fn main() {
    let mut parser: Parser = setup_parser();

    while parser.can_step() {
        //let line1: &String = &parser.get_line(parser.get_cur_index());
        let line1: &String = &parser.get_line(parser.get_cur_index()).clone(); //this needs clone(), thanks durka42
        //FIXME: find out if unsafe{} can somehow be used to bypass the need for clone
        //at first glance, apparently not! because the lifetime of a line that you get from within
        //'parser' must be tied to 'parser', so when parser goes away(out of scope/Drop-ed) then
        //that line does too (unless, clone()-d ofc)
        //but since you have to (immutable)borrow the whole parser struct, you cannot then also borrow it as
        //mutable, for the parser.step() to work!
        //now, if partial immutable borrow of parser could happen for 'lines' field, and then partial mutable borrow of parser for 'current_index' could happen, then this clone() could be avoided! but then how would compiler know if maybe those two fields are tied together by code and must be somehow kept in sync - which isn't the case currently - as long as 'lines' cannot be modified!! if it could, then current_index would depend on lines.len(), altho current_index can be currently (because we're in the same module) accessed directly anyway!
        parser.step();
        if parser.can_step() {
            let line2: &String = parser.get_cur_line();
            println!("line1 == line2: {}", line1 == line2);
        } else {
            println!("Last line reached");
        }
    }
}

fn setup_parser() -> Parser {
    // pretend I'm reading lines from a file into a Vec<String>
    let mut lines: Vec<String> = Vec::new();
    lines.push(String::from("foo"));
    lines.push(String::from("foo"));
    lines.push(String::from("bar"));

    Parser {
        lines,
        current_index: 0,
    }
}

