struct Point<T> {
    x: T,
    y: T,
}

// 为泛型T提供方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为指定类型f32提供方法, 自动获得了 impl<T> Point<T> 的方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod generics_one_t_test {
    use super::*;

    #[test]
    fn test_generics_logic() {
        let p = Point { x: 2, y: 4 };
        println!("泛型p.x():{}", p.x());
        println!("泛型p.x:{}", p.x);
    }

    #[test]
    fn test_generics_the() {
        let p = Point { x: 2.2, y: 4.0 };
        println!("f32:{}", p.x());
        println!("f32:{}", p.x);
        println!("distance_from_origin:{}", p.distance_from_origin());
    }
}
// =====================================================

struct PointMutil<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMutil<T, U> {
    fn mix<V, W>(self, other: PointMutil<V, W>) -> PointMutil<T, V> {
        PointMutil {
            x: self.x,
            y: other.x,
        }
    }
}

#[cfg(test)]
mod generics_mutil_t_test {
    use super::*;

    #[test]
    fn test_mix() {
        let p1 = PointMutil { x: 1, y: 2 };
        let p2 = PointMutil { x: "xxx", y: 'y' };

        let p3 = p1.mix(p2);

        // 非词法作用域生命周期
        // println!("p1.x:{}", p1.x); // p1 的所有权丢失
        // println!("p2.x:{}", p2.x); // p2 的所有权丢失
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
