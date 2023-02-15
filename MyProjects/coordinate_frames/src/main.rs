

fn main() {
    println!("Hello, world!");

    let pos: ECEFCoords = ECEFCoords {
                                    x: 1199816.63380,
                                    y: -4812201.6785,
                                    z: 4016140.62425
                                };

    let pos_a = pos.to_geodetic();

    println!("Geodetic Latitude: {}", pos_a.lat.to_degrees());
    println!("Geodetic Longitude: {}", pos_a.long.to_degrees());
    println!("Geodetic Altitude: {}", pos_a.ht);
}

struct ECEFCoords {
    x: f64, // x position
    y: f64, // y position
    z: f64, //  z position
}

struct GeodeticCoords {
    lat: f64, // latitude
    long: f64, //longitude
    ht: f64, //height
}

impl ECEFCoords {

    fn to_geodetic(&self) -> GeodeticCoords {

        let mut _lat: f64;
        let mut _long: f64;
        let mut _ht: f64;

        let a: f64 = 6378137.0; /*wgs-84*/
        let e2: f64 = 6.6943799901377997e-3;
        let a1: f64 = 4.2697672707157535e+4;
        let a2: f64 = 1.8230912546075455e+9;
        let a3: f64 = 1.4291722289812413e+2;
        let a4: f64 = 4.5577281365188637e+9;
        let a5: f64 = 4.2840589930055659e+4;
        let a6: f64 = 9.9330562000986220e-1;

        let (mut zp, mut w2, mut w, mut z2, mut r2,
            mut r, mut s2, mut c2, mut s, mut c,
            mut ss): (f64,f64,f64,f64,f64,f64,f64,
                      f64,f64,f64,f64);
        
        let (mut g, mut rg, mut rf, mut u, mut v,
            mut m, mut f, mut p): (f64,f64,f64,f64,
                                   f64,f64,f64,f64);

        zp = self.z.abs();
        w2 = self.x * self.x + self.y * self.y;
        w = w2.sqrt();
        z2 = self.z * self.z;
        r2 = w2 + z2;
        r = r2.sqrt();

        if r < 100000_f64 {
            return GeodeticCoords {
                   lat: (0_f64),
                   long: (0_f64),
                    ht: (-1e+7_f64)
                }
        }

        _long = self.y.atan2(self.x);
        s2 = z2 / r2;
        c2 = w2 / r2;
        u = a2 / r;
        v = a3 - a4/r;

        if c2 > 0.3_f64 {
            s = (zp/r)*(1. + c2 * (a1 + u + s2*v)/r); 
            _lat= s.asin();
            ss = s * s;
            c = (1_f64 - ss).sqrt();
        } else {
            c = (w/r) * (1_f64 - s2 * (a5 - u - c2*v)/r);
            _lat = c.acos();
            ss = 1_f64 - c * c;
            s = ss.sqrt();
        }

        g = 1_f64 - e2*ss;
        rg = a/g.sqrt();
        rf = a6*rg;
        u = w - rg*c;
        v = zp - rf*s;
        f = c*u + s*v;
        m = c*v - s*u;
        p = m/(rf/g + f);
        _lat = _lat + p;
        _ht = f + m*p/2_f64;
        
        if self.z <0_f64 {
            _lat = - _lat;
        }

        GeodeticCoords { lat: (_lat),
                          long: (_long),
                          ht: (_ht) }
    }
}



