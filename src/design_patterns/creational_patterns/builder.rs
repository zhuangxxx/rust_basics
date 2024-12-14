pub trait Builder {
    type CarType;

    fn set_brand(self, brand: &str) -> Self;
    fn set_engine(self, engine: &str) -> Self;
    fn set_seats(self, seats: u8) -> Self;
    fn set_gsp_navigator(self, gps_navigator: &str) -> Self;
    fn build(self) -> Self::CarType;
}

pub struct Car {
    pub brand: String,
    pub engine: String,
    pub seats: u8,
    pub gps_navigator: Option<String>,
}

#[derive(Default)]
pub struct CarBuilder {
    brand: Option<String>,
    engine: Option<String>,
    seats: Option<u8>,
    gps_navigator: Option<String>,
}

impl Builder for CarBuilder {
    type CarType = Car;

    fn set_brand(mut self, brand: &str) -> Self {
        self.brand = Some(brand.to_owned());
        self
    }
    fn set_engine(mut self, engine: &str) -> Self {
        self.engine = Some(engine.to_owned());
        self
    }
    fn set_seats(mut self, seats: u8) -> Self {
        self.seats = Some(seats);
        self
    }
    fn set_gsp_navigator(mut self, gps_navigator: &str) -> Self {
        self.gps_navigator = Some(gps_navigator.to_owned());
        self
    }
    fn build(self) -> Self::CarType {
        Car {
            brand: self.brand.expect("请设置车辆品牌"),
            engine: self.engine.expect("请设置车辆引擎"),
            seats: self.seats.expect("请设置座椅数量"),
            gps_navigator: self.gps_navigator,
        }
    }
}

#[test]
fn test_builder() {
    println!("==============");
    let car = CarBuilder::default()
        .set_brand("小米")
        .set_engine("V6s")
        .set_seats(5)
        .set_gsp_navigator("北斗卫星导航系统")
        .build();
    println!("车辆信息");
    println!("车辆品牌：{}", car.brand);
    println!("车辆引擎：{}", car.engine);
    println!("乘员数量：{}", car.seats);
    println!(
        "定位系统：{}",
        if let Some(ref gps_navigator) = car.gps_navigator {
            gps_navigator
        } else {
            "未配备"
        }
    );
    println!("==============");
}
