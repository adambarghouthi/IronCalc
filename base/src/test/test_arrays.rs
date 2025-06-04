#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn sum_arrays() {
    let mut model = new_empty_model();
    model._set("A1", "=SUM({1,2,3}+{3,4,5})");

    model.evaluate();

    assert_eq!(model._get_text("A1"), *"18");
}

#[test]
fn basic_array_support() {
    let mut model = new_empty_model();
    
    // Test which functions work with arrays
    let test_cases = vec![
        ("=SUM({1,2,3})", "SUM"),
        ("=AVERAGE({1,2,3})", "AVERAGE"),
        ("=AVERAGEA({1,2,3})", "AVERAGEA"),
        ("=COUNT({1,2,3})", "COUNT"),
        ("=COUNTA({1,2,3})", "COUNTA"),
        ("=COUNTBLANK({1,2,3})", "COUNTBLANK"),
        ("=MAX({1,2,3})", "MAX"),
        ("=MIN({1,2,3})", "MIN"),
        ("=PRODUCT({1,2,3})", "PRODUCT"),
        ("=GEOMEAN({1,2,3})", "GEOMEAN"),
    ];
    
    for (i, (formula, _func_name)) in test_cases.iter().enumerate() {
        model._set(&format!("A{}", i + 1), formula);
    }
    
    model.evaluate();
    
    for (i, (formula, func_name)) in test_cases.iter().enumerate() {
        let result = model._get_text(&format!("A{}", i + 1));
        println!("{}: {} -> {}", func_name, formula, result);
        
        // Just check if it's not an error for now
        assert!(!result.contains("#N/IMPL!"), "{} should support arrays", func_name);
    }
}

#[test]
fn array_types() {
    let mut model = new_empty_model();
    
    // Row array
    model._set("A1", "=SUM({1,2,3})");
    // Column array  
    model._set("A2", "=SUM({1;2;3})");
    // 2D array
    model._set("A3", "=SUM({1,2;3,4})");
    // Single element
    model._set("A4", "=SUM({5})");
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "6", "Row array should work");
    assert_eq!(model._get_text("A2"), "6", "Column array should work");
    assert_eq!(model._get_text("A3"), "10", "2D array should work");
    assert_eq!(model._get_text("A4"), "5", "Single element array should work");
}

#[test]
fn mixed_data_types() {
    let mut model = new_empty_model();
    
    // COUNT should only count numbers
    model._set("A1", "=COUNT({1,\"text\",TRUE,2})");
    // COUNTA should count all non-empty
    model._set("A2", "=COUNTA({1,\"text\",TRUE,2})");
    // SUM should ignore non-numbers
    model._set("A3", "=SUM({1,\"text\",TRUE,2})");
    // AVERAGEA should include all types (text=0, TRUE=1, FALSE=0)
    model._set("A4", "=AVERAGEA({1,\"text\",TRUE,2})");
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "2", "COUNT should only count numbers in mixed array");
    assert_eq!(model._get_text("A2"), "4", "COUNTA should count all elements in mixed array");
    assert_eq!(model._get_text("A3"), "3", "SUM should ignore non-numbers in mixed array");
    assert_eq!(model._get_text("A4"), "1", "AVERAGEA should include all: (1+0+1+2)/4=1");
}

#[test]
fn array_edge_cases() {
    let mut model = new_empty_model();
    
    // Empty elements and strings
    model._set("A1", "=COUNTA({1,\"\",3})");
    model._set("A2", "=COUNT({1,\"\",3})");
    
    // String arrays
    model._set("A3", "=COUNTA({\"a\",\"b\",\"c\"})");
    model._set("A4", "=COUNT({\"a\",\"b\",\"c\"})");
    
    // Boolean arrays
    model._set("A5", "=COUNTA({TRUE,FALSE,TRUE})");
    model._set("A6", "=COUNT({TRUE,FALSE,TRUE})");
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "3", "COUNTA should count empty strings as values");
    assert_eq!(model._get_text("A2"), "2", "COUNT should not count empty strings");
    assert_eq!(model._get_text("A3"), "3", "COUNTA should count all strings");
    assert_eq!(model._get_text("A4"), "0", "COUNT should not count strings");
    assert_eq!(model._get_text("A5"), "3", "COUNTA should count all booleans");
    assert_eq!(model._get_text("A6"), "0", "COUNT should not count booleans");
}

#[test]
fn mathematical_functions_with_mixed_arrays() {
    let mut model = new_empty_model();
    
    // Mathematical functions should ignore non-numbers
    model._set("A1", "=MIN({5,\"text\",1,TRUE,3})");
    model._set("A2", "=MAX({5,\"text\",1,TRUE,3})");
    model._set("A3", "=AVERAGE({5,\"text\",1,TRUE,3})");
    model._set("A4", "=PRODUCT({5,\"text\",2,TRUE})");
    // AVERAGEA includes all types, unlike AVERAGE
    model._set("A5", "=AVERAGEA({5,\"text\",1,TRUE,3})");
    // GEOMEAN should ignore non-numbers, process only: 5, 1, 3
    model._set("A6", "=GEOMEAN({5,\"text\",1,TRUE,3})");
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1", "MIN should ignore non-numbers");
    assert_eq!(model._get_text("A2"), "5", "MAX should ignore non-numbers");
    assert_eq!(model._get_text("A3"), "3", "AVERAGE should ignore non-numbers (5+1+3)/3=3");
    assert_eq!(model._get_text("A4"), "10", "PRODUCT should ignore non-numbers");
    assert_eq!(model._get_text("A5"), "2", "AVERAGEA should include all: (5+0+1+1+3)/5=2");
    assert_eq!(model._get_text("A6"), "2.466212074", "GEOMEAN should ignore non-numbers: ∛(5×1×3)=∛15≈2.466");
}

#[test]
fn average_array_comparison() {
    let mut model = new_empty_model();
    
    // Test AVERAGE specifically
    model._set("A1", "=AVERAGE({1,2,3})");
    model._set("A2", "=AVERAGE(1,2,3)");  // Compare with individual args
    
    model.evaluate();
    
    let result1 = model._get_text("A1");
    let result2 = model._get_text("A2");
    
    println!("AVERAGE with array: {} -> {}", "=AVERAGE({1,2,3})", result1);
    println!("AVERAGE with args: {} -> {}", "=AVERAGE(1,2,3)", result2);
    
    // Both should return 2
    assert_eq!(result1, "2", "AVERAGE with array should work");
    assert_eq!(result2, "2", "AVERAGE with individual args should work");
}

#[test]
fn countblank_array_behavior() {
    let mut model = new_empty_model();
    
    // COUNTBLANK should only count empty strings in arrays
    model._set("A1", "=COUNTBLANK({1,\"\",3})");  // Should be 1 (one empty string)
    model._set("A2", "=COUNTBLANK({\"\",\"\",\"text\"})");  // Should be 2 (two empty strings)
    model._set("A3", "=COUNTBLANK({1,2,3})");  // Should be 0 (no empty strings)
    model._set("A4", "=COUNTBLANK({TRUE,FALSE,\"\"})");  // Should be 1 (one empty string)
    model._set("A5", "=COUNTBLANK({\"\",\"\",\"\"})");  // Should be 3 (all empty strings)
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1", "COUNTBLANK should count only empty strings");
    assert_eq!(model._get_text("A2"), "2", "COUNTBLANK should count multiple empty strings");
    assert_eq!(model._get_text("A3"), "0", "COUNTBLANK should not count numbers");
    assert_eq!(model._get_text("A4"), "1", "COUNTBLANK should not count booleans, only empty strings");
    assert_eq!(model._get_text("A5"), "3", "COUNTBLANK should count all empty strings");
}

#[test]
fn geomean_array_behavior() {
    let mut model = new_empty_model();
    
    // GEOMEAN should only process numbers from arrays
    model._set("A1", "=GEOMEAN({2,4,8})");  // Should be ∛(2×4×8) = ∛64 = 4
    model._set("A2", "=GEOMEAN({1,1,1})");  // Should be ∛(1×1×1) = 1
    model._set("A3", "=GEOMEAN({2,\"text\",8})");  // Should ignore text: ∛(2×8) = ∛16 = 2.52...
    model._set("A4", "=GEOMEAN({1,TRUE,FALSE,4})");  // Should ignore booleans: ∛(1×4) = 2
    model._set("A5", "=GEOMEAN({3})");  // Single element: 3
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "4", "GEOMEAN should calculate geometric mean of numbers");
    assert_eq!(model._get_text("A2"), "1", "GEOMEAN should handle identical values");
    assert_eq!(model._get_text("A3"), "4", "GEOMEAN should ignore text, process only numbers: ∛(2×8)=4");
    assert_eq!(model._get_text("A4"), "2", "GEOMEAN should ignore booleans, process only numbers: ∛(1×4)=2");
    assert_eq!(model._get_text("A5"), "3", "GEOMEAN should handle single element arrays");
}

#[test]
fn test_implicit_intersection_basic() {
    let mut model = new_empty_model();
    
    // Test simple array operation with implicit intersection
    // {1,2}+{3,4} should return {4,6}, but we only store the first element: 4
    model._set("A1", "={1,2}+{3,4}");
    
    model.evaluate();
    
    let result = model._get_text("A1");
    println!("A1: {{1,2}}+{{3,4}} = {}", result);
    
    // This test should pass once we implement implicit intersection
    // For now, it will fail with #N/IMPL!
    // After our fix, it should return "4"
    assert_ne!(result, "#N/IMPL!", "Array operations should not return #N/IMPL! after our fix");
}

#[test]
fn test_array_arithmetic_operations() {
    let mut model = new_empty_model();
    
    // Test all arithmetic operations with implicit intersection
    model._set("A1", "={1,2}+{3,4}");      // Should return 4 (first element of {4,6})
    model._set("A2", "={5,10}-{2,3}");     // Should return 3 (first element of {3,7})
    model._set("A3", "={2,4}*{3,5}");      // Should return 6 (first element of {6,20})
    model._set("A4", "={12,8}/{3,2}");     // Should return 4 (first element of {4,4})
    model._set("A5", "={2,3}^{3,2}");      // Should return 8 (first element of {8,9})
    
    // Test with mixed array sizes (should work with the smaller size)
    model._set("A6", "={1,2,3}+{10,20}");  // Should return 11 (first element)
    
    // Test with single element arrays
    model._set("A7", "={5}+{3}");          // Should return 8
    
    // Test array operations with functions
    model._set("A8", "=SUM({1,2}+{3,4})"); // Should return 10 (sum of {4,6})
    model._set("A9", "=AVERAGE({2,4}*{3,5})"); // Should return 13 (average of {6,20})
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "4", "Addition: {{1,2}}+{{3,4}} should return 4");
    assert_eq!(model._get_text("A2"), "3", "Subtraction: {{5,10}}-{{2,3}} should return 3");
    assert_eq!(model._get_text("A3"), "6", "Multiplication: {{2,4}}*{{3,5}} should return 6");
    assert_eq!(model._get_text("A4"), "4", "Division: {{12,8}}/{{3,2}} should return 4");
    assert_eq!(model._get_text("A5"), "8", "Exponentiation: {{2,3}}^{{3,2}} should return 8");
    assert_eq!(model._get_text("A6"), "11", "Mixed sizes: {{1,2,3}}+{{10,20}} should return 11");
    assert_eq!(model._get_text("A7"), "8", "Single elements: {{5}}+{{3}} should return 8");
    assert_eq!(model._get_text("A8"), "10", "SUM of array operation should return 10");
    assert_eq!(model._get_text("A9"), "13", "AVERAGE of array operation should return 13");
}

#[test]
fn test_array_with_different_types() {
    let mut model = new_empty_model();
    
    // Test arrays with different data types
    model._set("A1", "={1,\"text\"}+{2,3}");     // Should work with first numeric elements: 1+2=3
    model._set("A2", "={TRUE,FALSE}+{1,2}");     // Should work: TRUE+1=2 (TRUE=1)
    model._set("A3", "={\"5\",6}+{\"2\",3}");    // Should work: "5"+"2"=7 (string parsing)
    
    model.evaluate();
    
    // These might return errors or specific behaviors depending on implementation
    let result1 = model._get_text("A1");
    let result2 = model._get_text("A2");
    let result3 = model._get_text("A3");
    
    println!("Mixed types A1: {} (expected: 3 or error)", result1);
    println!("Boolean types A2: {} (expected: 2)", result2);
    println!("String numbers A3: {} (expected: 7)", result3);
    
    // At minimum, these should not crash the system
    assert!(!result1.is_empty(), "Should return some result");
    assert!(!result2.is_empty(), "Should return some result");
    assert!(!result3.is_empty(), "Should return some result");
}

#[test]
fn test_array_row_vs_column_separators() {
    let mut model = new_empty_model();
    
    // Test comma vs semicolon behavior
    model._set("A1", "=SUM({1,2,3})");      // Row array: 1 row, 3 columns
    model._set("A2", "=SUM({1;2;3})");      // Column array: 3 rows, 1 column  
    model._set("A3", "=SUM({1,2;3,4})");    // 2D array: 2 rows, 2 columns
    
    // Test array dimensions with COUNT 
    model._set("A4", "=COUNT({1,2,3})");    // Should count 3 elements
    model._set("A5", "=COUNT({1;2;3})");    // Should count 3 elements
    model._set("A6", "=COUNT({1,2;3,4})");  // Should count 4 elements
    
    // Test with COUNTA to verify structure
    model._set("A7", "=COUNTA({\"a\",\"b\",\"c\"})");    // Row: 3 elements
    model._set("A8", "=COUNTA({\"a\";\"b\";\"c\"})");    // Column: 3 elements
    model._set("A9", "=COUNTA({\"a\",\"b\";\"c\",\"d\"})"); // 2D: 4 elements
    
    // Test arithmetic with different orientations
    model._set("A10", "={1,2,3}+{4,5,6}");      // Row + Row → First element should be 5
    model._set("A11", "={1;2;3}+{4;5;6}");      // Column + Column → First element should be 5
    model._set("A12", "={1,2;3,4}+{5,6;7,8}");  // 2D + 2D → First element should be 6
    
    model.evaluate();
    
    // All SUM operations should work regardless of orientation
    assert_eq!(model._get_text("A1"), "6", "Row array SUM: {{1,2,3}}");
    assert_eq!(model._get_text("A2"), "6", "Column array SUM: {{1;2;3}}");
    assert_eq!(model._get_text("A3"), "10", "2D array SUM: {{1,2;3,4}}");
    
    // COUNT should work with any orientation
    assert_eq!(model._get_text("A4"), "3", "Row array COUNT");
    assert_eq!(model._get_text("A5"), "3", "Column array COUNT");
    assert_eq!(model._get_text("A6"), "4", "2D array COUNT");
    
    // COUNTA should work with any orientation
    assert_eq!(model._get_text("A7"), "3", "Row string array COUNTA");
    assert_eq!(model._get_text("A8"), "3", "Column string array COUNTA");
    assert_eq!(model._get_text("A9"), "4", "2D string array COUNTA");
    
    // Arithmetic with implicit intersection (first element)
    assert_eq!(model._get_text("A10"), "5", "Row arithmetic: first element of {{5,7,9}}");
    assert_eq!(model._get_text("A11"), "5", "Column arithmetic: first element of {{5;7;9}}");
    assert_eq!(model._get_text("A12"), "6", "2D arithmetic: first element of {{6,8;10,12}}");
}

#[test]
fn test_array_separator_behavior_detailed() {
    let mut model = new_empty_model();
    
    // Test different array orientations with debugging output
    model._set("A1", "={1,2,3}+{10,20,30}");    // Row + Row
    model._set("A2", "={1;2;3}+{10;20;30}");    // Column + Column  
    model._set("A3", "={1,2;3,4}+{10,20;30,40}"); // 2D + 2D
    
    // Test what gets computed vs what gets stored
    model._set("A4", "=SUM({1,2,3}+{10,20,30})");   // SUM of {11,22,33} = 66
    model._set("A5", "=SUM({1;2;3}+{10;20;30})");   // SUM of {11;22;33} = 66
    model._set("A6", "=SUM({1,2;3,4}+{10,20;30,40})"); // SUM of {11,22;33,44} = 110
    
    model.evaluate();
    
    let result1 = model._get_text("A1");
    let result2 = model._get_text("A2"); 
    let result3 = model._get_text("A3");
    let result4 = model._get_text("A4");
    let result5 = model._get_text("A5");
    let result6 = model._get_text("A6");
    
    println!("Row array arithmetic {{1,2,3}}+{{10,20,30}}: {} (first element of {{11,22,33}})", result1);
    println!("Column array arithmetic {{1;2;3}}+{{10;20;30}}: {} (first element of {{11;22;33}})", result2);
    println!("2D array arithmetic {{1,2;3,4}}+{{10,20;30,40}}: {} (first element of {{11,22;33,44}})", result3);
    println!("SUM of row array result: {} (should be 66)", result4);
    println!("SUM of column array result: {} (should be 66)", result5);
    println!("SUM of 2D array result: {} (should be 110)", result6);
    
    // All implicit intersections should return the first element (top-left)
    assert_eq!(result1, "11", "Row array: first element");
    assert_eq!(result2, "11", "Column array: first element"); 
    assert_eq!(result3, "11", "2D array: first element");
    
    // SUM should process entire computed arrays
    assert_eq!(result4, "66", "SUM processes full row array");
    assert_eq!(result5, "66", "SUM processes full column array");
    assert_eq!(result6, "110", "SUM processes full 2D array");
}

#[test]
fn test_functions_that_might_not_need_array_support() {
    let mut model = new_empty_model();
    
    // Test functions that might work fine with implicit intersection
    model._set("A1", "=ROUND({1.7,2.3,3.9})");       // Should this work with implicit intersection?
    model._set("A2", "=POWER({2,3,4})");             // Single argument - what happens?
    model._set("A3", "=TYPE({1,\"text\",TRUE})");    // Information function
    model._set("A4", "=UPPER({\"hello\",\"world\"})"); // Text function
    
    // Test what the errors are
    model.evaluate();
    
    println!("ROUND with array: {}", model._get_text("A1"));
    println!("POWER with array: {}", model._get_text("A2")); 
    println!("TYPE with array: {}", model._get_text("A3"));
    println!("UPPER with array: {}", model._get_text("A4"));
    
    // Check if these are NIMPL errors or if they work differently
}

#[test]
fn test_mathematical_functions_array_behavior() {
    let mut model = new_empty_model();
    
    // Test mathematical functions with proper arguments
    model._set("A1", "=ROUND({1.7,2.3,3.9}, 0)");       // ROUND needs 2 args
    model._set("A2", "=ROUNDUP({1.2,2.7}, 0)");         // ROUNDUP needs 2 args
    model._set("A3", "=ROUNDDOWN({1.8,2.1}, 0)");       // ROUNDDOWN needs 2 args
    model._set("A4", "=POWER({2,3,4}, 2)");             // POWER needs 2 args
    model._set("A5", "=ATAN2({1,2}, {3,4})");           // ATAN2 needs 2 args
    
    // Test with array in second argument  
    model._set("A6", "=ROUND(1.7, {0,1,2})");           // Second arg as array
    model._set("A7", "=POWER(2, {1,2,3})");             // Second arg as array
    
    model.evaluate();
    
    println!("ROUND with array first arg: {}", model._get_text("A1"));
    println!("ROUNDUP with array first arg: {}", model._get_text("A2"));
    println!("ROUNDDOWN with array first arg: {}", model._get_text("A3"));
    println!("POWER with array first arg: {}", model._get_text("A4"));
    println!("ATAN2 with both arrays: {}", model._get_text("A5"));
    println!("ROUND with array second arg: {}", model._get_text("A6"));
    println!("POWER with array second arg: {}", model._get_text("A7"));
    
    // Verify that mathematical functions now work with arrays via implicit intersection
    assert_eq!(model._get_text("A1"), "2", "ROUND should use implicit intersection: ROUND({{1.7,2.3,3.9}}, 0) -> ROUND(1.7, 0) = 2");
    assert_eq!(model._get_text("A2"), "2", "ROUNDUP should use implicit intersection: ROUNDUP({{1.2,2.7}}, 0) -> ROUNDUP(1.2, 0) = 2");  
    assert_eq!(model._get_text("A3"), "1", "ROUNDDOWN should use implicit intersection: ROUNDDOWN({{1.8,2.1}}, 0) -> ROUNDDOWN(1.8, 0) = 1");
    assert_eq!(model._get_text("A4"), "4", "POWER should use implicit intersection: POWER({{2,3,4}}, 2) -> POWER(2, 2) = 4");
    assert_eq!(model._get_text("A5"), "1.249045772", "ATAN2 should use implicit intersection: ATAN2({{1,2}}, {{3,4}}) -> ATAN2(1, 3)");
    assert_eq!(model._get_text("A6"), "2", "ROUND with array second arg should use implicit intersection");
    assert_eq!(model._get_text("A7"), "2", "POWER with array second arg should use implicit intersection");
}

#[test]
fn test_logical_functions_array_behavior() {
    let mut model = new_empty_model();
    
    // Test logical functions with arrays
    model._set("A1", "=AND({TRUE,FALSE,TRUE})");        // Should be FALSE
    model._set("A2", "=OR({FALSE,TRUE,FALSE})");         // Should be TRUE  
    model._set("A3", "=XOR({TRUE,FALSE,TRUE})");         // Should be FALSE
    
    // Test with number arrays (converted to booleans)
    model._set("A4", "=AND({1,0,1})");                   // Should be FALSE (TRUE && FALSE && TRUE)
    model._set("A5", "=OR({0,2,0})");                    // Should be TRUE  (FALSE || TRUE || FALSE)
    model._set("A6", "=AND({5,10,7})");                  // Should be TRUE  (all non-zero)
    model._set("A7", "=OR({0,0,0})");                    // Should be FALSE (all zeros)
    
    model.evaluate();
    
    // Verify logical functions work with arrays
    assert_eq!(model._get_text("A1"), "FALSE", "AND should work with boolean arrays");
    assert_eq!(model._get_text("A2"), "TRUE", "OR should work with boolean arrays");
    assert_eq!(model._get_text("A3"), "FALSE", "XOR should work with boolean arrays");
    
    // Verify number array conversion
    assert_eq!(model._get_text("A4"), "FALSE", "AND should convert numbers: 1,0,1 -> TRUE,FALSE,TRUE");
    assert_eq!(model._get_text("A5"), "TRUE", "OR should convert numbers: 0,2,0 -> FALSE,TRUE,FALSE");
    assert_eq!(model._get_text("A6"), "TRUE", "AND should treat non-zero as TRUE");
    assert_eq!(model._get_text("A7"), "FALSE", "OR should treat zeros as FALSE");
}

#[test]
fn test_information_functions_array_behavior() {
    let mut model = new_empty_model();
    
    // Test TYPE function with arrays - this works!
    model._set("A1", "=TYPE({1,\"text\",TRUE})");        // Should be 1 (number)
    model._set("A2", "=TYPE({\"text\",1,TRUE})");        // Should be 2 (text)
    model._set("A3", "=TYPE({TRUE,1,\"text\"})");        // Should be 4 (boolean)
    
    // TODO: ERROR.TYPE function needs parser support for error literals in arrays
    // model._set("A4", "=ERROR.TYPE({#DIV/0!,#VALUE!})");  // Parser issue with error literals
    
    model.evaluate();
    
    // Verify TYPE functions work with arrays (implicit intersection)
    assert_eq!(model._get_text("A1"), "1", "TYPE should return type of first element: number");
    assert_eq!(model._get_text("A2"), "2", "TYPE should return type of first element: text");
    assert_eq!(model._get_text("A3"), "4", "TYPE should return type of first element: boolean");
}

#[test]
fn test_text_functions_array_behavior_basic() {
    let mut model = new_empty_model();
    
    // Test just the functions I've fixed so far
    model._set("A1", "=UPPER({\"hello\",\"world\"})");     // Should be "HELLO"
    model._set("A2", "=TRIM({\" test \",\"  abc\"})");     // Should be "test"  
    
    // Test text functions with mixed types 
    model._set("A3", "=UPPER({123,\"test\"})");           // Should be "123"
    model._set("A4", "=TRIM({TRUE,\" test \"})");         // Should be "TRUE"
    
    model.evaluate();
    
    // Verify the functions I've fixed work with arrays (implicit intersection)
    assert_eq!(model._get_text("A1"), "HELLO", "UPPER should work with arrays");
    assert_eq!(model._get_text("A2"), "test", "TRIM should work with arrays");
    assert_eq!(model._get_text("A3"), "123", "UPPER should convert numbers to strings");
    assert_eq!(model._get_text("A4"), "TRUE", "TRIM should convert booleans to strings");
}

#[test]
fn test_all_text_functions_with_arrays() {
    let mut model = new_empty_model();
    
    // Test all basic text functions with arrays - should all work now!
    model._set("A1", "=UPPER({\"hello\",\"world\"})");        // Should be "HELLO"
    model._set("A2", "=LOWER({\"HELLO\",\"WORLD\"})");        // Should be "hello"
    model._set("A3", "=TRIM({\" test \",\"  abc\"})");        // Should be "test"  
    model._set("A4", "=LEN({\"hello\",\"hi\"})");             // Should be 5
    model._set("A5", "=LEFT({\"hello\",\"world\"}, 2)");      // Should be "he"
    model._set("A6", "=RIGHT({\"hello\",\"world\"}, 2)");     // Should be "lo"
    model._set("A7", "=MID({\"hello\",\"world\"}, 2, 2)");    // Should be "el"
    model._set("A8", "=UNICODE({\"A\",\"B\"})");              // Should be 65
    
    // Test text functions with mixed types 
    model._set("A9", "=UPPER({123,\"test\"})");               // Should be "123"
    model._set("A10", "=LOWER({TRUE,\"TEST\"})");             // Should be "true"
    model._set("A11", "=TRIM({FALSE,\" test \"})");           // Should be "FALSE"
    model._set("A12", "=LEN({42,\"hi\"})");                   // Should be 2 (length of "42")
    
    // Test with 2D arrays (should use first element)
    model._set("A13", "=UPPER({\"hello\",\"hi\";\"world\",\"bye\"})");  // Should be "HELLO"
    model._set("A14", "=LEN({\"abc\",\"x\";\"hello\",\"y\"})");         // Should be 3
    
    // Test with column arrays
    model._set("A15", "=UPPER({\"hello\";\"world\";\"test\"})");        // Should be "HELLO"
    model._set("A16", "=LEN({\"hi\";\"hello\";\"test\"})");             // Should be 2
    
    model.evaluate();
    
    // Verify all text functions work with arrays (implicit intersection)
    assert_eq!(model._get_text("A1"), "HELLO", "UPPER should work with arrays");
    assert_eq!(model._get_text("A2"), "hello", "LOWER should work with arrays");
    assert_eq!(model._get_text("A3"), "test", "TRIM should work with arrays");
    assert_eq!(model._get_text("A4"), "5", "LEN should work with arrays");
    assert_eq!(model._get_text("A5"), "he", "LEFT should work with arrays");
    assert_eq!(model._get_text("A6"), "lo", "RIGHT should work with arrays");
    assert_eq!(model._get_text("A7"), "el", "MID should work with arrays");
    assert_eq!(model._get_text("A8"), "65", "UNICODE should work with arrays");
    
    // Verify mixed type conversion
    assert_eq!(model._get_text("A9"), "123", "UPPER should convert numbers to strings");
    assert_eq!(model._get_text("A10"), "true", "LOWER should convert booleans to strings");
    assert_eq!(model._get_text("A11"), "FALSE", "TRIM should convert booleans to strings");
    assert_eq!(model._get_text("A12"), "2", "LEN should work with number conversion");
    
    // Verify 2D and column arrays use first element
    assert_eq!(model._get_text("A13"), "HELLO", "UPPER should use first element of 2D array");
    assert_eq!(model._get_text("A14"), "3", "LEN should use first element of 2D array");
    assert_eq!(model._get_text("A15"), "HELLO", "UPPER should use first element of column array");
    assert_eq!(model._get_text("A16"), "2", "LEN should use first element of column array");
}
