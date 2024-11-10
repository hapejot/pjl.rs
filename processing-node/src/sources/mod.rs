use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::{cell::RefCell, collections::HashMap, io::Read, rc::Rc};

use crate::parser;
use crate::parser::scanner::chunks::ChunkedLexer;
use crate::{
    parser::{
        rbparser::{Node as STNode, Parser},
        scanner::Scanner,
    },
    sync::PacketStream,
    Message, MessageDispatch, Node, ObjectID, Value,
};
use clap::{Parser as ClapParser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    Plain,
    Chunks,
    GNU,
}

#[derive(Debug, Clone)]
pub struct STClass {
    name: String,
    parent: String,
    instance_vars: Vec<String>,
    class_vars: Vec<String>,
    pool_dicts: Vec<String>,
    category: String,
    comment: String,
}
pub mod browser;

impl STClass {
    pub fn new(parent: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: parent.to_string(),
            instance_vars: vec![],
            class_vars: vec![],
            pool_dicts: vec![],
            category: String::new(),
            comment: String::new(),
        }
    }

    pub fn add_instance_vars(&mut self, ns: Vec<String>) {
        for x in ns {
            self.instance_vars.push(x);
        }
    }

    pub fn set_comment(&mut self, comment: &str) {
        self.comment = String::from(comment);
    }

    fn set_category(&mut self, cat: &str) {
        self.category = String::from(cat);
    }
}

pub struct Sources<T> {
    chunks: Option<Rc<RefCell<ChunkedLexer<T>>>>,
    class: HashMap<String, STClass>,
}

impl<T> Sources<T>
where
    T: Read,
{
    pub fn eval(&mut self, n: &STNode) {
        match n {
            STNode::Identifier(_) => todo!(),
            STNode::Statements(vec) => {
                for x in vec {
                    self.eval(x);
                }
            }
            STNode::Expression { receiver, messages } => {
                let mut r = receiver;
                for x in messages {
                    self.execute_message(receiver, x);
                }
            }
            STNode::Message {
                selector,
                params,
                outer,
            } => todo!(),
            STNode::None => {}
            STNode::Assign { target, value } => todo!(),
            STNode::Symbol(_) => todo!(),
            STNode::String(s) => println!("{s}"),
            STNode::Literal(_) => todo!(),
            STNode::Method {
                selector,
                args,
                body,
            } => todo!(),
            STNode::Block { args, body } => todo!(),
            STNode::ArrayLiteral => todo!(),
        }
    }

    fn execute_message(&mut self, receiver: &STNode, x: &STNode) {
        match x {
            STNode::Message {
                selector,
                params,
                outer,
                ..
            } => {
                let mut tmp_receiver = None;
                let mut new_receiver = receiver;
                match selector.as_str() {
                    "subclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                        if let STNode::Identifier(parent) = new_receiver {
                            if let (STNode::Symbol(child),STNode::String(ivars),STNode::String(cvars),STNode::String(dicts),STNode::String(cat)) = (&params[0], &params[1], &params[2], &params[3], &params[4]) {
                                self.register_new_class(child, parent, ivars, cvars, dicts, cat);
                            }
                            else {
                                println!("!!!! {:?}", params);
                            }
                        }
                    },
                    "variableSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                        if let STNode::Identifier(parent) = new_receiver {
                            if let (STNode::Symbol(child),STNode::String(ivars),STNode::String(cvars),STNode::String(dicts),STNode::String(cat)) = (&params[0], &params[1], &params[2], &params[3], &params[4]) {
                                self.register_new_class(child, parent, ivars, cvars, dicts, cat);
                            }
                            else {
                                println!("!!!! {:?}", params);
                            }
                        }
                    }
                    "variableByteSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                        if let STNode::Identifier(parent) = new_receiver {
                            if let (STNode::Symbol(child),STNode::String(ivars),STNode::String(cvars),STNode::String(dicts),STNode::String(cat)) = (&params[0], &params[1], &params[2], &params[3], &params[4]) {
                                self.register_new_class(child, parent, ivars, cvars, dicts, cat);
                            }
                            else {
                                println!("!!!! {:?}", params);
                            }
                        }
                    }
                    "variableWordSubclass:instanceVariableNames:classVariableNames:poolDictionaries:category:" => {
                        if let STNode::Identifier(parent) = new_receiver {
                            if let (STNode::Symbol(child),STNode::String(ivars),STNode::String(cvars),STNode::String(dicts),STNode::String(cat)) = (&params[0], &params[1], &params[2], &params[3], &params[4]) {
                                self.register_new_class(child, parent, ivars, cvars, dicts, cat);
                            }
                            else {
                                println!("!!!! {:?}", params);
                            }
                        }
                    }
                    "instanceVariableNames:" => {
                        if let (STNode::Identifier(cls), STNode::String(names)) = (receiver, &params[0]) {
                            let ns = names.split(" ").map(|x|x.to_string()).collect::<Vec<_>>();
                            if let Some(c) = self.class.get_mut(cls){
                                c.add_instance_vars(ns);
                                new_receiver = &STNode::None;
                            }
                            else {
                                panic!("class {} not found", cls);
                            }
                        }
                    }
                    "methodsFor:" => {
                        let mut f = self.chunks.as_ref().unwrap().borrow_mut();
                        while let Some(c) = f.next_chunk() {
                            if c.is_empty() {
                                break;
                            }
                            let lexing_rules = parser::lexer();
                            match santiago::lexer::lex(&lexing_rules, c.text()) {
                                Ok(lexemes) => {
                                    let mut p = Parser::new(lexemes);
                                    let m = p.parse_method();
                                    // println!("::: {:#?}", m);
                                }
                                Err(x) => {
                                    println!("error in code: {}", c.text());
                                    todo!("{x}")
                                }
                            }
                        }
                    }
                    "initialize" => {}
                    "comment:" => {
                        if let (STNode::Identifier(cls), STNode::String(comment)) = (new_receiver, &params[0]) {
                            if let Some(c) = self.class.get_mut(cls){
                                c.set_comment(&comment);
                                new_receiver = &STNode::None;
                            }
                            else {
                                eprintln!("class {} not found", cls);
                            }
                        }
                }
                    "class" => {
                        if let STNode::Identifier(n) = new_receiver{
                            tmp_receiver = Some(STNode::Identifier(format!("$Meta{}", n)));
                            new_receiver = tmp_receiver.as_ref().unwrap();
                            }
                            else {
                                panic!("only expecting Identifier in class method -> {:?}", new_receiver);
                            }
                    }
                    _ => println!("{}", selector),
                }
                if let Some(o) = outer {
                    self.execute_message(new_receiver, o);
                }
            }
            _ => todo!(),
        }
    }

    fn register_new_class(
        &mut self,
        child: &String,
        parent: &String,
        ivars: &String,
        cvars: &String,
        dicts: &String,
        cat: &String,
    ) {
        if self.class.contains_key(child) {
            eprintln!("duplicate class {}", child);
        } else {
            let mut v = STClass::new(child, parent);
            v.set_category(cat);
            self.class.insert(child.clone(), v);
            let meta = format!("$Meta{}", child);
            let v = STClass::new("Meta", &meta);
            self.class.insert(meta.clone(), v);
        }
    }

    pub fn new() -> Self {
        Self {
            chunks: None,
            class: HashMap::new(),
        }
    }

    pub fn set_lexer(&mut self, lexer: ChunkedLexer<T>) -> Rc<RefCell<ChunkedLexer<T>>> {
        let r = Rc::new(RefCell::new(lexer));
        self.chunks = Some(r.clone());
        r
    }

    pub fn dump(&self) {
        println!("{} classes defined.", self.class.len());
        for (
            _,
            STClass {
                name,
                parent,
                comment,
                ..
            },
        ) in self.class.iter()
        {
            println!("{:30} {:30} {}", parent, name, comment);
        }
    }

    pub fn categories(&self) -> Vec<&str> {
        let mut r = HashSet::new();
        for x in self.class.values() {
            r.insert(&x.category);
        }
        r.iter().map(|x| x.as_str()).collect()
    }
}

pub fn parse(
    format: Format,
    dump_tokens: bool,
    path: &Path,
    src: &str,
) -> Result<Sources<File>, String> {
    let mut sources = Sources::new();
    match format {
        Format::Plain => {
            let mut dst = String::new();
            let mut f = File::open(path).unwrap();
            f.read_to_string(&mut dst).unwrap();
            let lexing_rules = super::parser::lexer();
            match santiago::lexer::lex(&lexing_rules, src) {
                Ok(lexemes) => {
                    if dump_tokens {
                        for x in lexemes.iter() {
                            println!("{:?}", x);
                        }
                        let mut parser = super::parser::rbparser::Parser::new(lexemes);
                    }
                }
                Err(err) => {
                    println!("{:?}", err.states_stack);
                    println!(
                        "{}:{}",
                        err.byte_index,
                        &src[err.byte_index..err.byte_index + 10]
                    );
                    // Err(format!("parse error: {err}"))
                }
            };
            // Ok(parser.parse_statements())
            Ok(sources)
        }
        Format::Chunks => {
            let f = ChunkedLexer::new(File::open(path).unwrap());
            let ff = sources.set_lexer(f);
            let mut x = { ff.borrow_mut().next_chunk() };
            while let Some(c) = &x {
                let lexing_rules = super::parser::lexer();
                match santiago::lexer::lex(&lexing_rules, c.text()) {
                    Ok(lexemes) => {
                        let mut p = Parser::new(lexemes);
                        let n = p.parse_statements();
                        sources.eval(&n);
                    }
                    _ => todo!(),
                }
                x = ff.borrow_mut().next_chunk();
            }
            // sources.dump();
            Ok(sources)
        }
        Format::GNU => todo!(),
    }
}
