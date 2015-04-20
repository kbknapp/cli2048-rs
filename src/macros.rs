macro_rules! for_match {
    ($it:ident, $($p:pat => $($e:expr);+),*) => {
        for i in $it {
            match i {
            $(
                $p => { $($e)+ }
            )*
            }
        }
    };
}