//! CAD/CAM Toolpath Generation Example
//! 
//! This example demonstrates real-world usage of Clipper2 in CAD/CAM applications,
//! showing how to generate toolpaths for CNC machining operations.

use clipper2::{
    Clipper64, ClipType, FillRule, JoinType, EndType,
    MakePath, InflatePaths, Union, Difference, area, Length,
    Path64, Paths64
};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("🔧 CAD/CAM Toolpath Generation Example");
    println!("======================================\n");

    // Create output directory
    std::fs::create_dir_all("output")?;

    // Example: Machining a part with holes and pockets
    let part_outline = create_part_outline();
    let holes = create_holes();
    let pockets = create_pockets();

    println!("📐 Part Geometry:");
    println!("  • Main outline: {} vertices, area: {:.1}",
        part_outline.len(), area(&part_outline).abs());
    println!("  • Holes: {} features", holes.len());
    println!("  • Pockets: {} features", pockets.len());

    // Generate toolpaths for different operations
    generate_roughing_toolpath(&part_outline, &holes, &pockets)?;
    generate_finishing_toolpath(&part_outline, &holes)?;
    generate_hole_drilling_pattern(&holes)?;
    generate_pocket_milling(&pockets)?;

    println!("\n✅ All toolpaths generated successfully!");
    println!("   Check the output/ directory for G-code files.");

    Ok(())
}

fn generate_roughing_toolpath(outline: &Path64, holes: &Paths64, pockets: &Paths64) -> std::io::Result<()> {
    println!("\n🔨 Generating Roughing Toolpath");
    println!("-------------------------------");

    let tool_diameter = 10.0; // 10mm end mill
    let step_over = tool_diameter * 0.6; // 60% stepover
    let stock_to_leave = 0.5; // 0.5mm stock for finishing

    // Create the material removal area
    let part_paths = vec![outline.clone()];
    
    // Subtract holes and pockets from the part
    let mut material_area = part_paths.clone();
    if !holes.is_empty() {
        material_area = Difference(&material_area, holes, FillRule::NonZero);
    }
    if !pockets.is_empty() {
        material_area = Difference(&material_area, pockets, FillRule::NonZero);
    }

    // Generate offset toolpaths from outside to inside
    let mut toolpaths = Vec::new();
    let mut current_area = material_area;
    let mut pass = 1;

    loop {
        // Offset inward by tool radius + stock allowance
        let tool_offset = -(tool_diameter / 2.0 + stock_to_leave);
        let offset_area = InflatePaths(&current_area, tool_offset, JoinType::Round, EndType::Polygon);
        
        if offset_area.is_empty() {
            break;
        }

        toolpaths.extend(offset_area.clone());
        
        // Next pass - offset by stepover
        current_area = InflatePaths(&offset_area, -step_over, JoinType::Round, EndType::Polygon);
        
        if current_area.is_empty() {
            break;
        }

        pass += 1;
        if pass > 20 { // Safety limit
            break;
        }
    }

    let total_length = toolpaths.iter().map(|path| Length(path)).sum::<f64>();
    
    println!("  • Passes generated: {}", pass);
    println!("  • Total toolpath length: {:.1}mm", total_length);
    println!("  • Estimated machining time: {:.1}min (at 1000mm/min)", total_length / 1000.0);

    // Export to G-code
    export_gcode(&toolpaths, "output/roughing_toolpath.nc", "Roughing Toolpath", 1000.0)?;

    Ok(())
}

fn generate_finishing_toolpath(outline: &Path64, holes: &Paths64) -> std::io::Result<()> {
    println!("\n✨ Generating Finishing Toolpath");
    println!("--------------------------------");

    let tool_diameter = 6.0; // 6mm end mill for finishing
    let tool_radius = tool_diameter / 2.0;

    // Finish the part outline
    let outline_toolpath = InflatePaths(&vec![outline.clone()], -tool_radius, JoinType::Round, EndType::Polygon);
    
    // Finish the holes
    let mut hole_toolpaths = Vec::new();
    for hole in holes {
        let hole_finish = InflatePaths(&vec![hole.clone()], tool_radius, JoinType::Round, EndType::Polygon);
        hole_toolpaths.extend(hole_finish);
    }

    let outline_passes = if outline_toolpath.is_empty() { 0 } else { 1 };
    let mut all_toolpaths = outline_toolpath;
    all_toolpaths.extend(hole_toolpaths);

    let total_length = all_toolpaths.iter().map(|path| Length(path)).sum::<f64>();
    
    println!("  • Outline passes: {}", outline_passes);
    println!("  • Hole finish passes: {}", holes.len());
    println!("  • Total toolpath length: {:.1}mm", total_length);
    println!("  • Estimated machining time: {:.1}min (at 500mm/min)", total_length / 500.0);

    export_gcode(&all_toolpaths, "output/finishing_toolpath.nc", "Finishing Toolpath", 500.0)?;

    Ok(())
}

fn generate_hole_drilling_pattern(holes: &Paths64) -> std::io::Result<()> {
    println!("\n🕳️  Generating Hole Drilling Pattern");
    println!("-----------------------------------");

    let mut drill_points = Vec::new();
    
    for hole in holes {
        // Calculate hole center
        let center = calculate_centroid(hole);
        drill_points.push(center);
    }

    println!("  • Drill points: {}", drill_points.len());
    println!("  • Estimated drilling time: {:.1}min", drill_points.len() as f64 * 0.5);

    // Export drill G-code
    let mut gcode = String::new();
    gcode.push_str("; Hole Drilling Pattern\n");
    gcode.push_str("G90 ; Absolute positioning\n");
    gcode.push_str("G01 F300 ; Feed rate 300mm/min\n");
    gcode.push_str("M03 S1200 ; Spindle on, 1200 RPM\n\n");

    for (i, point) in drill_points.iter().enumerate() {
        gcode.push_str(&format!("; Hole {}\n", i + 1));
        gcode.push_str(&format!("G00 X{:.3} Y{:.3} ; Rapid to position\n", point.0, point.1));
        gcode.push_str("G01 Z-10.000 ; Drill down\n");
        gcode.push_str("G00 Z5.000 ; Retract\n\n");
    }

    gcode.push_str("M05 ; Spindle off\n");
    gcode.push_str("G00 Z50.000 ; Final retract\n");

    let mut file = File::create("output/drilling_pattern.nc")?;
    file.write_all(gcode.as_bytes())?;
    println!("  • Saved: output/drilling_pattern.nc");

    Ok(())
}

fn generate_pocket_milling(pockets: &Paths64) -> std::io::Result<()> {
    println!("\n🔲 Generating Pocket Milling Toolpath");
    println!("-------------------------------------");

    let tool_diameter = 8.0; // 8mm end mill
    let step_over = tool_diameter * 0.5; // 50% stepover for pockets

    let mut all_pocket_paths = Vec::new();
    
    for (i, pocket) in pockets.iter().enumerate() {
        println!("  Processing pocket {} ...", i + 1);
        
        let pocket_paths = vec![pocket.clone()];
        let mut current_area = pocket_paths;
        let mut pass = 1;

        loop {
            // Offset inward by tool radius
            let offset_area = InflatePaths(&current_area, -(tool_diameter / 2.0), JoinType::Round, EndType::Polygon);
            
            if offset_area.is_empty() {
                break;
            }

            all_pocket_paths.extend(offset_area.clone());
            
            // Next pass
            current_area = InflatePaths(&offset_area, -step_over, JoinType::Round, EndType::Polygon);
            
            if current_area.is_empty() {
                break;
            }

            pass += 1;
            if pass > 15 { // Safety limit
                break;
            }
        }

        println!("    • Passes: {}", pass);
    }

    let total_length = all_pocket_paths.iter().map(|path| Length(path)).sum::<f64>();
    
    println!("  • Total pockets: {}", pockets.len());
    println!("  • Total toolpath length: {:.1}mm", total_length);
    println!("  • Estimated machining time: {:.1}min (at 800mm/min)", total_length / 800.0);

    export_gcode(&all_pocket_paths, "output/pocket_milling.nc", "Pocket Milling", 800.0)?;

    Ok(())
}

fn export_gcode(toolpaths: &Paths64, filename: &str, description: &str, feed_rate: f64) -> std::io::Result<()> {
    let mut gcode = String::new();
    
    // G-code header
    gcode.push_str(&format!("; {}\n", description));
    gcode.push_str("; Generated by Clipper2 Rust CAD/CAM Example\n");
    gcode.push_str("G90 ; Absolute positioning\n");
    gcode.push_str(&format!("G01 F{:.0} ; Feed rate {:.0}mm/min\n", feed_rate, feed_rate));
    gcode.push_str("M03 S1800 ; Spindle on, 1800 RPM\n");
    gcode.push_str("G00 Z5.000 ; Safe height\n\n");

    for (i, path) in toolpaths.iter().enumerate() {
        if path.is_empty() {
            continue;
        }

        gcode.push_str(&format!("; Path {}\n", i + 1));
        
        // Move to start point
        let start = &path[0];
        gcode.push_str(&format!("G00 X{:.3} Y{:.3} ; Move to start\n", start.x as f64 / 1000.0, start.y as f64 / 1000.0));
        gcode.push_str("G01 Z-2.000 ; Plunge\n");

        // Cut the path
        for point in path.iter().skip(1) {
            gcode.push_str(&format!("G01 X{:.3} Y{:.3}\n", point.x as f64 / 1000.0, point.y as f64 / 1000.0));
        }

        // Close the path if it's a closed contour
        if path.len() > 2 {
            let start = &path[0];
            gcode.push_str(&format!("G01 X{:.3} Y{:.3} ; Close path\n", start.x as f64 / 1000.0, start.y as f64 / 1000.0));
        }

        gcode.push_str("G00 Z5.000 ; Retract\n\n");
    }

    // G-code footer
    gcode.push_str("M05 ; Spindle off\n");
    gcode.push_str("G00 Z50.000 ; Final retract\n");
    gcode.push_str("M30 ; Program end\n");

    let mut file = File::create(filename)?;
    file.write_all(gcode.as_bytes())?;
    println!("  • Saved: {}", filename);

    Ok(())
}

// Helper functions to create part geometry

fn create_part_outline() -> Path64 {
    // Create a complex part shape
    MakePath(&[
        0, 0,           // Bottom left
        200, 0,         // Bottom edge
        250, 20,        // Chamfer
        300, 20,        // Bottom right
        300, 80,        // Right edge
        280, 100,       // Chamfer
        280, 150,       // Right edge
        300, 170,       // Chamfer
        300, 200,       // Top right
        200, 200,       // Top edge
        180, 220,       // Chamfer
        120, 220,       // Top edge
        100, 200,       // Chamfer
        0, 200,         // Top left
        0, 150,         // Left edge
        20, 130,        // Chamfer
        20, 70,         // Left edge
        0, 50,          // Chamfer
    ])
}

fn create_holes() -> Paths64 {
    vec![
        // Hole 1
        create_circle(50, 50, 15, 16),
        // Hole 2
        create_circle(250, 50, 12, 16),
        // Hole 3
        create_circle(50, 170, 10, 12),
        // Hole 4 (slot)
        create_rectangle(200, 120, 60, 20),
    ]
}

fn create_pockets() -> Paths64 {
    vec![
        // Pocket 1 - rectangular
        create_rectangle(100, 80, 80, 40),
        // Pocket 2 - circular
        create_circle(180, 160, 25, 20),
    ]
}

fn create_rectangle(x: i64, y: i64, width: i64, height: i64) -> Path64 {
    MakePath(&[x, y, x + width, y, x + width, y + height, x, y + height])
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

fn calculate_centroid(path: &Path64) -> (f64, f64) {
    if path.is_empty() {
        return (0.0, 0.0);
    }

    let sum_x: i64 = path.iter().map(|p| p.x).sum();
    let sum_y: i64 = path.iter().map(|p| p.y).sum();
    
    (
        sum_x as f64 / path.len() as f64 / 1000.0, // Convert to mm
        sum_y as f64 / path.len() as f64 / 1000.0
    )
}