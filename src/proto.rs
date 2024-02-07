use std::collections::HashMap;

use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "dprint";
static BIN: &str = "dprint";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::CLI,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/dprint/dprint")?
        .iter()
        .map(|tag| tag.to_owned())
        .collect();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;
    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [
                HostArch::X64, HostArch::Arm64
            ],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = &input.context.version;
    let target = get_target_triple(&env, NAME)?;
    let file_name = format!("dprint-{target}.zip");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/dprint/dprint/releases/download/{version}/{file_name}"
        ),
        download_name: Some(file_name),
        checksum_url: Some(format!(
            "https://github.com/dprint/dprint/releases/download/{version}/SHASUMS256.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(input): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;
    let tool_dir = input.context.tool_dir.real_path();

    let exe_name = env.os.get_exe_name(BIN);
    let mut primary = ExecutableConfig::new(exe_name);
    primary.shim_env_vars = Some(HashMap::from_iter([(
        // DPRINT_INSTALL seemingly has no effect currently, but the install script suggests adding it to bashrc
        "DPRINT_INSTALL".into(),
        tool_dir
            .and_then(|p| p.into_os_string().into_string().ok())
            .unwrap_or_default(),
    )]));

    Ok(Json(LocateExecutablesOutput {
        primary: Some(primary),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn pre_run(Json(input): Json<RunHook>) -> FnResult<()> {
    let args = &input.passthrough_args;

    if !args.is_empty() && args[0] == "upgrade" {
        return Err(plugin_err!(
            "The dprint installation is managed by proto and must be upgraded by it.\nLearn more: <url>{}</url>",
            "https://moonrepo.dev/docs/proto/detection"
        ));
    }

    Ok(())
}
