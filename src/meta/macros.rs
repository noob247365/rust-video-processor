// SPDX-FileCopyrightText: 2026 noob247365
// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_export]
macro_rules! enum_with_members {
    (
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_ident:ident {
            $(
                $(#[$member_meta:meta])*
                $member_ident:ident $(= $member_expr:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        $enum_vis enum $enum_ident {
            $(
                $(#[$member_meta])*
                $member_ident $(= $member_expr)?,
            )*
        }

        impl $enum_ident {
            $enum_vis const MEMBERS: &'static [Self] = &[
                $(Self::$member_ident),*
            ];
        }
    };
}
