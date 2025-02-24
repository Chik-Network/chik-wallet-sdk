use klvmr::{
    reduction::{EvalErr, Reduction},
    Allocator, NodePtr,
};

pub fn run_puzzle(
    allocator: &mut Allocator,
    puzzle: NodePtr,
    solution: NodePtr,
) -> Result<NodePtr, EvalErr> {
    let Reduction(_cost, output) = klvmr::run_program(
        allocator,
        &klvmr::ChikDialect::new(0),
        puzzle,
        solution,
        11_000_000_000,
    )?;
    Ok(output)
}
