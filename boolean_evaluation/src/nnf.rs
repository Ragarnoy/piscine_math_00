use crate::node::Node;
use crate::operator::Operator;
use Node::*;
use Operator::*;

pub fn node_to_negation_normal_form(node: Node) -> Node {
    match node {
        UnaryExpr { .. } => unary_expr_to_nnf(node),
        BinaryExpr {
            op: op @ (And | Or),
            mut lhs,
            mut rhs,
        } => {
            *lhs = node_to_negation_normal_form(*lhs);
            *rhs = node_to_negation_normal_form(*rhs);
            BinaryExpr { op, lhs, rhs }
        }
        BinaryExpr { .. } => binary_expr_to_nnf(node),
        _ => node,
    }
}

pub fn binary_expr_to_nnf(node: Node) -> Node {
    if let BinaryExpr { op, lhs, rhs } = node {
        match op {
            Imply => BinaryExpr {
                op: Or,
                lhs: Box::new(UnaryExpr {
                    op: Not,
                    child: Box::new(node_to_negation_normal_form(*lhs)),
                }),
                rhs: Box::new(node_to_negation_normal_form(*rhs)),
            },
            Xnor => BinaryExpr {
                op: Or,
                lhs: Box::new(BinaryExpr {
                    op: And,
                    lhs: Box::new(node_to_negation_normal_form(*lhs.clone())),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                }),
                rhs: Box::new(BinaryExpr {
                    op: And,
                    lhs: Box::new(UnaryExpr {
                        op: Not,
                        child: Box::new(node_to_negation_normal_form(*lhs)),
                    }),
                    rhs: Box::new(UnaryExpr {
                        op: Not,
                        child: Box::new(node_to_negation_normal_form(*rhs)),
                    }),
                }),
            },
            Xor => BinaryExpr {
                op: Or,
                lhs: Box::new(BinaryExpr {
                    op: And,
                    lhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*lhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                }),
                rhs: Box::new(BinaryExpr {
                    op: And,
                    lhs: Box::new(node_to_negation_normal_form(*lhs)),
                    rhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*rhs),
                    })),
                }),
            },
            _ => unreachable!(),
        }
    } else {
        node
    }
}

pub fn unary_expr_to_nnf(node: Node) -> Node {
    if let UnaryExpr { child, .. } = node {
        match *child {
            UnaryExpr { child, .. } => node_to_negation_normal_form(*child),
            BinaryExpr { op, lhs, rhs } => match op {
                Imply => BinaryExpr {
                    op: Or,
                    lhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(*rhs)),
                },

                Xnor => BinaryExpr {
                    op: Or,
                    lhs: Box::new(BinaryExpr {
                        op: And,
                        lhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                            op: Not,
                            child: Box::new(*lhs.clone()),
                        })),
                        rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                    }),
                    rhs: Box::new(BinaryExpr {
                        op: And,
                        lhs: Box::new(node_to_negation_normal_form(*lhs)),
                        rhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                            op: Not,
                            child: Box::new(*rhs),
                        })),
                    }),
                },

                And => BinaryExpr {
                    op: Or,
                    lhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*rhs),
                    })),
                },

                Or => BinaryExpr {
                    op: And,
                    lhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(UnaryExpr {
                        op: Not,
                        child: Box::new(*rhs),
                    })),
                },

                Xor => BinaryExpr {
                    op: Or,
                    lhs: Box::new(node_to_negation_normal_form(BinaryExpr {
                        op: And,
                        lhs: Box::new(UnaryExpr {
                            op: Not,
                            child: Box::new(*lhs.clone()),
                        }),
                        rhs: Box::new(*rhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(BinaryExpr {
                        op: And,
                        lhs: Box::new(*lhs),
                        rhs: Box::new(UnaryExpr {
                            op: Not,
                            child: Box::new(*rhs),
                        }),
                    })),
                },
                Not => {
                    unreachable!();
                }
            },
            _ => UnaryExpr { op: Not, child },
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod nnf_test {
    use crate::nnf::{node_to_negation_normal_form, unary_expr_to_nnf};

    use crate::tree::Tree;
    use std::str::FromStr;

    #[test]
    fn test_nnf_not_and() {
        let node = Tree::from_str("AB&!").unwrap();
        let result = Tree::from_str("A!B!|").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_nnf_imply() {
        let node = Tree::from_str("AB>").unwrap();
        let result = Tree::from_str("A!B|").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_nnf_xnor() {
        let node = Tree::from_str("AB=").unwrap();
        let result = Tree::from_str("AB&A!B!&|").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_nnf_xor() {
        let node = Tree::from_str("AB^").unwrap();
        let result = Tree::from_str("A!B&AB!&|").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_nnf_complex() {
        let node = Tree::from_str("AB|C&!").unwrap();
        let result = Tree::from_str("A!B!&C!|").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_nnf_not_or() {
        let node = Tree::from_str("AB|!").unwrap();
        let result = Tree::from_str("A!B!&").unwrap();
        assert_eq!(node_to_negation_normal_form(node.root), result.root);
    }

    #[test]
    fn test_not_unary_odd() {
        let three = Tree::from_str("1!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(three.root),
            Tree::from_str("1!").unwrap().root
        );
        let five = Tree::from_str("1!!!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(five.root),
            Tree::from_str("1!").unwrap().root
        );
    }

    #[test]
    fn test_not_unary_even() {
        let two = Tree::from_str("1!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(two.root),
            Tree::from_str("1").unwrap().root
        );
        let four = Tree::from_str("1!!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(four.root),
            Tree::from_str("1").unwrap().root
        );
    }
}
