use api_request_utils_rs::serde::{Deserialize,Serialize};

#[derive(Debug)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
#[derive(Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Market {
    id : u16,
    name: &'static str,
}

impl Market {
    const fn new(id: u16, name: &'static str) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }
}


lazy_static::lazy_static! {
    
    pub static ref MARKETS: std::collections::HashMap<u16,Market> = std::collections::HashMap::from([
        (1,Market::new(1,"Birmingham & More")),
        (2,Market::new(2,"Charlotte")),
        (3,Market::new(3,"Chicagoland & Northern IL")),
        (4,Market::new(4,"Cincinnati & Dayton")),
        (5,Market::new(5,"Dallas - Fort Worth & More")),
        (6,Market::new(6,"Denver & More")),
        (7,Market::new(7,"Detroit, Toledo & More")),
        (8,Market::new(8,"El Paso & New Mexico")),
        (9,Market::new(9,"Grand Rapids & More")),
        (10,Market::new(10,"Greater Atlanta Area")),
        (11,Market::new(11,"Greater Boston Area")),
        (12,Market::new(12,"Cleveland, Youngstown & More")),
        (13,Market::new(13,"Greater Columbus Area")),
        (14,Market::new(14,"Greater Las Vegas Area")),
        (15,Market::new(15,"Greater Miami Area")),
        (16,Market::new(16,"Minneapolis/St. Paul & More")),
        (17,Market::new(17,"Greater Orlando Area")),
        (18,Market::new(18,"Greater Philadelphia Area")),
        (19,Market::new(19,"Greater Pittsburgh Area")),
        (20,Market::new(20,"Greater San Diego Area")),
        (21,Market::new(21,"Greater Tampa Area")),
        (22,Market::new(22,"Houston & More")),
        (23,Market::new(23,"Indianapolis & More")),
        (24,Market::new(24,"Iowa")),
        (25,Market::new(25,"Jacksonville & More")),
        (26,Market::new(26,"Kansas City & More")),
        (27,Market::new(27,"Greater Los Angeles Area")),
        (28,Market::new(28,"Louisville & Lexington")),
        (29,Market::new(29,"Memphis, Little Rock & More")),
        (30,Market::new(30,"Milwaukee & WI")),
        (31,Market::new(31,"Nashville, Knoxville & More")),
        (33,Market::new(33,"New England")),
        (34,Market::new(34,"New Orleans & More")),
        (35,Market::new(35,"New York/Tri-State Area")),
        (36,Market::new(36,"Phoenix & Tucson")),
        (37,Market::new(37,"Portland & More")),
        (38,Market::new(38,"Raleigh & Durham")),
        (39,Market::new(39,"Saint Louis & More")),
        (40,Market::new(40,"San Antonio & Austin")),
        (41,Market::new(41,"N. California/N. Nevada")),
        (42,Market::new(42,"Greater Seattle Area")),
        (43,Market::new(43,"North & South Dakota")),
        (44,Market::new(44,"Upstate New York")),
        (45,Market::new(45,"Utah & Montana")),
        (46,Market::new(46,"Virginia")),
        (47,Market::new(47,"Washington, DC and Maryland")),
        (48,Market::new(48,"West Virginia")),
        (49,Market::new(49,"Hawaii")),
        (50,Market::new(50,"Alaska")),
        (52,Market::new(52,"Nebraska")),
        (53,Market::new(53,"Springfield")),
        (54,Market::new(54,"Central Illinois")),
        (55,Market::new(55,"Northern New Jersey")),
        (121,Market::new(121,"South Carolina")),
        (122,Market::new(122,"South Texas")),
        (123,Market::new(123,"Beaumont")),
        (124,Market::new(124,"Connecticut")),
        (125,Market::new(125,"Oklahoma")),
        (102,Market::new(102,"Toronto, Hamilton & Area")),
        (103,Market::new(103,"Ottawa & Eastern Ontario")),
        (106,Market::new(106,"Manitoba")),
        (107,Market::new(107,"Edmonton & Northern Alberta")),
        (108,Market::new(108,"Calgary & Southern Alberta")),
        (110,Market::new(110,"B.C. Interior")),
        (111,Market::new(111,"Vancouver & Area")),
        (112,Market::new(112,"Saskatchewan")),
        (120,Market::new(120,"Montréal & Area")),
        (202,Market::new(202,"London (UK)")),
        (203,Market::new(203,"South (UK)")),
        (204,Market::new(204,"Midlands and Central (UK)")),
        (205,Market::new(205,"Wales and North West (UK)")),
        (206,Market::new(206,"North and North East (UK)")),
        (207,Market::new(207,"Scotland")),
        (208,Market::new(208,"Ireland")),
        (209,Market::new(209,"Northern Ireland")),
        (210,Market::new(210,"Germany")),
        (211,Market::new(211,"Netherlands")),
        (500,Market::new(500,"Sweden")),
        (501,Market::new(501,"Spain")),
        (502,Market::new(502,"Barcelona (Spain)")),
        (503,Market::new(503,"Madrid (Spain)")),
        (600,Market::new(600,"Turkey")),
        (302,Market::new(302,"New South Wales/Australian Capital Territory")),
        (303,Market::new(303,"Queensland")),
        (304,Market::new(304,"Western Australi")),
        (305,Market::new(305,"Victoria/Tasmania")),
        (306,Market::new(306,"Western Australia")),
        (351,Market::new(351,"North Island")),
        (352,Market::new(352,"South Island")),
        (402,Market::new(402,"Mexico City and Metropolitan Area")),
        (403,Market::new(403,"Monterrey")),
        (404,Market::new(404,"Guadalajara")),
    ]);

}