//! SVG Visualization Example for Clipper2 Operations
//! 
//! This example creates SVG visualizations of polygon clipping operations,
//! showing before/after results with different colors for easy comparison.

use clipper2::{
    Clipper64, ClipType, FillRule, JoinType, EndType,
    MakePath, InflatePaths, Union, Intersect, Difference, Xor,
    Path64, Paths64, area
};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("🎨 Clipper2 SVG Visualizer");
    println!("==========================\n");

    // Create output directory
    std::fs::create_dir_all("output")?;

    // Generate various visualizations
    generate_boolean_operations_svg()?;
    generate_fill_rule_comparison_svg()?;
    generate_offsetting_showcase_svg()?;
    generate_complex_clipping_svg()?;

    println!("✅ All SVG visualizations generated in ./output/ directory");
    println!("   Open the SVG files in a web browser to view the results!");

    Ok(())
}

fn generate_boolean_operations_svg() -> std::io::Result<()> {
    println!("📊 Generating boolean operations visualization...");

    let mut svg = SvgWriter::new(800, 600, "Boolean Operations Showcase");

    // Create test shapes
    let rect1 = create_rectangle(100, 150, 200, 120);
    let rect2 = create_rectangle(200, 100, 200, 120);
    let subjects = vec![rect1.clone()];
    let clips = vec![rect2.clone()];

    // Original shapes
    svg.add_text(50, 40, "Original Shapes", "title");
    svg.add_paths(&subjects, "subject", Some("Subject"));
    svg.add_paths(&clips, "clip", Some("Clip"));

    // Boolean operations
    let operations = [
        ("Union", Union(&subjects, &clips, FillRule::NonZero), 450, 50),
        ("Intersection", Intersect(&subjects, &clips, FillRule::NonZero), 450, 150),
        ("Difference", Difference(&subjects, &clips, FillRule::NonZero), 450, 250),
        ("XOR", Xor(&subjects, &clips, FillRule::NonZero), 450, 350),
    ];

    for (name, result, x, y) in &operations {
        svg.add_text(*x, *y - 20, name, "operation");
        svg.add_paths_at(result, "result", *x, *y, None);
        
        let total_area = result.iter().map(|p| area(p).abs()).sum::<f64>();
        svg.add_text(*x, *y + 80, &format!("Area: {:.0}", total_area), "small");
    }

    svg.save("output/boolean_operations.svg")
}

fn generate_fill_rule_comparison_svg() -> std::io::Result<()> {
    println!("📐 Generating fill rule comparison...");

    let mut svg = SvgWriter::new(1000, 400, "Fill Rule Comparison");

    // Create self-intersecting shape (bowtie)
    let bowtie = MakePath(&[
        50, 50,   // bottom left  
        150, 150, // center intersection
        250, 50,  // bottom right
        250, 250, // top right
        150, 150, // center intersection again
        50, 250,  // top left
    ]);

    let subjects = vec![bowtie];

    let fill_rules = [
        (FillRule::EvenOdd, "EvenOdd", 100),
        (FillRule::NonZero, "NonZero", 300),
        (FillRule::Positive, "Positive", 500), 
        (FillRule::Negative, "Negative", 700),
    ];

    svg.add_text(50, 30, "Self-Intersecting Polygon with Different Fill Rules", "title");

    for (fill_rule, name, x) in &fill_rules {
        // Show original shape
        svg.add_paths_at(&subjects, "wireframe", *x, 60, Some("Original"));
        
        // Show filled result
        let result = Union(&subjects, &vec![], *fill_rule);
        svg.add_paths_at(&result, "result", *x, 200, Some(name));
        
        let area_sum = result.iter().map(|p| area(p).abs()).sum::<f64>();
        svg.add_text(*x, 340, &format!("{}: {:.0}", name, area_sum), "operation");
    }

    svg.save("output/fill_rules.svg")
}

fn generate_offsetting_showcase_svg() -> std::io::Result<()> {
    println!("📏 Generating offsetting showcase...");

    let mut svg = SvgWriter::new(900, 700, "Path Offsetting Showcase");

    // Create test shapes
    let star = create_star(150, 150, 80, 40, 8);
    let paths = vec![star];

    svg.add_text(50, 30, "Path Offsetting with Different Join Types", "title");
    svg.add_paths_at(&paths, "subject", 150, 100, Some("Original Star"));

    let join_types = [
        (JoinType::Round, "Round", 350, 100),
        (JoinType::Square, "Square", 550, 100),
        (JoinType::Miter, "Miter", 750, 100),
    ];

    let deltas = [20.0, -10.0];
    let delta_names = ["Expand (+20)", "Contract (-10)"];

    for (i, (delta, delta_name)) in deltas.iter().zip(&delta_names).enumerate() {
        let base_y = (100 + i * 200) as i64;
        svg.add_text(50, base_y + 150, delta_name, "operation");
        
        for (join_type, name, x, _) in &join_types {
            let result = InflatePaths(&paths, *delta, *join_type, EndType::Polygon);
            svg.add_paths_at(&result, "result", *x, base_y + 100, Some(name));
            
            let area_sum = result.iter().map(|p| area(p).abs()).sum::<f64>();
            svg.add_text(*x, base_y + 180, &format!("{:.0}", area_sum), "small");
        }
    }

    svg.save("output/offsetting.svg")
}

fn generate_complex_clipping_svg() -> std::io::Result<()> {
    println!("⭐ Generating complex clipping visualization...");

    let mut svg = SvgWriter::new(800, 600, "Complex Polygon Clipping");

    // Create complex shapes
    let star = create_star(200, 200, 120, 60, 12);
    let circle = create_circle(200, 200, 100, 24);
    let subjects = vec![star];
    let clips = vec![circle];

    svg.add_text(50, 30, "Complex Star-Circle Intersection", "title");

    // Show original shapes
    svg.add_paths(&subjects, "subject", Some("12-point Star"));
    svg.add_paths(&clips, "clip", Some("24-sided Circle"));

    // Show intersection result
    let intersection = Intersect(&subjects, &clips, FillRule::EvenOdd);
    svg.add_paths_at(&intersection, "result", 450, 100, Some("Intersection Result"));

    let area_sum = intersection.iter().map(|p| area(p).abs()).sum::<f64>();
    svg.add_text(450, 320, &format!("Result Area: {:.0}", area_sum), "operation");
    svg.add_text(450, 340, &format!("Paths: {}", intersection.len()), "small");

    // Show difference result  
    let difference = Difference(&subjects, &clips, FillRule::EvenOdd);
    svg.add_paths_at(&difference, "difference", 450, 380, Some("Difference (Star - Circle)"));

    svg.save("output/complex_clipping.svg")
}

// Helper functions

fn create_rectangle(x: i64, y: i64, width: i64, height: i64) -> Path64 {
    MakePath(&[x, y, x + width, y, x + width, y + height, x, y + height])
}

fn create_star(cx: i64, cy: i64, outer_radius: i64, inner_radius: i64, points: usize) -> Path64 {
    let mut coords = Vec::new();
    
    for i in 0..(points * 2) {
        let angle = (i as f64 * std::f64::consts::PI) / points as f64;
        let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
        
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        
        coords.push(x);
        coords.push(y);
    }
    
    MakePath(&coords)
}

fn create_circle(cx: i64, cy: i64, radius: i64, sides: usize) -> Path64 {
    let mut coords = Vec::new();
    
    for i in 0..sides {
        let angle = (i as f64 * 2.0 * std::f64::consts::PI) / sides as f64;
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        
        coords.push(x);
        coords.push(y);
    }
    
    MakePath(&coords)
}

// SVG Writer utility

struct SvgWriter {
    width: u32,
    height: u32,
    content: String,
}

impl SvgWriter {
    fn new(width: u32, height: u32, title: &str) -> Self {
        let mut writer = SvgWriter {
            width,
            height,
            content: String::new(),
        };

        writer.content.push_str(&format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
<title>{}</title>
<defs>
<style>
.subject {{ fill: rgba(100, 149, 237, 0.5); stroke: #4169E1; stroke-width: 2; }}
.clip {{ fill: rgba(255, 165, 0, 0.5); stroke: #FF8C00; stroke-width: 2; }}
.result {{ fill: rgba(50, 205, 50, 0.7); stroke: #228B22; stroke-width: 2; }}
.difference {{ fill: rgba(255, 99, 71, 0.7); stroke: #DC143C; stroke-width: 2; }}
.wireframe {{ fill: none; stroke: #666; stroke-width: 1; stroke-dasharray: 5,5; }}
.title {{ font-family: Arial, sans-serif; font-size: 18px; font-weight: bold; fill: #333; }}
.operation {{ font-family: Arial, sans-serif; font-size: 14px; font-weight: bold; fill: #333; }}
.small {{ font-family: Arial, sans-serif; font-size: 12px; fill: #666; }}
</style>
</defs>
"#, width, height, title));

        writer
    }

    fn add_paths(&mut self, paths: &Paths64, class: &str, label: Option<&str>) {
        self.add_paths_at(paths, class, 0, 0, label);
    }

    fn add_paths_at(&mut self, paths: &Paths64, class: &str, offset_x: i64, offset_y: i64, label: Option<&str>) {
        for path in paths {
            if path.len() < 3 {
                continue;
            }

            let mut path_data = String::from("M");
            for (i, point) in path.iter().enumerate() {
                let x = point.x + offset_x;
                let y = point.y + offset_y;
                
                if i == 0 {
                    path_data.push_str(&format!(" {} {}", x, y));
                } else {
                    path_data.push_str(&format!(" L {} {}", x, y));
                }
            }
            path_data.push_str(" Z");

            self.content.push_str(&format!(
                r#"<path d="{}" class="{}"/>"#,
                path_data, class
            ));
        }

        if let Some(label) = label {
            self.add_text(offset_x, offset_y - 10, label, "operation");
        }
    }

    fn add_text(&mut self, x: i64, y: i64, text: &str, class: &str) {
        self.content.push_str(&format!(
            r#"<text x="{}" y="{}" class="{}">{}</text>"#,
            x, y, class, text
        ));
    }

    fn save(&mut self, filename: &str) -> std::io::Result<()> {
        self.content.push_str("</svg>");
        
        let mut file = File::create(filename)?;
        file.write_all(self.content.as_bytes())?;
        
        println!("   Saved: {}", filename);
        Ok(())
    }
}