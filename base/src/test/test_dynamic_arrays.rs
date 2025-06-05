//! Tests for Dynamic Array functionality
//! 
//! This module tests the Excel-compatible dynamic array spilling feature.
//! Dynamic arrays automatically "spill" their results into adjacent cells
//! when a formula returns an array result.
//!
//! Example: A1="={1,2,3}" results in A1=1, B1=2, C1=3

use crate::test::util::new_empty_model;
use crate::cell::CellValue;

#[test]
fn test_basic_array_spill_infrastructure() {
    let mut model = new_empty_model();
    
    // Test that array literal gets processed by spill logic
    model.set_user_input(0, 1, 1, "={1,2,3}".to_string()).unwrap();
    model.evaluate();
    
    // Verify the source cell contains the first element
    assert_eq!(model.get_cell_content(0, 1, 1).unwrap(), "={1,2,3}");
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "1"); // First element
    
    // Array should spill to adjacent cells
    assert_eq!(model.get_formatted_cell_value(0, 1, 2).unwrap(), "2"); // B1
    assert_eq!(model.get_formatted_cell_value(0, 1, 3).unwrap(), "3"); // C1
}

#[test]
fn test_spill_conflict_detection() {
    let mut model = new_empty_model();
    
    // Put some content in B1 to create a conflict
    model.set_user_input(0, 1, 2, "existing content".to_string()).unwrap();
    model.evaluate();
    
    // Try to spill ={1,2,3} into A1:C1, should conflict with B1
    model.set_user_input(0, 1, 1, "={1,2,3}".to_string()).unwrap();
    model.evaluate();
    
    // A1 should show #SPILL! error
    assert_eq!(model.get_cell_content(0, 1, 1).unwrap(), "={1,2,3}");
    let formatted_value = model.get_formatted_cell_value(0, 1, 1).unwrap();
    assert!(formatted_value.contains("SPILL") || formatted_value.contains("#SPILL!"));
    
    // B1 should still contain the original content
    assert_eq!(model.get_formatted_cell_value(0, 1, 2).unwrap(), "existing content");
    
    // C1 should remain empty (spill was blocked)
    assert_eq!(model.get_formatted_cell_value(0, 1, 3).unwrap(), "");
}

#[test]
fn test_spill_conflict_preserves_first_array() {
    let mut model = new_empty_model();
    
    // Step 1: Put ={1,2,3} in A2 (should spill to A2:C2)
    model.set_user_input(0, 2, 1, "={1,2,3}".to_string()).unwrap();
    model.evaluate();
    
    // Verify first array spills correctly
    assert_eq!(model.get_formatted_cell_value(0, 2, 1).unwrap(), "1"); // A2
    assert_eq!(model.get_formatted_cell_value(0, 2, 2).unwrap(), "2"); // B2
    assert_eq!(model.get_formatted_cell_value(0, 2, 3).unwrap(), "3"); // C2
    
    // Step 2: Put ={1;2;3} in B1 (should conflict at B2 and show #SPILL! in B1)
    model.set_user_input(0, 1, 2, "={1;2;3}".to_string()).unwrap();
    model.evaluate();
    
    // EXPECTED: A2 should keep working, B1 should show #SPILL!
    assert_eq!(model.get_formatted_cell_value(0, 2, 1).unwrap(), "1"); // A2 should still work
    assert_eq!(model.get_formatted_cell_value(0, 2, 2).unwrap(), "2"); // B2 should still work  
    assert_eq!(model.get_formatted_cell_value(0, 2, 3).unwrap(), "3"); // C2 should still work
    assert!(model.get_formatted_cell_value(0, 1, 2).unwrap().contains("#SPILL!")); // B1 should show error
}

#[test]
fn test_vertical_array_spill() {
    let mut model = new_empty_model();
    
    // Test vertical array spilling 
    model.set_user_input(0, 1, 1, "={1;2;3}".to_string()).unwrap();
    model.evaluate();
    
    // Should spill vertically: A1=1, A2=2, A3=3
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "1"); // A1
    assert_eq!(model.get_formatted_cell_value(0, 2, 1).unwrap(), "2"); // A2
    assert_eq!(model.get_formatted_cell_value(0, 3, 1).unwrap(), "3"); // A3
}

#[test]
fn test_2d_array_spill() {
    let mut model = new_empty_model();
    
    // Test 2D array spilling: 2 rows, 3 columns
    model.set_user_input(0, 1, 1, "={1,2,3;4,5,6}".to_string()).unwrap();
    model.evaluate();
    
    // First row: A1=1, B1=2, C1=3
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "1"); // A1
    assert_eq!(model.get_formatted_cell_value(0, 1, 2).unwrap(), "2"); // B1
    assert_eq!(model.get_formatted_cell_value(0, 1, 3).unwrap(), "3"); // C1
    
    // Second row: A2=4, B2=5, C2=6
    assert_eq!(model.get_formatted_cell_value(0, 2, 1).unwrap(), "4"); // A2
    assert_eq!(model.get_formatted_cell_value(0, 2, 2).unwrap(), "5"); // B2
    assert_eq!(model.get_formatted_cell_value(0, 2, 3).unwrap(), "6"); // C2
}

#[test]
fn test_spill_cleanup_on_formula_change() {
    let mut model = new_empty_model();
    
    // First, spill a 3-element array
    model.set_user_input(0, 1, 1, "={1,2,3}".to_string()).unwrap();
    model.evaluate();
    
    // Verify initial spill
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "1"); // A1
    assert_eq!(model.get_formatted_cell_value(0, 1, 2).unwrap(), "2"); // B1
    assert_eq!(model.get_formatted_cell_value(0, 1, 3).unwrap(), "3"); // C1
    
    // Change to a non-array formula
    model.set_user_input(0, 1, 1, "=42".to_string()).unwrap();
    model.evaluate();
    
    // The source cell should now show 42, and spilled cells should be cleared
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "42"); // A1
    assert!(model.is_empty_cell(0, 1, 2).unwrap()); // B1 cleared
    assert!(model.is_empty_cell(0, 1, 3).unwrap()); // C1 cleared
}

#[test]
fn test_mixed_data_types_spill() {
    let mut model = new_empty_model();
    
    // Test array with mixed data types
    model.set_user_input(0, 1, 1, "={42,\"hello\",TRUE}".to_string()).unwrap();
    model.evaluate();
    
    // Verify different data types spill correctly
    assert_eq!(model.get_formatted_cell_value(0, 1, 1).unwrap(), "42");    // Number
    assert_eq!(model.get_formatted_cell_value(0, 1, 2).unwrap(), "hello"); // String
    assert_eq!(model.get_formatted_cell_value(0, 1, 3).unwrap(), "TRUE");  // Boolean
}

#[test]
fn test_empty_array_handling() {
    let mut model = new_empty_model();
    
    // Test empty array - this might produce an error due to parsing
    model.set_user_input(0, 1, 1, "={}".to_string()).unwrap();
    model.evaluate();
    
    // Empty array syntax might not be supported, could result in error
    let result = model.get_formatted_cell_value(0, 1, 1).unwrap();
    // Accept either an error or a numeric result
    assert!(result.contains("ERROR") || result.contains("#") || result == "0" || result.is_empty());
}

#[test]
fn test_comprehensive_source_cell_operations() {
    let mut model = new_empty_model();
    
    // Test Case 1: Edit source cell with new dynamic array
    
    // Step 1: Set ={1,2,3} in A1 → should spill to A1=1, B1=2, C1=3  
    model.set_user_input(0, 1, 1, "={1,2,3}".to_string()).unwrap();
    model.evaluate();
    
    // Verify initial spill
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::Number(1.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 2).unwrap(), CellValue::Number(2.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 3).unwrap(), CellValue::Number(3.0));
    
    // Step 2: Edit A1 to ={4,5,6,7} → should clear old spill and create new spill
    model.set_user_input(0, 1, 1, "={4,5,6,7}".to_string()).unwrap();
    model.evaluate();
    
    // Verify new spill (4 cells now)
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::Number(4.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 2).unwrap(), CellValue::Number(5.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 3).unwrap(), CellValue::Number(6.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 4).unwrap(), CellValue::Number(7.0));
    
    // Test Case 2: Edit source cell to non-array value
    
    // Step 3: Edit A1 to =42 (non-array) → should clear all spilled cells
    model.set_user_input(0, 1, 1, "=42".to_string()).unwrap();
    model.evaluate();
    
    // Verify A1 has the new value and B1:D1 are empty
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::Number(42.0));
    assert!(model.is_empty_cell(0, 1, 2).unwrap());
    assert!(model.is_empty_cell(0, 1, 3).unwrap());
    assert!(model.is_empty_cell(0, 1, 4).unwrap());
    
    // Test Case 3: Delete source cell
    
    // Step 4: Set ={10,20,30} in A1 again
    model.set_user_input(0, 1, 1, "={10,20,30}".to_string()).unwrap();
    model.evaluate();
    
    // Verify spill
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::Number(10.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 2).unwrap(), CellValue::Number(20.0));
    assert_eq!(model.get_cell_value_by_index(0, 1, 3).unwrap(), CellValue::Number(30.0));
    
    // Step 5: Delete A1 content → should clear all spilled cells
    model.cell_clear_contents(0, 1, 1).unwrap();
    model.evaluate();
    
    // Verify all cells are now empty
    assert!(model.is_empty_cell(0, 1, 1).unwrap());
    assert!(model.is_empty_cell(0, 1, 2).unwrap());
    assert!(model.is_empty_cell(0, 1, 3).unwrap());
    
    // Test Case 4: Vertical array operations
    
    // Step 6: Set ={1;2;3} in A1 → should spill vertically to A1:A3
    model.set_user_input(0, 1, 1, "={1;2;3}".to_string()).unwrap();
    model.evaluate();
    
    // Verify vertical spill
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::Number(1.0));
    assert_eq!(model.get_cell_value_by_index(0, 2, 1).unwrap(), CellValue::Number(2.0));
    assert_eq!(model.get_cell_value_by_index(0, 3, 1).unwrap(), CellValue::Number(3.0));
    
    // Step 7: Edit A1 to different content → should clear vertical spill
    model.set_user_input(0, 1, 1, "test".to_string()).unwrap();
    model.evaluate();
    
    // Verify cleanup
    assert_eq!(model.get_cell_value_by_index(0, 1, 1).unwrap(), CellValue::String("test".to_string()));
    assert!(model.is_empty_cell(0, 2, 1).unwrap());
    assert!(model.is_empty_cell(0, 3, 1).unwrap());
} 