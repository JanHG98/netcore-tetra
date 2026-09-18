import math
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from geometry import cap_geometry, circle, contains, distance_m, validate_geometry


class GeometryTests(unittest.TestCase):
    def setUp(self):
        self.polygon = {"type": "Polygon", "coordinates": [
            [[6, 49], [8, 49], [8, 51], [6, 51], [6, 49]],
            [[6.5, 49.5], [7.5, 49.5], [7.5, 50.5], [6.5, 50.5], [6.5, 49.5]],
        ]}

    def test_polygon_holes_and_boundaries(self):
        area = validate_geometry(self.polygon)
        self.assertTrue(contains(area, 49.25, 7))
        self.assertTrue(contains(area, 49, 7))
        self.assertFalse(contains(area, 50, 7))
        self.assertFalse(contains(area, 49.5, 7))
        self.assertFalse(contains(area, 52, 7))
        self.assertFalse(contains(area, 7, 49.25))  # lat/lon must not be swapped

    def test_multi_polygon_and_feature_collection(self):
        second = [[10, 50], [11, 50], [11, 51], [10, 51], [10, 50]]
        area = validate_geometry({"type": "FeatureCollection", "features": [
            {"type": "Feature", "geometry": {"type": "MultiPolygon", "coordinates": [self.polygon["coordinates"], [second]]}},
        ]})
        self.assertTrue(contains(area, 50.5, 10.5))
        self.assertTrue(contains(area, 49.25, 7))
        self.assertFalse(contains(area, 50, 7))
        self.assertFalse(contains(area, 50.5, 9))

    def test_circle_and_haversine(self):
        area = circle(52.52, 13.405, 1000)
        self.assertTrue(contains(area, 52.52, 13.405))
        self.assertTrue(contains(area, 52.525, 13.405))
        self.assertFalse(contains(area, 52.53, 13.405))
        self.assertAlmostEqual(distance_m(0, 0, 0, 1), 111195.08, delta=0.1)

    def test_cap_polygon_and_circle_order_and_units(self):
        area = cap_geometry({"area": [
            {"polygon": ["49,6 49,8 51,8 51,6 49,6"]},
            {"circle": ["52.52,13.405 2.5"]},
        ]})
        self.assertTrue(contains(area, 50, 7))
        self.assertTrue(contains(area, 52.54, 13.405))
        self.assertFalse(contains(area, 52.55, 13.405))
        self.assertIsNone(cap_geometry({"area": [{"areaDesc": "Berlin"}]}))

    def test_antimeridian_polygon(self):
        area = validate_geometry({"type": "Polygon", "coordinates": [
            [[179, -1], [-179, -1], [-179, 1], [179, 1], [179, -1]],
        ]})
        self.assertTrue(contains(area, 0, 179.5))
        self.assertTrue(contains(area, 0, -179.5))
        self.assertFalse(contains(area, 0, 0))

    def test_invalid_coordinates_and_geometry_fail_closed(self):
        for coordinate in (math.nan, math.inf, -math.inf, True, "52.52"):
            self.assertFalse(contains(circle(52, 13, 100), coordinate, 13))
        invalid = [
            {"type": "Point", "coordinates": [13, 52]},
            {"type": "Circle", "coordinates": [181, 52], "radius_m": 100},
            {"type": "Circle", "coordinates": [13, 52], "radius_m": -100},
            {"type": "Polygon", "coordinates": [[[0, 0], [1, 0], [1, 1], [0, 1]]]},
            {"type": "Polygon", "coordinates": [[[0, 0], [0, 0], [0, 0], [0, 0]]]},
            {"type": "FeatureCollection", "features": []},
        ]
        for value in invalid:
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate_geometry(value)
        self.assertFalse(contains({"type": "GeometryCollection", "geometries": []}, 52, 13))


if __name__ == "__main__":
    unittest.main()
