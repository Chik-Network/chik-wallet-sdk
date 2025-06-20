use std::sync::{Arc, Mutex};

use chik_sdk_driver::{HashedPtr, SpendContext};
use klvmr::{Allocator, NodePtr};

use crate::Program;

pub(crate) trait AsProgram {
    type AsProgram;

    fn as_program(&self, klvm: &Arc<Mutex<SpendContext>>) -> Self::AsProgram;
}

pub(crate) trait AsPtr {
    type AsPtr;

    fn as_ptr(&self, allocator: &Allocator) -> Self::AsPtr;
}

impl AsProgram for NodePtr {
    type AsProgram = Program;

    fn as_program(&self, klvm: &Arc<Mutex<SpendContext>>) -> Self::AsProgram {
        Program(klvm.clone(), *self)
    }
}

impl AsProgram for HashedPtr {
    type AsProgram = Program;

    fn as_program(&self, klvm: &Arc<Mutex<SpendContext>>) -> Self::AsProgram {
        Program(klvm.clone(), self.ptr())
    }
}

impl AsPtr for Program {
    type AsPtr = HashedPtr;

    fn as_ptr(&self, allocator: &Allocator) -> Self::AsPtr {
        HashedPtr::from_ptr(allocator, self.1)
    }
}
