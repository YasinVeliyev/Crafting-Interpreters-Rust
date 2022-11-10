macro_rules! generate_ast{

}

fn func(){
    let k =[
            "Block      : Rc<Vec<Rc<Stmt>>> statements",
            "Class      : Token name, Option<Rc<Expr>> superclass, Rc<Vec<Rc<Stmt>>> methods",
            "Break      : Token token",
            "Expression : Rc<Expr> expression",
            "Function   : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body",
            "If         : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch",
            "Print      : Rc<Expr> expression",
            "Return     : Token keyword, Option<Rc<Expr>> value",
            "Var        : Token name, Option<Rc<Expr>> initializer",
            "While      : Rc<Expr> condition, Rc<Stmt> body",
        ];
    }