use std::ops::{Add, Mul, Neg};
use image::save_buffer;



#[derive(Debug, Clone, Copy)]
struct F<const P: usize> {
    x: usize
}

impl<const P: usize> PartialEq for F<P> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl<const P: usize> F<P> {
    fn new(x: isize) -> F<P> {
        let mut y = x % P as isize;
        if y < 0 {
            y += P as isize;
        }
        F {x: y as usize}
    }

    fn inv(&self) -> F<P> {

        let mut r = self.x as isize;
        let mut u = 1;
        let mut r2 = P as isize;
        let mut u2 = 0;

        let mut q;

        let mut rs;
        let mut us;

        while r2 != 0 {
            q = r / r2;
            
            rs = r2;
            us = u2;

            r2 = r - q * r2;
            u2 = u - q * u2;

            u = us;
            r = rs;

        }

        assert_eq!(r, 1);
        F::new(u as isize)
    }
}

impl<const P: usize> Add for F<P> {
    type Output = F<P>;

    fn add(self, rhs: Self) -> Self::Output {
        F {x: (self.x + rhs.x) % P}
    }
}


impl<const P: usize> Mul for F<P> {
    type Output = F<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        F {x: (self.x * rhs.x) % P}
    }
}

impl<const P: usize> Neg for F<P> {
    type Output = F<P>;

    fn neg(self) -> Self::Output {
        F {x: (P - self.x) % P}
    }
}



struct EllipticCurve<const P: usize> {

    a: F<P>,
    b: F<P>
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum Point<const P: usize> {
    Infini,
    Fini(F<P>, F<P>)


}


impl<const P: usize> EllipticCurve<P> {


    fn discriminant(&self) -> F<P> {
        let a = self.a;
        let b = self.b;
        -F::new(16) * (F::new(4)*a*a*a + F::new(27)*b*b)
    }

    fn contient(&self, point: Point<P>) -> bool {
        match point {
            Point::Infini => true,
            Point::Fini(x, y) => y*y == x*x*x + self.a*x + self.b
        }
    }

    fn add(&self, p1: Point<P>, p2: Point<P>) -> Point<P> {
        match (p1, p2) {
            (Point::Infini, _) => p2,
            (_, Point::Infini) => p1,
            (Point::Fini(xp, yp), Point::Fini(xq, yq)) => {

                if xp != xq {
                    let s = (yp + -yq) * (xp + -xq).inv();
                    let t = (yq*xp + -yp*xq) * (xp + -xq).inv();

                    Point::Fini(s*s + -xp + -xq, -s*(s*s + -xp + -xq) + -t)


                } else if yp != yq {
                    assert_eq!(yp, -yq);
                    Point::Infini

                } else if yp != F::new(0) {
                    let s = (F::new(3)*xp*xp + self.a) * (yp + yp).inv();
                    // let t = yp + -xp*(F::new(3)*xp*xp + self.a) * (yp + yp).inv();

                    Point::Fini(s*s + -(xp + xp), -yp + s*(xp + -s*s + xp + xp))

                } else {
                    Point::Infini
                }
                

            }


        }
    }

    fn mul(&self, p: Point<P>, i: usize) -> Point<P> {

        if i == 0 {
            Point::Infini
        }
        else if i % 2 == 0 {
            let m2 = self.mul(p, i / 2);
            self.add(m2, m2)
        } else {
            let m2 = self.mul(p, i / 2);
            self.add(self.add(m2, m2), p)
        }


    }

    fn get_point(&self, x: F<P>) -> Point<P> {

        for y in 0..P {
            if self.contient(Point::Fini(x, F::new(y as isize))) {return Point::Fini(x, F::new(y as isize));}
        }

        Point::Infini

    }


}

















const P: usize = 757;

fn main() {
    let curve: EllipticCurve<P> = EllipticCurve {a: F::new(70), b: F::new(17)};
    let point = curve.get_point(F::new(0));

    println!("{:?}", point);

    for i in 2..500 {
        println!("{:?}", curve.mul(point, i));
        if curve.mul(point, i) == Point::Infini {break;}
    }




    let mut buffer: Vec<u8> = Vec::new();
    for x in 0..P {
        for y in 0..P {
            let point = Point::Fini(F{x}, F{x:y});
            if curve.contient(point) {
                buffer.push(255);
                buffer.push(240);
                buffer.push(240);
            } else {
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
            }
        }
    }

    save_buffer("map.png", &buffer, P as u32, P as u32, image::ColorType::Rgb8).expect("welp");

}
