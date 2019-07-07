mod helpers;

use jsonprima;

// Unterminated string without graphemes.
test!(test_0, "\"", vec![("E104", 0, 1)]);

// Unterminated string with graphemes.
test!(test_1, "\"aade\\u002c", vec![("E104", 0, 11)]);

// One raw horizontal tab character.
test!(test_2, "\"\t\"", vec![("E101", 0, 2)]);

// Multiple raw horizontal tab characters.
test!(test_3, "\"\t\t\t\"", vec![("E101", 0, 2)]);

// One raw carriage return character.
test!(test_4, "\"\r\"", vec![("E101", 0, 2)]);

// Multiple raw carriage return characters.
test!(test_5, "\"\r\r\r\"", vec![("E101", 0, 2)]);

// One raw new line character.
test!(test_6, "\"\n\"", vec![("E101", 0, 2)]);

// Multiple raw new line characters.
test!(test_7, "\"\n\n\n\"", vec![("E101", 0, 2)]);

// One raw CRLF grapheme.
test!(test_8, "\"\r\n\"", vec![("E101", 0, 2)]);

// Multiple raw new CRLF graphemes.
test!(test_9, "\"\r\n\r\n\r\n\"", vec![("E101", 0, 2)]);

// Start of escape value without more graphemes to parse.
test!(test_10, "\"\\", vec![("E104", 0, 2)]);

// Start of escape value followed by raw horizontal tab character.
test!(test_11, "\"\\\t\"", vec![("E101", 0, 3)]);

// Start of escape value followed by raw carriage return character.
test!(test_12, "\"\\\r\"", vec![("E101", 0, 3)]);

// Start of escape value followed by raw new line character.
test!(test_13, "\"\\\n\"", vec![("E101", 0, 3)]);

// Start of escape value followed by raw CRLF.
test!(test_14, "\"\\\r\n\"", vec![("E101", 0, 3)]);

// Start of escape value followed by invalid escape character.
test!(test_15, "\"\\a\"", vec![("E116", 0, 3)]);

// Valid escape value without more graphemes to parse
test!(test_16, "\"\\n", vec![("E104", 0, 3)]);

// Start of Unicode escape sequence without any HEX digit.
test!(test_17, "\"\\u", vec![("E104", 0, 3)]);

// Start of Unicode escape sequence followed by closing string.
test!(test_18, "\"\\u\"", vec![("E117", 0, 4)]);

// Start of Unicode escape sequence with invalid HEX digit.
test!(test_19, "\"\\ur\"", vec![("E117", 0, 4)]);

// Start of Unicode escape sequence, second HEX digit does not exist.
test!(test_20, "\"\\ua", vec![("E104", 0, 4)]);

// Start of Unicode escape sequence, closing string in position of second HEX digit.
test!(test_21, "\"\\ua\"", vec![("E117", 0, 5)]);

// Start of Unicode escape sequence, second HEX digit invalid.
test!(test_22, "\"\\uar", vec![("E117", 0, 5)]);

// Start of Unicode escape sequence, third HEX digit does not exist.
test!(test_23, "\"\\ua0", vec![("E104", 0, 5)]);

// Start of Unicode escape sequence, closing string in position of third HEX digit.
test!(test_24, "\"\\ua0\"", vec![("E117", 0, 6)]);

// Start of Unicode escape sequence, third HEX digit invalid.
test!(test_25, "\"\\ua0r", vec![("E117", 0, 6)]);

// Start of Unicode escape sequence, fourth HEX digit does not exist.
test!(test_26, "\"\\ua00", vec![("E104", 0, 6)]);

// Start of Unicode escape sequence, closing string in position of fourth HEX digit.
test!(test_27, "\"\\ua00\"", vec![("E117", 0, 7)]);

// Start of Unicode escape sequence, fourth HEX digit invalid.
test!(test_28, "\"\\ua00r", vec![("E117", 0, 7)]);

// Invalid value of Unicode escape sequence.
test!(test_29, "\"\\uffff", vec![("E104", 0, 7)]);

// Second surrogate does not exist in Unicode escape sequences.
test!(test_31, "\"\\uD834", vec![("E119", 0, 7)]);

// Second surrogate does not exist in Unicode escape sequences and closing string.
test!(test_32, "\"\\uD834\"", vec![("E119", 0, 8)]);

// Second surrogate does not exist in Unicode escape sequences.
test!(test_33, "\"\\uD834\\", vec![("E119", 0, 8)]);

// Second surrogate does not exist in Unicode escape sequences and closing string.
test!(test_34, "\"\\uD834\\\"", vec![("E119", 0, 9)]);

// Second surrogate Unicode escape sequence without any HEX digit.
test!(test_35, "\"\\uD834\\u", vec![("E104", 0, 9)]);

// Second surrogate Unicode escape sequence without any HEX digit and closing string.
test!(test_36, "\"\\uD834\\u\"", vec![("E117", 0, 10)]);

// Second surrogate Unicode escape sequence with invalid HEX digit.
test!(test_37, "\"\\uD834\\ur", vec![("E117", 0, 10)]);

// Second surrogate Unicode escape sequence, second HEX digit does not exist.
test!(test_38, "\"\\uD834\\ua", vec![("E104", 0, 10)]);

// Second surrogate Unicode escape sequence, closing string in position of second HEX digit.
test!(test_39, "\"\\uD834\\ua\"", vec![("E117", 0, 11)]);

// Second surrogate Unicode escape sequence, invalid second HEX digit.
test!(test_40, "\"\\uD834\\uar", vec![("E117", 0, 11)]);

// Second surrogate Unicode escape sequence, third HEX digit does not exist.
test!(test_41, "\"\\uD834\\ua0", vec![("E104", 0, 11)]);

// Second surrogate Unicode escape sequence, closing string in position of third HEX digit.
test!(test_42, "\"\\uD834\\ua0\"", vec![("E117", 0, 12)]);

// Second surrogate Unicode escape sequence, invalid third HEX digit.
test!(test_43, "\"\\uD834\\ua0r", vec![("E117", 0, 12)]);

// Second surrogate Unicode escape sequence, fourth HEX digit does not exist.
test!(test_44, "\"\\uD834\\ua00", vec![("E104", 0, 12)]);

// Second surrogate Unicode escape sequence, closing string in position of fourth HEX digit.
test!(test_45, "\"\\uD834\\ua00\"", vec![("E117", 0, 13)]);

// Second surrogate Unicode escape sequence, invalid fourth HEX digit.
test!(test_46, "\"\\uD834\\ua00r", vec![("E117", 0, 13)]);

// Valid surrogate pair, unclosing string.
test!(test_48, "\"\\uD834\\uDD1E", vec![("E104", 0, 13)]);

// Invalid escape Emoji within string.
test!(test_49, "\"\\🌀\"", vec![("E116", 0, 3)]);

// Grapheme before string.
test!(test_50, "a\"\"", vec![("E106", 0, 1)]);

// Invalid character (\u0025) before string.
test!(test_51, "\u{0025}\"asd\"", vec![("E106", 0, 1)]);

// Invalid Unicode.
test!(test_52, "\"\\udead\"", vec![("E119", 0, 8)]);

// Non ASCII hex in first character in Unicode escape sequence
test!(test_53, "\"\\ur\"", vec![("E117", 0, 4)]);

// Non ASCII hex in second character in Unicode escape sequence
test!(test_54, "\"\\uar\"", vec![("E117", 0, 5)]);

// Non ASCII hex in third character in Unicode escape sequence
test!(test_55, "\"\\uaar\"", vec![("E117", 0, 6)]);

// Non ASCII hex in fourth character in Unicode escape sequence
test!(test_56, "\"\\uaaar\"", vec![("E117", 0, 7)]);

// string string
test!(test_57, "\n \"\"  \n \"\"", vec![("E114", 8, 9)]);

// string true
test!(test_58, "\"\"true", vec![("E103", 2, 3)]);
test!(test_59, "\"\" true", vec![("E103", 3, 4)]);
test!(test_60, " \"\" true", vec![("E103", 4, 5)]);
test!(test_61, " \n\r \"\" \ttrue", vec![("E103", 8, 9)]);

// string false
test!(test_62, "\"\"false", vec![("E107", 2, 3)]);
test!(test_63, "\"\" false", vec![("E107", 3, 4)]);
test!(test_64, " \"\" false", vec![("E107", 4, 5)]);
test!(test_65, " \n\r \"\" \nfalse", vec![("E107", 8, 9)]);

// string null
test!(test_66, "\"\"null", vec![("E108", 2, 3)]);
test!(test_67, "\"\" null", vec![("E108", 3, 4)]);
test!(test_68, " \"\" null", vec![("E108", 4, 5)]);
test!(test_69, " \n\r \"\" \tnull", vec![("E108", 8, 9)]);

// string number
test!(test_70, "\"\"0", vec![("E109", 2, 3)]);
test!(test_71, "\"\" 0", vec![("E109", 3, 4)]);
test!(test_72, " \"\" 0", vec![("E109", 4, 5)]);
test!(test_73, " \n\r \"\" \t0", vec![("E109", 8, 9)]);

// string begin-array
test!(test_74, "\"\"[", vec![("E125", 2, 3)]);
test!(test_75, "\"\" [", vec![("E125", 3, 4)]);
test!(test_76, " \"\" [", vec![("E125", 4, 5)]);
test!(test_77, " \n\r \"\" \t[", vec![("E125", 8, 9)]);

// string end-array
test!(test_78, "\"\"]", vec![("E126", 2, 3)]);
test!(test_79, "\"\" ]", vec![("E126", 3, 4)]);
test!(test_80, " \"\" ]", vec![("E126", 4, 5)]);
test!(test_81, " \n\r \"\" \t]", vec![("E126", 8, 9)]);

// string value-separator
test!(test_82, "\"\",", vec![("E124", 2, 3)]);
test!(test_83, "\"\" ,", vec![("E124", 3, 4)]);
test!(test_84, " \"\" ,", vec![("E124", 4, 5)]);
test!(test_85, " \n\r \"\" \t,", vec![("E124", 8, 9)]);
