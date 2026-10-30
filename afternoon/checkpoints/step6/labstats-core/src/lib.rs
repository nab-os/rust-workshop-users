use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub sample: String,
    pub value: f64,
}

pub fn parse_line(line: &str) -> Result<Measurement, String> {
    let (sample, value) = line
        .split_once(',')
        .ok_or(format!("expected `sample,value`, got `{line}`"))?;

    let sample = sample.trim();
    if sample.is_empty() {
        return Err("empty sample name".to_string());
    }

    let value: f64 = value
        .trim()
        .parse()
        .map_err(|e| format!("bad value `{}`: {e}", value.trim()))?;

    Ok(Measurement {
        sample: sample.to_string(),
        value,
    })
}

pub fn parse(text: &str) -> (Vec<Measurement>, Vec<String>) {
    let mut measurements = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_line(line) {
            Ok(m) => measurements.push(m),
            Err(e) => errors.push(format!("line {}: {e}", i + 1)),
        }
    }
    (measurements, errors)
}

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub count: usize,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
}

pub fn group_by_sample(measurements: &[Measurement]) -> BTreeMap<&str, Vec<f64>> {
    let mut groups: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for m in measurements {
        groups.entry(m.sample.as_str()).or_default().push(m.value);
    }
    groups
}

pub fn summarize(values: &[f64]) -> Option<Summary> {
    if values.is_empty() {
        return None;
    }
    let count = values.len();
    let mean = values.iter().sum::<f64>() / count as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Some(Summary {
        count,
        mean,
        min,
        max,
    })
}

pub fn report(measurements: &[Measurement]) -> String {
    let mut out = String::from("sample\tcount\tmean\tmin\tmax\n");
    for (sample, values) in group_by_sample(measurements) {
        if let Some(s) = summarize(&values) {
            out.push_str(&format!(
                "{sample}\t{}\t{:.3}\t{:.3}\t{:.3}\n",
                s.count, s.mean, s.min, s.max
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_line() {
        let m = parse_line("A1, 0.412").unwrap();
        assert_eq!(
            m,
            Measurement {
                sample: "A1".to_string(),
                value: 0.412
            }
        );
    }

    #[test]
    fn rejects_bad_lines() {
        assert!(parse_line("A1").is_err());
        assert!(parse_line("A1,abc").is_err());
        assert!(parse_line(",1.0").is_err());
    }

    #[test]
    fn skips_comments_and_reports_errors_with_line_numbers() {
        let text = "# header\nA1,1.0\n\nA1,oops\nB2,2.0\n";
        let (measurements, errors) = parse(text);
        assert_eq!(measurements.len(), 2);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].starts_with("line 4:"));
    }

    #[test]
    fn summarizes_values() {
        let s = summarize(&[1.0, 2.0, 6.0]).unwrap();
        assert_eq!(
            s,
            Summary {
                count: 3,
                mean: 3.0,
                min: 1.0,
                max: 6.0
            }
        );
        assert_eq!(summarize(&[]), None);
    }

    #[test]
    fn groups_are_sorted_by_sample() {
        let (measurements, _) = parse("B2,1.0\nA1,2.0\nB2,3.0");
        let groups = group_by_sample(&measurements);
        let names: Vec<&str> = groups.keys().copied().collect();
        assert_eq!(names, vec!["A1", "B2"]);
        assert_eq!(groups["B2"], vec![1.0, 3.0]);
    }
}
