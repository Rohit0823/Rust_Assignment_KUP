///Coordinate enum which used to encapsulate the X_axis as Abscissa and Y_axis as Ordinate
///
/// #variant
///
/// Abscissa & Ordinate:-Is the variant of enum Coordinate
///
#[derive(PartialEq, Eq, Debug)]
pub enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
///Position enum which used to describe the Position of Quadrant
///
/// #field
///
/// First,Second,Third,Fourth:Are the field of enum Position
///
/// XAxis,YAxis:IS the field of enum Position
///
/// Origin: Origin is the field of enum Position
#[derive(PartialEq, Eq, Debug)]
pub enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
}
/// check_coordinate which used check the Quadrant of the given point
///
/// #Arguments
///
///point: A tuple of two integer type value
///
/// #Return
///
/// Returns Result enum which used give the Position lied point and handle Error as well....
pub fn check_coordinate(points: (i32, i32)) -> Result<Position, String> {
    match points {
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => Ok(Position::First(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => Ok(Position::Second(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => Ok(Position::Third(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => Ok(Position::Fourth(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis != 0 => Ok(Position::YAxis(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis != 0 && y_axis == 0 => Ok(Position::XAxis(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => Ok(Position::Origin(
            Coordinate::Abscissa(points.0),
            Coordinate::Ordinate(points.1),
        )),
        _ => Err("Wrong Ip".to_string()),
    }
}
fn main() {
    println!("(check_coordinates)");
    let point_check = (2, -2);
    println!("{:?}", check_coordinate(point_check));
}