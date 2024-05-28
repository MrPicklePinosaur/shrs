#[macro_export]
macro_rules! all_the_tuples {
    ($n1:ident, $n2:ident) => {
        $n2!(T1);
        $n2!(T1, T2);
        $n2!(T1, T2, T3);
        $n2!(T1, T2, T3, T4);
        $n2!(T1, T2, T3, T4, T5);
        $n2!(T1, T2, T3, T4, T5, T6);
        $n2!(T1, T2, T3, T4, T5, T6, T7);
        $n2!(T1, T2, T3, T4, T5, T6, T7, T8);
        $n2!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $n2!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $n2!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $n2!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

        $n1!(T1);
        $n1!(T1, T2);
        $n1!(T1, T2, T3);
        $n1!(T1, T2, T3, T4);
        $n1!(T1, T2, T3, T4, T5);
        $n1!(T1, T2, T3, T4, T5, T6);
        $n1!(T1, T2, T3, T4, T5, T6, T7);
        $n1!(T1, T2, T3, T4, T5, T6, T7, T8);
        $n1!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $n1!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $n1!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $n1!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    };
}
