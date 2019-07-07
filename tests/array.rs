mod helpers;

use jsonprima;

// [
test!(test_1, "[", vec![("E127", 0, 1)]);
test!(test_2, " [", vec![("E127", 1, 2)]);
test!(test_3, "[ ", vec![("E127", 1, 2)]);
test!(test_4, " [ ", vec![("E127", 2, 3)]);

// ]
test!(test_5, "]", vec![("E126", 0, 1)]);
test!(test_6, " ]", vec![("E126", 1, 2)]);
test!(test_7, "] ", vec![("E126", 0, 1)]);
test!(test_8, " ] ", vec![("E126", 1, 2)]);

// []
test!(test_9, "[]", vec![]);
test!(test_10, " []", vec![]);
test!(test_11, "[] ", vec![]);
test!(test_12, " [ \n \n \r \t] ", vec![]);

// []]
test!(test_13, "[]]", vec![("E126", 2, 3)]);
test!(test_14, " []]", vec![("E126", 3, 4)]);
test!(test_15, "[]] ", vec![("E126", 2, 3)]);
test!(test_16, " []] ", vec![("E126", 3, 4)]);

// Nested arrays
test!(test_17, "[ [] ]", vec![]);
test!(test_18, " [ [ [ \n [ ]]]]", vec![]);

// Trailing end-array
test!(test_19, "[[[]]]] ", vec![("E126", 6, 7)]);
test!(test_20, " [[[ \n ]]]] ", vec![("E126", 10, 11)]);

// Unterminated nested arrays
test!(test_21, "[[[[[[[]]]] ", vec![("E127", 11, 12)]);
test!(test_22, " [[[\n\n[[[[[[ \n ]]]] ", vec![("E127", 19, 20)]);

// true as array element
test!(test_23, "[ true ]", vec![]);
test!(test_24, "[ [ true  ]]", vec![]);
test!(test_25, "[[[true]", vec![("E127", 7, 8)]);
test!(test_26, "[true]]]]", vec![("E126", 6, 7)]);

// false as array element
test!(test_27, "[ false ]", vec![]);
test!(test_28, "[ [ false  ]]", vec![]);
test!(test_29, "[[[false]", vec![("E127", 8, 9)]);
test!(test_30, "[false]]]]", vec![("E126", 7, 8)]);

// null as array element
test!(test_31, "[ null ]", vec![]);
test!(test_32, "[ [ null  ]]", vec![]);
test!(test_33, "[[[null]", vec![("E127", 7, 8)]);
test!(test_34, "[null]]]]", vec![("E126", 6, 7)]);

// Number as array element
test!(test_35, "[ 12.9 ]", vec![]);
test!(test_36, "[ [ 12.9  ]]", vec![]);
test!(test_37, "[[[12.9]", vec![("E127", 7, 8)]);
test!(test_38, "[12.9]]]]", vec![("E126", 6, 7)]);

// String as array element
test!(test_39, "[ \"55\" ]", vec![]);
test!(test_40, "[ [ \"55\"  ]]", vec![]);
test!(test_41, "[[[\"55\"]", vec![("E127", 7, 8)]);
test!(test_42, "[\"55\"]]]]", vec![("E126", 6, 7)]);

// Multiple true as array element
test!(test_43, "[ true, true, true ]", vec![]);
test!(test_44, "[ [ true, true, true  ], true, true]", vec![]);
test!(test_45, "[[[true, true, true]", vec![("E127", 19, 20)]);
test!(test_46, "[true, true, true]]]]", vec![("E126", 18, 19)]);

// Multiple false as array element
test!(test_47, "[ false, false, false ]", vec![]);
test!(
  test_48,
  "[ [ false, false, false  ], false, false, false]",
  vec![]
);
test!(test_49, "[[[false, false, false]", vec![("E127", 22, 23)]);
test!(test_50, "[false, false, false]]]]", vec![("E126", 21, 22)]);

// Multiple null as array element
test!(test_51, "[ null, null, null ]", vec![]);
test!(
  test_52,
  "[ [ null, null, null  ], null, null, null]",
  vec![]
);
test!(test_53, "[[[null, null, null]", vec![("E127", 19, 20)]);
test!(test_54, "[null, null, null]]]]", vec![("E126", 18, 19)]);

// Multiple number as array element
test!(test_55, "[ 12.9, 12.9, 12.9 ]", vec![]);
test!(
  test_56,
  "[ [ 12.9, 12.9, 12.9  ], 12.9, 12.9, 12.9]",
  vec![]
);
test!(test_57, "[[[12.9, 12.9, 12.9]", vec![("E127", 19, 20)]);
test!(test_58, "[12.9, 12.9, 12.9]]]]", vec![("E126", 18, 19)]);

// Multiple string as array element
test!(test_59, "[ \"55\", \"55\", \"55\" ]", vec![]);
test!(
  test_60,
  "[ [ \"55\", \"55\", \"55\"  ], \"55\", \"55\", \"55\"]",
  vec![]
);
test!(
  test_61,
  "[[[\"55\", \"55\", \"55\"]",
  vec![("E127", 19, 20)]
);
test!(
  test_62,
  "[\"55\", \"55\", \"55\"]]]]",
  vec![("E126", 18, 19)]
);

// array true
test!(test_63, "[10]true", vec![("E103", 4, 5)]);
test!(test_64, "[10] true", vec![("E103", 5, 6)]);
test!(test_65, " [10] true", vec![("E103", 6, 7)]);
test!(test_66, " \n\r [10] \ttrue", vec![("E103", 10, 11)]);

// array <any_token>
test!(test_67, "[10]w", vec![("E106", 4, 5)]);
test!(test_68, "[10] (", vec![("E106", 5, 6)]);
test!(test_69, " [10] ***", vec![("E106", 6, 7)]);
test!(test_70, " \n\r [10] \tq", vec![("E106", 10, 11)]);
test!(test_71, " [10]😋", vec![("E106", 5, 6)]);

// array false
test!(test_72, "[10]false", vec![("E107", 4, 5)]);
test!(test_73, "[10] false", vec![("E107", 5, 6)]);
test!(test_74, " [10] false", vec![("E107", 6, 7)]);
test!(test_75, " \n\r [10] \tfalse", vec![("E107", 10, 11)]);

// array null
test!(test_76, "[10]null", vec![("E108", 4, 5)]);
test!(test_77, "[10] null", vec![("E108", 5, 6)]);
test!(test_78, " [10] null", vec![("E108", 6, 7)]);
test!(test_79, " \n\r [10] \r\nnull", vec![("E108", 11, 12)]);

// array number
test!(test_80, "[10]0", vec![("E109", 4, 5)]);
test!(test_81, "[10] 0", vec![("E109", 5, 6)]);
test!(test_82, " [10] 0", vec![("E109", 6, 7)]);
test!(test_83, " \n\r [10] \t0", vec![("E109", 10, 11)]);

// array string
test!(test_84, "[10]\"\"", vec![("E114", 4, 5)]);
test!(test_85, "[10] \"\"", vec![("E114", 5, 6)]);
test!(test_86, " [10] \"\"", vec![("E114", 6, 7)]);
test!(test_87, " \n\r [10] \t\"\"", vec![("E114", 10, 11)]);

// array array
test!(test_88, "[10][]", vec![("E125", 4, 5)]);
test!(test_89, "[10] []", vec![("E125", 5, 6)]);
test!(test_90, " [10] []", vec![("E125", 6, 7)]);
test!(test_91, " \n\r [10] \t[]", vec![("E125", 10, 11)]);

// array value-separator
test!(test_92, "[10],", vec![("E124", 4, 5)]);
test!(test_93, "[10] ,", vec![("E124", 5, 6)]);
test!(test_94, " [10] ,", vec![("E124", 6, 7)]);
test!(test_95, " \n\r [10] \t,", vec![("E124", 10, 11)]);

// Trailing comma
test!(test_96, "[true,]", vec![("E129", 6, 7)]);
test!(test_97, "[false,]", vec![("E129", 7, 8)]);
test!(test_98, "[null,]", vec![("E129", 6, 7)]);
test!(test_99, "[1000,]", vec![("E129", 6, 7)]);
test!(test_100, "[\"00\",]", vec![("E129", 6, 7)]);
test!(test_101, "[[10],]", vec![("E129", 6, 7)]);

// Double trailing comma
test!(test_102, "[true,,]", vec![("E123", 6, 7)]);
test!(test_103, "[false,,]", vec![("E123", 7, 8)]);
test!(test_104, "[null,,]", vec![("E123", 6, 7)]);
test!(test_105, "[1000,,]", vec![("E123", 6, 7)]);
test!(test_106, "[\"00\",,]", vec![("E123", 6, 7)]);
test!(test_107, "[[10],,]", vec![("E123", 6, 7)]);

// Trailing comma in unterminated array
test!(test_108, "[true,", vec![("E127", 5, 6)]);
test!(test_109, "[false,", vec![("E127", 6, 7)]);
test!(test_110, "[null,", vec![("E127", 5, 6)]);
test!(test_111, "[1000,", vec![("E127", 5, 6)]);
test!(test_112, "[\"00\",", vec![("E127", 5, 6)]);
test!(test_113, "[[10],", vec![("E127", 5, 6)]);

// Double trailing comma in unterminated array
test!(test_114, "[true,,", vec![("E123", 6, 7)]);
test!(test_115, "[false,,", vec![("E123", 7, 8)]);
test!(test_116, "[null,,", vec![("E123", 6, 7)]);
test!(test_117, "[1000,,", vec![("E123", 6, 7)]);
test!(test_118, "[\"00\",,", vec![("E123", 6, 7)]);
test!(test_119, "[[10],,", vec![("E123", 6, 7)]);
