mod helpers;

use jsonprima;

// TODO
// - Test for structural characters (comma, colon, etc) before and after the value.

// n
test!(test_1, "n", vec![("E104", 0, 1)]);
test!(test_2, " n", vec![("E104", 1, 2)]);
test!(test_3, "n ", vec![("E105", 0, 2)]);
test!(test_4, " n ", vec![("E105", 1, 3)]);
test!(test_5, " n😋", vec![("E105", 1, 3)]);

// nu
test!(test_6, "nu", vec![("E104", 0, 2)]);
test!(test_7, " nu", vec![("E104", 1, 3)]);
test!(test_8, "nu ", vec![("E105", 0, 3)]);
test!(test_9, " nu ", vec![("E105", 1, 4)]);
test!(test_10, " nu😋", vec![("E105", 1, 4)]);

// nul
test!(test_11, "nul", vec![("E104", 0, 3)]);
test!(test_12, " nul", vec![("E104", 1, 4)]);
test!(test_13, "nul ", vec![("E105", 0, 4)]);
test!(test_14, " nul ", vec![("E105", 1, 5)]);
test!(test_15, " nul😋", vec![("E105", 1, 5)]);

// null
test!(test_16, "null", vec![]);
test!(test_17, " \n\t\rnull", vec![]);
test!(test_18, "null \n\t\r", vec![]);
test!(test_19, " \n\t\rnull \n\t\r", vec![]);

// Ignore byte order mark.
test!(test_20, "\u{feff} null", vec![]);

// null null
test!(test_21, "nullnull", vec![("E108", 4, 5)]);
test!(test_22, "null null", vec![("E108", 5, 6)]);
test!(test_23, " null null", vec![("E108", 6, 7)]);
test!(test_24, " \n\r null \tnull", vec![("E108", 10, 11)]);

// null false
test!(test_25, "nullfalse", vec![("E107", 4, 5)]);
test!(test_26, "null false", vec![("E107", 5, 6)]);
test!(test_27, " null false", vec![("E107", 6, 7)]);
test!(test_28, " \n\r null \tfalse", vec![("E107", 10, 11)]);

// null true
test!(test_29, "nulltrue", vec![("E103", 4, 5)]);
test!(test_30, "null true", vec![("E103", 5, 6)]);
test!(test_31, " null true", vec![("E103", 6, 7)]);
test!(test_32, " \n\r null \ttrue", vec![("E103", 10, 11)]);

// null <any_token>
test!(test_33, "nullw", vec![("E106", 4, 5)]);
test!(test_34, "null (", vec![("E106", 5, 6)]);
test!(test_35, " null ***", vec![("E106", 6, 7)]);
test!(test_36, " \n\r null \tq", vec![("E106", 10, 11)]);
test!(test_37, " null😋", vec![("E106", 5, 6)]);

// null number
test!(test_38, "null0", vec![("E109", 4, 5)]);
test!(test_39, "null 0", vec![("E109", 5, 6)]);
test!(test_40, " null 0", vec![("E109", 6, 7)]);
test!(test_41, " \n\r null \t0", vec![("E109", 10, 11)]);

// null string
test!(test_42, "null\"\"", vec![("E114", 4, 5)]);
test!(test_43, "null \"\"", vec![("E114", 5, 6)]);
test!(test_44, " null \"\"", vec![("E114", 6, 7)]);
test!(test_45, " \n\r null \t\"\"", vec![("E114", 10, 11)]);

// null begin-array
test!(test_46, "null[", vec![("E125", 4, 5)]);
test!(test_47, "null [", vec![("E125", 5, 6)]);
test!(test_48, " null [", vec![("E125", 6, 7)]);
test!(test_49, " \n\r null \t[", vec![("E125", 10, 11)]);

// null end-array
test!(test_50, "null]", vec![("E126", 4, 5)]);
test!(test_51, "null ]", vec![("E126", 5, 6)]);
test!(test_52, " null ]", vec![("E126", 6, 7)]);
test!(test_53, " \n\r null \t]", vec![("E126", 10, 11)]);

// null value-separator
test!(test_54, "null,", vec![("E124", 4, 5)]);
test!(test_55, "null ,", vec![("E124", 5, 6)]);
test!(test_56, " null ,", vec![("E124", 6, 7)]);
test!(test_57, " \n\r null \t,", vec![("E124", 10, 11)]);

// null begin-object
test!(test_58, "null{", vec![("E130", 4, 5)]);
test!(test_59, "null {", vec![("E130", 5, 6)]);
test!(test_60, " null {", vec![("E130", 6, 7)]);
test!(test_61, " \n\r null \t{", vec![("E130", 10, 11)]);

// null end-object
test!(test_62, "null}", vec![("E131", 4, 5)]);
test!(test_63, "null }", vec![("E131", 5, 6)]);
test!(test_64, " null }", vec![("E131", 6, 7)]);
test!(test_65, " \n\r null \t}", vec![("E131", 10, 11)]);

// null name-separator
test!(test_66, "null:", vec![("E136", 4, 5)]);
test!(test_67, "null :", vec![("E136", 5, 6)]);
test!(test_68, " null :", vec![("E136", 6, 7)]);
test!(test_69, " \n\r null \t:", vec![("E136", 10, 11)]);
