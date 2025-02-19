use std::rc::Rc;
use std::collections::HashMap;

use ino::*;


fn main() {
    let emp = Rc::new(Sort::Prim(1));
    let dept = Rc::new(Sort::Prim(2));
    let mgr = Func{name: "mgr".to_string(), sort: Rc::new(Sort::Func(emp.clone(), emp.clone())) };
    let wrk = Func{name: "wrk".to_string(), sort: Rc::new(Sort::Func(emp.clone(), dept.clone())) };
    let sec = Func{name: "sec".to_string(), sort: Rc::new(Sort::Func(dept.clone(), emp.clone())) };

    let str = Rc::new(Sort::Prim(3));
    let int = Rc::new(Sort::Prim(4));
    let bool = Rc::new(Sort::Prim(5));
    let last = Func{name: "last".to_string(), sort: Rc::new(Sort::Func(emp.clone(), str.clone())) };
    let name = Func{name: "name".to_string(), sort: Rc::new(Sort::Func(dept.clone(), str.clone())) };
    let sal = Func{name: "sal".to_string(), sort: Rc::new(Sort::Func(emp.clone(), int.clone())) };
    let tru = Func{name: "true".to_string(), sort: bool.clone() };
    let le = Func{name: "<=".to_string(), sort: Rc::new(Sort::Func(Rc::new(Sort::Mult(int.clone(), int.clone())), bool.clone())) };

    let mut context1 = HashMap::new();
    context1.insert("e".to_string(), emp.clone());
    let rule1 = Equation {
        context: Context(context1), 
        left: TermInner::Fun(mgr.clone(), vec![Rc::new(TermInner::Fun(mgr.clone(), vec![Rc::new(TermInner::Var("e".to_string()))]))]),
        right: TermInner::Fun(mgr.clone(), vec![Rc::new(TermInner::Var("e".to_string()))]),
    };

    let mut context2 = HashMap::new();
    context2.insert("e".to_string(), emp.clone());
    let rule2 = Equation {
        context: Context(context2), 
        left: TermInner::Fun(wrk.clone(), vec![Rc::new(TermInner::Fun(mgr.clone(), vec![Rc::new(TermInner::Var("e".to_string()))]))]),
        right: TermInner::Fun(wrk.clone(), vec![Rc::new(TermInner::Var("e".to_string()))]),
    };

    let mut context3 = HashMap::new();
    context3.insert("d".to_string(), dept.clone());
    let rule3 = Equation {
        context: Context(context3),
        left: TermInner::Fun(wrk.clone(), vec![Rc::new(TermInner::Fun(sec.clone(), vec![Rc::new(TermInner::Var("d".to_string()))]))]),
        right: TermInner::Var("d".to_string()),
    };

    let mut context4 = HashMap::new();
    context4.insert("e".to_string(), emp.clone());
    let rule4 = Equation {
        context: Context(context4), 
        left: TermInner::Fun(wrk.clone(), vec![Rc::new(TermInner::Fun(mgr.clone(), vec![Rc::new(TermInner::Var("e".to_string()))]))]),
        right: TermInner::Fun(tru.clone(), vec![]),
    };

    let _s = Schema {
        sorts: vec![emp.clone(), dept.clone(), str.clone(), int.clone(), bool.clone()],
        funcs: vec![
            mgr, wrk, sec,
            last, name, sal,
            tru, le
        ],
        equations: vec![rule1, rule2, rule3, rule4],
    };
}


// fn main_old() {
    // let _zero = Zero;
    // let one = Rc::new(One);
    // let cats = Rc::new(Cats);

    // let _zero_schema = Rc::new((one.clone(), cats.clone()));
    // let one_schema = Rc::new((one.clone(), cats.clone()));
    // let _sets = Rc::new((one.clone(), cats.clone()));

    // let schema_s = Rc::new((one.clone(), cats.clone()));

    // let entity_emp = Rc::new((one_schema.clone(), schema_s.clone()));
    // let entity_dept = Rc::new((one_schema.clone(), schema_s.clone()));
    // let entity_one = Rc::new((one_schema.clone(), schema_s.clone()));
    // let type_str = Rc::new((one_schema.clone(), schema_s.clone()));
    // let type_int = Rc::new((one_schema.clone(), schema_s.clone()));
    // let type_bool = Rc::new((one_schema.clone(), schema_s.clone()));

    // let edge_id_dept = Rc::new((entity_dept.clone(), entity_dept.clone()));

    // let edge_mgr = Rc::new((entity_emp.clone(), entity_emp.clone()));
    // let edge_wrk = Rc::new((entity_emp.clone(), entity_dept.clone()));
    // let edge_sec = Rc::new((entity_dept.clone(), entity_emp.clone()));

    // let _attr_last = Rc::new((entity_emp.clone(), type_str.clone()));
    // let _attr_name = Rc::new((entity_dept.clone(), type_str.clone()));
    // let attr_sal = Rc::new((entity_emp.clone(), type_int.clone()));
    // let attr_true = Rc::new((entity_one.clone(), type_bool.clone()));

    // let _patheq_1 = Rc::new(((edge_mgr.clone(), [edge_mgr.clone()]), edge_mgr.clone()));
    // let _patheq_2 = Rc::new(((edge_mgr.clone(), [edge_wrk.clone()]), edge_wrk.clone()));
    // let _patheq_3 = Rc::new(((edge_sec.clone(), [edge_wrk.clone()]), edge_id_dept.clone()));

    // let _obseq_1 = Rc::new(([attr_sal.clone(), edge_wrk.clone()], attr_true.clone()));
// }