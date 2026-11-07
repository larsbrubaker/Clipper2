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
