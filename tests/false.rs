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

// false null
test!(test_39, "falsenull", vec![("E108", 5, 6)]);
test!(test_40, "false null", vec![("E108", 6, 7)]);
test!(test_41, " false null", vec![("E108", 7, 8)]);
test!(test_42, " \n\r false \tnull", vec![("E108", 11, 12)]);

// false number
test!(test_43, "false0", vec![("E109", 5, 6)]);
test!(test_44, "false 0", vec![("E109", 6, 7)]);
test!(test_45, " false 0", vec![("E109", 7, 8)]);
test!(test_46, " \n\r false \t0", vec![("E109", 11, 12)]);

// false string
test!(test_47, "false\"\"", vec![("E114", 5, 6)]);
test!(test_48, "false \"\"", vec![("E114", 6, 7)]);
test!(test_49, " false \"\"", vec![("E114", 7, 8)]);
test!(test_50, " \n\r false \t\"\"", vec![("E114", 11, 12)]);

// false begin-array
test!(test_51, "false[", vec![("E125", 5, 6)]);
test!(test_52, "false [", vec![("E125", 6, 7)]);
test!(test_53, " false [", vec![("E125", 7, 8)]);
test!(test_54, " \n\r false \t[", vec![("E125", 11, 12)]);

// false end-array
test!(test_55, "false]", vec![("E126", 5, 6)]);
test!(test_56, "false ]", vec![("E126", 6, 7)]);
test!(test_57, " false ]", vec![("E126", 7, 8)]);
test!(test_58, " \n\r false \t]", vec![("E126", 11, 12)]);

// false value-separator
test!(test_59, "false,", vec![("E124", 5, 6)]);
test!(test_60, "false ,", vec![("E124", 6, 7)]);
test!(test_61, " false ,", vec![("E124", 7, 8)]);
test!(test_62, " \n\r false \t,", vec![("E124", 11, 12)]);

// false begin-object
test!(test_63, "false{", vec![("E130", 5, 6)]);
test!(test_64, "false {", vec![("E130", 6, 7)]);
test!(test_65, " false {", vec![("E130", 7, 8)]);
test!(test_66, " \n\r false \t{", vec![("E130", 11, 12)]);

// false end-object
test!(test_67, "false}", vec![("E131", 5, 6)]);
test!(test_68, "false }", vec![("E131", 6, 7)]);
test!(test_69, " false }", vec![("E131", 7, 8)]);
test!(test_70, " \n\r false \t}", vec![("E131", 11, 12)]);

// false name-separator
test!(test_71, "false:", vec![("E136", 5, 6)]);
test!(test_72, "false :", vec![("E136", 6, 7)]);
test!(test_73, " false :", vec![("E136", 7, 8)]);
test!(test_74, " \n\r false \t:", vec![("E136", 11, 12)]);
