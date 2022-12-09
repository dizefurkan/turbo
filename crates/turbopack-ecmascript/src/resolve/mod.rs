pub mod node_native_binding;

use anyhow::Result;
use turbopack_core::resolve::{
    handle_resolve_error,
    options::{ConditionValue, ResolveIntoPackage, ResolveOptions, ResolveOptionsVc},
    origin::ResolveOriginVc,
    parse::RequestVc,
    resolve, ResolveResultVc,
};

#[turbo_tasks::function]
pub async fn apply_esm_specific_options(options: ResolveOptionsVc) -> Result<ResolveOptionsVc> {
    let mut options: ResolveOptions = options.await?.clone_value();
    for item in options.into_package.iter_mut() {
        match item {
            ResolveIntoPackage::ExportsField { conditions, .. } => {
                conditions.insert("import".to_string(), ConditionValue::Set);
                conditions.insert("require".to_string(), ConditionValue::Unset);
            }
            ResolveIntoPackage::MainField(_) | ResolveIntoPackage::Default(_) => {}
        }
    }
    Ok(options.into())
}

#[turbo_tasks::function]
pub async fn apply_cjs_specific_options(options: ResolveOptionsVc) -> Result<ResolveOptionsVc> {
    let mut options: ResolveOptions = options.await?.clone_value();
    for item in options.into_package.iter_mut() {
        match item {
            ResolveIntoPackage::ExportsField { conditions, .. } => {
                conditions.insert("import".to_string(), ConditionValue::Unset);
                conditions.insert("require".to_string(), ConditionValue::Set);
            }
            ResolveIntoPackage::MainField(_) | ResolveIntoPackage::Default(_) => {}
        }
    }
    Ok(options.into())
}

#[turbo_tasks::function]
pub async fn esm_resolve(origin: ResolveOriginVc, request: RequestVc) -> Result<ResolveResultVc> {
    let options = apply_esm_specific_options(origin.resolve_options());
    specific_resolve(origin, request, options, "esm request").await
}

#[turbo_tasks::function]
pub async fn cjs_resolve(origin: ResolveOriginVc, request: RequestVc) -> Result<ResolveResultVc> {
    let options = apply_cjs_specific_options(origin.resolve_options());
    specific_resolve(origin, request, options, "commonjs request").await
}

#[turbo_tasks::function]
pub async fn relative_asset_resolve(
    origin: ResolveOriginVc,
    request: RequestVc,
) -> Result<ResolveResultVc> {
    let resolve_options = origin.resolve_options();
    let rel_request = request.as_relative();
    let rel_result = resolve(origin.origin_path().parent(), rel_request, resolve_options);
    let result = if *rel_result.is_unresolveable().await? && rel_request.resolve().await? != request
    {
        resolve(origin.origin_path().parent(), request, resolve_options)
            .add_references(rel_result.await?.get_references().clone())
    } else {
        rel_result
    };
    handle_resolve_error(result, "asset", origin, request, resolve_options).await?;
    Ok(origin.context().process_resolve_result(result))
}

async fn specific_resolve(
    origin: ResolveOriginVc,
    request: RequestVc,
    options: ResolveOptionsVc,
    request_type: &str,
) -> Result<ResolveResultVc> {
    let result = origin.resolve_asset(request, options);

    handle_resolve_error(result, request_type, origin, request, options).await
}
