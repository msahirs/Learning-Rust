use std::time::{Duration, Instant};

fn main() {
    // println!("Hello, world!");

    let pos: ECEFCoords = ECEFCoords {
                                    x: 1199816.63380,
                                    y: -4812201.6785,
                                    z: 4016140.62425
                                };
    let now = Instant::now();                               
    let pos_a = pos.to_geodetic();
    let duration = now.elapsed();
    // println!("Geodetic Latitude: {}", pos_a.lat.to_degrees());
    // println!("Geodetic Longitude: {}", pos_a.long.to_degrees());
    // println!("Geodetic Altitude: {}", pos_a.ht);
    println!("{:?}", duration);
}

struct ECEFCoords {
    x: f32, // x position
    y: f32, // y position
    z: f32, //  z position
}

struct GeodeticCoords {
    lat: f32, // latitude
    long: f32, //longitude
    ht: f32, //height
}

impl ECEFCoords {

    fn to_geodetic(&self) -> GeodeticCoords {

        let mut _lat: f32;
        let mut _long: f32;
        let mut _ht: f32;

        let a: f32 = 6378137.0; /*wgs-84*/
        let e2: f32 = 6.6943799901377997e-3;
        let a1: f32 = 4.2697672707157535e+4;
        let a2: f32 = 1.8230912546075455e+9;
        let a3: f32 = 1.4291722289812413e+2;
        let a4: f32 = 4.5577281365188637e+9;
        let a5: f32 = 4.2840589930055659e+4;
        let a6: f32 = 9.9330562000986220e-1;

        let (zp, w2, w, z2, r2,
            r, s2, c2, s, c,
             ss): (f32,f32,f32,f32,f32,f32,f32,
                      f32,f32,f32,f32);
        
        let (mut g, mut rg, mut rf, mut u, mut v,
            mut m, mut f, mut p): (f32,f32,f32,f32,
                                   f32,f32,f32,f32);

        zp = self.z.abs();
        w2 = self.x * self.x + self.y * self.y;
        w = w2.sqrt();
        z2 = self.z * self.z;
        r2 = w2 + z2;
        r = r2.sqrt();

        if r < 100000_f32 {
            return GeodeticCoords {
                   lat: (0_f32),
                   long: (0_f32),
                    ht: (-1e+7_f32)
                }
        }

        _long = self.y.atan2(self.x);
        s2 = z2 / r2;
        c2 = w2 / r2;
        u = a2 / r;
        v = a3 - a4/r;

        if c2 > 0.3_f32 {
            s = (zp/r)*(1. + c2 * (a1 + u + s2*v)/r); 
            _lat= s.asin();
            ss = s * s;
            c = (1_f32 - ss).sqrt();
        } else {
            c = (w/r) * (1_f32 - s2 * (a5 - u - c2*v)/r);
            _lat = c.acos();
            ss = 1_f32 - c * c;
            s = ss.sqrt();
        }

        g = 1_f32 - e2*ss;
        rg = a/g.sqrt();
        rf = a6*rg;
        u = w - rg*c;
        v = zp - rf*s;
        f = c*u + s*v;
        m = c*v - s*u;
        p = m/(rf/g + f);
        _lat = _lat + p;
        _ht = f + m*p/2_f32;
        
        if self.z < 0_f32 {
            _lat = - _lat;
        }

        GeodeticCoords { lat: (_lat),
                          long: (_long),
                          ht: (_ht) }
    }
}



