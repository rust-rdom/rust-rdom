use std::sync::Weak;

use crate::internal_prelude::*;

pub trait SandboxMemberBehavior {
    fn get_context(&self) -> Weak<Sandbox>;
}
