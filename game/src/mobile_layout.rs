use std::collections::HashMap;

use egui::accesskit::{Rect, Vec2};
use kson::{BtLane, Side};

use crate::button_codes::UscButton;

#[derive(Debug, Clone, Copy)]
pub(crate) struct NormalizedRect {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl NormalizedRect {
    pub(crate) const fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub(crate) fn resolve(self, screen_size: Vec2) -> Rect {
        Rect::new(
            self.min_x * screen_size.x,
            self.min_y * screen_size.y,
            self.max_x * screen_size.x,
            self.max_y * screen_size.y,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TouchZone {
    button: UscButton,
    bounds: NormalizedRect,
}

impl TouchZone {
    const fn new(button: UscButton, bounds: NormalizedRect) -> Self {
        Self { button, bounds }
    }
}

#[derive(Debug)]
pub(crate) struct MobileLayout {
    touch_zones: Vec<TouchZone>,
}

#[derive(Debug)]
pub(crate) struct ResolvedMobileLayout {
    pub(crate) button_areas: HashMap<UscButton, Rect>,
}

impl MobileLayout {
    pub(crate) fn legacy() -> Self {
        const ONE_SIXTH: f64 = 1.0 / 6.0;
        const ONE_QUARTER: f64 = 1.0 / 4.0;

        let touch_zones = vec![
            TouchZone::new(
                UscButton::Laser(Side::Left, Side::Left),
                NormalizedRect::new(0.0, 0.0, ONE_SIXTH, 0.5),
            ),
            TouchZone::new(
                UscButton::Laser(Side::Left, Side::Right),
                NormalizedRect::new(0.0, 0.5, ONE_SIXTH, 1.0),
            ),
            TouchZone::new(
                UscButton::Laser(Side::Right, Side::Left),
                NormalizedRect::new(5.0 * ONE_SIXTH, 0.0, 1.0, 0.5),
            ),
            TouchZone::new(
                UscButton::Laser(Side::Right, Side::Right),
                NormalizedRect::new(5.0 * ONE_SIXTH, 0.5, 1.0, 1.0),
            ),
            TouchZone::new(
                UscButton::Back,
                NormalizedRect::new(ONE_SIXTH, 0.0, 5.0 * ONE_SIXTH, ONE_QUARTER),
            ),
            TouchZone::new(
                UscButton::Start,
                NormalizedRect::new(ONE_SIXTH, ONE_QUARTER, 4.0 * ONE_SIXTH, 2.0 * ONE_QUARTER),
            ),
            TouchZone::new(
                UscButton::BT(BtLane::A),
                NormalizedRect::new(
                    ONE_SIXTH,
                    2.0 * ONE_QUARTER,
                    2.0 * ONE_SIXTH,
                    3.0 * ONE_QUARTER,
                ),
            ),
            TouchZone::new(
                UscButton::BT(BtLane::B),
                NormalizedRect::new(
                    2.0 * ONE_SIXTH,
                    2.0 * ONE_QUARTER,
                    3.0 * ONE_SIXTH,
                    3.0 * ONE_QUARTER,
                ),
            ),
            TouchZone::new(
                UscButton::BT(BtLane::C),
                NormalizedRect::new(
                    3.0 * ONE_SIXTH,
                    2.0 * ONE_QUARTER,
                    4.0 * ONE_SIXTH,
                    3.0 * ONE_QUARTER,
                ),
            ),
            TouchZone::new(
                UscButton::BT(BtLane::D),
                NormalizedRect::new(
                    4.0 * ONE_SIXTH,
                    2.0 * ONE_QUARTER,
                    5.0 * ONE_SIXTH,
                    3.0 * ONE_QUARTER,
                ),
            ),
            TouchZone::new(
                UscButton::FX(Side::Left),
                NormalizedRect::new(ONE_SIXTH, 3.0 * ONE_QUARTER, 3.0 * ONE_SIXTH, 1.0),
            ),
            TouchZone::new(
                UscButton::FX(Side::Right),
                NormalizedRect::new(3.0 * ONE_SIXTH, 3.0 * ONE_QUARTER, 5.0 * ONE_SIXTH, 1.0),
            ),
        ];

        Self { touch_zones }
    }

    pub(crate) fn resolve(&self, screen_size: Vec2) -> ResolvedMobileLayout {
        let button_areas = self
            .touch_zones
            .iter()
            .map(|zone| (zone.button, zone.bounds.resolve(screen_size)))
            .collect();

        ResolvedMobileLayout { button_areas }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const EPSILON: f64 = 1.0e-9;

    fn legacy_oracle(screen_size: Vec2) -> HashMap<UscButton, Rect> {
        let col_width = screen_size.x / 6.0;
        let row_height = screen_size.y / 4.0;
        let mut areas = HashMap::new();

        areas.insert(
            UscButton::Laser(Side::Left, Side::Left),
            Rect::new(0.0, 0.0, col_width, row_height * 2.0),
        );
        areas.insert(
            UscButton::Laser(Side::Left, Side::Right),
            Rect::new(0.0, row_height * 2.0, col_width, row_height * 4.0),
        );
        areas.insert(
            UscButton::Laser(Side::Right, Side::Left),
            Rect::new(col_width * 5.0, 0.0, col_width * 6.0, row_height * 2.0),
        );
        areas.insert(
            UscButton::Laser(Side::Right, Side::Right),
            Rect::new(
                col_width * 5.0,
                row_height * 2.0,
                col_width * 6.0,
                row_height * 4.0,
            ),
        );
        areas.insert(
            UscButton::Back,
            Rect::new(col_width, 0.0, col_width * 5.0, row_height),
        );
        areas.insert(
            UscButton::Start,
            Rect::new(col_width, row_height, col_width * 4.0, row_height * 2.0),
        );
        for (index, lane) in [BtLane::A, BtLane::B, BtLane::C, BtLane::D]
            .into_iter()
            .enumerate()
        {
            areas.insert(
                UscButton::BT(lane),
                Rect::new(
                    col_width + col_width * index as f64,
                    row_height * 2.0,
                    col_width * 2.0 + col_width * index as f64,
                    row_height * 3.0,
                ),
            );
        }
        areas.insert(
            UscButton::FX(Side::Left),
            Rect::new(
                col_width,
                row_height * 3.0,
                col_width * 3.0,
                row_height * 4.0,
            ),
        );
        areas.insert(
            UscButton::FX(Side::Right),
            Rect::new(
                col_width * 3.0,
                row_height * 3.0,
                col_width * 5.0,
                row_height * 4.0,
            ),
        );

        areas
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_rect_close(actual: Rect, expected: Rect) {
        assert_close(actual.x0, expected.x0);
        assert_close(actual.y0, expected.y0);
        assert_close(actual.x1, expected.x1);
        assert_close(actual.y1, expected.y1);
    }

    fn assert_matches_legacy_oracle(screen_size: Vec2) {
        let actual = MobileLayout::legacy().resolve(screen_size).button_areas;
        let expected = legacy_oracle(screen_size);

        assert_eq!(actual.len(), expected.len());
        for (button, expected_rect) in expected {
            let actual_rect = actual
                .get(&button)
                .unwrap_or_else(|| panic!("missing resolved area for {button:?}"));
            assert_rect_close(*actual_rect, expected_rect);
        }
    }

    #[test]
    fn legacy_has_twelve_unique_button_zones() {
        let layout = MobileLayout::legacy();
        let buttons: HashSet<_> = layout.touch_zones.iter().map(|zone| zone.button).collect();

        assert_eq!(layout.touch_zones.len(), 12);
        assert_eq!(buttons.len(), 12);
    }

    #[test]
    fn legacy_rectangles_are_valid_normalized_geometry() {
        for zone in MobileLayout::legacy().touch_zones {
            let bounds = zone.bounds;
            assert!((0.0..=1.0).contains(&bounds.min_x));
            assert!((0.0..=1.0).contains(&bounds.min_y));
            assert!((0.0..=1.0).contains(&bounds.max_x));
            assert!((0.0..=1.0).contains(&bounds.max_y));
            assert!(bounds.max_x > bounds.min_x);
            assert!(bounds.max_y > bounds.min_y);
        }
    }

    #[test]
    fn legacy_resolves_like_previous_grid_at_600_by_400() {
        assert_matches_legacy_oracle(Vec2::new(600.0, 400.0));
    }

    #[test]
    fn legacy_resolves_like_previous_grid_for_wide_android_viewport() {
        assert_matches_legacy_oracle(Vec2::new(2340.0, 1080.0));
    }

    #[test]
    fn normalized_rect_scales_each_axis_to_physical_coordinates() {
        let actual = NormalizedRect::new(0.1, 0.2, 0.75, 0.9).resolve(Vec2::new(2340.0, 1080.0));

        assert_rect_close(actual, Rect::new(234.0, 216.0, 1755.0, 972.0));
    }
}
