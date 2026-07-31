use super::super::deterministic_math::{atan2, hypot};

#[derive(Debug, Clone, Copy)]
pub(super) struct PathPlacement {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PathSamplingError {
    InvalidCopyCount,
    InvalidPointCount,
    ZeroLengthSegment { point_index: usize },
}

struct PathSegment {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    delta_x: f64,
    delta_y: f64,
    length: f64,
    rotation: f64,
}

pub(super) fn along_path_placements(
    copies: u64,
    points: &[(f64, f64)],
    rotate_items: bool,
) -> Result<Vec<PathPlacement>, PathSamplingError> {
    if copies < 2 {
        return Err(PathSamplingError::InvalidCopyCount);
    }
    if points.len() < 2 {
        return Err(PathSamplingError::InvalidPointCount);
    }

    let mut segments = Vec::with_capacity(points.len() - 1);
    let mut total_length = 0.0;
    for (index, pair) in points.windows(2).enumerate() {
        let (start_x, start_y) = pair[0];
        let (end_x, end_y) = pair[1];
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        let length = hypot(delta_x, delta_y);
        if length == 0.0 {
            return Err(PathSamplingError::ZeroLengthSegment {
                point_index: index + 1,
            });
        }
        total_length += length;
        segments.push(PathSegment {
            start_x,
            start_y,
            end_x,
            end_y,
            delta_x,
            delta_y,
            length,
            rotation: atan2(delta_y, delta_x),
        });
    }

    let capacity = usize::try_from(copies).unwrap_or_default();
    let mut placements = Vec::with_capacity(capacity);
    let last_copy = copies - 1;
    let mut segment_index = 0;
    let mut segment_start_distance = 0.0;

    for index in 0..copies {
        let segment = &segments[segment_index];
        let (x, y, rotation) = if index == 0 {
            (segment.start_x, segment.start_y, segment.rotation)
        } else if index == last_copy {
            let last_segment = &segments[segments.len() - 1];
            (
                last_segment.end_x,
                last_segment.end_y,
                last_segment.rotation,
            )
        } else {
            let target_distance = total_length * index as f64 / last_copy as f64;
            while segment_index + 1 < segments.len()
                && target_distance >= segment_start_distance + segments[segment_index].length
            {
                segment_start_distance += segments[segment_index].length;
                segment_index += 1;
            }
            let active_segment = &segments[segment_index];
            let progress = (target_distance - segment_start_distance) / active_segment.length;
            (
                active_segment.start_x + active_segment.delta_x * progress,
                active_segment.start_y + active_segment.delta_y * progress,
                active_segment.rotation,
            )
        };
        placements.push(PathPlacement {
            x,
            y,
            rotation: if rotate_items { rotation } else { 0.0 },
        });
    }

    Ok(placements)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::{PathSamplingError, along_path_placements};

    #[test]
    fn samples_equal_arc_length_with_outgoing_vertex_tangents() {
        let placements = along_path_placements(5, &[(0.0, 0.0), (60.0, 0.0), (60.0, 60.0)], true)
            .expect("valid path sampling");

        let expected = [
            (0.0, 0.0, 0.0),
            (30.0, 0.0, 0.0),
            (60.0, 0.0, FRAC_PI_2),
            (60.0, 30.0, FRAC_PI_2),
            (60.0, 60.0, FRAC_PI_2),
        ];
        for (placement, (x, y, rotation)) in placements.iter().zip(expected) {
            assert_eq!(placement.x, x);
            assert_eq!(placement.y, y);
            assert_eq!(placement.rotation, rotation);
        }
    }

    #[test]
    fn spaces_copies_by_total_length_across_unequal_segments() {
        let placements =
            along_path_placements(4, &[(0.0, 0.0), (20.0, 0.0), (20.0, 100.0)], false)
                .expect("valid unequal path sampling");

        let expected = [(0.0, 0.0), (20.0, 20.0), (20.0, 60.0), (20.0, 100.0)];
        for (placement, (x, y)) in placements.iter().zip(expected) {
            assert_eq!(placement.x, x);
            assert_eq!(placement.y, y);
            assert_eq!(placement.rotation, 0.0);
        }
    }

    #[test]
    fn rejects_consecutive_duplicate_points() {
        let error = along_path_placements(3, &[(0.0, 0.0), (0.0, 0.0), (20.0, 0.0)], true)
            .expect_err("duplicate points must fail");
        assert_eq!(
            error,
            PathSamplingError::ZeroLengthSegment { point_index: 1 }
        );
    }
}
