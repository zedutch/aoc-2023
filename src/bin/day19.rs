#![allow(unused)]

use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("./day19.in");
    let result = part2(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut input_parts = input.trim().split("\n\n");
    let mut workflow = parse_workflow(input_parts.next().unwrap());
    let parts = parse_parts(input_parts.next().unwrap());
    simplify_workflow(&mut workflow);
    parts
        .iter()
        .filter(|p| run_workflow(p, &workflow, "in"))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

fn part2(input: &str) -> u64 {
    let workflow = parse_workflow(input);
    let tree = workflow_as_tree(workflow, "in");
    let mut intervals = flatten_tree(tree);
    intervals.retain(|int| int.accept == Some(true));
    intervals
        .iter()
        .map(|int| {
            let x = int.x_max as u64 - int.x_min as u64 - 1;
            let m = int.m_max as u64 - int.m_min as u64 - 1;
            let a = int.a_max as u64 - int.a_min as u64 - 1;
            let s = int.s_max as u64 - int.s_min as u64 - 1;
            x * m * a * s
        })
        .sum()
}

fn parse_workflow(input: &str) -> Workflow {
    let mut workflow = HashMap::new();
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            println!("Workflow parsed");
            break;
        }
        let mut it = line.split('{');
        let name = it.next().expect("Invalid rule line: {line}");
        let rule_line = it.next().unwrap().split('}').next().unwrap();
        let mut rules = Vec::new();
        // println!("==================");
        // println!("Parsing rule '{name}'");
        for r in rule_line.split(',') {
            // println!("Parsing: {r}");
            let mut result = parse_rule_result(r);
            let rule = if r.contains(':') {
                // (field)(<|>)(value):(result)
                let pattern = Regex::new(r"(\w)(<|>)(\d+):(\w*)").unwrap();
                let (_, [field, op, value, res]) = pattern.captures(r).unwrap().extract();
                result = parse_rule_result(res);
                let value = value.parse::<u32>().expect("Invalid value: {value}");
                let field = match field {
                    "x" => Field::X,
                    "m" => Field::M,
                    "a" => Field::A,
                    "s" => Field::S,
                    _ => panic!("Field not supported: {field}"),
                };
                match op {
                    ">" => Rule::GT {
                        field,
                        value,
                        then: result,
                    },
                    "<" => Rule::LT {
                        field,
                        value,
                        then: result,
                    },
                    _ => panic!("Operator not supported: {op}"),
                }
            } else {
                Rule::Result(result)
            };
            rules.push(rule);
            // println!("\t {rule:?}")
        }
        workflow.insert(name.to_string(), rules);
    }
    workflow
}

fn parse_rule_result(r: &str) -> RuleResult {
    match r {
        "A" => RuleResult::Accept,
        "R" => RuleResult::Reject,
        name => RuleResult::Goto(name.to_string()),
    }
}

fn simplify_workflow(workflow: &mut Workflow) {
    let mut todo = Vec::new();
    loop {
        for (name, rules) in workflow.iter() {
            let first_result = get_result(rules.first().unwrap());
            if rules.iter().all(|rule| *first_result == *get_result(rule)) {
                // println!("Should simplify: {name} => {rules:?}");
                todo.push((name.clone(), first_result.clone()));
            }
        }
        if todo.is_empty() {
            println!("Simplification done");
            return;
        }
        for (to_remove, replace_result) in todo.drain(0..) {
            // println!("Replacing rule {to_remove} with {replace_result:?}");
            workflow.remove(&to_remove);
            for (_, rules) in workflow.iter_mut() {
                for rule in rules.iter_mut() {
                    match rule {
                        Rule::LT { then, .. } | Rule::GT { then, .. } | Rule::Result(then)
                            if *then == RuleResult::Goto(to_remove.clone()) =>
                        {
                            *then = replace_result.clone();
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn get_result(rule: &Rule) -> &RuleResult {
    match rule {
        Rule::LT { then, .. } => then,
        Rule::GT { then, .. } => then,
        Rule::Result(res) => res,
    }
}

fn parse_parts(input: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            println!("Parts parsed");
            break;
        }
        let pattern = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        let (_, [x, m, a, s]) = pattern.captures(line).unwrap().extract();
        let x = x.parse::<u32>().expect("Invalid x: {x}");
        let m = m.parse::<u32>().expect("Invalid m: {m}");
        let a = a.parse::<u32>().expect("Invalid a: {a}");
        let s = s.parse::<u32>().expect("Invalid s: {s}");
        parts.push(Part { x, m, a, s });
    }
    parts
}

fn run_workflow(part: &Part, workflow: &Workflow, start: impl ToString) -> bool {
    let start = start.to_string();
    let mut rules = workflow.get(&start).unwrap();
    loop {
        for rule in rules {
            match apply_rule(part, rule) {
                Some(RuleResult::Goto(new_rule)) => {
                    rules = workflow.get(new_rule).unwrap();
                    break;
                }
                Some(RuleResult::Accept) => return true,
                Some(RuleResult::Reject) => return false,
                Some(RuleResult::Tree(..)) => {
                    panic!("Running tree rules is not supported")
                }
                None => continue,
            }
        }
    }
}

fn apply_rule<'a>(part: &'a Part, rule: &'a Rule) -> Option<&'a RuleResult> {
    match rule {
        Rule::LT { field, value, then } => match field {
            Field::X => (part.x < *value).then_some(then),
            Field::M => (part.m < *value).then_some(then),
            Field::A => (part.a < *value).then_some(then),
            Field::S => (part.s < *value).then_some(then),
        },
        Rule::GT { field, value, then } => match field {
            Field::X => (part.x > *value).then_some(then),
            Field::M => (part.m > *value).then_some(then),
            Field::A => (part.a > *value).then_some(then),
            Field::S => (part.s > *value).then_some(then),
        },
        Rule::Result(a) => Some(a),
    }
}

fn workflow_as_tree(mut workflow: Workflow, start: impl ToString) -> Box<TreeNode> {
    let start = start.to_string();
    let mut todo = Vec::new();
    loop {
        for (name, rules) in workflow.iter() {
            let has_gotos = rules.iter().any(|rule| {
                matches!(
                    rule,
                    Rule::LT {
                        then: RuleResult::Goto(_),
                        ..
                    } | Rule::GT {
                        then: RuleResult::Goto(_),
                        ..
                    } | Rule::Result(RuleResult::Goto(_))
                )
            });
            if !has_gotos {
                let tree = build_tree(rules);
                if name == &start {
                    println!("Workflow converted to tree");
                    return tree;
                }
                todo.push((name.to_owned(), tree));
            }
        }
        if todo.is_empty() {
            panic!("Something went wrong during parsing");
        }
        for (to_remove, tree) in todo.drain(0..) {
            println!("Replacing rule \"{to_remove}\"...");
            workflow.remove(&to_remove);
            for (_, rules) in workflow.iter_mut() {
                for rule in rules.iter_mut() {
                    match rule {
                        Rule::LT { then, .. } | Rule::GT { then, .. } | Rule::Result(then)
                            if *then == RuleResult::Goto(to_remove.clone()) =>
                        {
                            *then = RuleResult::Tree(tree.clone());
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn build_tree(rules: &[Rule]) -> Box<TreeNode> {
    let last_rule = rules.last().unwrap();
    let mut previous = tree_from_terminal_rule(last_rule);
    for rule in rules.iter().rev().skip(1) {
        match rule {
            Rule::LT { field, value, then } => match then {
                RuleResult::Accept | RuleResult::Reject => {
                    previous = Box::new(TreeNode {
                        is_accept: false,
                        is_reject: false,
                        expr: Some(ExprNode {
                            field: *field,
                            lt: true,
                            value: *value,
                            ok: tree_from_terminal_rule(&Rule::Result(then.clone())),
                            not_ok: previous,
                        }),
                    })
                }
                RuleResult::Tree(subtree) => {
                    previous = Box::new(TreeNode {
                        is_accept: false,
                        is_reject: false,
                        expr: Some(ExprNode {
                            field: *field,
                            lt: true,
                            value: *value,
                            ok: subtree.clone(),
                            not_ok: previous,
                        }),
                    })
                }
                _ => panic!("Cannot build tree for rule: {rule:?}"),
            },
            Rule::GT { field, value, then } => match then {
                RuleResult::Accept | RuleResult::Reject => {
                    previous = Box::new(TreeNode {
                        is_accept: false,
                        is_reject: false,
                        expr: Some(ExprNode {
                            field: *field,
                            lt: false,
                            value: *value,
                            ok: tree_from_terminal_rule(&Rule::Result(then.clone())),
                            not_ok: previous,
                        }),
                    })
                }
                RuleResult::Tree(subtree) => {
                    previous = Box::new(TreeNode {
                        is_accept: false,
                        is_reject: false,
                        expr: Some(ExprNode {
                            field: *field,
                            lt: false,
                            value: *value,
                            ok: subtree.clone(),
                            not_ok: previous,
                        }),
                    })
                }
                _ => panic!("Cannot build tree for rule: {rule:?}"),
            },
            _ => panic!("Cannot build tree for rule: {rule:?}"),
        }
    }
    previous
}

fn tree_from_terminal_rule(result: &Rule) -> Box<TreeNode> {
    match result {
        Rule::Result(RuleResult::Accept) => Box::new(TreeNode {
            is_accept: true,
            is_reject: false,
            expr: None,
        }),
        Rule::Result(RuleResult::Reject) => Box::new(TreeNode {
            is_accept: false,
            is_reject: true,
            expr: None,
        }),
        Rule::Result(RuleResult::Tree(subtree)) => subtree.clone(),
        _ => panic!("Result not convertible to tree: {result:?}"),
    }
}

fn flatten_tree(tree: Box<TreeNode>) -> Vec<Interval> {
    let interval = Interval::default();
    let mut intervals = Vec::new();
    evaluate_tree_node(tree, &interval, &mut intervals);
    // Remove all "impossible" intervals
    intervals.retain(|int| {
        int.x_min < int.x_max
            && int.m_min < int.m_max
            && int.a_min < int.a_max
            && int.s_min < int.s_max
    });
    intervals
}

fn evaluate_tree_node(node: Box<TreeNode>, interval: &Interval, intervals: &mut Vec<Interval>) {
    let mut interval = interval.to_owned();
    if node.is_accept {
        interval.accept = Some(true);
        intervals.push(interval);
    } else if node.is_reject {
        interval.accept = Some(false);
        intervals.push(interval);
    } else if let Some(expr) = node.expr {
        let [int1, int2] = split_interval(interval, expr.field, expr.lt, expr.value);
        evaluate_tree_node(expr.ok, &int1, intervals);
        evaluate_tree_node(expr.not_ok, &int2, intervals);
    } else {
        panic!("Invalid tree node: {node:?}");
    }
}

fn split_interval(interval: Interval, field: Field, lt: bool, value: u32) -> [Interval; 2] {
    // println!("-------------------");
    // println!("Expr: {:?} {} {}", field, if lt { '<' } else { '>' }, value);
    // println!("Input: {interval:?}");
    let mut interval1 = interval;
    let mut interval2 = interval;
    let delta: i32 = if lt {
        -1
    } else {
        1
    };
    update_interval(&mut interval1, field, lt, value);
    update_interval(&mut interval2, field, !lt, ((value as i32) + delta) as u32);
    // println!("First: {interval1:?}");
    // println!("Second: {interval2:?}");
    // println!("-------------------");
    [interval1, interval2]
}

fn update_interval(interval: &mut Interval, field: Field, lt: bool, value: u32) {
    match (field, lt) {
        (Field::X, true) => interval.x_max = interval.x_max.min(value),
        (Field::X, false) => interval.x_min = interval.x_min.max(value),
        (Field::M, true) => interval.m_max = interval.m_max.min(value),
        (Field::M, false) => interval.m_min = interval.m_min.max(value),
        (Field::A, true) => interval.a_max = interval.a_max.min(value),
        (Field::A, false) => interval.a_min = interval.a_min.max(value),
        (Field::S, true) => interval.s_max = interval.s_max.min(value),
        (Field::S, false) => interval.s_min = interval.s_min.max(value),
    }
}

type Workflow = HashMap<String, Vec<Rule>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    LT {
        field: Field,
        value: u32,
        then: RuleResult,
    },
    GT {
        field: Field,
        value: u32,
        then: RuleResult,
    },
    Result(RuleResult),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleResult {
    Goto(String),
    Accept,
    Reject,
    // part 2:
    Tree(Box<TreeNode>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeNode {
    is_accept: bool,
    is_reject: bool,
    expr: Option<ExprNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExprNode {
    field: Field,
    lt: bool,
    value: u32,
    ok: Box<TreeNode>,
    not_ok: Box<TreeNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    x_min: u32,
    x_max: u32,
    m_min: u32,
    m_max: u32,
    a_min: u32,
    a_max: u32,
    s_min: u32,
    s_max: u32,
    accept: Option<bool>,
}

impl Default for Interval {
    fn default() -> Self {
        // Off by one wrt puzzle because I'm working entirely exclusively
        Self {
            x_min: 0,
            x_max: 4001,
            m_min: 0,
            m_max: 4001,
            a_min: 0,
            a_max: 4001,
            s_min: 0,
            s_max: 4001,
            accept: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day19p1_all() {
        let input = r#"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "#;
        assert_eq!(part1(input), 19114);
    }

    #[test]
    pub fn test_day19p1_rules() {
        let input = r#"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            ignoreme
        "#;
        let workflow = parse_workflow(input);
        assert_eq!(workflow.len(), 11);
    }

    #[test]
    pub fn test_day19p1_simplify() {
        let input = r#"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            ignoreme
        "#;
        // will be simplified: gd, lnx, qs
        let mut workflow = parse_workflow(input);
        simplify_workflow(&mut workflow);
        assert_eq!(workflow.len(), 8);
    }

    #[test]
    pub fn test_day19p1_parts() {
        let input = r#"
            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "#;
        assert_eq!(
            parse_parts(input),
            vec![
                Part {
                    x: 787,
                    m: 2655,
                    a: 1222,
                    s: 2876,
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496,
                },
                Part {
                    x: 2036,
                    m: 264,
                    a: 79,
                    s: 2244,
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291,
                },
                Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013,
                },
            ]
        );
    }

    #[test]
    pub fn test_day19p2_all() {
        let input = r#"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "#;
        assert_eq!(part2(input), 167409079868000);
    }

    #[test]
    pub fn test_day19p2_flatten() {
        let input = r#"
            in{a>716:px,qqz}
            px{a<1342:R,A}
            qqz{a>3333:R,A}
        "#;
        let workflow = parse_workflow(input);
        let tree = workflow_as_tree(workflow, "in");
        let intervals = flatten_tree(tree);
        assert_eq!(intervals.len(), 3);
    }
}
