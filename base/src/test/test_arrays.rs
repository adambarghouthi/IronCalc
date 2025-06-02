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
        ("=COUNT({1,2,3})", "COUNT"),
        ("=COUNTA({1,2,3})", "COUNTA"),
        ("=MAX({1,2,3})", "MAX"),
        ("=MIN({1,2,3})", "MIN"),
        ("=PRODUCT({1,2,3})", "PRODUCT"),
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
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "2", "COUNT should only count numbers in mixed array");
    assert_eq!(model._get_text("A2"), "4", "COUNTA should count all elements in mixed array");
    assert_eq!(model._get_text("A3"), "3", "SUM should ignore non-numbers in mixed array");
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
    
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1", "MIN should ignore non-numbers");
    assert_eq!(model._get_text("A2"), "5", "MAX should ignore non-numbers");
    assert_eq!(model._get_text("A3"), "3", "AVERAGE should ignore non-numbers (5+1+3)/3=3");
    assert_eq!(model._get_text("A4"), "10", "PRODUCT should ignore non-numbers");
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
