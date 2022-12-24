use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

impl Value {
    fn compare_lists(lhs: &[Value], rhs: &[Value]) -> Ordering {
        let cnt = lhs.len().min(rhs.len());
        for i in 0..cnt {
            let cmp = lhs[i].compare(&rhs[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        return lhs.len().cmp(&rhs.len());
    }

    fn compare(&self, other: &Value) -> Ordering {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => i.cmp(j),
                Self::List(l) => Self::compare_lists(&[Self::Integer(*i)], &l),
            },
            Self::List(l) => match other {
                Self::Integer(j) => Self::compare_lists(l, &[Self::Integer(*j)]),
                Self::List(j) => Self::compare_lists(l, j),
            },
        }
    }

    fn parse_str(s: &str) -> Self {
        let chars: Vec<char> = s.trim().chars().collect();
        Self::parse(&chars).0
    }

    fn parse(s: &[char]) -> (Self, usize) {
        if s[0] == '[' {
            let mut ofs = 1;
            let mut list = Vec::new();
            while ofs < s.len() && s[ofs] != ']' {
                if s[ofs] == ',' {
                    ofs += 1;
                }
                if ofs < s.len() {
                    let ret = Self::parse(&s[ofs..]);
                    list.push(ret.0);
                    ofs += ret.1;
                }
            }
            (Self::List(list), ofs + 1)
        } else {
            let ofs = s
                .iter()
                .position(|c| *c == ']' || *c == ',')
                .unwrap_or(s.len());
            let s: String = s[0..ofs].iter().collect();
            (Self::Integer(s.parse().unwrap()), ofs)
        }
    }

    fn to_string(&self) -> String {
        match self {
            Value::Integer(v) => v.to_string(),
            Value::List(l) => {
                let mut s = String::from("[");
                for i in 0..l.len() {
                    if i > 0 {
                        s += ",";
                    }
                    s += &l[i].to_string();
                }
                s += "]";
                s
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

pub fn solve() {
    let data = include_str!("inputs/13.txt");
    let mut values: Vec<Value> = data
        .split('\n')
        .filter_map(|s| {
            let chars: Vec<char> = s.trim().chars().collect();
            if chars.is_empty() {
                None
            } else {
                Some(Value::parse(&chars).0)
            }
        })
        .collect();

    let mut task1 = 0;
    for i in 0..values.len() / 2 {
        if values[2 * i] < values[2 * i + 1] {
            task1 += i + 1;
        }
    }

    let div1 = Value::parse_str("[[2]]");
    let div2 = Value::parse_str("[[6]]");
    values.push(div1.clone());
    values.push(div2.clone());
    values.sort();

    let idx1 = values.iter().position(|v| *v == div1).unwrap();
    let idx2 = values.iter().position(|v| *v == div2).unwrap();

    println!("[day 13] task 1: {}", task1);
    println!("[day 13] task 2: {}", (idx1 + 1) * (idx2 + 1));
}
