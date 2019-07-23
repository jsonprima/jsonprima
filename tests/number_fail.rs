mod helpers;

use jsonprima;

// Empty document.
test!(test_0, "", vec![("E100", 0, 1)]);

// Start with plus sign (+).
test!(test_1, "+1", vec![("E106", 0, 1)]);

// Positive Infinity.
test!(test_2, "Infinity", vec![("E106", 0, 1)]);

// Negative Infinity.
test!(test_3, "-Infinity", vec![("E110", 0, 2)]);

// Positive number with one leading zero.
test!(test_4, "01", vec![("E111", 0, 2)]);

// Positive number with multiple leading zeros.
test!(test_5, "000001", vec![("E111", 0, 2)]);

// Negative number with one leading zero.
test!(test_6, "-01", vec![("E111", 0, 3)]);

// Negative number with multiple leading zeros.
test!(test_7, "-0001", vec![("E111", 0, 3)]);

// Positive number with dot.
test!(test_8, "1005.", vec![("E110", 0, 5)]);

// Negative number with dot.
test!(test_9, "-100.", vec![("E110", 0, 5)]);

// NaN.
test!(test_10, "NaN", vec![("E106", 0, 1)]);

// Number with wrong exponential (-) in fractional part.
test!(test_11, "1.-5", vec![("E110", 0, 3)]);

// Number with wrong exponential (+) in fractional part.
test!(test_12, "1.+5", vec![("E110", 0, 3)]);

// Number without integer part.
test!(test_13, ".5e+1", vec![("E106", 0, 1)]);

// Number with wrong exponential (only e) in fractional part.
test!(test_14, "0.5e", vec![("E110", 0, 4)]);

// Number with wrong exponential (only E) in fractional part.
test!(test_15, "0.5E", vec![("E110", 0, 4)]);

// Exponential (e) after dot.
test!(test_16, "0.e", vec![("E110", 0, 3)]);

// Exponential (E) after dot.
test!(test_17, "0.E", vec![("E110", 0, 3)]);

// Number with wrong exponential (wrong position of +) in integer part.
test!(test_18, "1115e+", vec![("E110", 0, 6)]);

// Number with wrong exponential (wrong position of -) in integer part.
test!(test_19, "1115e-", vec![("E110", 0, 6)]);

// Number with wrong exponential (only e) in integer part.
test!(test_20, "1115e", vec![("E110", 0, 5)]);

// Number with wrong exponential (only E) in integer part.
test!(test_21, "1115E", vec![("E110", 0, 5)]);

// Number with wrong exponential (e and E) in integer part.
test!(test_22, "1115Ee+1", vec![("E110", 0, 6)]);

// Number with wrong exponential (e and E) in fractional part.
test!(test_23, "0.1115Ee+1", vec![("E110", 0, 8)]);

// Full width 1 (U+FF11).
test!(test_24, "１", vec![("E106", 0, 1)]);

// Exponential number with two operators (+ and -).
test!(test_25, "1.1e+-1", vec![("E110", 0, 6)]);

// Number starting with dot.
test!(test_26, ".2", vec![("E106", 0, 1)]);

// Number starting with dot and minus sign.
test!(test_27, "+.2", vec![("E106", 0, 1)]);

// Non finite positive number.
test!(test_28, "10E4000000", vec![("E112", 0, 10)]);

// Non finite negative number.
test!(test_29, "-10E4000000", vec![("E112", 0, 11)]);

// Non-zero number followed by decimal point.
test!(test_30, "1.", vec![("E110", 0, 2)]);

// Zero number followed by decimal point.
test!(test_31, "0.", vec![("E110", 0, 2)]);

// Control character inside number.
test!(test_32, "0.\n", vec![("E110", 0, 3)]);

// Reverse solidus at the beginning of the number.
test!(test_33, "\\0", vec![("E106", 0, 1)]);

// Reverse solidus after decimal point character.
test!(test_35, "0.\\", vec![("E110", 0, 3)]);

// Reverse solidus after exponential (e) character.
test!(test_36, "0.5e\\", vec![("E110", 0, 5)]);

// Reverse solidus after exponential (E) character.
test!(test_37, "0.5E\\", vec![("E110", 0, 5)]);

// Reverse solidus after exponential minus sign (-) character.
test!(test_38, "0.5E-\\", vec![("E110", 0, 6)]);

// Reverse solidus after exponential addition sign (+) character.
test!(test_39, "0.5E+\\", vec![("E110", 0, 6)]);

// number number
test!(test_40, "\n 1  \n 12", vec![("E109", 7, 8)]);

// number true
test!(test_41, "0true", vec![("E103", 1, 2)]);
test!(test_42, "0 true", vec![("E103", 2, 3)]);
test!(test_43, " 0 true", vec![("E103", 3, 4)]);
test!(test_44, " \n\r 0 \ttrue", vec![("E103", 7, 8)]);

// number false
test!(test_45, "0false", vec![("E107", 1, 2)]);
test!(test_46, "0 false", vec![("E107", 2, 3)]);
test!(test_47, " 0 false", vec![("E107", 3, 4)]);
test!(test_48, " \n\r 0 \nfalse", vec![("E107", 7, 8)]);

// number null
test!(test_49, "0null", vec![("E108", 1, 2)]);
test!(test_50, "0 null", vec![("E108", 2, 3)]);
test!(test_51, " 0 null", vec![("E108", 3, 4)]);
test!(test_52, " \n\r 0 \tnull", vec![("E108", 7, 8)]);

// number string
test!(test_53, "0\"\"", vec![("E114", 1, 2)]);
test!(test_54, "0 \"\"", vec![("E114", 2, 3)]);
test!(test_55, " 0 \"\"", vec![("E114", 3, 4)]);
test!(test_56, " \n\r 0 \t\"\"", vec![("E114", 7, 8)]);

// number begin-array
test!(test_57, "0[", vec![("E125", 1, 2)]);
test!(test_58, "0 [", vec![("E125", 2, 3)]);
test!(test_59, " 0 [", vec![("E125", 3, 4)]);
test!(test_60, " \n\r 0 \t[", vec![("E125", 7, 8)]);

// number end-array
test!(test_61, "0]", vec![("E126", 1, 2)]);
test!(test_62, "0 ]", vec![("E126", 2, 3)]);
test!(test_63, " 0 ]", vec![("E126", 3, 4)]);
test!(test_64, " \n\r 0 \t]", vec![("E126", 7, 8)]);

// number value-separator
test!(test_65, "0,", vec![("E124", 1, 2)]);
test!(test_66, "0 ,", vec![("E124", 2, 3)]);
test!(test_67, " 0 ,", vec![("E124", 3, 4)]);
test!(test_68, " \n\r 0 \t,", vec![("E124", 7, 8)]);

// number begin-object
test!(test_69, "0{", vec![("E130", 1, 2)]);
test!(test_70, "0 {", vec![("E130", 2, 3)]);
test!(test_71, " 0 {", vec![("E130", 3, 4)]);
test!(test_72, " \n\r 0 \t{", vec![("E130", 7, 8)]);

// number end-object
test!(test_73, "0}", vec![("E131", 1, 2)]);
test!(test_74, "0 }", vec![("E131", 2, 3)]);
test!(test_75, " 0 }", vec![("E131", 3, 4)]);
test!(test_76, " \n\r 0 \t}", vec![("E131", 7, 8)]);

// number name-separator
test!(test_77, "0:", vec![("E136", 1, 2)]);
test!(test_78, "0 :", vec![("E136", 2, 3)]);
test!(test_79, " 0 :", vec![("E136", 3, 4)]);
test!(test_80, " \n\r 0 \t:", vec![("E136", 7, 8)]);
