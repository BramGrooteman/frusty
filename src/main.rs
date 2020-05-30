use std::fmt;
// trait OnlineSearch {
//     fn query(&self, corpus: String, query: String) -> Vec<String>;
// }

// trait OfflineSearch {
//     fn new(corpus: String) -> Self;
//     fn query(&self, query: String) -> Vec<String>;
// }

// struct BruteForce {
//     results: Vec<String>
// }

// impl OnlineSearch for BruteForce {
//     fn query(&self, corpus: String, query: String) -> Vec<String> {
//         vec![String::from("ABC")]
//     }
// }

struct QMatch {
    pos: usize,
    len: usize,
    score: usize,
}

impl fmt::Display for QMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.pos, self.len, self.score)
    }
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn stupid_levenstein_distance(s: &str, t: &str) -> usize {
    if s.is_empty() {
        return t.len();
    }
    if t.is_empty() {
        return s.len();
    }
    let (first_s, rem_s) = car_cdr(s);
    let (first_t, rem_t) = car_cdr(t);
    if first_s == first_t {
        return stupid_levenstein_distance(rem_s, rem_t);
    } else {
        let opt_1 = stupid_levenstein_distance(s, rem_t);
        let opt_2 = stupid_levenstein_distance(rem_s, t);
        let opt_3 = stupid_levenstein_distance(rem_s, rem_t);
        let values = vec![opt_1, opt_2, opt_3];
        return 1 + match values.iter().min() {
            Some(min) => *min,
            None => return 0,
        };
    }
}
fn compute_stupid(corpus: &str, query: &str) -> Vec<QMatch> {
    let mut min_score = corpus.len();
    let mut start_idx: usize;

    let mut results: Vec<QMatch> = vec![];
    for c_idx in 1..=corpus.len() {
        start_idx = 0;
        if query.len() < c_idx {
            start_idx = c_idx - query.len();
        }
        let t_j = &corpus[start_idx..c_idx];
        let score = stupid_levenstein_distance(t_j, &query);
        if score == min_score {
            results.push(QMatch {
                pos: start_idx,
                len: c_idx - start_idx,
                score: score,
            });
        } else if score < min_score {
            min_score = score;
            results = vec![QMatch {
                pos: start_idx,
                len: c_idx - start_idx,
                score: score,
            }];
        }
    }
    results
}

// Now use the matrix method
fn tmin(a: usize, b: usize, c: usize) -> usize {
    if a <= b && a <= c {
        return a;
    } else if b <= a && b <= c {
        return b;
    } else {
        return c;
    }
}

// Returns the path from solution back to start
fn find_path(grid:Vec<Vec<usize>>, idx: usize, score:usize) -> usize {

    0
}

fn find_winner(grid: Vec<Vec<usize>>) -> Vec<QMatch> {
    let mut min_idx = 0;
    let mut min_score = grid.len();
    // Find minimal score(s)
    for i in 1..grid.len() {
        let cur_score = grid[i][grid[i].len() - 1];
        if cur_score < min_score {
            min_idx = i;
            min_score = cur_score;
        }
    }
    // Find path
    find_path(grid, min_idx, min_score)
    println!("score: {} - pos: {}", min_score, min_idx);
    vec![]
}

fn compute_smarter(corpus: &str, query: &str) -> Vec<QMatch> {
    let mut grid = vec![vec![0; query.len() + 1]; corpus.len() + 1]; //grid[corpus_idx][pattern_idx]

    for i in 1..corpus.len() + 1 {
        grid[i][0] = 0
    }
    for j in 1..query.len() + 1 {
        grid[0][j] = j
    }
    for (i, c_sym) in corpus.char_indices() {
        for (j, q_sym) in query.char_indices() {
            let sub_cost;
            if c_sym == q_sym {
                sub_cost = 0;
            } else {
                sub_cost = 1;
            }
            let x = i + 1;
            let y = j + 1;
            grid[x][y] = tmin(
                grid[x - 1][y] + 1,
                grid[x][y - 1] + 1,
                grid[x - 1][y - 1] + sub_cost,
            )
        }
    }
    find_winner(grid)
}

fn main() {
    // Start of with naive approach
    let corpus = String::from("I was looking at a house in Zwolle");
    let query = String::from("look");

    let res = compute_stupid(&corpus, &query);
    for r in res.iter() {
        println!("{} - {}", &corpus[r.pos..r.pos + r.len], r.score)
    }
    compute_smarter(&corpus, &query);
}
