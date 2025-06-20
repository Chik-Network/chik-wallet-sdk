use std::{collections::HashMap, fs, io, path::Path, rc::Rc};

use klvm_tools_rs::{
    classic::klvm_tools::klvmc::compile_klvm_text,
    compiler::{compiler::DefaultCompilerOpts, comptypes::CompilerOpts},
};
use klvm_utils::{tree_hash, TreeHash};
use klvmr::{serde::node_to_bytes, Allocator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadKlvmError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid file name")]
    InvalidFileName,

    #[error("Compiler error: {0}")]
    Compiler(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Compilation {
    pub reveal: Vec<u8>,
    pub hash: TreeHash,
}

pub fn load_klvm<P: AsRef<Path>>(
    path: P,
    include_paths: &[String],
) -> Result<Compilation, LoadKlvmError> {
    let path = path.as_ref();

    let mut allocator = Allocator::new();

    let opts = Rc::new(DefaultCompilerOpts::new(
        path.file_name()
            .ok_or(LoadKlvmError::InvalidFileName)?
            .to_str()
            .ok_or(LoadKlvmError::InvalidFileName)?,
    ))
    .set_search_paths(include_paths);

    let text = fs::read_to_string(path)?;

    let ptr = compile_klvm_text(
        &mut allocator,
        opts,
        &mut HashMap::new(),
        &text,
        path.to_str().ok_or(LoadKlvmError::InvalidFileName)?,
        false,
    )
    .map_err(|error| LoadKlvmError::Compiler(format!("{error:?}")))?;

    let hash = tree_hash(&allocator, ptr);
    let reveal = node_to_bytes(&allocator, ptr)?;

    Ok(Compilation { reveal, hash })
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use klvm_traits::{FromKlvm, ToKlvm};
    use klvm_utils::CurriedProgram;
    use klvmr::{serde::node_from_bytes, NodePtr};
    use once_cell::sync::Lazy;

    use crate::{run_puzzle, Mod};

    use super::*;

    #[test]
    fn test_load_klvm() -> anyhow::Result<()> {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, ToKlvm, FromKlvm)]
        #[klvm(curry)]
        struct TestArgs {
            a: u64,
            b: u64,
        }

        static TEST_MOD: Lazy<Compilation> = Lazy::new(|| {
            load_klvm(
                "load_klvm_test.clsp",
                &[".".to_string(), "include".to_string()],
            )
            .unwrap()
        });

        impl Mod for TestArgs {
            fn mod_reveal() -> Cow<'static, [u8]> {
                Cow::Owned(TEST_MOD.reveal.clone())
            }

            fn mod_hash() -> TreeHash {
                TEST_MOD.hash
            }
        }

        let args = TestArgs { a: 10, b: 20 };

        let mut allocator = Allocator::new();

        let mod_ptr = node_from_bytes(&mut allocator, TestArgs::mod_reveal().as_ref())?;

        let ptr = CurriedProgram {
            program: mod_ptr,
            args,
        }
        .to_klvm(&mut allocator)?;

        let output = run_puzzle(&mut allocator, ptr, NodePtr::NIL)?;

        assert_eq!(hex::encode(node_to_bytes(&allocator, output)?), "8200e6");

        Ok(())
    }
}
