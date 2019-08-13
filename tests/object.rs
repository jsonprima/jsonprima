mod helpers;

use jsonprima;

// {
test!(test_1, "{", vec![("E128", 0, 1)]);
test!(test_2, " {", vec![("E128", 1, 2)]);
test!(test_3, "{ ", vec![("E128", 1, 2)]);
test!(test_4, " { ", vec![("E128", 2, 3)]);

// }
test!(test_5, "}", vec![("E131", 0, 1)]);
test!(test_6, " }", vec![("E131", 1, 2)]);
test!(test_7, "} ", vec![("E131", 0, 1)]);
test!(test_8, " } ", vec![("E131", 1, 2)]);

// {}
test!(test_9, "{}", vec![]);
test!(test_10, " {}", vec![]);
test!(test_11, "{} ", vec![]);
test!(test_12, " { \n \n \r \t} ", vec![]);

// {}}
test!(test_13, "{}}", vec![("E131", 2, 3)]);
test!(test_14, " {}}", vec![("E131", 3, 4)]);
test!(test_15, "{}} ", vec![("E131", 2, 3)]);
test!(test_16, " {}} ", vec![("E131", 3, 4)]);

// Wrong object names
test!(test_17, "{true}", vec![("E138", 1, 2)]);
test!(test_18, "{false}", vec![("E139", 1, 2)]);
test!(test_19, "{null}", vec![("E140", 1, 2)]);
test!(test_20, "{0}", vec![("E137", 1, 2)]);
test!(test_21, "{[]}", vec![("E125", 1, 2)]);
test!(test_22, "{{}}", vec![("E130", 1, 2)]);

test!(test_23, "{\"\": 0, true}", vec![("E138", 8, 9)]);
test!(test_24, "{\"\": 0, false}", vec![("E139", 8, 9)]);
test!(test_25, "{\"\": 0, null}", vec![("E140", 8, 9)]);
test!(test_26, "{\"\": 0, 0}", vec![("E137", 8, 9)]);
test!(test_27, "{\"\": 0, []}", vec![("E142", 8, 9)]);
test!(test_28, "{\"\": 0, {}}", vec![("E143", 8, 9)]);

// Valid objects
test!(test_29, " { \n \n \"some\": true \r \t} ", vec![]);
test!(test_30, " { \n \n \"some\": false \r \t} ", vec![]);
test!(test_31, " { \n \n \"some\": \n \n  null \r \t} ", vec![]);
test!(test_32, " { \n\"some\": \"\" \r \t} ", vec![]);
test!(test_33, " { \n \n \"some\": 10e+10 \r \t} ", vec![]);
test!(test_34, " { \n \n \"some\":   [] \r \t} ", vec![]);
test!(test_35, " { \n \n \"some\":   {} \r \t} ", vec![]);

// Wrong object members, no member value
test!(test_36, "{\"\" : }", vec![("E134", 6, 7)]);
test!(test_37, "{\"\" : ", vec![("E141", 5, 6)]);
test!(test_38, "{\"\" : :", vec![("E136", 6, 7)]);
test!(test_39, "{\"\" : ,", vec![("E123", 6, 7)]);

// Trailing comma
test!(test_40, "{\"\" : 0, }", vec![("E132", 9, 10)]);
test!(test_41, "{\"\" : 0, , }", vec![("E123", 9, 10)]);

// Colon rather than comma
test!(test_42, "{\"\" : 0  : 0 }", vec![("E136", 9, 10)]);

// Valid nested objects
test!(test_43, "{\"\" : {} }", vec![]);

// Duplicates
test!(test_44, "{\"\" : { \"\": 0} }", vec![]);
test!(test_45, "{\"\" : { \"\": true} }", vec![]);
test!(test_46, "{\"\" : { \"\": false} }", vec![]);
test!(test_47, "{\"\" : { \"\": null} }", vec![]);
test!(test_48, "{\"\" : { \"\": \"\"} }", vec![]);
test!(test_49, "{\"\" : { \"\": []} }", vec![]);
test!(test_50, "{\"\" : { \"\": {}} }", vec![]);

test!(test_51, "{ \"\": \"\", \"\": \"\"}", vec![("E144", 10, 12)]);
test!(
  test_52,
  "{ \"\": \"\", \"\": \"a\"}",
  vec![("E144", 10, 12)]
);
test!(
  test_53,
  "{ \"\": \"a\", \"\": \"\"}",
  vec![("E144", 11, 13)]
);
test!(test_54, "{ \"a\": \"\", \"\": \"\"}", vec![]);
test!(test_55, "{ \"\": \"\", \"a\": \"\"}", vec![]);

test!(test_56, "{ \"\": {\"\": \"\"}, \"a\": \"\"}", vec![]);
test!(
  test_57,
  "{ \"aw\": {\"\": \"\", \"\": \"\"}, \"a\": \"\"}",
  vec![("E144", 17, 19)]
);

test!(
  test_58,
  "{ \"a\\\\b\": \"\", \"a\\u005Cb\": \"\"}",
  vec![("E144", 14, 24)]
);
