use super::Obj;
use cate::Result;
fn unwrap_list_of_lists(args: Vec<Obj>) -> Result<Vec<Vector<Obj>>> {
    args.iter().map(unwrap_single_list).collect()
}

fn unwrap_single_list(exp: &Obj) -> Result<Vector<Obj>> {
    match exp {
        Obj::Vec(lst) => Ok(lst.unwrap()),
        _ => stop!(Type => "expected a list"),
    }
}
