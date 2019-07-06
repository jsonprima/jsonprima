mod helpers;

use jsonprima;

// Empty document.
test!(test_1, "", vec![("E100", 0, 1)]);

// Empty document with Space characters.
test!(test_2, " ", vec![("E100", 0, 1)]);
test!(test_3, "    ", vec![("E100", 3, 4)]);

// Empty document with New Line characters.
test!(test_4, "\n", vec![("E100", 0, 1)]);
test!(test_5, "\n\n\n", vec![("E100", 2, 3)]);

// Empty document with Carriage Return characters.
test!(test_6, "\r", vec![("E100", 0, 1)]);
test!(test_7, "\r\r\r", vec![("E100", 2, 3)]);

// Empty document with Horizontal Tab characters.
test!(test_8, "\t", vec![("E100", 0, 1)]);
test!(test_9, "\t\t\t", vec![("E100", 2, 3)]);

// Empty document multiple whitespace characters.
test!(test_10, "  \t \n \r \r\n \t\t ", vec![("E100", 13, 14)]);
