# Clipper2 C++ to Rust Function Mapping

This document tracks every function that needs to be ported from C++ to Rust for 100% compatibility.

## Status Legend
- ✅ **COMPLETE** - Function fully implemented and tested
- 🚧 **IN_PROGRESS** - Function being implemented
- ❌ **NOT_STARTED** - Function not yet implemented
- ⚠️ **NEEDS_REVIEW** - Function implemented but needs verification

## Core Data Structures

### Point64 and Basic Geometry
- ✅ `Point64` struct
- ✅ `Paths64` and `Path64` types
- ❌ `GetDx(pt1, pt2)` - Calculate delta X per unit Y
- ❌ `TopX(ae, currentY)` - Calculate X coordinate at Y for active edge
- ❌ `CrossProduct(pt1, pt2, pt3)` - Cross product calculation
- ❌ `DotProduct(pt1, pt2, pt3)` - Dot product calculation

### Vertex Structure
- ❌ `Vertex` struct with next/prev pointers
- ❌ `NextVertex(e)` - Get next vertex in winding direction
- ❌ `PrevPrevVertex(ae)` - Get previous-previous vertex

### Active Edge Structure
- ❌ `Active` struct complete implementation
- ❌ `SetDx(e)` - Set delta X for active edge
- ❌ `IsHorizontal(e)` - Check if edge is horizontal
- ❌ `IsHeadingRightHorz(e)` - Check horizontal direction
- ❌ `IsHeadingLeftHorz(e)` - Check horizontal direction
- ❌ `SwapActives(e1, e2)` - Swap two active edges

### OutRec and OutPt Structures
- ❌ `OutRec` struct complete implementation
- ❌ `OutPt` struct complete implementation
- ❌ `IsInvalidPath(op)` - Check if OutPt path is valid
- ❌ `IsFront(e)` - Check if edge is front edge of OutRec

### Local Minima
- ❌ `LocalMinima` struct
- ❌ `GetPolyType(e)` - Get polytype from active edge
- ❌ `IsSamePolyType(e1, e2)` - Compare polytypes

### Intersection Node
- ❌ `IntersectNode` struct
- ❌ Intersection node list management

### Horizontal Segments
- ❌ `HorzSegment` struct
- ❌ `HorzJoin` struct
- ❌ Horizontal segment processing

## ClipperBase Core Functions

### Initialization and Cleanup
- ❌ `ClipperBase()` constructor
- ❌ `~ClipperBase()` destructor
- ❌ `Clear()` - Clear all data structures
- ❌ `CleanUp()` - Cleanup preserving paths
- ❌ `Reset()` - Reset for new operation
- ❌ `DisposeAllOutRecs()` - Cleanup OutRecs
- ❌ `DisposeVerticesAndLocalMinima()` - Cleanup vertices

### Path Addition
- ❌ `AddPath(path, polytype, is_open)` - Add single path
- ❌ `AddPaths(paths, polytype, is_open)` - Add multiple paths
- ❌ `AddLocMin(vert, polytype, is_open)` - Add local minimum

### Scanline Management
- ❌ `InsertScanline(y)` - Insert Y coordinate into scanline queue
- ❌ `PopScanline(y)` - Pop next Y coordinate
- ❌ `PopLocalMinima(y, local_minima)` - Get local minima at Y

### Main Algorithm Loop
- ❌ `ExecuteInternal(cliptype, fillrule, use_polytrees)` - Main execution
- ❌ `InsertLocalMinimaIntoAEL(bot_y)` - Insert local mins into AEL
- ❌ `DoIntersections(top_y)` - Process intersections
- ❌ `DoTopOfScanbeam(top_y)` - Process top of scanbeam

### Active Edge List (AEL) Management
- ❌ `InsertLeftEdge(e)` - Insert edge on left side of AEL
- ❌ `UpdateEdgeIntoAEL(e)` - Update edge position in AEL
- ❌ `DeleteFromAEL(e)` - Remove edge from AEL
- ❌ `DeleteEdges(e)` - Delete multiple edges
- ❌ `AdjustCurrXAndCopyToSEL(top_y)` - Update current X and copy to SEL
- ❌ `SwapPositionsInAEL(edge1, edge2)` - Swap edge positions

### Intersection Processing
- ❌ `BuildIntersectList(top_y)` - Build list of intersections
- ❌ `ProcessIntersectList()` - Process all intersections
- ❌ `AddNewIntersectNode(e1, e2, top_y)` - Add intersection node
- ❌ `IntersectEdges(e1, e2, pt)` - Process edge intersection

### Winding Count Calculations
- ❌ `SetWindCountForClosedPathEdge(edge)` - Set winding count for closed paths
- ❌ `SetWindCountForOpenPathEdge(e)` - Set winding count for open paths
- ❌ `IsContributingClosed(e)` - Check if closed edge contributes
- ❌ `IsContributingOpen(e)` - Check if open edge contributes

### Output Record Management
- ❌ `NewOutRec()` - Create new output record
- ❌ `AddOutPt(e, pt)` - Add output point
- ❌ `AddLocalMinPoly(e1, e2, pt, is_new)` - Add local minimum polygon
- ❌ `AddLocalMaxPoly(e1, e2, pt)` - Add local maximum polygon
- ❌ `JoinOutrecPaths(e1, e2)` - Join output record paths

### Horizontal Edge Processing
- ❌ `PushHorz(e)` - Push edge to horizontal list
- ❌ `PopHorz(e)` - Pop edge from horizontal list
- ❌ `DoHorizontal(horz)` - Process horizontal edge
- ❌ `ResetHorzDirection(horz, max_vertex, horz_left, horz_right)` - Reset horizontal direction

### Maxima Processing
- ❌ `DoMaxima(e)` - Process edge maxima

### Path Building and Validation
- ❌ `StartOpenPath(e, pt)` - Start new open path
- ❌ `CheckBounds(outrec)` - Check output record bounds
- ❌ `CleanCollinear(outrec)` - Remove collinear points

### Self-Intersection Fixing
- ❌ `FixSelfIntersects(outrec)` - Fix self-intersecting polygons
- ❌ `DoSplitOp(outRec, splitOp)` - Perform split operation

### Horizontal Join Processing
- ❌ `AddTrialHorzJoin(op)` - Add trial horizontal join
- ❌ `ConvertHorzSegsToJoins()` - Convert horizontal segments to joins
- ❌ `ProcessHorzJoins()` - Process horizontal joins

### Edge Splitting and Joining
- ❌ `Split(e, pt)` - Split edge at point
- ❌ `CheckJoinLeft(e, pt, check_curr_x)` - Check for left join
- ❌ `CheckJoinRight(e, pt, check_curr_x)` - Check for right join

### PolyTree Support
- ❌ `CheckSplitOwner(outrec, splits)` - Check split ownership
- ❌ `RecursiveCheckOwners(outrec, polypath)` - Recursive owner checking

## Utility Functions (from clipper.core.h)

### Fill Rule Processing
- ✅ `FillRule` enum
- ❌ Fill rule evaluation logic

### Area and Orientation
- ✅ `Area(path)` - Calculate polygon area
- ✅ `IsPositive(path)` - Check polygon orientation

### Point-in-Polygon
- ❌ `PointInPolygon(pt, path)` - Point-in-polygon test

### Path Manipulation
- ❌ `ReversePath(path)` - Reverse path direction
- ❌ `TrimCollinear(path, is_open)` - Remove collinear points
- ❌ `SimplifyPath(path, epsilon, is_closed)` - Simplify path

## High-Level API Functions (from clipper.h)

### Boolean Operations
- ❌ `BooleanOp(cliptype, fillrule, subjects, clips)` - Generic boolean operation
- ❌ `Union(subjects, clips, fillrule)` - Union operation
- ❌ `Intersect(subjects, clips, fillrule)` - Intersection operation
- ❌ `Difference(subjects, clips, fillrule)` - Difference operation
- ❌ `Xor(subjects, clips, fillrule)` - XOR operation

### Path Operations
- ❌ `InflatePaths(paths, delta, jointype, endtype)` - Path offsetting
- ❌ `TranslatePath(path, dx, dy)` - Path translation
- ❌ `ScalePath(path, scale)` - Path scaling

## Implementation Priority

### Phase 1: Core Data Structures (Week 1)
1. Complete `Point64` and basic geometry functions
2. Implement `Vertex`, `Active`, `OutRec`, `OutPt` structures
3. Implement `LocalMinima` and `IntersectNode`

### Phase 2: Basic Algorithm Framework (Week 2)
1. Scanline management functions
2. AEL basic operations
3. Path addition and initialization

### Phase 3: Core Algorithm (Week 3-4)
1. Intersection detection and processing
2. Winding count calculations
3. Edge insertion and management

### Phase 4: Output Processing (Week 5)
1. Output record management
2. Path building from OutPt chains
3. Horizontal edge processing

### Phase 5: Advanced Features (Week 6)
1. Self-intersection fixing
2. Horizontal joins
3. PolyTree support

### Phase 6: Testing and Optimization (Week 7)
1. Comprehensive test coverage
2. Performance optimization
3. Edge case handling

## Testing Strategy

Each function will be tested with:
1. **Unit Tests** - Individual function behavior
2. **Integration Tests** - Function interaction
3. **Reference Tests** - Output compared to C++ reference
4. **Edge Case Tests** - Boundary conditions and error cases

## Progress Tracking

- **Total Functions**: ~120 functions to implement
- **Completed**: 6 functions (5%)
- **In Progress**: 1 function
- **Remaining**: 113 functions (95%)

Last Updated: Current Date
Next Review: Weekly