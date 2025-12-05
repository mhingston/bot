//! Result types for image template matching

/// Result of a template match
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// X coordinate of match (top-left)
    pub x: u32,
    /// Y coordinate of match (top-left)
    pub y: u32,
    /// Width of matched region
    pub width: u32,
    /// Height of matched region
    pub height: u32,
    /// Match confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Scale at which match was found
    pub scale: f32,
}

impl MatchResult {
    /// Create a new match result
    pub fn new(x: u32, y: u32, width: u32, height: u32, confidence: f32, scale: f32) -> Self {
        Self { x, y, width, height, confidence, scale }
    }

    /// Get center point of match
    pub fn center(&self) -> (u32, u32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Get center point as f32 (more precise)
    pub fn center_f32(&self) -> (f32, f32) {
        (self.x as f32 + self.width as f32 / 2.0, self.y as f32 + self.height as f32 / 2.0)
    }

    /// Get bounding box as (x, y, width, height)
    pub fn bounds(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Get right edge x coordinate
    pub fn right(&self) -> u32 {
        self.x + self.width
    }

    /// Get bottom edge y coordinate
    pub fn bottom(&self) -> u32 {
        self.y + self.height
    }

    /// Calculate area of the match region
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if this match overlaps with another
    pub fn overlaps(&self, other: &MatchResult) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Calculate Intersection over Union (IoU) with another match
    pub fn iou(&self, other: &MatchResult) -> f32 {
        if !self.overlaps(other) {
            return 0.0;
        }

        let inter_x1 = self.x.max(other.x);
        let inter_y1 = self.y.max(other.y);
        let inter_x2 = self.right().min(other.right());
        let inter_y2 = self.bottom().min(other.bottom());

        let inter_area = (inter_x2 - inter_x1) * (inter_y2 - inter_y1);
        let union_area = self.area() + other.area() - inter_area;

        if union_area == 0 { 0.0 } else { inter_area as f32 / union_area as f32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_result_center() {
        let result = MatchResult::new(100, 200, 50, 40, 0.9, 1.0);
        assert_eq!(result.center(), (125, 220));
    }

    #[test]
    fn test_match_result_bounds() {
        let result = MatchResult::new(100, 200, 50, 40, 0.9, 1.0);
        assert_eq!(result.bounds(), (100, 200, 50, 40));
        assert_eq!(result.right(), 150);
        assert_eq!(result.bottom(), 240);
        assert_eq!(result.area(), 2000);
    }

    #[test]
    fn test_match_result_overlaps() {
        let r1 = MatchResult::new(100, 100, 50, 50, 0.9, 1.0);
        let r2 = MatchResult::new(120, 120, 50, 50, 0.8, 1.0);
        let r3 = MatchResult::new(200, 200, 50, 50, 0.7, 1.0);

        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn test_match_result_iou() {
        let r1 = MatchResult::new(0, 0, 100, 100, 0.9, 1.0);
        let r2 = MatchResult::new(50, 50, 100, 100, 0.8, 1.0);

        // Intersection: 50x50 = 2500
        // Union: 10000 + 10000 - 2500 = 17500
        // IoU: 2500 / 17500 ≈ 0.143
        let iou = r1.iou(&r2);
        assert!((iou - 0.143).abs() < 0.01);
    }

    #[test]
    fn test_match_result_iou_no_overlap() {
        let r1 = MatchResult::new(0, 0, 50, 50, 0.9, 1.0);
        let r2 = MatchResult::new(100, 100, 50, 50, 0.8, 1.0);

        assert_eq!(r1.iou(&r2), 0.0);
    }
}
