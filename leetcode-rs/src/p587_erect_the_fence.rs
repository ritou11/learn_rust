// P587. Erect the Fence
/**
 * You are given an array trees where trees[i] = [xi, yi] represents the location of a tree in the garden.
 * You are asked to fence the entire garden using the minimum length of rope as it is expensive.
 * The garden is well fenced only if all the trees are enclosed.
 * Return the coordinates of trees that are exactly located on the fence perimeter.
 */
use crate::Solution;
use std::cmp;

impl Solution {
    fn angle(x: &Vec<i32>, p0: &Vec<i32>) -> f64 {
        let x0 = (x[0] - p0[0]) as f64;
        let x1 = (x[1] - p0[1]) as f64;
        let a1 = (x1.atan2(x0) + (2.0 * std::f64::consts::PI)) % (2.0 * std::f64::consts::PI);
        a1
    }
    fn cmp_angle(x: &Vec<i32>, y: &Vec<i32>, p0: &Vec<i32>) -> cmp::Ordering {
        let x0 = (x[0] - p0[0]) as f64;
        let y0 = (y[0] - p0[0]) as f64;
        let a1 = Self::angle(x, p0);
        let a2 = Self::angle(y, p0);
        // println!("{:?}, {}, {}, {:?}, {}, {}, {}, {}", x, x0, x1, y, y0, y1, a1, a2);
        if a1 != a2 {
            return a1.partial_cmp(&a2).unwrap();
        }
        return x0.partial_cmp(&y0).unwrap();
    }
    fn cross_mul(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
        return (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]);
    }
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() < 2 {
            return trees;
        }
        trees.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut points: Vec<Vec<i32>> = Vec::new();
        points.push(trees[0].clone());
        // println!("{:?}", points);

        let a0 = Self::angle(&trees[1], &trees[0]);
        let mut linear = true;
        for i in 2..trees.len() {
            if Self::angle(&trees[i], &trees[0]) != a0 {
                linear = false;
                break;
            }
        }
        if linear {
            return trees;
        }

        trees.sort_by(|a, b| Self::cmp_angle(&a, &b, &points[0]));
        points.push(trees[1].clone());
        // println!("{:?}", trees);
        for i in 2..trees.len() {
            let mut n = points.len();
            while n > 1 && Self::cross_mul(&points[n - 2], &points[n - 1], &trees[i]) < 0 {
                // println!("{:?}, {:?}, {:?}, {}", &points[n-2], &points[n-1], &trees[i], Self::cross_mul(&points[n-2], &points[n-1], &trees[i]));
                points.pop();
                n -= 1;
            }
            points.push(trees[i].clone());
        }
        let k = points.len();
        for i in (0..trees.len() - 1).rev() {
            let mut n = points.len();
            while n > k && Self::cross_mul(&points[n - 2], &points[n - 1], &trees[i]) < 0 {
                // println!("{:?}, {:?}, {:?}, {}", &points[n-2], &points[n-1], &trees[i], Self::cross_mul(&points[n-2], &points[n-1], &trees[i]));
                points.pop();
                n -= 1;
            }
            points.push(trees[i].clone());
        }
        if points.len() > 1 {
            points.pop();
        }
        points
    }
}
