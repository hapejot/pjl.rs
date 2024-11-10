use std::rc::Rc;

use santiago::lexer::{Lexeme, LexerRules};
use tracing::{error, info};

pub fn lexer() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "ASSIGN" = string "_";
        "DEFAULT" | "INT" = pattern r"[0-9]+";
        "DEFAULT" | "NUMBER" = pattern r"[0-9]+[.][0-9]+";
        "DEFAULT" | "IDENTIFIER" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*";
        "DEFAULT" | "SYMBOL" = pattern r"#[!%&*+,/<=>?@\\~|a-zA-Z_-][a-zA-Z_0-9%&*+,/<=>?@\\~|:-]*";
        "DEFAULT" | "KEYWORD" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*:";
        "DEFAULT" | "STRING_START" = string "'" => |l| {l.push_state("STRING"); l.skip()};
         // "DEFAULT" | "LOCAL" = pattern r":[a-zA-Z_][a-zA-Z_0-9]*";
        "DEFAULT" | "COMMENT" = pattern "\"[^\"]*\"" => |l| {l.skip()};
        "DEFAULT" | "SHELL_COMMENT" = pattern r"#!.*" => |l| l.skip();
        "DEFAULT" | ":" = string ":";
        "DEFAULT" | ";" = string ";";
        "DEFAULT" | "." = string ".";
        "DEFAULT" | "[" = string "[";
        "DEFAULT" | "]" = string "]";
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "{" = string "{";
        "DEFAULT" | "}" = string "}";
        "DEFAULT" | "ASSIGN" = string ":=";
        "DEFAULT" | "ASSIGN" = string "<-";
        "DEFAULT" | "BINARY" = pattern r"[!%&*+,-/<=>?@\\~|]+";
        "DEFAULT" | "CHAR" = pattern r"\$.";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
        "DEFAULT" | "RETURN" = string "^";
        "DEFAULT" | "#" = string "#";
        "STRING"  | "QUOTE" = string "''";
        "STRING"  | "STRING_PART" = pattern r"[^']*";
        "STRING"  | "END_OF_STRING" = string "'" => |l| {l.pop_state(); l.take()};
    )
}

pub mod scanner {

    pub mod chunks {
        use crate::parser::lexer;
        use santiago::lexer::{LexerRules, Position};
        use std::{
            fs::File,
            io::{BufRead, BufReader, Read},
        };

        use super::{Chunk, Scanner};

        pub struct ChunkedLexer<R>
        where
            R: Sized,
        {
            in_file: BufReader<R>,
            line: Option<Vec<char>>,
            lexing_rules: LexerRules,
            start: Position,
            index: usize,
        }

        impl<R> ChunkedLexer<R>
        where
            R: Read,
        {
            pub fn new(in_file: R) -> Self {
                let lexing_rules = lexer();
                let in_file = BufReader::new(in_file);
                Self {
                    in_file,
                    line: None,
                    lexing_rules,
                    start: Position { line: 1, column: 1 },
                    index: 0,
                }
            }

            fn get_next_char(&mut self) -> Option<char> {
                if let Some(r) = &self.line {
                    let c = r.get(self.index).unwrap().clone();
                    self.index += 1;
                    Some(c)
                } else {
                    None
                }
            }

            fn fill_line(&mut self) {
                if self.line == None || self.index >= self.line.as_ref().unwrap().len() {
                    let mut buf = String::new();
                    match self.in_file.read_line(&mut buf) {
                        Ok(l) => {
                            if l == 0 {
                                self.line = None;
                            } else {
                                if self.line == None && buf.len() > 2 && &buf[0..2] == "#!" {
                                    self.fill_line();
                                } else {
                                    self.index = 0;
                                    self.line = Some(buf.chars().collect());
                                }
                            }
                        }
                        Err(e) => todo!("handle {e}"),
                    }
                }
            }

            fn unget(&mut self) {
                self.index -= 1;
            }
        }

        impl<R> Scanner for ChunkedLexer<R>
        where
            R: Read,
        {
            fn next_chunk(&mut self) -> Option<Chunk> {
                let mut current = self.start.clone();
                let mut text = String::new();
                loop {
                    self.fill_line();
                    let c = self.get_next_char();
                    if c == Some('!') {
                        let c2 = self.get_next_char();
                        if c2 == Some('!') {
                            text.push('!');
                        } else {
                            self.unget();
                            break;
                        }
                    } else {
                        if let Some(c) = c {
                            text.push(c);
                        } else {
                            if text.len() == 0 {
                                return None;
                            }
                            break;
                        }
                    }
                }

                Some(Chunk {
                    start: self.start.clone(),
                    end: current.clone(),
                    text: text.trim().to_string(),
                })
            }

            fn all_tokens(&mut self) -> Vec<santiago::lexer::Lexeme> {
                todo!()
            }
        }

        #[test]
        fn t1() {
            let src = r#"#!.....
            Ein Test!
            !noch einer!
            ! !
            und hier kommt ein echtes Zeichen!! !
            "#;
            let s = src.bytes().collect::<Vec<_>>();
            let mut cut = ChunkedLexer::new(s.as_slice());
            assert_eq!(cut.next_chunk().unwrap().text, "Ein Test");
            assert_eq!(cut.next_chunk().unwrap().text, "");
            assert_eq!(cut.next_chunk().unwrap().text, "noch einer");
            let c = cut.next_chunk().unwrap();
            assert_eq!(c.text, "");
            assert!(c.is_empty());
            assert_eq!(cut.next_chunk().unwrap().text, "");
            assert_eq!(
                cut.next_chunk().unwrap().text,
                "und hier kommt ein echtes Zeichen!"
            );
            assert_eq!(cut.next_chunk().unwrap().text, "");
            assert!(cut.next_chunk().is_none());
        }
    }

    use santiago::lexer::{Lexeme, Position};

    pub struct Chunk {
        start: Position,
        end: Position,
        text: String,
    }
    impl Chunk {
        pub fn text(&self) -> &str {
            &self.text
        }

        pub fn is_empty(&self) -> bool {
            // let s = self.text.trim();
            let s = &self.text;
            s.is_empty()
        }
    }
    pub trait Scanner {
        fn next_chunk(&mut self) -> Option<Chunk>;
        fn all_tokens(&mut self) -> Vec<Lexeme>;
    }
}

// use santiago::grammar::Associativity;

pub struct Grammar {
    lex: Vec<Rc<Lexeme>>,
}

impl Grammar {
    pub fn new(lex: Vec<Rc<Lexeme>>) -> Self {
        Self { lex }
    }

    pub fn parse(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn parse_statements(&self) -> Result<(), String> {
        Ok(())
    }
}

// Übersetzung aus GNU-Smalltalk
pub mod rbparser {
    use santiago::lexer::{Lexeme, Position};
    use std::{collections::HashMap, rc::Rc, sync::mpsc::Receiver};
    use trace::trace;

    #[derive(Debug)]
    pub enum Node {
        Identifier(String),
        Statements(Vec<Node>),
        Expression {
            receiver: Box<Node>,
            messages: Vec<Node>,
        },
        Message {
            selector: String,
            params: Vec<Node>,
            outer: Option<Box<Node>>,
        },
        None,
        Assign {
            target: String,
            value: Box<Node>,
        },
        Symbol(String),
        String(String),
        Literal(String),
        Method {
            selector: String,
            args: Vec<String>,
            body: Box<Node>,
        },
        Block {
            args: Vec<Node>,
            body: Box<Node>,
        },
        ArrayLiteral,
    }

    impl Node {
        pub fn set_outer(&mut self, n: Option<Node>) {
            match n {
                Some(n) => match self {
                    Node::Message {
                        selector,
                        params,
                        outer,
                    } => match outer {
                        Some(outer) => outer.set_outer(Some(n)),
                        None => *outer = Some(Box::new(n)),
                    },
                    _ => panic!(),
                },
                None => {}
            }
        }

        fn as_str(&self) -> &str {
            match self {
                Node::Identifier(s) => s.as_str(),
                Node::Symbol(s) => s.as_str(),
                Node::String(s) => s.as_str(),
                Node::Literal(s) => s.as_str(),
                _ => todo!("as_str from {:?}", self),
            }
        }
    }

    #[derive(Debug)]
    pub struct STClass {
        comment: String,
        protocols: Vec<STProtocol>,
        parent: String,
    }
    impl STClass {
        fn add_method(&mut self, protocol: &str, is_class: bool, m: Node) {
            let p = self.protocols.iter_mut().find(|x| x.name == protocol);
            if p.is_none() {
                self.protocols.push(STProtocol {
                    is_class,
                    name: protocol.to_string(),
                    methods: vec![],
                });
            }
            let p = self
                .protocols
                .iter_mut()
                .find(|x| x.name == protocol)
                .unwrap();
            p.methods.push(m);
        }

        pub fn set_comment(&mut self, comment: &str) {
            self.comment = String::from(comment);
        }
    }
    #[derive(Debug)]
    pub struct STProtocol {
        is_class: bool,
        name: String,
        methods: Vec<Node>,
    }
    // Object subclass: RBParser [
    // | scanner currentToken nextToken errorBlock tags source methodCategory |
    #[derive(Debug)]
    pub struct Parser {
        scanner: Vec<Rc<Lexeme>>,
        current_token: Option<Rc<Lexeme>>,
        next_token: Option<Rc<Lexeme>>,
        error_block: String,
        tags: Vec<String>,
        source: String,
        method_category: String,
        classes: HashMap<String, STClass>,
    }

    trait Lex {
        fn is_binary(&self) -> bool;
        fn is_pipe(&self) -> bool;
        fn is_return(&self) -> bool;
        fn is_assignment(&self) -> bool;
        fn is_identifier(&self) -> bool;
        fn is_literal(&self) -> bool;
        fn is_keyword(&self) -> bool;
        fn is_symbol(&self) -> bool;
        fn is_char(&self) -> bool;
        fn is_eof(&self) -> bool;
        fn is_kind(&self, s: &str) -> bool;
        fn is_string(&self) -> bool;
        fn is(&self, s: &str) -> bool;
    }
    // #[trace]
    impl Lex for Rc<Lexeme> {
        fn is_binary(&self) -> bool {
            self.kind == "BINARY"
        }

        fn is_pipe(&self) -> bool {
            self.raw == "|"
        }

        fn is_return(&self) -> bool {
            self.raw == "^"
        }

        fn is_assignment(&self) -> bool {
            self.kind == "ASSIGN"
        }

        fn is_identifier(&self) -> bool {
            self.kind == "IDENTIFIER"
        }

        fn is_literal(&self) -> bool {
            self.is_string() || self.kind == "NUMBER" || self.kind == "INT" || self.kind == "CHAR"
        }

        fn is(&self, s: &str) -> bool {
            self.raw == s
        }

        fn is_keyword(&self) -> bool {
            self.kind == "KEYWORD"
        }

        fn is_symbol(&self) -> bool {
            self.kind == "SYMBOL"
        }

        fn is_eof(&self) -> bool {
            self.kind == "EOF"
        }

        fn is_char(&self) -> bool {
            self.kind == "CHAR"
        }

        fn is_kind(&self, s: &str) -> bool {
            self.kind == s
        }

        fn is_string(&self) -> bool {
            self.kind == "STRING_PART" || self.kind == "END_OF_STRING" || self.kind == "QUOTE"
        }
    }

    trace::init_depth_var!();

    // #[trace]
    impl Parser {
        #[trace]
        fn add_method(&mut self, classname: &str, protocol: &str, is_class: bool, m: Node) {
            if !self.classes.contains_key(classname) {
                self.classes.insert(
                    classname.to_string(),
                    STClass {
                        comment: String::new(),
                        parent: String::new(),
                        protocols: vec![],
                    },
                );
            }
            if let Some(cls) = self.classes.get_mut(classname) {
                cls.add_method(protocol, is_class, m);
            } else {
                panic!()
            }
        }

        fn new_class(&mut self, name: &str, parent: &str) {
            self.classes.insert(
                name.to_string(),
                STClass {
                    comment: String::new(),
                    parent: parent.to_string(),
                    protocols: vec![],
                },
            );
        }

        pub fn current_token(&self) -> Rc<Lexeme> {
            self.current_token.as_ref().unwrap().clone()
        }

        pub fn new(scanner: Vec<Rc<Lexeme>>) -> Self {
            // for x in scanner.iter() {
            //     println!("{:?}", x);
            // }

            let current_token = None;
            let next_token = None;
            let error_block = String::new();
            let tags = vec![];
            let source = String::new();
            let method_category = String::new();
            let classes = HashMap::new();
            let mut r = Self {
                scanner,
                current_token,
                next_token,
                error_block,
                tags,
                source,
                method_category,
                classes,
            };
            r.step();
            r.step();
            r
        }
        fn step(&mut self) {
            // step [
            // <category: 'private'>
            // nextToken notNil
            //     ifTrue:
            // 	[currentToken := nextToken.
            // 	nextToken := nil.
            // 	^currentToken].
            // currentToken := scanner next
            // ]
            let p = self.current_token.clone();
            self.current_token = self.next_token.clone();
            self.fill_next_token();
            // println!("{:?}", self.current_token);
        }

        fn expect(&mut self, c: &str) -> bool {
            // println!("expect {} {:?}", c, self.currentToken);
            if self.current_token().raw == c {
                self.step();
                true
            } else {
                false
            }
        }

        fn expect_or_fail(&mut self, c: &str) -> Result<(), String> {
            if !self.expect(c) {
                return Err(format!("expected: {c} but found {:?}", self.current_token));
            }
            Ok(())
        }

        fn fill_next_token(&mut self) {
            if self.scanner.len() > 0 {
                self.next_token = Some(self.scanner.remove(0));
            } else {
                self.next_token = Some(Rc::new(Lexeme {
                    kind: String::from("EOF"),
                    raw: String::new(),
                    position: Position { line: 0, column: 0 },
                }));
            }
        }

        fn get_class_mut(&mut self, classname: &str) -> Option<&mut STClass> {
            self.classes.get_mut(classname)
        }

        // <category: 'Refactory-Parser'>
        // <comment: nil>

        // RBParser class >> parseMethod: aString [
        // <category: 'accessing'>
        // ^self
        //     parseMethod: aString
        //     category: nil
        //     onError: nil
        // ]

        // fn parse
        // RBParser class >> parseMethod: aString category: aCategory [
        // <category: 'accessing'>
        // ^self
        //     parseMethod: aString
        //     category: aCategory
        //     onError: nil
        // ]

        // RBParser class >> parseMethod: aString onError: aBlock [
        // <category: 'accessing'>
        // ^self
        //     parseMethod: aString
        //     category: nil
        //     onError: aBlock
        // ]

        // RBParser class >> parseRewriteExpression: aString [
        // <category: 'accessing'>
        // ^self parseRewriteExpression: aString onError: nil
        // ]

        fn parse_rewrite_expression(&mut self, s: &str) {
            // RBParser class >> parseRewriteExpression: aString onError: aBlock [
            // <category: 'accessing'>
            // | node parser |
            // parser := self new.
            // parser errorBlock: aBlock.
            // parser initializeParserWith: aString type: #rewriteOn:errorBlock:.
            // node := parser parseExpression.
            // ^(node statements size == 1 and: [node temporaries isEmpty])
            //     ifTrue: [node statements first]
            //     ifFalse: [node]
            // ]
        }

        // RBParser class >> parseRewriteMethod: aString [
        // <category: 'accessing'>
        // ^self parseRewriteMethod: aString onError: nil
        // ]

        fn parse_rewrite_method(&mut self, s: &str) {
            // RBParser class >> parseRewriteMethod: aString onError: aBlock [
            // <category: 'accessing'>
            // | parser |
            // parser := self new.
            // parser errorBlock: aBlock.
            // parser initializeParserWith: aString type: #rewriteOn:errorBlock:.
            // ^parser parseMethod: aString
            // ]
        }

        fn parse_method_pattern(&mut self, s: &str) {
            // RBParser class >> parseMethodPattern: aString [
            // <category: 'parsing'>
            // | parser |
            // parser := self new.
            // parser errorBlock: [:error :position | ^nil].
            // parser initializeParserWith: aString type: #on:errorBlock:.
            // ^parser parseMessagePattern selector
            // ]
        }
        // methodCategory [
        // <category: 'accessing'>
        // ^methodCategory
        // ]

        // methodCategory: aCategory [
        // <category: 'accessing'>
        // methodCategory := aCategory
        // ]

        // errorBlock: aBlock [
        // <category: 'accessing'>
        // errorBlock := aBlock.
        // scanner notNil ifTrue: [scanner errorBlock: aBlock]
        // ]

        // initializeParserWith: aString type: aSymbol [
        // <category: 'accessing'>
        // source := aString.
        // self scanner: (self scannerClass
        // 	    perform: aSymbol
        // 	    with: (ReadStream on: aString)
        // 	    with: self errorBlock)
        // ]

        // initializeParserWithStream: aStream type: aSymbol [
        // <category: 'accessing'>
        // source := nil.
        // self scanner: (self scannerClass
        // 	    perform: aSymbol
        // 	    with: aStream
        // 	    with: self errorBlock)
        // ]

        fn parse_expression(&mut self) {
            // RBParser class >> parseExpression: aString [
            // <category: 'accessing'>
            // ^self parseExpression: aString onError: nil
            // ]

            // RBParser class >> parseExpression: aString onError: aBlock [
            // <category: 'accessing'>
            // | node parser |
            // parser := self new.
            // parser errorBlock: aBlock.
            // parser initializeParserWith: aString type: #on:errorBlock:.
            // node := parser parseExpression.
            // ^(node statements size == 1 and: [node temporaries isEmpty])
            //     ifTrue: [node statements first]
            //     ifFalse: [node]
            // ]

            // parseExpression [
            // <category: 'accessing'>
            // | node |
            // node := self parseStatements: false.
            // self atEnd ifFalse: [self parserError: 'Unknown input at end'].
            // ^node
            // ]
        }

        // scannerClass [
        // <category: 'accessing'>
        // ^RBScanner
        // ]

        // errorBlock [
        // <category: 'error handling'>
        // ^errorBlock isNil ifTrue: [[:message :position | ]] ifFalse: [errorBlock]
        // ]

        // errorFile [
        // <category: 'error handling'>
        // ^scanner stream name
        // ]

        // errorLine [
        // <category: 'error handling'>
        // ^(scanner stream copyFrom: 1 to: self errorPosition) readStream lines
        //     contents size
        // ]

        // errorPosition [
        // <category: 'error handling'>
        // ^currentToken start
        // ]

        // parserWarning: aString [
        // "Raise a Warning"

        // <category: 'error handling'>
        // Warning signal: aString
        // ]

        // parserError: aString [
        // "Evaluate the block. If it returns raise an error"

        // <category: 'error handling'>
        // self errorBlock value: aString value: self errorPosition.
        // self
        //     error: '%1:%2: %3' %
        // 		{self errorFile.
        // 		self errorLine.
        // 		aString}
        // ]

        // scanner: aScanner [
        // <category: 'initialize-release'>
        // scanner := aScanner.
        // tags := nil.
        // self step
        // ]

        // addCommentsTo: aNode [
        // <category: 'private'>
        // aNode comments: scanner getComments
        // ]

        // currentToken [
        // <category: 'private'>
        // ^currentToken
        // ]

        // nextToken [
        // <category: 'private'>
        // ^nextToken isNil ifTrue: [nextToken := scanner next] ifFalse: [nextToken]
        // ]

        fn parse_arg(&mut self) -> Node {
            // parseArgs [
            // <category: 'private-parsing'>
            // | args |
            // args := OrderedCollection new.
            // [currentToken isIdentifier] whileTrue: [args add: self parseVariableNode].
            if self.current_token.as_ref().unwrap().is_identifier() {
                let r = Node::Identifier(self.current_token.as_ref().unwrap().raw.clone());
                self.step();
                r
            } else {
                panic!("expected identifier {:?}", self.current_token)
            }
            // ^args
            // ]
        }
        fn parse_array_constructor(&mut self) {
            // parseArrayConstructor [
            // <category: 'private-parsing'>
            // | position node |
            // position := currentToken start.
            // self step.
            // node := RBArrayConstructorNode new.
            // node left: position.
            // node body: (self parseStatements: false).
            // (currentToken isSpecial and: [currentToken value == $}])
            //     ifFalse: [self parserError: '''}'' expected'].
            // node right: currentToken start.
            // self step.
            // ^node
            // ]
        }
        fn parse_assignment(&mut self) -> Node {
            // parseAssignment [
            // "Need one token lookahead to see if we have a ':='. This method could
            //  make it possible to assign the literals true, false and nil."

            // <category: 'private-parsing'>
            // | node position |
            if self.current_token().is_identifier() && self.next_token().is_assignment() {
                // (currentToken isIdentifier and: [self nextToken isAssignment])
                //     ifFalse: [^self parseCascadeMessage].
                let target = self.current_token().raw.clone();
                self.step();
                self.step();
                Node::Assign {
                    target,
                    value: Box::new(self.parse_assignment()),
                }
            } else {
                self.parse_cascade_message()
            }
            // node := self parseVariableNode.
            // position := currentToken start.
            // self step.
            // ^RBAssignmentNode
            //     variable: node
            //     value: self parseAssignment
            //     position: position
            // ]
        }
        fn parse_binary_argument(&mut self) -> Node {
            let arg = self.parse_primitive_object();
            match self.parse_unary_messages() {
                Some(msgs) => Node::Expression {
                    receiver: Box::new(arg),
                    messages: vec![msgs],
                },
                None => arg,
            }
        }
        // fn parse_binary_message(&mut self) -> Node {
        //     // parseBinaryMessage [
        //     // <category: 'private-parsing'>
        //     // | node |
        //     // node := self parseUnaryMessage.
        //     let mut r = self.parse_unary_message();
        //     while self.current_token.is_binary() {
        //         let selector = self.current_token.raw.clone();

        //         self.step();
        //         r = Node::Message {
        //             selector,
        //             params: vec![self.parse_unary_message()],
        //             outer: None,
        //         };
        //     }
        //     // [currentToken isBinary]
        //     //     whileTrue: [node := self parseBinaryMessageWith: node].
        //     // ^node
        //     // ]
        //     r
        // }
        fn parse_binary_messages(&mut self) -> Option<Node> {
            if self.current_token().is_binary() {
                let selector = self.current_token().raw.clone();
                let pos = self.current_token().position.clone();
                self.step();
                let arg = self.parse_binary_argument();
                match self.parse_binary_messages() {
                    Some(msg) => Some(Node::Message {
                        selector,
                        params: vec![self.parse_binary_argument()],
                        outer: Some(Box::new(msg)),
                    }),
                    None => Some(Node::Message {
                        selector,
                        params: vec![self.parse_binary_argument()],
                        outer: None,
                    }),
                }
            } else {
                None
            }
        }
        fn parse_binary_message_no_greater(&mut self) {
            // parseBinaryMessageNoGreater [
            // <category: 'private-parsing'>
            // | node |
            // node := self parseUnaryMessage.
            // [currentToken isBinary and: [currentToken value ~~ #>]]
            //     whileTrue: [node := self parseBinaryMessageWith: node].
            // ^node
            // ]
        }
        fn parse_binary_message_with(&mut self, node: &str) {
            // parseBinaryMessageWith: aNode [
            // <category: 'private-parsing'>
            // | binaryToken |
            // binaryToken := currentToken.
            // self step.
            // ^RBMessageNode
            //     receiver: aNode
            //     selectorParts: (Array with: binaryToken)
            //     arguments: (Array with: self parseUnaryMessage)
            // ]
        }
        fn parse_binary_pattern(&mut self) -> Node {
            // parseBinaryPattern [
            // <category: 'private-parsing'>
            // | binaryToken |
            // currentToken isBinary
            //     ifFalse: [self parserError: 'Message pattern expected'].
            let selector = self.current_token().raw.clone();
            // binaryToken := currentToken.
            // self step.
            self.step();
            // ^RBMethodNode selectorParts: (Array with: binaryToken)
            //     arguments: (Array with: self parseVariableNode)
            // ]
            let args = vec![self.current_token().raw.clone()];
            self.step();
            Node::Method {
                selector,
                args,
                body: Box::new(Node::None),
            }
        }
        fn parse_block(&mut self) -> Node {
            // parseBlock [
            // <category: 'private-parsing'>
            // | position node |
            self.expect_or_fail("[");
            // position := currentToken start.
            // self step.
            // node := self parseBlockArgsInto: RBBlockNode new.
            let r = if let Node::Block { args, .. } = self.parse_block_args_into() {
                let body = Box::new(self.parse_statment_list());
                Node::Block { args, body }
            } else {
                Node::None
            };
            self.expect_or_fail("]");
            // node left: position.
            // node body: (self parseStatements: false).

            // (currentToken isSpecial and: [currentToken value == $]])
            //     ifFalse: [self parserError: ''']'' expected'].
            // node right: currentToken start.
            // self step.
            // ^node
            // ]
            r
        }
        fn parse_block_args_into(&mut self) -> Node {
            // parseBlockArgsInto: node [
            // <category: 'private-parsing'>
            // | verticalBar args colons |
            // args := OrderedCollection new: 2.
            // colons := OrderedCollection new: 2.
            // verticalBar := false.
            let mut args = vec![];
            if self.current_token().is_kind(":") {
                while self.current_token().is_kind(":") {
                    self.step();
                    if self.current_token().is_identifier() {
                        args.push(Node::Identifier(self.current_token().raw.clone()));
                        self.step();
                    } else {
                        panic!("expected identifier")
                    }
                }
                if self.current_token().is("|") {
                    self.step();
                } else if self.current_token().is_kind("]") {
                }
            }
            Node::Block {
                args,
                body: Box::new(Node::None),
            }
            // [currentToken isSpecial and: [currentToken value == $:]] whileTrue:
            // 	[colons add: currentToken start.
            // 	self step.	":"
            // 	verticalBar := true.
            // 	args add: self parseVariableNode].
            // verticalBar
            //     ifTrue:
            // 	[currentToken isBinary
            // 	    ifTrue:
            // 		[node bar: currentToken start.
            // 		currentToken value == #|
            // 		    ifTrue: [self step]
            // 		    ifFalse:
            // 			[currentToken value == #'||'
            // 			    ifTrue:
            // 				["Hack the current token to be the start
            // 				 of temps bar"

            // 				currentToken
            // 				    value: #|;
            // 				    start: currentToken start + 1]
            // 			    ifFalse: [self parserError: '''|'' expected']]]
            // 	    ifFalse:
            // 		[(currentToken isSpecial and: [currentToken value == $]])
            // 		    ifFalse: [self parserError: '''|'' expected']]].
            // node
            //     arguments: args;
            //     colons: colons.
            // ^node
            // ]
        }
        // pub fn parse_chunks(&mut self) -> Node {
        //     while !self.current_token().is_eof() {
        //         let cmd = self.parse_assignment();
        //         if self.current_token().is_eof() {
        //             break;
        //         }
        //         self.expect_or_fail("!");
        //         self.handle_chunk_expression(cmd);
        //         // n -= 1;
        //     }
        //     println!("{:#?}", self.classes);
        //     Node::None
        // }

        fn handle_chunk_expression(&mut self, cmd: Node) {
            match cmd {
                Node::Expression { receiver, messages } => {
                    for msg in messages.iter() {
                        match receiver.as_ref() {
                            Node::None => println!("{:?}", msg),
                            x => self.handle_chunk_message(receiver.as_str(), false, msg),
                        }
                    }
                }
                Node::None => {}
                x => {
                    println!("-> {:?}", x)
                }
            }
        }
        #[trace]
        fn handle_chunk_message(&mut self, classname: &str, is_class: bool, msg: &Node) {
            let mut l_is_class = is_class;
            match msg {
                Node::Message {
                    // receiver,
                    selector,
                    params,
                    outer,
                } => {
                    match selector.as_str() {
                        "methodsFor:" => {
                            let protocol = params[0].as_str();
                            loop {
                                let m = self.parse_method();
                                self.add_method(classname, protocol, l_is_class, m);

                                while !self.current_token().is("!") {
                                    println!("leftovers ...... {:?}", self.current_token);
                                    self.step();
                                }
                                self.step();
                                if self.current_token().is("!") {
                                    break;
                                }
                            }
                        }
                        "class" => {
                            l_is_class = true;
                        }
                        "variableWordSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                            let name = params[0].as_str();
                            self.new_class(name, classname);
                        }
                        "variableSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                            let name = params[0].as_str();
                            self.new_class(name, classname);
                        }
                        "subclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                            let name = params[0].as_str();
                            self.new_class(name, classname);
                        }
                        "variableByteSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                            let name = params[0].as_str();
                            self.new_class(name, classname);
                        }
                        "comment:" => {
                            if let Some(c) = self.get_class_mut(classname) {
                                let comment = params[0].as_str();
                                c.set_comment(comment);
                            } else {
                                eprintln!("class {classname} not defined.")
                            }
                        }
                        s => println!("{s}"),
                    };
                    if let Some(outer) = outer {
                        self.handle_chunk_message(classname, l_is_class, outer);
                    }
                }
                Node::None => {}
                x => {
                    println!("-> {:?}", x)
                }
            }
        }

        fn parse_cascade_message(&mut self) -> Node {
            // <primary> [<messages> <cascaded messages>]
            // <messages> ::= <unary message>+ <binary message>* [<keyword message>]
            //              | <binary message>+ [<keyword message>]
            //              | <keyword message>

            // parseCascadeMessage [
            // <category: 'private-parsing'>
            // | node receiver messages semicolons |
            // node := self parseKeywordMessage.
            let receiver = self.parse_primitive_object();
            if let Some(message) = self.parse_messages() {
                let mut messages = vec![message];
                // let r = self.parse_keyword_message();
                // (currentToken isSpecial
                //     and: [currentToken value == $; and: [node isMessage]]) ifFalse: [^node].
                // receiver := node receiver.
                // messages := OrderedCollection new: 3.
                // semicolons := OrderedCollection new: 3.
                // messages add: node.
                // [currentToken isSpecial and: [currentToken value == $;]] whileTrue:
                while self.current_token().is(";") {
                    self.step();
                    if let Some(message) = self.parse_messages() {
                        messages.push(message);
                    } else {
                        panic!("{:?}", self.current_token)
                    }
                }
                Node::Expression {
                    receiver: Box::new(receiver),
                    messages: messages,
                }
            } else {
                receiver
            }
        }

        fn parse_keyword_argument(&mut self) -> Node {
            let receiver = self.parse_primitive_object();

            let mut k = self.parse_unary_messages();
            let j = self.parse_binary_messages();
            match &mut k {
                Some(k) => k.set_outer(j),
                None => k = j,
            }
            match k {
                Some(k) => Node::Expression {
                    receiver: Box::new(receiver),
                    messages: vec![k],
                },
                None => receiver,
            }
        }
        fn parse_keyword_message(&mut self) -> Option<Node> {
            if self.current_token().is_keyword() {
                let mut selector = String::new();
                let mut params = vec![];
                while self.current_token().is_keyword() {
                    selector.push_str(&self.current_token().raw);
                    self.step();
                    params.push(self.parse_keyword_argument());
                }

                Some(Node::Message {
                    selector,
                    params,
                    outer: None,
                })
            } else {
                None
            }
        }
        fn parse_keyword_pattern(&mut self) -> Node {
            // parseKeywordPattern [
            // <category: 'private-parsing'>
            // | keywords args |
            // keywords := OrderedCollection new: 2.
            let mut keywords = vec![];
            // args := OrderedCollection new: 2.
            let mut args = vec![];

            // [currentToken isKeyword] whileTrue:
            while self.current_token().is_keyword() {
                // 	[keywords add: currentToken.
                keywords.push(self.current_token().raw.clone());
                // 	self step.
                self.step();
                // 	args add: self parseVariableNode].
                assert!(self.current_token().is_identifier());
                args.push(self.current_token().raw.clone());
                // ^RBMethodNode selectorParts: keywords arguments: args
                // ]
                self.step();
            }
            Node::Method {
                selector: keywords.join(""),
                args,
                body: Box::new(Node::None),
            }
        }
        fn parse_message_pattern(&mut self) -> Node {
            // parseMessagePattern [
            // <category: 'private-parsing'>
            if self.current_token().is_identifier() {
                self.parse_unary_pattern()
            } else if self.current_token().is_keyword() {
                self.parse_keyword_pattern()
            } else {
                self.parse_binary_pattern()
            }
        }

        fn parse_messages(&mut self) -> Option<Node> {
            if self.current_token().is_identifier() {
                if let Some(mut msg0) = self.parse_unary_messages() {
                    msg0.set_outer(self.parse_binary_messages());
                    msg0.set_outer(self.parse_keyword_message());
                    Some(msg0)
                } else {
                    panic!()
                }
            } else if self.current_token().is_binary() {
                if let Some(mut msg0) = self.parse_binary_messages() {
                    msg0.set_outer(self.parse_keyword_message());
                    Some(msg0)
                } else {
                    panic!()
                }
            } else if self.current_token().is_keyword() {
                self.parse_keyword_message()
            } else {
                None
            }
        }

        pub fn parse_method(&mut self) -> Node {
            // parseMethod [
            // <category: 'private-parsing'>
            // | methodNode |
            // methodNode := self parseMessagePattern.
            if let Node::Method {
                selector,
                args,
                body,
            } = self.parse_message_pattern()
            {
                let _ = body;
                let body = Box::new(self.parse_statements());

                Node::Method {
                    selector,
                    args,
                    body,
                }
            } else {
                panic!()
            }
            // ^self parseMethodInto: methodNode
            // ]
            // RBParser class >> parseMethod: aString category: aCategory onError: aBlock [
            // <category: 'accessing'>
            // | parser |
            // parser := self new.
            // parser methodCategory: aCategory.
            // parser errorBlock: aBlock.
            // parser initializeParserWith: aString type: #on:errorBlock:.
            // ^parser parseMethod: aString
            // ]

            // parseMethod: aString [
            // <category: 'accessing'>
            // | node |
            // node := self parseMethod.
            // self atEnd ifFalse: [self parserError: 'Unknown input at end'].
            // node source: aString.
            // ^node
            // ]
            // parseMethodInto: methodNode [
            // <category: 'private-parsing'>
            // tags := nil.
            // self parseResourceTag.
            // self addCommentsTo: methodNode.
            // methodNode body: (self parseStatements: true).
            // methodNode tags: tags.
            // methodNode category: methodCategory.
            // ^methodNode
            // ]
        }
        // fn parse_optimized_expression(&mut self) {
        //     // parseOptimizedExpression [
        //     // <category: 'private-parsing'>
        //     // | position node |
        //     // position := currentToken start.
        //     // self step.
        //     // node := RBOptimizedNode
        //     // 	    left: position
        //     // 	    body: (self parseStatements: false)
        //     // 	    right: currentToken start.
        //     // (currentToken isSpecial and: [currentToken value == $)])
        //     //     ifFalse: [self parserError: ''')'' expected'].
        //     // self step.
        //     // ^node
        //     // ]
        // }
        fn parse_parenthesized_expression(&mut self) -> Node {
            // parseParenthesizedExpression [
            // <category: 'private-parsing'>
            // | leftParen node |
            // leftParen := currentToken start.
            assert!(self.expect("("));
            // self step.
            // node := self parseAssignment.
            let r = self.parse_assignment();
            // ^(currentToken isSpecial and: [currentToken value == $)])
            //     ifTrue:
            self.expect_or_fail(")");
            // 	[node addParenthesis: (leftParen to: currentToken start).
            // 	self step.
            // 	node]
            //     ifFalse: [self parserError: ''')'' expected']
            // ]
            r
        }
        // fn parse_pattern_block(&mut self) {
        //     // parsePatternBlock [
        //     // <category: 'private-parsing'>
        //     // | position node |
        //     // position := currentToken start.
        //     // self step.
        //     // node := self parseBlockArgsInto: RBPatternBlockNode new.
        //     // node left: position.
        //     // node body: (self parseStatements: false).
        //     // (currentToken isSpecial and: [currentToken value == $}])
        //     //     ifFalse: [self parserError: '''}'' expected'].
        //     // node right: currentToken start.
        //     // self step.
        //     // ^node
        //     // ]
        // }
        fn parse_primitive_identifier(&mut self) -> Node {
            // parsePrimitiveIdentifier [
            // <category: 'private-parsing'>
            // | value token |
            // token := currentToken.
            // value := currentToken value.
            // self step.
            // value = 'true'
            //     ifTrue:
            // 	[^RBLiteralNode literalToken: (RBLiteralToken
            // 		    value: true
            // 		    start: token start
            // 		    stop: token start + 3)].
            // value = 'false'
            //     ifTrue:
            // 	[^RBLiteralNode literalToken: (RBLiteralToken
            // 		    value: false
            // 		    start: token start
            // 		    stop: token start + 4)].
            // value = 'nil'
            //     ifTrue:
            // 	[^RBLiteralNode literalToken: (RBLiteralToken
            // 		    value: nil
            // 		    start: token start
            // 		    stop: token start + 2)].
            // ^RBVariableNode identifierToken: token
            // ]
            let r = Node::Identifier(self.current_token().raw.to_string());
            self.step();
            r
        }
        // fn parse_negated_number(&mut self) {
        //     // parseNegatedNumber [
        //     // <category: 'private-parsing'>
        //     // | token |
        //     // self step.
        //     // token := currentToken.
        //     // (token value respondsTo: #negated) ifFalse: [
        //     //     ^self parserError: 'Number expected' ].
        //     // token value negative ifTrue: [
        //     //     ^self parserError: 'Positive number expected' ].
        //     // token makeNegative.
        //     // self step.
        //     // ^RBLiteralNode literalToken: token
        //     // ]
        // }
        fn parse_primitive_literal(&mut self) -> Node {
            // parsePrimitiveLiteral [
            // <category: 'private-parsing'>
            // | token |
            // token := currentToken.
            // self step.
            // ^RBLiteralNode literalToken: token
            // ]
            let r = if self.current_token().is_string() {
                let mut s = String::new();
                while self.current_token().kind != "END_OF_STRING" {
                    s.push_str(&self.current_token().raw);
                    self.step();
                }
                Node::String(s)
            } else if self.current_token().is_char() {
                Node::String(self.current_token().raw[1..2].to_string())
            } else {
                Node::Literal(self.current_token().raw.clone())
            };
            self.step();
            r
        }

        fn parse_array_literal(&mut self) -> Node {
            self.step();
            self.expect("(");
            let mut level = 1;
            loop {
                if self.current_token().is_kind("(") {
                    level += 1;
                }
                if self.current_token().is_kind(")") {
                    level -= 1;
                }
                if level == 0 || self.current_token().is_eof() {
                    break;
                }
                self.step();
            }
            if self.current_token().is_kind(")") {
                self.step();
            }

            Node::ArrayLiteral
        }

        fn parse_primitive_object(&mut self) -> Node {
            // parsePrimitiveObject [
            // <category: 'private-parsing'>
            // currentToken isIdentifier ifTrue: [^self parsePrimitiveIdentifier].
            if self.current_token().is_identifier() {
                self.parse_primitive_identifier()
            }
            // currentToken isLiteral ifTrue: [^self parsePrimitiveLiteral].
            else if self.current_token().is_literal() {
                self.parse_primitive_literal()
            } else if self.current_token().is_symbol() {
                let r = Node::Symbol(self.current_token().raw[1..].to_string());
                self.step();
                r
            }
            // (currentToken isBinary and: [ currentToken value == #- ])
            //     ifTrue: [^self parseNegatedNumber].
            // currentToken isSpecial
            //     ifTrue:
            // 	[currentToken value == $[ ifTrue: [^self parseBlock].
            // 	currentToken value == ${ ifTrue: [^self parseArrayConstructor].
            // 	currentToken value == $( ifTrue: [^self parseParenthesizedExpression]].
            else if self.current_token().is_kind("(") {
                self.parse_parenthesized_expression()
            } else if self.current_token().is_kind("[") {
                self.parse_block()
            } else if self.current_token().is_kind("#") {
                self.parse_array_literal()
            } else {
                Node::None
            }
            // currentToken isPatternBlock ifTrue: [^self parsePatternBlock].
            // currentToken isOptimized ifTrue: [^self parseOptimizedExpression].
            // self parserError: 'Variable expected'
            // ]
        }

        //     parseResourceTag [
        // 	<category: 'private-parsing'>
        // 	| start |
        // 	[currentToken isBinary and: [currentToken value == #<]] whileTrue:
        // 		[start := currentToken start.
        // 		self step.
        // 		[scanner atEnd or: [currentToken isBinary and: [currentToken value == #>]]]
        // 		    whileFalse: [self step].
        // 		(currentToken isBinary and: [currentToken value == #>])
        // 		    ifFalse: [self parserError: '''>'' expected'].
        // 		tags isNil
        // 		    ifTrue: [tags := OrderedCollection with: (start to: currentToken stop)]
        // 		    ifFalse: [tags add: (start to: currentToken stop)].
        // 		self step]
        //     ]
        fn parse_statment_list(&mut self) -> Node {
            //     parseStatementList: tagBoolean into: sequenceNode [
            // 	<category: 'private-parsing'>
            // 	| statements return periods returnPosition node |
            // 	return := false.
            // 	statements := OrderedCollection new.
            // 	periods := OrderedCollection new.
            // 	self addCommentsTo: sequenceNode.
            // 	tagBoolean ifTrue: [self parseResourceTag].
            // 	[self atEnd
            // 	    or: [currentToken isSpecial and: ['!])}' includes: currentToken value]]]
            let mut stmts = vec![];
            loop {
                // 		whileFalse:
                // 		    [return ifTrue: [self parserError: 'End of statement list encountered'].
                if self.current_token().is_return() {
                    // 		    (currentToken isSpecial and: [currentToken value == $^])
                    // 			ifTrue:
                    // 			    [returnPosition := currentToken start.
                    // 			    self step.
                    self.step();
                    // 			    node := RBReturnNode return: returnPosition value: self parseAssignment.
                    let n = self.parse_assignment();
                    stmts.push(n);
                    // 			    self addCommentsTo: node.
                    // 			    statements add: node.
                    // 			    return := true]
                } else {
                    // 			ifFalse:
                    // 			    [node := self parseAssignment.
                    let n = self.parse_assignment();
                    stmts.push(n);
                    // 			    self addCommentsTo: node.
                    // 			    statements add: node].
                }
                // 		    (currentToken isSpecial and: [currentToken value == $.])
                // 			ifTrue:
                // 			    [periods add: currentToken start.
                if !self.expect(".") {
                    break;
                }
                // 			    self step]
                // 			ifFalse: [return := true]].
            }
            // 	sequenceNode
            // 	    statements: statements;
            // 	    periods: periods.
            // 	^sequenceNode
            //     ]
            Node::Statements(stmts)
        }

        pub fn parse_statements(&mut self) -> Node {
            //  parseStatements: tagBoolean [
            // 	<category: 'private-parsing'>
            // 	| args leftBar rightBar |
            // 	args := #().
            // 	leftBar := rightBar := nil.
            // 	currentToken isBinary
            let mut locals = vec![];
            if self.current_token().is_pipe() {
                // 	    ifTrue:
                // 		[currentToken value == #|
                // 		    ifTrue:
                // 			[leftBar := currentToken start.
                // 			self step.
                self.step();
                // 			args := self parseArgs.
                while !self.expect("|") {
                    locals.push(self.parse_arg());
                }
                // 			(currentToken isBinary and: [currentToken value = #|])
                // 			    ifFalse: [self parserError: '''|'' expected'].
                // 			rightBar := currentToken start.
                // 			self step]
                // 		    ifFalse:
                // 			[currentToken value == #'||'
                // 			    ifTrue:
                // 				[rightBar := (leftBar := currentToken start) + 1.
                // 				self step]]].
            }
            let r = self.parse_statment_list();
            // 	^self parseStatementList: tagBoolean
            // 	    into: (RBSequenceNode
            // 		    leftBar: leftBar
            // 		    temporaries: args
            // 		    rightBar: rightBar)
            //     ]
            r
        }

        fn parse_unary_messages(&mut self) -> Option<Node> {
            if self.current_token().is_identifier() {
                let selector = self.current_token().raw.clone();
                self.step();
                match self.parse_unary_messages() {
                    Some(msg) => Some(Node::Message {
                        selector,
                        params: vec![],
                        outer: Some(Box::new(msg)),
                    }),
                    None => Some(Node::Message {
                        selector,
                        params: vec![],
                        outer: None,
                    }),
                }
            } else {
                None
            }
        }
        // //     parseUnaryMessage [
        // fn parse_unary_message(&mut self) -> Node {
        //     // 	<category: 'private-parsing'>
        //     // 	| node |
        //     // 	node := self parsePrimitiveObject.
        //     let mut r = self.parse_primitive_object();
        //     // 	[currentToken isIdentifier]
        //     // 	    whileTrue: [node := self parseUnaryMessageWith: node].
        //     // 	^node
        //     //     ]

        //     //     parseUnaryMessageWith: aNode [
        //     // 	<category: 'private-parsing'>
        //     // 	| selector |
        //     // 	selector := currentToken.
        //     // 	self step.
        //     // 	^RBMessageNode
        //     // 	    receiver: aNode
        //     // 	    selectorParts: (Array with: selector)
        //     // 	    arguments: #()
        //     //     ]
        //     r
        // }
        //     parseUnaryPattern [
        fn parse_unary_pattern(&mut self) -> Node {
            let selector = self.current_token().raw.clone();
            self.step();
            let args = vec![];
            Node::Method {
                selector,
                args,
                body: Box::new(Node::None),
            }
            // 	<category: 'private-parsing'>
            // 	| selector |
            // 	selector := currentToken.
            // 	self step.
            // 	^RBMethodNode selectorParts: (Array with: selector) arguments: #()
            //     ]
        }

        fn next_token(&self) -> Rc<Lexeme> {
            let x = self.next_token.as_ref().unwrap().clone();
            x
        }

        //     parseVariableNode [
        // 	<category: 'private-parsing'>
        // 	| node |
        // 	currentToken isIdentifier
        // 	    ifFalse: [self parserError: 'Variable name expected'].
        // 	node := RBVariableNode identifierToken: currentToken.
        // 	self step.
        // 	^node
        //     ]

        //     atEnd [
        // 	<category: 'testing'>
        // 	^currentToken class == RBToken
        //     ]
        // ]

        //
        // Stream subclass: RBScanner [
        //     | stream buffer tokenStart currentCharacter characterType classificationTable saveComments comments extendedLanguage errorBlock |

        //     <category: 'Refactory-Parser'>
        //     <comment: nil>

        //     ClassificationTable := nil.
        //     PatternVariableCharacter := nil.

        //     RBScanner class >> classificationTable [
        // 	<category: 'accessing'>
        // 	ClassificationTable isNil ifTrue: [self initialize].
        // 	^ClassificationTable
        //     ]

        //     RBScanner class >> patternVariableCharacter [
        // 	<category: 'accessing'>
        // 	^PatternVariableCharacter
        //     ]

        //     RBScanner class >> initialize [
        // 	<category: 'class initialization'>
        // 	PatternVariableCharacter := $`.
        // 	ClassificationTable := Array new: 255.
        // 	self
        // 	    initializeChars: 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_'
        // 	    to: #alphabetic.
        // 	self initializeChars: '01234567890' to: #digit.
        // 	self initializeChars: '%&*+,-/<=>?@\~|' to: #binary.
        // 	self initializeChars: '{}().:;[]^!' to: #special.
        // 	#(9 10 12 13 26 32) do: [:i | ClassificationTable at: i put: #separator]
        //     ]

        //     RBScanner class >> initializeChars: characters to: aSymbol [
        // 	<category: 'class initialization'>
        // 	characters do: [:c | ClassificationTable at: c asInteger put: aSymbol]
        //     ]

        //     RBScanner class >> on: aStream [
        // 	<category: 'instance creation'>
        // 	| str |
        // 	str := self basicNew on: aStream.
        // 	str step.
        // 	str stripSeparators.
        // 	^str
        //     ]

        //     RBScanner class >> on: aStream errorBlock: aBlock [
        // 	<category: 'instance creation'>
        // 	| str |
        // 	str := self basicNew on: aStream.
        // 	str
        // 	    errorBlock: aBlock;
        // 	    step;
        // 	    stripSeparators.
        // 	^str
        //     ]

        //     RBScanner class >> rewriteOn: aStream [
        // 	<category: 'instance creation'>
        // 	| str |
        // 	str := self basicNew on: aStream.
        // 	str
        // 	    extendedLanguage: true;
        // 	    ignoreComments.
        // 	str step.
        // 	str stripSeparators.
        // 	^str
        //     ]

        //     RBScanner class >> rewriteOn: aStream errorBlock: aBlock [
        // 	<category: 'instance creation'>
        // 	| str |
        // 	str := self basicNew on: aStream.
        // 	str
        // 	    extendedLanguage: true;
        // 	    ignoreComments;
        // 	    errorBlock: aBlock;
        // 	    step;
        // 	    stripSeparators.
        // 	^str
        //     ]

        //     RBScanner class >> isSelector: aSymbol [
        // 	<category: 'testing'>
        // 	| scanner token |
        // 	scanner := self basicNew.
        // 	scanner on: (ReadStream on: aSymbol asString).
        // 	scanner step.
        // 	token := scanner scanAnySymbol.
        // 	token isLiteral ifFalse: [^false].
        // 	token value isEmpty ifTrue: [^false].
        // 	^scanner atEnd
        //     ]

        //     RBScanner class >> isVariable: aString [
        // 	<category: 'testing'>
        // 	| scanner token |
        // 	aString isString ifFalse: [^false].
        // 	aString isEmpty ifTrue: [^false].
        // 	(ClassificationTable at: aString first asInteger) == #alphabetic
        // 	    ifFalse: [^false].
        // 	scanner := self basicNew.
        // 	scanner on: (ReadStream on: aString asString).
        // 	scanner errorBlock: [:s :p | ^false].
        // 	scanner step.
        // 	token := scanner scanIdentifierOrKeyword.
        // 	token isKeyword ifTrue: [^false].
        // 	^scanner atEnd
        //     ]

        //     classificationTable: anObject [
        // 	<category: 'accessing'>
        // 	classificationTable := anObject
        //     ]

        //     contents [
        // 	<category: 'accessing'>
        // 	| contentsStream |
        // 	contentsStream := WriteStream on: (Array new: 50).
        // 	self do: [:each | contentsStream nextPut: each].
        // 	^contentsStream contents
        //     ]

        //     errorBlock: aBlock [
        // 	<category: 'accessing'>
        // 	errorBlock := aBlock
        //     ]

        //     extendedLanguage [
        // 	<category: 'accessing'>
        // 	^extendedLanguage
        //     ]

        //     extendedLanguage: aBoolean [
        // 	<category: 'accessing'>
        // 	extendedLanguage := aBoolean
        //     ]

        //     flush [
        // 	<category: 'accessing'>

        //     ]

        //     getComments [
        // 	<category: 'accessing'>
        // 	| oldComments |
        // 	comments isEmpty ifTrue: [^nil].
        // 	oldComments := comments.
        // 	comments := OrderedCollection new: 1.
        // 	^oldComments
        //     ]

        //     ignoreComments [
        // 	<category: 'accessing'>
        // 	saveComments := false
        //     ]

        //     next [
        // 	<category: 'accessing'>
        // 	| token |
        // 	buffer reset.
        // 	tokenStart := stream position.
        // 	characterType == #eof ifTrue: [^RBToken start: tokenStart + 1].	"The EOF token should occur after the end of input"
        // 	token := self scanToken.
        // 	self stripSeparators.
        // 	^token
        //     ]

        //     nextPut: anObject [
        // 	"Provide an error notification that the receiver does not
        // 	 implement this message."

        // 	<category: 'accessing'>
        // 	self shouldNotImplement
        //     ]

        //     saveComments [
        // 	<category: 'accessing'>
        // 	saveComments := true
        //     ]

        //     scanToken [
        // 	"fast-n-ugly. Don't write stuff like this. Has been found to cause cancer in laboratory rats. Basically a
        // 	 case statement. Didn't use Dictionary because lookup is pretty slow."

        // 	<category: 'accessing'>
        // 	characterType == #alphabetic ifTrue: [^self scanIdentifierOrKeyword].
        // 	characterType == #digit ifTrue: [^self scanNumber].
        // 	characterType == #binary ifTrue: [^self scanBinary: RBBinarySelectorToken].
        // 	characterType == #special ifTrue: [^self scanSpecialCharacter].
        // 	currentCharacter == $' ifTrue: [^self scanLiteralString].
        // 	currentCharacter == $# ifTrue: [^self scanLiteral].
        // 	currentCharacter == $$ ifTrue: [^self scanLiteralCharacter].
        // 	(extendedLanguage and: [currentCharacter == PatternVariableCharacter])
        // 	    ifTrue: [^self scanPatternVariable].
        // 	^self scannerError: 'Unknown character'
        //     ]

        //     position [
        // 	<category: 'accessing'>
        // 	^stream position
        //     ]

        //     stream [
        // 	<category: 'accessing'>
        // 	^stream
        //     ]

        //     errorBlock [
        // 	<category: 'error handling'>
        // 	^errorBlock isNil ifTrue: [[:message :position | ]] ifFalse: [errorBlock]
        //     ]

        //     errorPosition [
        // 	<category: 'error handling'>
        // 	^stream position
        //     ]

        //     scannerError: aString [
        // 	"Evaluate the block. If it returns raise an error"

        // 	<category: 'error handling'>
        // 	self errorBlock value: aString value: self errorPosition.
        // 	self error: aString
        //     ]

        //     on: aStream [
        // 	<category: 'initialize-release'>
        // 	buffer := WriteStream on: (String new: 60).
        // 	stream := aStream.
        // 	classificationTable := self class classificationTable.
        // 	saveComments := true.
        // 	extendedLanguage := false.
        // 	comments := OrderedCollection new
        //     ]

        //     classify: aCharacter [
        // 	<category: 'private'>
        // 	| index |
        // 	aCharacter isNil ifTrue: [^nil].
        // 	index := aCharacter asInteger.
        // 	index == 0 ifTrue: [^#separator].
        // 	index > 255 ifTrue: [^nil].
        // 	^classificationTable at: index
        //     ]

        //     previousStepPosition [
        // 	<category: 'private'>
        // 	^characterType == #eof
        // 	    ifTrue: [stream position]
        // 	    ifFalse: [stream position - 1]
        //     ]

        //     step [
        // 	<category: 'private'>
        // 	stream atEnd
        // 	    ifTrue:
        // 		[characterType := #eof.
        // 		^currentCharacter := nil].
        // 	currentCharacter := stream next.
        // 	characterType := self classify: currentCharacter.
        // 	^currentCharacter
        //     ]

        //     isDigit: aChar base: base [
        // 	<category: 'private-scanning numbers'>
        // 	aChar isNil ifTrue: [^false].
        // 	base <= 10
        // 	    ifTrue:
        // 		[aChar isDigit ifFalse: [^false].
        // 		^aChar value - $0 value < base].
        // 	^aChar isUppercase
        // 	    ifTrue: [aChar value - $A value < (base - 10)]
        // 	    ifFalse: [aChar isDigit]
        //     ]

        //     digitValue: aChar [
        // 	<category: 'private-scanning numbers'>
        //         ^ aChar digitValue
        //     ]

        //     scanDigits: ch base: base [
        // 	<category: 'private-scanning numbers'>
        // 	| c num |
        // 	c := ch.
        // 	num := 0.

        // 	[[c == $_] whileTrue:
        // 		[self step.
        // 		c := currentCharacter].
        // 	c notNil and: [self isDigit: c base: base]]
        // 		whileTrue:
        // 		    [num := num * base + (self digitValue: c).
        // 		    self step.
        // 		    c := currentCharacter].
        // 	^num
        //     ]

        //     scanExtendedLiterals [
        // 	<category: 'private-scanning numbers'>
        // 	| token |
        // 	self step.
        // 	currentCharacter == $(
        // 	    ifTrue:
        // 		[self step.
        // 		^RBOptimizedToken start: tokenStart].
        // 	self scannerError: 'Expecting parentheses'
        //     ]

        //     scanFraction: ch num: num base: base return: aBlock [
        // 	<category: 'private-scanning numbers'>
        // 	| c scale result |
        // 	c := ch.
        // 	scale := 0.
        // 	result := num.

        // 	[[c == $_] whileTrue:
        // 		[self step.
        // 		c := currentCharacter].
        // 	c notNil and: [self isDigit: c base: base]]
        // 		whileTrue:
        // 		    [result := result * base + (self digitValue: c).
        // 		    self step.
        // 		    c := currentCharacter.
        // 		    scale := scale - 1].
        // 	aBlock value: result value: scale
        //     ]

        //     scanNumberValue [
        // 	<category: 'private-scanning numbers'>
        // 	| isNegative base exponent scale ch num |
        // 	isNegative := false.
        // 	exponent := nil.

        // 	currentCharacter == $-  ifTrue:
        // 			[isNegative := true.
        // 			self step	"skip '-'"].

        // 	"could be radix or base-10 mantissa"
        // 	num := self scanDigits: currentCharacter base: 10.
        // 	currentCharacter == $r
        // 	    ifTrue:
        // 		[base := num truncated.
        // 		self step	"skip over 'r'".
        // 		currentCharacter == $-
        // 		    ifTrue:
        // 			[isNegative := true.
        // 			self step	"skip '-'"].
        // 		(self isDigit: currentCharacter base: base)
        // 		    ifTrue: [num := self scanDigits: currentCharacter base: base]
        // 		    ifFalse: [self error: 'malformed number']]
        // 	    ifFalse: [base := 10].

        // 	"Here we've either
        // 	 a) parsed base, an 'r' and are sitting on the following character
        // 	 b) parsed the integer part of the mantissa, and are sitting on the char
        // 	 following it, or
        // 	 c) parsed nothing and are sitting on a - sign."
        // 	currentCharacter == $.
        // 	    ifTrue:
        // 		[(self isDigit: stream peek base: base)
        // 		    ifTrue:
        // 			[self step.
        // 			self
        // 			    scanFraction: currentCharacter
        // 			    num: num
        // 			    base: base
        // 			    return:
        // 				[:n :s |
        // 				num := n.
        // 				exponent := s]]].
        // 	isNegative ifTrue: [num := num negated].
        // 	currentCharacter == $s
        // 	    ifTrue:
        // 		[self step.
        // 		currentCharacter isNil ifTrue: [currentCharacter := Character space].
        // 		exponent isNil ifTrue: [exponent := 0].
        // 		currentCharacter isDigit
        // 		    ifTrue: [scale := self scanDigits: currentCharacter base: 10]
        // 		    ifFalse:
        // 			["Might sit on the beginning of an identifier such as 123stu,
        // 			 or on a ScaledDecimal literal lacking the scale such as 123s"
        // 			(currentCharacter == $_ or: [currentCharacter isLetter])
        // 			    ifTrue:
        // 				[stream skip: -1.
        // 				currentCharacter := $s]
        // 			    ifFalse: [scale := exponent negated]].
        // 		^num asScaledDecimal: exponent radix: base scale: scale].
        // 	currentCharacter == $e
        // 	    ifTrue: [num := num asFloatE]
        // 	    ifFalse:
        // 		[currentCharacter == $d
        // 		    ifTrue: [num := num asFloatD]
        // 		    ifFalse:
        // 			[currentCharacter == $q
        // 			    ifTrue: [num := num asFloatQ]
        // 			    ifFalse:
        // 				[^exponent isNil
        // 				    ifTrue: [num]
        // 				    ifFalse: [num asFloat * (base raisedToInteger: exponent)]]]].
        // 	ch := currentCharacter.
        // 	self step.
        // 	currentCharacter isNil ifTrue: [currentCharacter := Character space].
        // 	(currentCharacter == $_ or: [currentCharacter isLetter])
        // 	    ifTrue:
        // 		[stream skip: -1.
        // 		currentCharacter := ch].
        // 	exponent isNil ifTrue: [exponent := 0].
        // 	currentCharacter == $-
        // 	    ifTrue:
        // 		[self step.
        // 		exponent := exponent - (self scanDigits: currentCharacter base: 10)]
        // 	    ifFalse:
        // 		[currentCharacter isDigit
        // 		    ifTrue: [exponent := exponent + (self scanDigits: currentCharacter base: 10)]].
        // 	^num * (base raisedToInteger: exponent)
        //     ]

        //     scanAnySymbol [
        // 	<category: 'private-scanning'>
        // 	characterType == #alphabetic ifTrue: [^self scanSymbol].
        // 	characterType == #binary ifTrue: [^self scanBinary: RBLiteralToken].
        // 	^RBToken new
        //     ]

        //     scanBinary: aClass [
        // 	"This doesn't parse according to the ANSI draft. It only parses 1 or 2 letter binary tokens."

        // 	<category: 'private-scanning'>
        // 	| val |
        // 	buffer nextPut: currentCharacter.
        // 	self step.
        // 	(characterType == #binary and: [currentCharacter ~~ $-])
        // 	    ifTrue:
        // 		[buffer nextPut: currentCharacter.
        // 		self step].
        // 	val := buffer contents.
        // 	val := val asSymbol.
        // 	^aClass value: val start: tokenStart
        //     ]

        //     scanByteArray [
        // 	<category: 'private-scanning'>
        // 	| byteStream number |
        // 	byteStream := WriteStream on: (ByteArray new: 100).
        // 	self step.

        // 	[self stripSeparators.
        // 	characterType == #digit] whileTrue:
        // 		    [number := self scanNumber value.
        // 		    (number isInteger and: [number between: 0 and: 255])
        // 			ifFalse: [self scannerError: 'Expecting 8-bit integer'].
        // 		    byteStream nextPut: number].
        // 	currentCharacter == $] ifFalse: [self scannerError: ''']'' expected'].
        // 	self step.	"]"
        // 	^RBLiteralToken
        // 	    value: byteStream contents
        // 	    start: tokenStart
        // 	    stop: self previousStepPosition
        //     ]

        //     scanIdentifierOrKeyword [
        // 	<category: 'private-scanning'>
        // 	| tokenType token |
        // 	currentCharacter == $_ ifTrue: [^self scanAssignment].
        // 	self scanName.
        // 	token := self scanNamespaceName.
        // 	token isNil
        // 	    ifTrue:
        // 		[tokenType := (currentCharacter == $: and: [stream peek ~~ $=])
        // 			    ifTrue:
        // 				[buffer nextPut: currentCharacter.
        // 				self step.	":"
        // 				RBKeywordToken]
        // 			    ifFalse: [RBIdentifierToken].
        // 		token := tokenType value: buffer contents start: tokenStart].
        // 	^token
        //     ]

        //     scanNamespaceName [
        // 	<category: 'private-scanning'>
        // 	| token |
        // 	currentCharacter == $.
        // 	    ifTrue:
        // 		[(stream atEnd or: [(self classify: stream peek) ~~ #alphabetic])
        // 		    ifTrue: [^nil]]
        // 	    ifFalse:
        // 		[(currentCharacter == $: and: [stream peek == $:]) ifFalse: [^nil].
        // 		self step].
        // 	buffer nextPut: $..
        // 	self step.
        // 	self scanName.
        // 	token := self scanNamespaceName.
        // 	token isNil
        // 	    ifTrue: [token := RBIdentifierToken value: buffer contents start: tokenStart].
        // 	^token
        //     ]

        //     scanLiteral [
        // 	<category: 'private-scanning'>
        // 	self step.
        // 	self stripSeparators.
        // 	characterType == #alphabetic ifTrue: [^self scanSymbol].
        // 	characterType == #binary
        // 	    ifTrue: [^(self scanBinary: RBLiteralToken) stop: self previousStepPosition].
        // 	currentCharacter == $' ifTrue: [^self scanStringSymbol].
        // 	currentCharacter == $( ifTrue: [^self scanLiteralArray].
        // 	currentCharacter == $[ ifTrue: [^self scanByteArray].
        // 	currentCharacter == ${ ifTrue: [^self scanQualifier].
        // 	currentCharacter == $# ifTrue: [^self scanExtendedLiterals].
        // 	self scannerError: 'Expecting a literal type'
        //     ]

        //     scanLiteralArray [
        // 	<category: 'private-scanning'>
        // 	| arrayStream start |
        // 	arrayStream := WriteStream on: (Array new: 10).
        // 	self step.
        // 	start := tokenStart.

        // 	[self stripSeparators.
        // 	tokenStart := stream position.
        // 	currentCharacter == $)]
        // 		whileFalse:
        // 		    [arrayStream nextPut: self scanLiteralArrayParts.
        // 		    buffer reset].
        // 	self step.
        // 	^RBLiteralToken
        // 	    value: arrayStream contents
        // 	    start: start
        // 	    stop: self previousStepPosition
        //     ]

        //     scanLiteralArrayParts [
        // 	<category: 'private-scanning'>
        // 	currentCharacter == $# ifTrue: [^self scanLiteral].
        // 	characterType == #alphabetic
        // 	    ifTrue:
        // 		[| token value |
        // 		token := self scanSymbol.
        // 		value := token value.
        // 		value == #nil ifTrue: [token value: nil].
        // 		value == #true ifTrue: [token value: true].
        // 		value == #false ifTrue: [token value: false].
        // 		^token].
        // 	(characterType == #digit
        // 	    or: [currentCharacter == $- and: [(self classify: stream peek) == #digit]])
        // 		ifTrue: [^self scanNumber].
        // 	characterType == #binary
        // 	    ifTrue: [^(self scanBinary: RBLiteralToken) stop: self previousStepPosition].
        // 	currentCharacter == $' ifTrue: [^self scanLiteralString].
        // 	currentCharacter == $$ ifTrue: [^self scanLiteralCharacter].
        // 	currentCharacter == $( ifTrue: [^self scanLiteralArray].
        // 	currentCharacter == $[ ifTrue: [^self scanByteArray].
        // 	^self scannerError: 'Unknown character in literal array'
        //     ]

        //     scanLiteralCharacter [
        // 	<category: 'private-scanning'>
        // 	| token value char tokenStop |
        // 	self step.	"$"
        // 	tokenStop := stream position.
        // 	char := currentCharacter.
        // 	self step.	"char"
        // 	char = $<
        // 	    ifTrue:
        // 		[self stripSeparators.
        // 		characterType == #digit
        // 		    ifTrue:
        // 			[value := self scanNumberValue.
        // 			(value isInteger and: [value between: 0 and: 1114111])
        // 			    ifFalse: [^self scannerError: 'Integer between 0 and 16r10FFFF expected'].
        // 			char := Character codePoint: value.
        // 			self stripSeparators.
        // 			tokenStop := stream position.
        // 			currentCharacter = $>
        // 			    ifTrue: [self step]
        // 			    ifFalse: [^self scannerError: '''>'' expected']]].
        // 	^RBLiteralToken
        // 	    value: char
        // 	    start: tokenStart
        // 	    stop: tokenStop
        //     ]

        //     scanLiteralString [
        // 	<category: 'private-scanning'>
        // 	self step.

        // 	[currentCharacter isNil
        // 	    ifTrue: [self scannerError: 'Unmatched '' in string literal.'].
        // 	currentCharacter == $' and: [self step ~~ $']]
        // 		whileFalse:
        // 		    [buffer nextPut: currentCharacter.
        // 		    self step].
        // 	^RBLiteralToken
        // 	    value: buffer contents
        // 	    start: tokenStart
        // 	    stop: self previousStepPosition
        //     ]

        //     scanPatternVariable [
        // 	<category: 'private-scanning'>
        // 	buffer nextPut: currentCharacter.
        // 	self step.
        // 	currentCharacter == ${
        // 	    ifTrue:
        // 		[self step.
        // 		^RBPatternBlockToken value: '`{' start: tokenStart].
        // 	[characterType == #alphabetic] whileFalse:
        // 		[characterType == #eof
        // 		    ifTrue: [self scannerError: 'Pattern variable expected'].
        // 		buffer nextPut: currentCharacter.
        // 		self step].
        // 	^self scanIdentifierOrKeyword
        //     ]

        //     scanName [
        // 	<category: 'private-scanning'>
        // 	[characterType == #alphabetic or: [characterType == #digit]] whileTrue:
        // 		[buffer nextPut: currentCharacter.
        // 		self step]
        //     ]

        //     scanNumber [
        //         | stop val string |
        // 	<category: 'private-scanning'>
        //         val := self scanNumberValue.
        //         stop := self previousStepPosition.

        //         "Get the parsed source"
        //         string := stream copyFrom: tokenStart - 1 to: stop - 1.

        // 	^RBNumberLiteralToken
        //             value: val
        //             start: tokenStart
        //             stop: stop
        //             source: string
        //     ]

        //     scanQualifier [
        // 	<category: 'private-scanning'>
        // 	| nameStream |
        // 	self step.	"{"
        // 	nameStream := WriteStream on: (String new: 10).
        // 	[currentCharacter == $}] whileFalse:
        // 		[nameStream nextPut: currentCharacter.
        // 		self step].
        // 	self step.	"}"
        // 	^RBBindingToken
        // 	    value: nameStream contents
        // 	    start: tokenStart
        // 	    stop: self previousStepPosition
        //     ]

        //     scanAssignment [
        // 	<category: 'private-scanning'>
        // 	self step.
        // 	^RBAssignmentToken start: tokenStart
        //     ]

        //     scanSpecialCharacter [
        // 	<category: 'private-scanning'>
        // 	| character |
        // 	currentCharacter == $:
        // 	    ifTrue:
        // 		[self step.
        // 		^currentCharacter == $=
        // 		    ifTrue: [self scanAssignment]
        // 		    ifFalse: [RBSpecialCharacterToken value: $: start: tokenStart]].
        // 	character := currentCharacter.
        // 	self step.
        // 	^RBSpecialCharacterToken value: character start: tokenStart
        //     ]

        //     scanStringSymbol [
        // 	<category: 'private-scanning'>
        // 	| literalToken |
        // 	literalToken := self scanLiteralString.
        // 	literalToken value: literalToken value asSymbol.
        // 	^literalToken
        //     ]

        //     scanSymbol [
        // 	<category: 'private-scanning'>
        // 	| lastPosition hasColon value startPosition |
        // 	hasColon := false.
        // 	startPosition := lastPosition := stream position.
        // 	[characterType == #alphabetic] whileTrue:
        // 		[self scanName.
        // 		currentCharacter == $:
        // 		    ifTrue:
        // 			[buffer nextPut: $:.
        // 			hasColon := true.
        // 			lastPosition := stream position.
        // 			self step]].
        // 	value := buffer contents.
        // 	(hasColon and: [value last ~~ $:])
        // 	    ifTrue:
        // 		[stream position: lastPosition.
        // 		self step.
        // 		value := value copyFrom: 1 to: lastPosition - startPosition + 1].
        // 	^RBLiteralToken
        // 	    value: value asSymbol
        // 	    start: tokenStart
        // 	    stop: self previousStepPosition
        //     ]

        //     stripComment [
        // 	<category: 'private-scanning'>
        // 	| start stop |
        // 	start := stream position.
        // 	[self step == $"] whileFalse:
        // 		[characterType == #eof
        // 		    ifTrue: [self scannerError: 'Unmatched " in comment.']].
        // 	stop := stream position.
        // 	self step.
        // 	saveComments ifFalse: [^self].
        // 	comments add: (start to: stop)
        //     ]

        //     stripSeparators [
        // 	<category: 'private-scanning'>

        // 	[[characterType == #separator] whileTrue: [self step].
        // 	currentCharacter == $"]
        // 		whileTrue: [self stripComment]
        //     ]

        //     atEnd [
        // 	<category: 'testing'>
        // 	^characterType == #eof
        //     ]

        //     isReadable [
        // 	<category: 'testing'>
        // 	^true
        //     ]

        //     isWritable [
        // 	<category: 'testing'>
        // 	^false
        //     ]
        // ]

        //

        // RBParser subclass: RBBracketedMethodParser [

        //     <category: 'Refactory-Parser'>
        //     <comment: 'A subclass of RBParser that discards a pair of brackets around
        // methods.'>

        //     skipToken: tokenValue [
        //         currentToken isValue ifFalse: [^false].
        //         (currentToken value = tokenValue)
        //             ifTrue: [self step. ^true]
        //             ifFalse: [^false]
        //     ]

        //     skipExpectedToken: tokenValue [
        //         (self skipToken: tokenValue)
        //             ifFalse: [self parserError: ('expected ' , tokenValue asSymbol)]
        //     ]

        //     parseMethodInto: methodNode [
        //         <category: 'private-parsing'>
        //         self skipExpectedToken: $[.
        //        super parseMethodInto: methodNode.
        //         self skipExpectedToken: $].
        //         ^methodNode
        //     ]
        // ]

        //
        // Eval [
        //     RBScanner initialize
        // ]
    }
}
