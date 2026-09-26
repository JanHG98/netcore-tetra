"""Small WGS84 geofencing helpers, with GeoJSON longitude/latitude ordering.

Polygon exteriors include their boundary. Hole interiors and boundaries are
excluded. Circle distances use the haversine formula, in metres.
"""
from __future__ import annotations

import math
from typing import Any

EARTH_RADIUS_M = 6_371_008.8
MAX_POSITIONS = 200_000


def _number(value: Any) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ValueError("Coordinate and radius values must be numbers")
    result = float(value)
    if not math.isfinite(result):
        raise ValueError("Coordinate and radius values must be finite")
    return result


def _position(value: Any) -> list[float]:
    if not isinstance(value, (list, tuple)) or len(value) < 2:
        raise ValueError("A position requires longitude and latitude")
    lon, lat = _number(value[0]), _number(value[1])
    if not -180 <= lon <= 180 or not -90 <= lat <= 90:
        raise ValueError("Position outside WGS84 bounds")
    return [lon, lat]


def validate_geometry(value: Any, *, allow_empty: bool = False) -> dict:
    """Return a validated, independent geometry (also accepts GeoJSON wrappers).

Fail closed on unsupported/malformed geometry instead of broadening the area.
The internal Circle extension is {type, coordinates: [lon, lat], radius_m}.
"""
    remaining = [MAX_POSITIONS]

    def visit(item: Any, depth: int = 0) -> dict:
        if depth > 20 or not isinstance(item, dict):
            raise ValueError("Invalid or excessively nested geometry")
        kind = item.get("type")
        if kind == "Feature":
            return visit(item.get("geometry"), depth + 1)
        if kind in ("FeatureCollection", "GeometryCollection"):
            children = item.get("features" if kind == "FeatureCollection" else "geometries")
            if not isinstance(children, list) or len(children) > MAX_POSITIONS:
                raise ValueError("Invalid geometry collection")
            if not children and not allow_empty:
                raise ValueError("Warning has no geographic area")
            return {"type": "GeometryCollection", "geometries": [visit(c, depth + 1) for c in children]}
        if kind == "Circle":
            radius = _number(item.get("radius_m"))
            if radius <= 0 or radius > math.pi * EARTH_RADIUS_M:
                raise ValueError("Circle radius must be positive and within Earth bounds")
            remaining[0] -= 1
            if remaining[0] < 0:
                raise ValueError("Too many geometry positions")
            return {"type": kind, "coordinates": _position(item.get("coordinates")), "radius_m": radius}
        if kind not in ("Polygon", "MultiPolygon"):
            raise ValueError(f"Unsupported warning geometry: {kind!r}")
        coords = item.get("coordinates")
        polygons = [coords] if kind == "Polygon" else coords
        if not isinstance(polygons, list) or not polygons:
            raise ValueError("Empty polygon geometry")
        cleaned = []
        for polygon in polygons:
            if not isinstance(polygon, list) or not polygon:
                raise ValueError("Polygon requires at least one ring")
            rings = []
            for ring in polygon:
                if not isinstance(ring, list) or len(ring) < 4:
                    raise ValueError("Polygon ring requires four positions")
                remaining[0] -= len(ring)
                if remaining[0] < 0:
                    raise ValueError("Too many geometry positions")
                points = [_position(p) for p in ring]
                if points[0] != points[-1] or len(set(map(tuple, points))) < 3:
                    raise ValueError("Polygon ring must be closed and have three distinct points")
                rings.append(points)
            cleaned.append(rings)
        return {"type": kind, "coordinates": cleaned[0] if kind == "Polygon" else cleaned}

    return visit(value)


def circle(latitude: float, longitude: float, radius_m: float) -> dict:
    return validate_geometry({"type": "Circle", "coordinates": [longitude, latitude], "radius_m": radius_m})


def distance_m(latitude_a: float, longitude_a: float, latitude_b: float, longitude_b: float) -> float:
    _, lat_a = _position([longitude_a, latitude_a])
    _, lat_b = _position([longitude_b, latitude_b])
    phi_a, phi_b = math.radians(lat_a), math.radians(lat_b)
    dphi = phi_b - phi_a
    dlon = math.radians(longitude_b - longitude_a)
    a = math.sin(dphi / 2) ** 2 + math.cos(phi_a) * math.cos(phi_b) * math.sin(dlon / 2) ** 2
    return 2 * EARTH_RADIUS_M * math.asin(math.sqrt(min(1.0, max(0.0, a))))


def _in_ring(ring: list, latitude: float, longitude: float) -> bool:
    # Unwrap across the antimeridian without changing ordinary local polygons.
    points = [list(ring[0][:2])]
    for raw in ring[1:]:
        x = raw[0]
        previous = points[-1][0]
        while x - previous > 180:
            x -= 360
        while x - previous < -180:
            x += 360
        points.append([x, raw[1]])
    centre = (min(p[0] for p in points) + max(p[0] for p in points)) / 2
    x = longitude + 360 * round((centre - longitude) / 360)
    y = latitude
    inside = False
    for (x1, y1), (x2, y2) in zip(points, points[1:]):
        cross = (x - x1) * (y2 - y1) - (y - y1) * (x2 - x1)
        if abs(cross) <= 1e-10 and min(x1, x2) - 1e-10 <= x <= max(x1, x2) + 1e-10 and min(y1, y2) - 1e-10 <= y <= max(y1, y2) + 1e-10:
            return True
        if (y1 > y) != (y2 > y) and x < x1 + (y - y1) * (x2 - x1) / (y2 - y1):
            inside = not inside
    return inside


def contains(geometry: dict, latitude: float, longitude: float) -> bool:
    """Test a validated geometry; malformed coordinates never match.

    Validate once when importing an alert, not repeatedly for each subscriber.
    """
    try:
        _position([longitude, latitude])
        kind = geometry.get("type")
        if kind == "Feature":
            return contains(geometry["geometry"], latitude, longitude)
        if kind == "FeatureCollection":
            return any(contains(f, latitude, longitude) for f in geometry["features"])
        if kind == "GeometryCollection":
            return any(contains(g, latitude, longitude) for g in geometry["geometries"])
        if kind == "Circle":
            lon, lat = geometry["coordinates"][:2]
            return distance_m(latitude, longitude, lat, lon) <= _number(geometry["radius_m"])
        if kind == "Polygon":
            rings = geometry["coordinates"]
            return _in_ring(rings[0], latitude, longitude) and not any(_in_ring(r, latitude, longitude) for r in rings[1:])
        if kind == "MultiPolygon":
            return any(contains({"type": "Polygon", "coordinates": p}, latitude, longitude) for p in geometry["coordinates"])
    except (ValueError, TypeError, KeyError, IndexError, AttributeError):
        return False
    return False


def cap_geometry(info: dict) -> dict | None:
    """Convert CAP area polygons (lat,lon) and circles (lat,lon radius-km)."""
    result = []
    areas = info.get("area", [])
    if not isinstance(areas, list):
        raise ValueError("Invalid CAP areas")
    for area in areas:
        if not isinstance(area, dict):
            raise ValueError("Invalid CAP area")
        for field in ("polygon", "circle"):
            values = area.get(field, [])
            values = [values] if isinstance(values, str) else values
            if not isinstance(values, list):
                raise ValueError(f"Invalid CAP {field}")
            for value in values:
                if not isinstance(value, str):
                    raise ValueError(f"Invalid CAP {field}")
                if field == "polygon":
                    points = []
                    for pair in value.split():
                        lat, lon = map(float, pair.split(","))
                        points.append([lon, lat])
                    result.append({"type": "Polygon", "coordinates": [points]})
                else:
                    centre, radius = value.split()
                    lat, lon = map(float, centre.split(","))
                    result.append({"type": "Circle", "coordinates": [lon, lat], "radius_m": float(radius) * 1000})
    if not result:
        return None
    return validate_geometry(result[0] if len(result) == 1 else {"type": "GeometryCollection", "geometries": result})
