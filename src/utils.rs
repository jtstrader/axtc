#[macro_export]
/// Initialize the target vector by accepting a set of rules (feature flags) and their associated
/// enum variants.
macro_rules! init_targets {
    ($($rule:expr => $variant:expr),*) => {{
        let mut v: Vec<AxtcTarget> = Vec::new();
        $(if $rule { v.push($variant); })*
        v
    }};
}

#[cfg(test)]
mod tests {
    use crate::AxtcTarget;

    #[test]
    fn init_targets() {
        let (herbstluftwm, polybar, neovim) = (true, false, true);

        let targets: Vec<AxtcTarget> = init_targets!(
            herbstluftwm => AxtcTarget::Herbstluftwm("herbstluftwm".into()),
            polybar => AxtcTarget::Polybar("polybar".into()),
            neovim => AxtcTarget::Neovim("neovim".into())
        );

        assert_eq!(
            targets,
            vec![
                AxtcTarget::Herbstluftwm("herbstluftwm".into()),
                AxtcTarget::Neovim("neovim".into())
            ]
        );
    }
}
