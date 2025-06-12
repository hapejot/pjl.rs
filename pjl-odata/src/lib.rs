use std::collections::{BTreeMap, HashMap};

use edm::value::Value;
use serde::Serialize;
mod parser {
    use santiago::grammar::Grammar;
    use santiago::lexer::LexerRules;
    use serde::Serialize;
    use tracing::trace;

    use super::{Condition, ConditionBag};

    #[derive(Debug, Clone, Serialize)]
    pub enum AST {
        Name(String),
        Eq,
        Str(String),
        Int(i64),
        Term(Vec<AST>),
        Operator(String),
    }
    impl AST {
        pub(crate) fn to_string(&self) -> String {
            match self {
                AST::Name(n) => n.clone(),
                AST::Eq => String::from("="),
                AST::Str(x) => x.clone(),
                AST::Int(x) => x.to_string(),
                AST::Term(_) => todo!(),
                AST::Operator(op) => op.clone(),
            }
        }
    }

    fn strip_first_and_last(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str()
    }

    pub fn grammar() -> Grammar<AST> {
        santiago::grammar!(
            "expr" => rules "name" "eq" "expr" => AST::Term;
            "expr" => rules "name" "ge" "expr" => AST::Term;
            "expr" => rules "expr" "and" "expr" => AST::Term;
            "expr" => rules "expr" "or" "expr" => AST::Term;
            "expr" => lexemes "INT" => |x| AST::Int(x[0].raw.parse::<i64>().unwrap());
            "expr" => lexemes "STR" => |x| {let value= x[0].raw.clone(); AST::Str(strip_first_and_last(value.as_str()).to_string())};
            "name" => lexemes "NAME"  => |x| AST::Name(x[0].raw.clone());
            "eq" => lexemes "EQ" => |_| AST::Eq;
            "ge" => lexemes "GE" => |x| AST::Operator(x[0].raw.clone());
            "and" => lexemes "AND" => |x| AST::Operator(x[0].raw.clone());
            "or" => lexemes "OR" => |x| AST::Operator(x[0].raw.clone());
        )
    }
    pub fn lexer_rules() -> LexerRules {
        santiago::lexer_rules!(
            // One more sequential digits from 0 to 9 will be mapped to an "INT"
            "DEFAULT" | "INT" = pattern r"-?[0-9]+";
            "DEFAULT" | "STR" = pattern r"'[^']*'";
            "DEFAULT" | "EQ" = string "eq";
            "DEFAULT" | "GE" = string "ge";
            "DEFAULT" | "AND" = string "and";
            "DEFAULT" | "OR" = string "or";
            "DEFAULT" | "NAME" = pattern r"[a-zA-Z0-9_]+";
             // A literal "+" will be mapped to "PLUS"
            // Whitespace " " will be skipped
            "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
        )
    }
    #[allow(unused)]
    fn translate(tree: &AST) -> ConditionBag {
        let mut r = ConditionBag::new();
        match tree {
            AST::Name(_) => todo!(),
            AST::Eq => todo!(),
            AST::Str(_) => todo!(),
            AST::Int(_) => todo!(),
            AST::Term(v) => match (&v[0], &v[1], &v[2]) {
                (AST::Name(n), AST::Eq, v) => r.add(
                    n.as_str(),
                    Condition {
                        op: super::Operator::Equals,
                        value: v.to_string(),
                    },
                ),
                (AST::Name(n), AST::Operator(op), AST::Str(x)) => r.add(
                    n.as_str(),
                    Condition {
                        op: super::Operator::Var(op.clone()),
                        value: x.clone(),
                    },
                ),
                (AST::Name(n), AST::Operator(op), AST::Int(x)) => r.add(
                    n.as_str(),
                    Condition {
                        op: super::Operator::Var(op.clone()),
                        value: format!("{x}"),
                    },
                ),
                (t1, AST::Operator(op), t2) => {
                    trace!("gen: {:?} - {:?} - {:?}", t1, op, t2);
                    let c1 = translate(t1);
                    let c2 = translate(t2);

                    r.merge(&c1);
                    r.merge(&c2);
                }
                _ => todo!("not impl: {:#?}", &v),
            },
            AST::Operator(_) => todo!(),
        }
        r
    }

    #[allow(unused)]
    pub fn parse_expression(x: &String) -> Result<ConditionBag, String> {
        let mut result = ConditionBag::new();
        let lexer_rules = lexer_rules();
        match santiago::lexer::lex(&lexer_rules, &x) {
            Ok(lexemes) => {
                let grammar = grammar();
                // println!("LEX: {:#?}", lexemes);
                match santiago::parser::parse(&grammar, &lexemes) {
                    Ok(parse_trees) => {
                        let tree = parse_trees[0].as_abstract_syntax_tree();
                        let r = translate(&tree);
                        result.merge(&r)
                        // println!("{:#?}", &result);
                    }
                    Err(e) => Err(format!("parser: {}", e))?,
                }
            }
            Err(e) => Err(format!("lexer: {}", e))?,
        }
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn parse_expression_tree(x: &String) -> Result<AST, String> {
        let lexer_rules = lexer_rules();
        match santiago::lexer::lex(&lexer_rules, &x) {
            Ok(lexemes) => {
                let grammar = grammar();
                // println!("LEX: {:#?}", lexemes);
                match santiago::parser::parse(&grammar, &lexemes) {
                    Ok(parse_trees) => {
                        let tree = parse_trees[0].as_abstract_syntax_tree();
                        Ok(tree)
                    }
                    Err(e) => Err(format!("parser: {}", e)),
                }
            }
            Err(e) => Err(format!("lexer: {}", e)),
        }
    }
}

pub trait DbSpecifics {
    fn start_field(&mut self, name: &str);
    fn end_field(&mut self);
    fn add_cond(&mut self, op: &str, value: &str);
    fn where_clause(&self) -> String;
    fn values(&self) -> Vec<Value>;
}

#[derive(Debug, Clone, Serialize)]
pub enum Operator {
    Equals,
    Var(String),
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "eq" => Operator::Equals,
            s => Operator::Var(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Condition {
    op: Operator,
    value: String,
}

pub type ConditionList = Vec<Condition>;

#[derive(Debug, Clone, Serialize)]
pub struct ConditionBag {
    fields: BTreeMap<String, ConditionList>,
}

impl ConditionBag {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }
    pub fn fields(&self) -> Vec<String> {
        self.fields.keys().map(|x| x.clone()).collect()
    }
    pub fn add(&mut self, field: &str, cond: Condition) {
        let field = field.to_lowercase();
        match self.fields.get_mut(&field) {
            Some(fs) => {
                fs.push(cond);
            }
            None => {
                let _ = self.fields.insert(field.to_string(), vec![cond]);
            }
        }
    }

    pub fn merge(&mut self, r: &ConditionBag) {
        for (k, v) in r.fields.iter() {
            match self.fields.get_mut(k) {
                Some(fs) => {
                    for c in v {
                        fs.push(c.clone())
                    }
                }
                None => {
                    let _ = self.fields.insert(k.clone(), v.clone());
                }
            }
        }
    }

    #[allow(unused)]
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    #[allow(unused)]
    pub fn get(&self, arg: &str) -> Vec<Condition> {
        if let Some(x) = self.fields.get(arg) {
            x.clone()
        } else {
            vec![]
        }
    }
}

#[allow(unused)]
type Conditions = Vec<(String, Vec<Condition>)>;

#[derive(Debug, Clone, Serialize)]
pub struct ODataQuery {
    filter: Option<String>,
    conditions: ConditionBag,
    filter_expr: Option<parser::AST>,
    top: Option<usize>,
    skip: Option<usize>,
    orderby: Option<String>,
    table: String,
}
impl ODataQuery {
    pub fn new_from(entity: &str, params: &HashMap<String, String>) -> Self {
        let mut r = ODataQuery {
            table: String::from(entity),
            filter: None,
            filter_expr: None,
            conditions: ConditionBag::new(),
            top: None,
            skip: None,
            orderby: None,
        };
        for (field, v) in params {
            r.add_condition(field, "eq", v);
        }
        if let Some(x) = params.get("$filter") {
            r.conditions = match parser::parse_expression(x) {
                Ok(expr) => expr,
                Err(err) => {
                    tracing::error!("filter expression {x} was not parsable. {err}");
                    ConditionBag::new()
                }
            };
            // r.filter_expr = Some(ast);

            r.filter = Some(x.clone());
        }
        if let Some(x) = params.get("$orderby") {
            r.orderby = Some(x.clone());
        }
        if let Some(x) = params.get("$top") {
            if let Ok(x) = x.parse::<usize>() {
                r.top = Some(x);
            }
        }
        if let Some(x) = params.get("$skip") {
            if let Ok(x) = x.parse::<usize>() {
                r.skip = Some(x);
            }
        }
        r
    }
    pub fn add_condition(&mut self, field: &str, operator: &str, value: &str) {
        let cond = Condition {
            op: operator.into(),
            value: value.into(),
        };
        self.conditions.add(field, cond);
    }
    pub fn get_fields_sql(&self) -> String {
        if self.conditions.field_count() > 0 {
            self.conditions.fields().join(",")
        } else {
            String::from("*")
        }
    }

    pub fn get_table(&self) -> String {
        self.table.clone()
    }

    pub fn get_where_sql(&self) -> String {
        let mut result = vec![];
        for (fld, cond) in self.conditions.fields.iter() {
            let mut field_result = vec![];
            for c in cond {
                let s = match &c.op {
                    Operator::Equals => format!("{} = '{}'", fld, c.value),
                    Operator::Var(v) => format!("{} {} {}", fld, map_op_to_sql(&v), c.value),
                };
                field_result.push(s);
            }
            if field_result.len() == 1 {
                result.push(field_result.last().unwrap().clone());
            } else {
                result.push(format!("({})", field_result.join(" or ")));
            }
        }
        result.join(" and ")
    }
    pub fn get_where_sql_specific<T: DbSpecifics>(&self, mut sqlspec: T) -> (String, Vec<Value>) {
        for (fld, cond) in self.conditions.fields.iter() {
            let mut field_result = vec![];
            sqlspec.start_field(fld);
            for c in cond {
                let s = match &c.op {
                    Operator::Equals => sqlspec.add_cond("=", &c.value),
                    Operator::Var(v) => sqlspec.add_cond(&v, &c.value),
                };
                field_result.push(s);
            }
            sqlspec.end_field();
            // if field_result.len() == 1 {
            //     result.push(field_result.last().unwrap().clone());
            // } else {
            //     result.push(format!("({})", field_result.join(" or ")));
            // }
        }
        // result.join(" and ")
        (sqlspec.where_clause(), sqlspec.values())
    }
    #[allow(unused)]
    pub fn conditions(&self) -> &ConditionBag {
        &self.conditions
    }

    pub fn orderby(&self) -> Option<&String> {
        self.orderby.as_ref()
    }

    pub fn skip(&self) -> Option<usize> {
        self.skip
    }

    pub fn top(&self) -> Option<usize> {
        self.top
    }
}

pub fn map_op_to_sql(op: &str) -> &str {
    match op {
        "ge" => ">=",
        x => todo!("operator {x} unknown and could not be mapped"),
    }
}
