mod helpers;

use jsonprima;

// f
test!(test_1, "f", vec![("E104", 0, 1)]);
test!(test_2, " f", vec![("E104", 1, 2)]);
test!(test_3, "f ", vec![("E105", 0, 2)]);
test!(test_4, " f ", vec![("E105", 1, 3)]);
test!(test_5, " f😋", vec![("E105", 1, 3)]);

// fa
test!(test_6, "fa", vec![("E104", 0, 2)]);
test!(test_7, " fa", vec![("E104", 1, 3)]);
test!(test_8, "fa ", vec![("E105", 0, 3)]);
test!(test_9, " fa ", vec![("E105", 1, 4)]);
test!(test_10, " fa😋", vec![("E105", 1, 4)]);

// fal
test!(test_11, "fal", vec![("E104", 0, 3)]);
test!(test_12, " fal", vec![("E104", 1, 4)]);
test!(test_13, "fal ", vec![("E105", 0, 4)]);
test!(test_14, " fal ", vec![("E105", 1, 5)]);
test!(test_15, " fal😋", vec![("E105", 1, 5)]);

// fals
test!(test_16, "fals", vec![("E104", 0, 4)]);
test!(test_17, " fals", vec![("E104", 1, 5)]);
test!(test_18, "fals ", vec![("E105", 0, 5)]);
test!(test_19, " fals ", vec![("E105", 1, 6)]);
test!(test_20, " fals😋", vec![("E105", 1, 6)]);

// false
test!(test_21, "false", vec![]);
test!(test_22, " \n\t\r false", vec![]);
test!(test_23, "false \n\t\r", vec![]);
test!(test_24, " \n\t\r false \n\t\r", vec![]);

// Ignore byte order mark.
test!(test_25, "\u{feff} false", vec![]);

// false false
test!(test_26, "falsefalse", vec![("E107", 5, 6)]);
test!(test_27, "false false", vec![("E107", 6, 7)]);
test!(test_28, " false false", vec![("E107", 7, 8)]);
test!(test_29, " \n\r false \tfalse", vec![("E107", 11, 12)]);

// false true
test!(test_30, "falsetrue", vec![("E103", 5, 6)]);
test!(test_31, "false true", vec![("E103", 6, 7)]);
test!(test_32, " false true", vec![("E103", 7, 8)]);
test!(test_33, " \n\r false \ttrue", vec![("E103", 11, 12)]);

// false <any_token>
test!(test_34, "falsew", vec![("E106", 5, 6)]);
test!(test_35, "false (", vec![("E106", 6, 7)]);
test!(test_36, " false ***", vec![("E106", 7, 8)]);
test!(test_37, " \n\r false \tq", vec![("E106", 11, 12)]);
test!(test_38, " false😋", vec![("E106", 6, 7)]);
