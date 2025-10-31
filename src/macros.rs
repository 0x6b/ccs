/// Macro for applying color with optional styles
/// Usage:
/// - styled!(f, value, color) - single color
/// - styled!(f, value, color, style1, style2, ...) - color with styles
#[macro_export]
macro_rules! styled {
    // Color only
    ($f:expr, $val:expr, $color:ident) => {
        write!($f, "{}", $val.$color())
    };
    // Color with styles
    ($f:expr, $val:expr, $color:ident, $($style:ident),+) => {
        write!($f, "{}", $val.$color()$(.$style())+)
    };
}

/// Macro to simplify conditional coloring with optional styles
/// Usage:
/// - styled_by!(f, value; condition1 => color1(style1, style2), condition2 => color2(style1))
#[macro_export]
macro_rules! styled_by {
    ($f:expr, $val:expr; $($cond:expr => $color:ident($($style:ident),+)),+ $(,)?) => {{
        $(
            if $cond {
                styled!($f, $val, $color, $($style),+)
            } else
        )+
        {
            write!($f, "{}", $val)
        }
    }};
}
