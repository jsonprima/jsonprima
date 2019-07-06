mod helpers;

use jsonprima;

// t
test!(test_1, "t", vec![("E104", 0, 1)]);
test!(test_2, " t", vec![("E104", 1, 2)]);
test!(test_3, "t ", vec![("E105", 0, 2)]);
test!(test_4, " t ", vec![("E105", 1, 3)]);
test!(test_5, " t😋", vec![("E105", 1, 3)]);

// tr
test!(test_6, "tr", vec![("E104", 0, 2)]);
test!(test_7, " tr", vec![("E104", 1, 3)]);
test!(test_8, "tr ", vec![("E105", 0, 3)]);
test!(test_9, " tr ", vec![("E105", 1, 4)]);
test!(test_10, " tr😋", vec![("E105", 1, 4)]);

// tru
test!(test_11, "tru", vec![("E104", 0, 3)]);
test!(test_12, " tru", vec![("E104", 1, 4)]);
test!(test_13, "tru ", vec![("E105", 0, 4)]);
test!(test_14, " tru ", vec![("E105", 1, 5)]);
test!(test_15, " tru😋", vec![("E105", 1, 5)]);

// true
test!(test_16, "true", vec![]);
test!(test_17, " \n\t\rtrue", vec![]);
test!(test_18, "true \n\t\r", vec![]);
test!(test_19, " \n\t\rtrue \n\t\r", vec![]);

// Ignore byte order mark.
test!(test_20, "\u{feff}true", vec![]);

// true true
test!(test_21, "truetrue", vec![("E103", 4, 5)]);
test!(test_22, "true true", vec![("E103", 5, 6)]);
test!(test_23, " true true", vec![("E103", 6, 7)]);
test!(test_24, " \n\r true \ttrue", vec![("E103", 10, 11)]);

// true <any_token>
test!(test_33, "truew", vec![("E106", 4, 5)]);
test!(test_34, "true (", vec![("E106", 5, 6)]);
test!(test_35, " true ***", vec![("E106", 6, 7)]);
test!(test_36, " \n\r true \tq", vec![("E106", 10, 11)]);
test!(test_37, " true😋", vec![("E106", 5, 6)]);

// true false
test!(test_38, "truefalse", vec![("E107", 4, 5)]);
test!(test_39, "true false", vec![("E107", 5, 6)]);
test!(test_40, " true false", vec![("E107", 6, 7)]);
test!(test_41, " \n\r true \tfalse", vec![("E107", 10, 11)]);

// true null
test!(test_43, "truenull", vec![("E108", 4, 5)]);
test!(test_44, "true null", vec![("E108", 5, 6)]);
test!(test_45, " true null", vec![("E108", 6, 7)]);
test!(test_46, " \n\r true \r\nnull", vec![("E108", 11, 12)]);
