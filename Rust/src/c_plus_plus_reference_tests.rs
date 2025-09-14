//! Tests based on exact C++ reference behavior
//! These tests ensure our Rust implementation matches C++ output exactly

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::orientation::FillRule;

    #[test]
    fn test_negative_orientation_exact_match() {
        use crate::clipper_complete::union_complete;

        // This test must produce the exact same result as the C++ reference
        let subjects = vec![
            MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]),
            MakePath(&[10, 10, 10, 110, 110, 110, 110, 10])
        ];

        let clips = vec![
            MakePath(&[50, 50, 50, 150, 150, 150, 150, 50])
        ];

        // Verify input orientations match C++ (all should be negative)
        assert_eq!(IsPositive(&subjects[0]), false, "subjects[0] should have negative orientation");
        assert_eq!(IsPositive(&subjects[1]), false, "subjects[1] should have negative orientation");
        assert_eq!(IsPositive(&clips[0]), false, "clips[0] should have negative orientation");

        // Verify areas match C++ (all should be -10000)
        assert_eq!(area(&subjects[0]), -10000.0, "subjects[0] area should be -10000");
        assert_eq!(area(&subjects[1]), -10000.0, "subjects[1] area should be -10000");
        assert_eq!(area(&clips[0]), -10000.0, "clips[0] area should be -10000");

        // Test the new complete engine
        let result = union_complete(&subjects, &clips, FillRule::Negative);

        // Must match C++ exactly: 1 path with 12 vertices
        assert_eq!(result.len(), 1, "Should produce exactly 1 result path");
        assert_eq!(result[0].len(), 12, "Result path should have exactly 12 vertices");

        // Expected vertices from C++ reference (exact order and coordinates)
        let expected_vertices = [
            (100, 0), (100, 10), (110, 10), (110, 50),
            (150, 50), (150, 150), (50, 150), (50, 110),
            (10, 110), (10, 100), (0, 100), (0, 0)
        ];

        // Verify each vertex matches exactly
        for (i, expected) in expected_vertices.iter().enumerate() {
            assert_eq!(result[0][i].x, expected.0, "Vertex {} x coordinate should be {}", i, expected.0);
            assert_eq!(result[0][i].y, expected.1, "Vertex {} y coordinate should be {}", i, expected.1);
        }

        // Calculate and verify the total area matches C++ expectation
        let result_area = area(&result[0]);
        let expected_area = 30000.0; // Based on union of the three polygons with negative orientation
        assert!((result_area - expected_area).abs() < 1.0,
               "Result area should be approximately {}, but got {}", expected_area, result_area);
    }

    #[test]
    fn test_negative_orientation_exact_match_old_engine() {
        // This test must produce the exact same result as the C++ reference
        let subjects = vec![
            MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]),
            MakePath(&[10, 10, 10, 110, 110, 110, 110, 10])
        ];

        let clips = vec![
            MakePath(&[50, 50, 50, 150, 150, 150, 150, 50])
        ];

        // Verify input orientations match C++ (all should be negative)
        assert_eq!(IsPositive(&subjects[0]), false, "subjects[0] should have negative orientation");
        assert_eq!(IsPositive(&subjects[1]), false, "subjects[1] should have negative orientation");
        assert_eq!(IsPositive(&clips[0]), false, "clips[0] should have negative orientation");

        // Verify areas match C++ (all should be -10000)
        assert_eq!(area(&subjects[0]), -10000.0, "subjects[0] area should be -10000");
        assert_eq!(area(&subjects[1]), -10000.0, "subjects[1] area should be -10000");
        assert_eq!(area(&clips[0]), -10000.0, "clips[0] area should be -10000");

        // Perform union with negative fill rule
        let result = Union(&subjects, &clips, FillRule::Negative);

        // Must match C++ exactly: 1 path with 12 vertices
        assert_eq!(result.len(), 1, "Should produce exactly 1 result path");
        assert_eq!(result[0].len(), 12, "Result path should have exactly 12 vertices");

        // Expected vertices from C++ reference (exact order and coordinates)
        let expected_vertices = [
            (100, 0), (100, 10), (110, 10), (110, 50),
            (150, 50), (150, 150), (50, 150), (50, 110),
            (10, 110), (10, 100), (0, 100), (0, 0)
        ];

        // Verify each vertex matches exactly
        for (i, expected) in expected_vertices.iter().enumerate() {
            assert_eq!(result[0][i].x, expected.0, "Vertex {} x coordinate should be {}", i, expected.0);
            assert_eq!(result[0][i].y, expected.1, "Vertex {} y coordinate should be {}", i, expected.1);
        }

        // Calculate and verify the total area matches C++ expectation
        let result_area = area(&result[0]);
        let expected_area = 30000.0; // Based on union of the three polygons with negative orientation
        assert!((result_area - expected_area).abs() < 1.0,
               "Result area should be approximately {}, but got {}", expected_area, result_area);
    }

    #[test]
    fn test_other_fill_rules_for_comparison() {
        // Test the same paths with other fill rules to understand the differences
        let subjects = vec![
            MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]),
            MakePath(&[10, 10, 10, 110, 110, 110, 110, 10])
        ];

        let clips = vec![
            MakePath(&[50, 50, 50, 150, 150, 150, 150, 50])
        ];

        // NonZero should produce same as Negative for this case (from C++ reference)
        let nonzero_result = Union(&subjects, &clips, FillRule::NonZero);
        assert_eq!(nonzero_result.len(), 1, "NonZero should produce 1 path");
        assert_eq!(nonzero_result[0].len(), 12, "NonZero result should have 12 vertices");

        // EvenOdd should produce different result (2 paths from C++ reference)
        let evenodd_result = Union(&subjects, &clips, FillRule::EvenOdd);
        assert_eq!(evenodd_result.len(), 2, "EvenOdd should produce 2 paths");

        // Positive should produce empty result (from C++ reference)
        let positive_result = Union(&subjects, &clips, FillRule::Positive);
        assert_eq!(positive_result.len(), 0, "Positive should produce no paths");
    }
}