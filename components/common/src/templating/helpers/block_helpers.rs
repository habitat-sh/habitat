// helpers for rendering blocks
// Copied from the `handlebars::each` helper.

use serde_json::value::Value as Json;

use handlebars::{BlockContext,
                 BlockParams,
                 Helper,
                 PathAndJson,
                 RenderError};

#[inline]
fn copy_on_push_vec<T>(input: &[T], el: T) -> Vec<T>
    where T: Clone
{
    let mut new_vec = Vec::with_capacity(input.len() + 1);
    new_vec.extend_from_slice(input);
    new_vec.push(el);
    new_vec
}

pub(super) fn create_block<'rc>(param: &PathAndJson<'rc>) -> BlockContext<'rc> {
    let mut block = BlockContext::new();

    if let Some(new_path) = param.context_path() {
        block.base_path_mut().clone_from(new_path);
    } else {
        // use clone for now
        block.set_base_value(param.value().clone());
    }

    block
}

pub(super) fn update_block_context(block: &mut BlockContext<'_>,
                                   base_path: Option<&Vec<String>>,
                                   relative_path: String,
                                   is_first: bool,
                                   value: &Json) {
    if let Some(p) = base_path {
        if is_first {
            *block.base_path_mut() = copy_on_push_vec(p, relative_path);
        } else if let Some(ptr) = block.base_path_mut().last_mut() {
            *ptr = relative_path;
        }
    } else {
        block.set_base_value(value.clone());
    }
}

pub(super) fn set_block_param<'rc>(block: &mut BlockContext<'rc>,
                                   h: &Helper<'rc>,
                                   base_path: Option<&Vec<String>>,
                                   k: &Json,
                                   v: &Json)
                                   -> Result<(), RenderError> {
    if let Some(bp_val) = h.block_param() {
        let mut params = BlockParams::new();
        if base_path.is_some() {
            params.add_path(bp_val, Vec::with_capacity(0))?;
        } else {
            params.add_value(bp_val, v.clone())?;
        }

        block.set_block_params(params);
    } else if let Some((bp_val, bp_key)) = h.block_param_pair() {
        let mut params = BlockParams::new();
        if base_path.is_some() {
            params.add_path(bp_val, Vec::with_capacity(0))?;
        } else {
            params.add_value(bp_val, v.clone())?;
        }
        params.add_value(bp_key, k.clone())?;

        block.set_block_params(params);
    }

    Ok(())
}
