use nalgebra::Point2;
use serde::Serialize;
use crabe_math::shape::Line;

/// Represents a penalty area on a soccer field. (all distances are in meters)
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    /// The width of the penalty area 
    pub width: f64,
    /// The depth of the penalty area 
    pub depth: f64,
    /// The front line of the penalty area
    pub front_line: Line,
    /// The left line of the penalty area (looking from the center of the field)
    pub left_line: Line,
    /// The right line of the penalty area (looking from the center of the field)
    pub right_line: Line,
}

impl Penalty {

    //return a penalty enlarged with the margin parameter 
    pub fn enlarged_penalty(&self, offset: f64) -> Penalty {
        let factor = self.front_line.start.x.signum();
        Penalty{
            width: self.width+offset*2.,
            depth: self.depth+offset,
            front_line: Line::new(
                Point2::new(self.front_line.start.x - factor * offset,self.front_line.start.y + factor*offset), 
                Point2::new(self.front_line.end.x - factor * offset, self.front_line.end.y - factor*offset)
            ),
            left_line: Line::new(
                Point2::new(self.left_line.start.x,self.left_line.start.y + factor*offset), 
                Point2::new(self.left_line.end.x - factor * offset,self.left_line.end.y + factor*offset), 
            ),
            right_line: Line::new(
                Point2::new(self.right_line.start.x,self.right_line.start.y - factor*offset), 
                Point2::new(self.right_line.end.x - factor * offset,self.right_line.end.y - factor*offset), 
            ),
        }
    }
}
