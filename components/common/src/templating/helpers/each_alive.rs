use super::{block_helpers::{create_block,
                            set_block_param,
                            update_block_context},
            JsonTruthy};

use handlebars::{to_json,
                 Context,
                 Handlebars,
                 Helper,
                 HelperDef,
                 HelperResult,
                 Output,
                 RenderContext,
                 RenderErrorReason,
                 Renderable};
use serde_json::Value as Json;

#[derive(Clone, Copy)]
pub struct EachAliveHelper;

impl HelperDef for EachAliveHelper {
    fn call<'reg: 'rc, 'rc>(&self,
                            h: &Helper<'rc>,
                            r: &'reg Handlebars<'reg>,
                            ctx: &'rc Context,
                            rc: &mut RenderContext<'reg, 'rc>,
                            out: &mut dyn Output)
                            -> HelperResult {
        let value = h.param(0)
                     .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("eachAlive", 0))?;

        if let Some(template) = h.template() {
            match (value.value().is_truthy(), value.value()) {
                (true, Json::Array(list)) => {
                    let first_alive_idx = list.iter().position(|m| {
                                                         m.as_object().is_some() && {
                            let m = m.as_object().unwrap();
                            m.contains_key("alive") && m["alive"].as_bool().unwrap()
                        }
                                                     });
                    let last_alive_idx = list.iter().rposition(|m| {
                                                        m.as_object().is_some() && {
                            let m = m.as_object().unwrap();
                            m.contains_key("alive") && m["alive"].as_bool().unwrap()
                        }
                                                    });
                    let array_path = value.context_path();

                    let block_context = create_block(value);
                    rc.push_block(block_context);

                    let mut alive_idx = 0;
                    for (i, member) in list.iter().enumerate() {
                        if let Some(m) = member.as_object() {
                            if m.contains_key("alive") && m["alive"].as_bool().unwrap() {
                                alive_idx += 1;
                                if let Some(ref mut block) = rc.block_mut() {
                                    let is_first = first_alive_idx == Some(i);
                                    let is_last = last_alive_idx == Some(i);
                                    let index = to_json(alive_idx);

                                    block.set_local_var("first", to_json(is_first));
                                    block.set_local_var("last", to_json(is_last));
                                    block.set_local_var("index", to_json(index.clone()));

                                    update_block_context(block,
                                                         array_path,
                                                         i.to_string(),
                                                         is_first,
                                                         member);
                                    set_block_param(block, h, array_path, &index, member)?;
                                }

                                template.render(r, ctx, rc, out)?;
                            }
                        }
                    }

                    rc.pop_block();
                    Ok(())
                }
                (true, Json::Object(obj)) => {
                    if !obj.contains_key("alive") || !obj["alive"].as_bool().unwrap() {
                        return Ok(());
                    }

                    let mut first = true;

                    let block_context = create_block(value);
                    rc.push_block(block_context);

                    let obj_path = value.context_path();

                    for (i, (k, v)) in obj.iter().enumerate() {
                        if let Some(ref mut block) = rc.block_mut() {
                            let is_first = i == 0usize;

                            let key = to_json(k);

                            block.set_local_var("first", to_json(is_first));
                            block.set_local_var("key", to_json(k));

                            update_block_context(block, obj_path, k.to_string(), is_first, v);
                            set_block_param(block, h, obj_path, &key, v)?;
                        }
                        template.render(r, ctx, rc, out)?;
                        if first {
                            first = false;
                        }
                    }

                    rc.pop_block();
                    Ok(())
                }
                (false, _) => {
                    if let Some(else_template) = h.inverse() {
                        else_template.render(r, ctx, rc, out)?;
                    }
                    Ok(())
                }
                _ => Err(RenderErrorReason::InvalidParamType("Param type is not iterable.").into()),
            }
        } else {
            Ok(())
        }
    }
}

pub static EACH_ALIVE: EachAliveHelper = EachAliveHelper;
