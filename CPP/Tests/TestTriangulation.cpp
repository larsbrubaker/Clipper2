#include <cmath>
#include <gtest/gtest.h>
#include "clipper2/clipper.h"
#include "clipper2/clipper.triangulation.h"

using namespace Clipper2Lib;

TEST(Clipper2Tests, TestTriangulateEdgeThroughVertex)
{
  // https://github.com/PolyClipping/Clipper2/issues/12
  const Path64 subject = MakePath({-209, 3155, 716, 3534, 3324, 2439, 298, 9844, 507, 6689});

  for (bool useDelaunay : {false, true})
  {
    SCOPED_TRACE(useDelaunay);
    Paths64 triangles;
    ASSERT_EQ(Triangulate({subject}, triangles, useDelaunay),
              TriangulateResult::success);

    double triangle_area = 0;
    for (const Path64 &triangle : triangles)
      triangle_area += std::abs(Area(triangle));

    EXPECT_DOUBLE_EQ(triangle_area, std::abs(Area(subject)));
    EXPECT_DOUBLE_EQ(triangle_area, 9498178.0);
  }
}

TEST(Clipper2Tests, TestTriangulateEdgeThroughHoleVertices)
{
  // https://github.com/PolyClipping/Clipper2/issues/12
  const Path64 outer = MakePath({320313, 10000, 310000, 160313, 99688, 150000, -313, 90000, 10000, 59688});
  const Path64 hole = MakePath({210000, 150000, 220000, 130000, 220000, 150000, 310000, 150000, 10000, 70000});

  for (bool useDelaunay : {false, true})
  {
    SCOPED_TRACE(useDelaunay);
    Paths64 triangles;
    ASSERT_EQ(Triangulate({outer, hole}, triangles, useDelaunay),
              TriangulateResult::success);

    double triangle_area = 0;
    for (const Path64 &triangle : triangles)
      triangle_area += std::abs(Area(triangle));

    EXPECT_DOUBLE_EQ(triangle_area,
                     std::abs(Area(outer)) - std::abs(Area(hole)));
    // Exact area for the issue's coordinates (reported there as 30025200468).
    EXPECT_DOUBLE_EQ(triangle_area, 30025200468.5);
  }
}

static void CheckTriangulatedArea(const Paths64& paths, double expected)
{
  for (bool useDelaunay : {false, true})
  {
    SCOPED_TRACE(useDelaunay);
    Paths64 triangles;
    ASSERT_EQ(Triangulate(paths, triangles, useDelaunay),
              TriangulateResult::success);

    double triangle_area = 0;
    for (const Path64 &triangle : triangles)
      triangle_area += std::abs(Area(triangle));
    EXPECT_DOUBLE_EQ(triangle_area, expected);
  }
}

TEST(Clipper2Tests, TestTriangulateInnerMinimumEdgeAvoidsEdgesOnItsRight)
{
  // Only edges inside the connecting edge's x range were checked for crossings,
  // and that range was inverted when the connecting edge headed right.
  CheckTriangulatedArea({MakePath({744413, 460911, 582000, 563000, 675000, 699000,
    483935, 477767, 486556, 455276, 566000, 496000, 729568, 441477})}, 14506790027.5);
  CheckTriangulatedArea({MakePath({20000, 21000, 27000, 13000, 48939, 25683,
    31000, 59000, 41790, 59378, 38000, 74000, 5000, 5000})}, 938833000.0);
}

TEST(Clipper2Tests, TestTriangulateVertexOnEdgeOfSamePath)
{
  // The vertex at (0, 3000) lies on the edge from (0, 4000) to (0, 2000).
  CheckTriangulatedArea({MakePath({5000, 0, 0, 3000, 3000, 3000, 0, 4000, 0, 2000, 2000, 0})},
    7000000.0);
}

TEST(Clipper2Tests, TestTriangulatePathTouchingItselfAtLowestPoint)
{
  // The lowest point is visited twice, which used to decide the orientation.
  Path64 path = MakePath({3000, 1000, 3000, 2000, 5000, 5000, 3000, 3000,
    2000, 3000, 5000, 5000, 5000, 2000, 4000, 1000});
  CheckTriangulatedArea({path}, 5500000.0);
  std::reverse(path.begin(), path.end());
  CheckTriangulatedArea({path}, 5500000.0);
}

TEST(Clipper2Tests, TestTriangulateEdgesSharedInOppositeDirectionsCancel)
{
  // Both sides of the shared edges are filled.
  CheckTriangulatedArea({MakePath({10000, 12000, 10000, 13000, 10000, 19000, 2000, 2000}),
    MakePath({10000, 7000, 13000, 7000, 10000, 13000})}, 37000000.0);
  // Both sides of the shared edges are empty.
  CheckTriangulatedArea({MakePath({4000, 5000, 0, 5000, 2333, 2666, 2000, 4000, 3000, 2000}),
    MakePath({3000, 2000, 2000, 4000, 2333, 2666})}, 6001500.0);
}

TEST(Clipper2Tests, TestTriangulateLobeTouchingReflexVertex)
{
  // At (2000, 10000) a lobe sits in the notch of a reflex vertex.
  CheckTriangulatedArea({MakePath({2000, 10000, 2000, 6000, 6571, 4857, 5200, 7600,
      2000, 10000, 5058, 7882, 0, 18000, 1000, 3000}),
    MakePath({5058, 7882, 5200, 7600, 10000, 4000, 6571, 4857, 8000, 2000, 15000, 1000})},
    47998800.0);
}

TEST(Clipper2Tests, TestTriangulateDegeneratePathsTerminate)
{
  // These made AddPath skip past its closing vertex and never finish.
  CheckTriangulatedArea({MakePath({0, 0, 10, 0, 10, 10, 5, 10, 5, 20, 5, 10, 4, 2, 3, 10, 0, 10})},
    92.0);
  CheckTriangulatedArea({MakePath({4, 4, 0, 0, 8, 0, 4, 4, 2, 4})}, 16.0);
  CheckTriangulatedArea({MakePath({7, 4, 4, 1, 7, 2, 9, 3, 8, 3, 7, 3, 7, 4, 6, 4})}, 4.0);
}

TEST(Clipper2Tests, TestTriangulateCollinearVerticesWithEitherWinding)
{
  Path64 path = MakePath({6000, 0, 8000, 6000, 4000, 4000, 0, 2000, 5000, 5000, 10000, 8000});
  CheckTriangulatedArea({path}, 8000000.0);
  std::reverse(path.begin(), path.end());
  CheckTriangulatedArea({path}, 8000000.0);
}

TEST(Clipper2Tests, TestTriangulateIgnoredTinyPathMeetingAnotherPath)
{
  // The first path is a triangle with a unit-length edge, too small to
  // triangulate, and one of its edges meets the second path within tolerance.
  CheckTriangulatedArea({MakePath({1, 3, 2, 3, 1, 5, 1, 4}),
    MakePath({2, 2, 4, 2, 2, 5, 2, 4, 1, 2})}, 4.5);
}
