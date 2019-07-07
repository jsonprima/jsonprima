mod helpers;

use jsonprima;

// Empty string.
test!(test_0, "\"\"", vec![]);

// A single space as only value within string.
test!(test_1, "\" \"", vec![]);

// Multiple spaces as only values within string.
test!(test_2, "\"     \"", vec![]);

// Reverse solidus.
test!(test_3, "\"\\\\\"", vec![]);

// Multiple reverse solidus.
test!(test_4, "\"\\\\\\\\\"", vec![]);

// Unescaped solidus.
test!(test_5, "\"/\"", vec![]);

// Multiple unescaped solidus.
test!(test_6, "\"///\"", vec![]);

// Escaped solidus.
test!(test_7, "\"\\/\"", vec![]);

// Multiple escaped solidus.
test!(test_8, "\"\\/\\/\\/\"", vec![]);

// \b.
test!(test_9, "\"\\b\"", vec![]);

// Multiple \b.
test!(test_10, "\"\\b\\b\\b\"", vec![]);

// \f.
test!(test_11, "\"\\f\"", vec![]);

// Multiple \f.
test!(test_12, "\"\\f\\f\\f\"", vec![]);

// Quotation mark.
test!(test_13, "\"\\\"\"", vec![]);

// Multiple quotation marks.
test!(test_14, "\"\\\"\\\"\\\"\"", vec![]);

// \n.
test!(test_15, "\"\\n\"", vec![]);

// Multiple \n.
test!(test_16, "\"\\n\\n\\n\"", vec![]);

// \r.
test!(test_17, "\"\\r\"", vec![]);

// Multiple \r.
test!(test_18, "\"\\r\\r\\r\"", vec![]);

// \t.
test!(test_19, "\"\\f\"", vec![]);

// Multiple \t.
test!(test_20, "\"\\t\\t\\t\"", vec![]);

// U+0000.
test!(test_21, "\"\\u0000\"", vec![]);

// 3 byte UTF-8 characters.
test!(test_22, "\"\\u0060\\u012a\\u12AB\"", vec![]);

// Valid surrogate pair.
test!(test_23, "\"\\uD801\\udc37\"", vec![]);

// Two surrogate pairs, one after the other.
test!(test_24, "\"\\ud83d\\ude39\\ud83d\\udc8d\"", vec![]);

// U escaped.
test!(test_25, "\"\\\\u0000\"", vec![]);

// Double escaped a.
test!(test_26, "\"\\\\a\"", vec![]);

// Double escaped n.
test!(test_27, "\"\\\\n\"", vec![]);

// Escape control character U+0012.
test!(test_28, "\"\\u0012\"", vec![]);

// Escape non character U+FFFF.
test!(test_29, "\"\\uFFFF\"", vec![]);

// Last surrogates 1 and 2.
test!(test_30, "\"\\uDBFF\\uDFFF\"", vec![]);

// Escape whitespace character U+u00A0 within string.
test!(test_31, "\"new\\u00A0line\"", vec![]);

// One byte UTF-8 U+002c.
test!(test_32, "\"\\u002c\"", vec![]);

// G glef.
test!(test_33, "\"\\uD834\\uDD1E\"", vec![]);

// Zero width escape value U+200B.
test!(test_34, "\"\\u200B\"", vec![]);

// Non UTF-8 character U+10FFFF.
test!(test_35, "\"􏿿\"", vec![]);

// Non UTF-8 character U+FFFF.
test!(test_36, "\"￿\"", vec![]);

// Character PI.
test!(test_37, "\"π\"", vec![]);

// simple ASCII.
test!(test_38, "\"asd \"", vec![]);

// del character.
test!(test_39, "\"aa\"", vec![]);

// Unescaped solidus (/).
test!(test_40, "\"/\"", vec![]);

// ISO Latin 1 character.
test!(test_41, "\"�\"", vec![]);

// Comments.
test!(test_42, "\"a/*b*/c/*d//e\"", vec![]);

// Combining marks.
test!(test_43, "\"\\u0061\\u030A\"", vec![]);

// Ignore byte order mark.
test!(test_44, "\u{feff}  \"\"", vec![]);
