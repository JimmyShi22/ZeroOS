use crate::spec::{
    get_arch_spec, load_target_profile, parse_target_triple, LLVMConfig, TargetRenderOptions,
};

///      --features "+m,+a,+c" --abi lp64 \
///      --data-layout "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128" \

#[derive(Debug, Clone, Default, clap::Args)]
pub struct GenerateTargetArgs {
    #[arg(long)]
    pub profile: Option<String>,

    #[arg(long)]
    pub target: Option<String>,

    #[arg(long)]
    pub llvm_target: Option<String>,

    /// LLVM ABI (e.g., "lp64", "lp64d", "ilp32"). Can override profile defaults
    #[arg(long)]
    pub abi: Option<String>,

    #[arg(long)]
    pub features: Option<String>,

    /// LLVM data layout string. Can override profile defaults
    #[arg(long)]
    pub data_layout: Option<String>,
}

pub fn generate_target_spec(
    args: GenerateTargetArgs,
    render_opts: TargetRenderOptions,
) -> Result<String, String> {
    let (config, arch_spec, llvm_config) = if let Some(ref profile_name) = args.profile {
        let profile = load_target_profile(profile_name).ok_or_else(|| {
            format!(
                "Unknown profile: '{}'. Available profiles: {}",
                profile_name,
                crate::spec::list_profiles().join(", ")
            )
        })?;

        let mut llvm_config = profile.llvm_config;
        if let Some(llvm_target) = args.llvm_target {
            llvm_config.llvm_target = llvm_target;
        }
        if let Some(features) = args.features {
            llvm_config.features = features;
        }
        if let Some(abi) = args.abi {
            llvm_config.abi = abi;
        }
        if let Some(data_layout) = args.data_layout {
            llvm_config.data_layout = data_layout;
        }

        (profile.config, profile.arch_spec, llvm_config)
    } else {
        let target = args
            .target
            .as_deref()
            .ok_or_else(|| "Either --profile or --target is required".to_string())?;

        let config = parse_target_triple(target)
            .ok_or_else(|| format!("Cannot parse target triple: {}", target))?;

        let arch_spec = get_arch_spec(&config.arch);

        let llvm_target = args
            .llvm_target
            .ok_or_else(|| "llvm_target is required when not using profile".to_string())?;
        let features = args
            .features
            .ok_or_else(|| "features is required when not using profile".to_string())?;
        let abi = args
            .abi
            .ok_or_else(|| "abi is required when not using profile".to_string())?;
        let data_layout = args
            .data_layout
            .ok_or_else(|| "data_layout is required when not using profile".to_string())?;

        let llvm_config = LLVMConfig {
            llvm_target,
            features,
            abi,
            data_layout,
        };

        (config, arch_spec, llvm_config)
    };

    let json_content = config.render(&arch_spec, &llvm_config, render_opts)?;

    Ok(json_content)
}
