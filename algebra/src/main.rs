fn main () {
  let point1 = Point {
    x: 3.0,
    y: 5.0,
  };

  let point2 = Point {
    x: -4.0,
    y: 7.0,
  };

  let slope_intercept = find_slope_intercept(&point1, &point2);
  println!("Slope Intercept: {slope_intercept}");
  println!("Point Slope: {}", find_point_slope(&point1, &point2));
}

struct Point {
  x: f32,
  y: f32,
}

fn find_slope_intercept(point1: &Point, point2: &Point) -> String {
  let delta_y = point2.y - point1.y;
  let delta_x = point2.x - point1.x;
  let y_intercept_enumerator = -(delta_y * point2.x) + (delta_x * point2.y);

  format!("y = ({delta_y}/{delta_x}) * x + ({y_intercept_enumerator}/{delta_x})")
}

fn find_point_slope(point1: &Point, point2: &Point) -> String {
  let delta_y = point2.y - point1.y;
  let delta_x = point2.x - point1.x;

  format!("(y - {}) = ({delta_y}/{delta_x}) * (x - {})", point2.y, point2.x)
}