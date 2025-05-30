use crate::quizzes::ClockReadingQuestion; // Assuming this path is correct
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AnalogClockProps {
    pub question: ClockReadingQuestion,
    pub size: Option<u32>, // Optional size in pixels
}

#[function_component(AnalogClock)]
pub fn analog_clock(props: &AnalogClockProps) -> Html {
    let size = props.size.unwrap_or(200); // Default size if None
    let center = size / 2;
    // Make radius a bit smaller than center to give padding, proportional to size
    let radius = (size as f64 * 0.9 / 2.0) as u32; // e.g., 90 for size 200

    // Calculate hand positions
    let hour_angle_rad = (props.question.hour_angle() - 90.0).to_radians();
    let minute_angle_rad = (props.question.minute_angle() - 90.0).to_radians();

    // Hand lengths proportional to radius
    let hour_hand_length = (radius as f64) * 0.55;
    let minute_hand_length = (radius as f64) * 0.75;

    // Calculate hand end points
    let hour_x = center as f64 + hour_hand_length * hour_angle_rad.cos();
    let hour_y = center as f64 + hour_hand_length * hour_angle_rad.sin();

    let minute_x = center as f64 + minute_hand_length * minute_angle_rad.cos();
    let minute_y = center as f64 + minute_hand_length * minute_angle_rad.sin();

    // Determine time period text and class
    let time_period_text = if props.question.is_afternoon() {
        "Eftermiddag"
    } else {
        "Förmiddag"
    };
    let period_indicator_class = if props.question.is_afternoon() {
        classes!("period-indicator", "afternoon")
    } else {
        classes!("period-indicator", "morning")
    };

    let svg_style = format!("--clock-viewbox-size: {};", size);

    html! {
        <div class="analog-clock-container">
            // SVG is now first
            <svg
                width={size.to_string()}
                height={size.to_string()}
                view_box={format!("0 0 {} {}", size, size)}
                class="analog-clock-svg"
                style={svg_style}
            >
                // Clock face
                <circle
                    class="clock-face"
                    cx={center.to_string()}
                    cy={center.to_string()}
                    r={radius.to_string()}
                />

                // Hour markers (thicker)
                {
                    (0..12).map(|i| {
                        let angle_rad = ((i as f64) * 30.0 - 90.0).to_radians();
                        let marker_length = radius as f64 * 0.15;
                        let outer_r = radius as f64;
                        let inner_r = outer_r - marker_length;

                        let x1 = center as f64 + inner_r * angle_rad.cos();
                        let y1 = center as f64 + inner_r * angle_rad.sin();
                        let x2 = center as f64 + outer_r * angle_rad.cos();
                        let y2 = center as f64 + outer_r * angle_rad.sin();

                        html! {
                            <line
                                class="hour-marker"
                                x1={x1.to_string()} y1={y1.to_string()}
                                x2={x2.to_string()} y2={y2.to_string()}
                                key={format!("h-marker-{}",i)}
                            />
                        }
                    }).collect::<Html>()
                }

                // Minute markers (thinner)
                {
                    (0..60).filter(|i| i % 5 != 0).map(|i| {
                        let angle_rad = ((i as f64) * 6.0 - 90.0).to_radians();
                        let marker_length = radius as f64 * 0.08;
                        let outer_r = radius as f64;
                        let inner_r = outer_r - marker_length;

                        let x1 = center as f64 + inner_r * angle_rad.cos();
                        let y1 = center as f64 + inner_r * angle_rad.sin();
                        let x2 = center as f64 + outer_r * angle_rad.cos();
                        let y2 = center as f64 + outer_r * angle_rad.sin();

                        html! {
                            <line
                                class="minute-marker"
                                x1={x1.to_string()} y1={y1.to_string()}
                                x2={x2.to_string()} y2={y2.to_string()}
                                key={format!("m-marker-{}",i)}
                            />
                        }
                    }).collect::<Html>()
                }

                // Hour numbers
                {
                    (1..=12).map(|i| {
                        let angle_rad = ((i as f64) * 30.0 - 90.0).to_radians();
                        let text_r = radius as f64 * 0.75;

                        let x = center as f64 + text_r * angle_rad.cos();
                        let y = center as f64 + text_r * angle_rad.sin();

                        html! {
                            <text
                                class="hour-number"
                                x={x.to_string()}
                                y={y.to_string()}
                                key={format!("h-num-{}",i)}
                            >
                                {i.to_string()}
                            </text>
                        }
                    }).collect::<Html>()
                }

                // Hour hand
                <line class="hour-hand"
                    x1={center.to_string()} y1={center.to_string()}
                    x2={hour_x.to_string()} y2={hour_y.to_string()}
                />

                // Minute hand
                <line class="minute-hand"
                    x1={center.to_string()} y1={center.to_string()}
                    x2={minute_x.to_string()} y2={minute_y.to_string()}
                />

                // Center dot
                <circle class="center-dot"
                    cx={center.to_string()}
                    cy={center.to_string()}
                    // r is set in SCSS relative to --clock-viewbox-size
                />
            </svg>

            // Period indicator div is now second
            <div class={period_indicator_class}>
                { time_period_text }
            </div>
        </div>
    }
}
