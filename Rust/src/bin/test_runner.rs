//! Test runner for Clipper2 official test cases
//! 
//! This executable parses and runs the official Clipper2 test files
//! to validate the Rust implementation against expected results.

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use clipper2::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TestClipType {
    Union,
    Intersection,
    Difference,
    Xor,
    Offset,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestFillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub caption: String,
    pub clip_type: TestClipType,
    pub fill_rule: TestFillRule,
    pub expected_area: f64,
    pub expected_count: i32,
    pub subjects: Vec<Path64>,
    pub clips: Vec<Path64>,
    pub subjects_open: Vec<Path64>,
}

#[derive(Debug)]
pub struct TestResult {
    pub caption: String,
    pub passed: bool,
    pub expected_area: f64,
    pub actual_area: f64,
    pub expected_count: i32,
    pub actual_count: i32,
    pub error_message: Option<String>,
}

impl TestCase {
    pub fn new() -> Self {
        Self {
            caption: String::new(),
            clip_type: TestClipType::Union,
            fill_rule: TestFillRule::NonZero,
            expected_area: 0.0,
            expected_count: 0,
            subjects: Vec::new(),
            clips: Vec::new(),
            subjects_open: Vec::new(),
        }
    }
}

/// Parse coordinate string handling multiple formats:
/// - "100,10, 150,100, 100,190, 50,100" (comma-separated with spaces)
/// - "100 10 150 100 100 190 50 100" (space-separated)  
/// - "31,1 32,5 35,1" (comma-separated pairs with space between pairs)
fn parse_coordinates(coord_str: &str) -> Result<Path64, String> {
    // Handle empty lines
    let trimmed = coord_str.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    
    let coords = if trimmed.contains(',') {
        // Handle comma-containing formats
        // First split by commas and collect all numbers
        let mut all_numbers = Vec::new();
        
        for part in trimmed.split(',') {
            let part = part.trim();
            // Each part might contain multiple space-separated numbers
            for num_str in part.split_whitespace() {
                match num_str.parse::<i64>() {
                    Ok(num) => all_numbers.push(num),
                    Err(e) => return Err(format!("Parse error in '{}': {}", coord_str, e)),
                }
            }
        }
        
        all_numbers
    } else {
        // Pure space-separated format
        match trimmed
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>() {
                Ok(coords) => coords,
                Err(e) => return Err(format!("Parse error in '{}': {}", coord_str, e)),
            }
    };
    
    if coords.len() == 0 {
        return Ok(Vec::new());
    }
    if coords.len() % 2 != 0 {
        return Err(format!("Odd number of coordinates: {} values in '{}'", coords.len(), coord_str));
    }
    
    Ok(make_path(&coords))
}

/// Parse a test file containing multiple test cases
fn parse_test_file(content: &str) -> Result<Vec<TestCase>, String> {
    let mut test_cases = Vec::new();
    let mut current_case = TestCase::new();
    let mut parsing_subjects = false;
    let mut parsing_clips = false;
    let mut parsing_subjects_open = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            // Empty line might indicate end of current test case
            if !current_case.caption.is_empty() {
                test_cases.push(current_case.clone());
                current_case = TestCase::new();
                parsing_subjects = false;
                parsing_clips = false;
                parsing_subjects_open = false;
            }
            continue;
        }
        
        if line.starts_with("CAPTION:") {
            // If we have a current case, save it before starting new one
            if !current_case.caption.is_empty() {
                test_cases.push(current_case.clone());
                current_case = TestCase::new();
            }
            current_case.caption = line[8..].trim().to_string();
            parsing_subjects = false;
            parsing_clips = false;
            parsing_subjects_open = false;
        } else if line.starts_with("CLIPTYPE:") {
            let clip_type_str = line[9..].trim().to_uppercase();
            current_case.clip_type = match clip_type_str.as_str() {
                "UNION" => TestClipType::Union,
                "INTERSECTION" => TestClipType::Intersection,
                "DIFFERENCE" => TestClipType::Difference,
                "XOR" => TestClipType::Xor,
                "OFFSET" => TestClipType::Offset,
                _ => return Err(format!("Unknown clip type: {}", clip_type_str)),
            };
        } else if line.starts_with("FILLRULE:") {
            let fill_rule_str = line[9..].trim().to_uppercase();
            current_case.fill_rule = match fill_rule_str.as_str() {
                "EVENODD" => TestFillRule::EvenOdd,
                "NONZERO" => TestFillRule::NonZero,
                "POSITIVE" => TestFillRule::Positive,
                "NEGATIVE" => TestFillRule::Negative,
                _ => return Err(format!("Unknown fill rule: {}", fill_rule_str)),
            };
        } else if line.starts_with("SOL_AREA:") {
            let area_str = line[9..].trim();
            current_case.expected_area = area_str.parse::<f64>()
                .map_err(|e| format!("Failed to parse area: {}", e))?;
        } else if line.starts_with("SOL_COUNT:") {
            let count_str = line[10..].trim();
            current_case.expected_count = count_str.parse::<i32>()
                .map_err(|e| format!("Failed to parse count: {}", e))?;
        } else if line == "SUBJECTS" {
            parsing_subjects = true;
            parsing_clips = false;
            parsing_subjects_open = false;
        } else if line == "CLIPS" {
            parsing_subjects = false;
            parsing_clips = true;
            parsing_subjects_open = false;
        } else if line == "SUBJECTS_OPEN" {
            parsing_subjects = false;
            parsing_clips = false;
            parsing_subjects_open = true;
        } else {
            // This should be a coordinate line
            if parsing_subjects {
                match parse_coordinates(line) {
                    Ok(path) => {
                        if !path.is_empty() {  // Only add non-empty paths
                            current_case.subjects.push(path);
                        }
                    },
                    Err(e) => return Err(format!("Error parsing subject coordinates on line '{}': {}", line, e)),
                }
            } else if parsing_clips {
                match parse_coordinates(line) {
                    Ok(path) => {
                        if !path.is_empty() {  // Only add non-empty paths
                            current_case.clips.push(path);
                        }
                    },
                    Err(e) => return Err(format!("Error parsing clip coordinates on line '{}': {}", line, e)),
                }
            } else if parsing_subjects_open {
                match parse_coordinates(line) {
                    Ok(path) => {
                        if !path.is_empty() {  // Only add non-empty paths
                            current_case.subjects_open.push(path);
                        }
                    },
                    Err(e) => return Err(format!("Error parsing open subject coordinates on line '{}': {}", line, e)),
                }
            } else {
                // If we're not in a parsing mode, this might be an unexpected line
                // but let's be lenient and just skip it
            }
        }
    }
    
    // Don't forget the last test case
    if !current_case.caption.is_empty() {
        test_cases.push(current_case);
    }
    
    Ok(test_cases)
}

/// Convert TestFillRule to clipper2 FillRule
fn test_fill_rule_to_clipper(rule: &TestFillRule) -> FillRule {
    match rule {
        TestFillRule::EvenOdd => FillRule::EvenOdd,
        TestFillRule::NonZero => FillRule::NonZero,
        TestFillRule::Positive => FillRule::Positive,
        TestFillRule::Negative => FillRule::Negative,
    }
}

/// Calculate total area of all paths in a solution
fn calculate_total_area(paths: &Paths64) -> f64 {
    paths.iter().map(|path| area(path).abs()).sum()
}

/// Execute a test case and return the result
fn execute_test_case(test_case: &TestCase) -> TestResult {
    let clipper_fill_rule = test_fill_rule_to_clipper(&test_case.fill_rule);
    
    // Skip offset tests for now as they require different handling
    if test_case.clip_type == TestClipType::Offset {
        return TestResult {
            caption: test_case.caption.clone(),
            passed: false,
            expected_area: test_case.expected_area,
            actual_area: 0.0,
            expected_count: test_case.expected_count,
            actual_count: 0,
            error_message: Some("OFFSET operations not implemented in test runner".to_string()),
        };
    }
    
    // Skip tests with negative expected values (usually offset tests)
    if test_case.expected_area < 0.0 || test_case.expected_count < 0 {
        return TestResult {
            caption: test_case.caption.clone(),
            passed: false,
            expected_area: test_case.expected_area,
            actual_area: 0.0,
            expected_count: test_case.expected_count,
            actual_count: 0,
            error_message: Some("Test case with negative expected values - likely offset test".to_string()),
        };
    }
    
    let result_paths = match test_case.clip_type {
        TestClipType::Union => {
            if test_case.clips.is_empty() {
                // Union of subjects only
                let empty_clips = Vec::new();
                union(&test_case.subjects, &empty_clips, clipper_fill_rule)
            } else {
                union(&test_case.subjects, &test_case.clips, clipper_fill_rule)
            }
        }
        TestClipType::Intersection => {
            if test_case.clips.is_empty() {
                Vec::new() // No intersection possible without clips
            } else {
                Intersect(&test_case.subjects, &test_case.clips, clipper_fill_rule)
            }
        }
        TestClipType::Difference => {
            if test_case.clips.is_empty() {
                test_case.subjects.clone() // Difference with nothing is the original
            } else {
                Difference(&test_case.subjects, &test_case.clips, clipper_fill_rule)
            }
        }
        TestClipType::Xor => {
            if test_case.clips.is_empty() {
                test_case.subjects.clone() // XOR with nothing is the original
            } else {
                Xor(&test_case.subjects, &test_case.clips, clipper_fill_rule)
            }
        }
        TestClipType::Offset => Vec::new(), // Already handled above
    };
    
    let actual_area = calculate_total_area(&result_paths);
    let actual_count = result_paths.len() as i32;
    
    // Define tolerance for floating point comparison
    let area_tolerance = 1.0; // Allow 1 unit² tolerance
    let area_matches = (actual_area - test_case.expected_area).abs() <= area_tolerance;
    let count_matches = actual_count == test_case.expected_count;
    
    TestResult {
        caption: test_case.caption.clone(),
        passed: area_matches && count_matches,
        expected_area: test_case.expected_area,
        actual_area,
        expected_count: test_case.expected_count,
        actual_count,
        error_message: if area_matches && count_matches {
            None
        } else {
            Some(format!(
                "Mismatch - Area: expected {:.1}, got {:.1} (diff: {:.1}), Count: expected {}, got {}",
                test_case.expected_area,
                actual_area,
                (actual_area - test_case.expected_area).abs(),
                test_case.expected_count,
                actual_count
            ))
        },
    }
}

/// Run all tests in a file and return results
fn run_test_file(file_path: &Path) -> Result<Vec<TestResult>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
    
    let test_cases = parse_test_file(&content)?;
    println!("Found {} test cases in {:?}", test_cases.len(), file_path.file_name().unwrap());
    
    let mut results = Vec::new();
    for (i, test_case) in test_cases.iter().enumerate() {
        print!("  Running test {}: {} ... ", i + 1, test_case.caption);
        let result = execute_test_case(test_case);
        
        if result.passed {
            println!("PASS");
        } else {
            println!("FAIL");
            if let Some(ref error) = result.error_message {
                println!("    Error: {}", error);
            }
        }
        
        results.push(result);
    }
    
    Ok(results)
}

fn main() {
    println!("Clipper2 Rust Test Runner");
    println!("========================");
    
    // Find all test files
    let test_dir = Path::new("../Tests");
    if !test_dir.exists() {
        eprintln!("Error: Tests directory not found");
        std::process::exit(1);
    }
    
    let test_files = [
        "Polygons.txt",
        "Lines.txt", 
        "Offsets.txt",
        "PolytreeHoleOwner.txt",
        "PolytreeHoleOwner2.txt"
    ];
    
    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut all_results: HashMap<String, Vec<TestResult>> = HashMap::new();
    
    for test_file in &test_files {
        let file_path = test_dir.join(test_file);
        if !file_path.exists() {
            println!("Warning: Test file {:?} not found, skipping", test_file);
            continue;
        }
        
        println!("\nRunning tests from {:?}:", test_file);
        println!("{}", "─".repeat(50));
        
        match run_test_file(&file_path) {
            Ok(results) => {
                let passed = results.iter().filter(|r| r.passed).count();
                let failed = results.len() - passed;
                
                total_passed += passed;
                total_failed += failed;
                
                println!("Results: {} passed, {} failed", passed, failed);
                all_results.insert(test_file.to_string(), results);
            }
            Err(e) => {
                println!("Error running tests: {}", e);
            }
        }
    }
    
    // Print summary
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY");
    println!("{}", "=".repeat(60));
    println!("Total tests: {}", total_passed + total_failed);
    println!("Passed: {} ({:.1}%)", total_passed, 
             100.0 * total_passed as f64 / (total_passed + total_failed) as f64);
    println!("Failed: {} ({:.1}%)", total_failed,
             100.0 * total_failed as f64 / (total_passed + total_failed) as f64);
    
    // Print detailed failure report
    if total_failed > 0 {
        println!("\nDETAILED FAILURE REPORT");
        println!("{}", "─".repeat(60));
        
        for (file_name, results) in &all_results {
            let failed_tests: Vec<_> = results.iter().filter(|r| !r.passed).collect();
            
            if !failed_tests.is_empty() {
                println!("\n{}:", file_name);
                for test in failed_tests {
                    println!("  {} - {}", test.caption, 
                           test.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
                }
            }
        }
    }
    
    println!("\nTest run completed!");
    
    // Exit with appropriate code
    if total_failed > 0 {
        std::process::exit(1);
    }
}