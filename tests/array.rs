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
