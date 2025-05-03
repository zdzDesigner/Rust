#[macro_export]
macro_rules! comp {
    (@eval {$mapping:expr} for {$pattern:pat} in {$iterator:expr} $(if {$condition:expr})*) => {
        <_ as ::core::iter::Iterator>::filter_map(
            <_ as ::core::iter::IntoIterator>::into_iter($iterator),
            move |$pattern| (true $(&& ($condition))*).then(|| ($mapping))
        )
    };

    (@eval {$mapping:expr}
        for {$pattern:pat} in {$iterator:expr} $(if {$condition:expr})*
        $(for {$pattern2:pat} in {$iterator2:expr} $(if {$condition2:expr})*)+
    ) => {
        <_ as ::core::iter::Iterator>::flatten(
            <_ as ::core::iter::Iterator>::filter_map(
                <_ as ::core::iter::IntoIterator>::into_iter($iterator),
                move |$pattern| (true $(&& ($condition))*).then(|| (
                    $crate::comp!(@eval {$mapping} $(for {$pattern2} in {$iterator2} $(if {$condition2})*)+)
                ))
            )
        )
    };



    (@scan mapping [] [$($group:tt)*] for $($tail:tt)*) => {
        $crate::comp!(@scan pattern [{$($group)*}] [] $($tail)*)
    };

    (@scan pattern [$($done:tt)*] [$($group:tt)*] in $($tail:tt)*) => {
        $crate::comp!(@scan iterator [$($done)* for {$($group)*}] [] $($tail)*)
    };

    (@scan iterator [$($done:tt)*] [$($group:tt)*]) => {
        $crate::comp!(@eval $($done)* in {$($group)*})
    };

    (@scan iterator [$($done:tt)*] [$($group:tt)*] if $($tail:tt)*) => {
        $crate::comp!(@scan condition [$($done)* in {$($group)*}] [] $($tail)*)
    };

    (@scan iterator [$($done:tt)*] [$($group:tt)*] for $($tail:tt)*) => {
        $crate::comp!(@scan pattern [$($done)* in {$($group)*}] [] $($tail)*)
    };

    (@scan condition [$($done:tt)*] [$($group:tt)*]) => {
        $crate::comp!(@eval $($done)* if {$($group)*})
    };

    (@scan condition [$($done:tt)*] [$($group:tt)*] if $($tail:tt)*) => {
        $crate::comp!(@scan condition [$($done)* if {$($group)*}] [] $($tail)*)
    };

    (@scan condition [$($done:tt)*] [$($group:tt)*] for $($tail:tt)*) => {
        $crate::comp!(@scan pattern [$($done)* if {$($group)*}] [] $($tail)*)
    };



    (@scan $kind:ident [$($done:tt)*] [$($group:tt)*] $head:tt $($tail:tt)*) => {
        $crate::comp!(@scan $kind [$($done)*] [$($group)* $head] $($tail)*)
    };



    // fallback to prevent infinite loop

    (@ $($token:tt)*) => {
        ::core::compile_error!(::core::concat!("$crate::comp!(@", ::core::stringify!($($token)*), ")"))
    };



    ($($token:tt)*) => {
        <_ as ::core::iter::Iterator>::collect::<Vec<_>>(
            $crate::comp!(@scan mapping [] [] $($token)*)
        )

    }
}

#[cfg(test)]
mod macro2 {

    #[test]
    fn test_macro2() {
        let pythagorean_triples =
            comp![(a, b, c) for a in 1..20 for b in a..20 for c in b..20 if a*a + b*b == c*c];

        println!("{:?}", pythagorean_triples);
    }
}

