#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileKind {
    Dirt,

    // Green Grass
    GreenGrass,

    GreenGrassInsideTL,
    GreenGrassInsideTR,
    GreenGrassInsideBL,
    GreenGrassInsideBR,

    GreenGrassSideT,
    GreenGrassSideR,
    GreenGrassSideB,
    GreenGrassSideL,

    GreenGrassCornerTL,
    GreenGrassCornerTR,
    GreenGrassCornerBL,
    GreenGrassCornerBR,

    // Yellow Grass
    YellowGrass,

    YellowGrassInsideTL,
    YellowGrassInsideTR,
    YellowGrassInsideBL,
    YellowGrassInsideBR,

    YellowGrassSideT,
    YellowGrassSideR,
    YellowGrassSideB,
    YellowGrassSideL,

    YellowGrassCornerTL,
    YellowGrassCornerTR,
    YellowGrassCornerBL,
    YellowGrassCornerBR,

    // Water
    Water,

    WaterInsideTL,
    WaterInsideTR,
    WaterInsideBL,
    WaterInsideBR,

    WaterSideT,
    WaterSideR,
    WaterSideB,
    WaterSideL,

    WaterCornerTL,
    WaterCornerTR,
    WaterCornerBL,
    WaterCornerBR,

    // Props
    BigTreeTL,
    BigTreeTR,
    BigTreeBL,
    BigTreeBR,

    SmallTree,

    Plant1,
    Plant2,
    Plant3,

    TreeStump1,
    TreeStump2,
    TreeStump3,

    Rock1,
    Rock2,
    Rock3,
    Rock4,
}

impl TileKind {
    pub fn sprite_index(self) -> usize {
        match self {
            TileKind::Dirt => 4,

            // Green Grass
            TileKind::GreenGrass => 5,

            TileKind::GreenGrassInsideTL => 6,
            TileKind::GreenGrassInsideTR => 7,
            TileKind::GreenGrassInsideBL => 14,
            TileKind::GreenGrassInsideBR => 15,

            TileKind::GreenGrassSideT => 18,
            TileKind::GreenGrassSideR => 19,
            TileKind::GreenGrassSideB => 27,
            TileKind::GreenGrassSideL => 26,

            TileKind::GreenGrassCornerTL => 16,
            TileKind::GreenGrassCornerTR => 17,
            TileKind::GreenGrassCornerBL => 24,
            TileKind::GreenGrassCornerBR => 25,

            // Yellow Grass
            TileKind::YellowGrass => 64,

            TileKind::YellowGrassInsideTL => 65,
            TileKind::YellowGrassInsideTR => 66,
            TileKind::YellowGrassInsideBL => 73,
            TileKind::YellowGrassInsideBR => 74,

            TileKind::YellowGrassSideT => 69,
            TileKind::YellowGrassSideR => 70,
            TileKind::YellowGrassSideB => 78,
            TileKind::YellowGrassSideL => 77,

            TileKind::YellowGrassCornerTL => 67,
            TileKind::YellowGrassCornerTR => 68,
            TileKind::YellowGrassCornerBL => 75,
            TileKind::YellowGrassCornerBR => 76,

            // Water
            TileKind::Water => 49,

            TileKind::WaterInsideTL => 50,
            TileKind::WaterInsideTR => 51,
            TileKind::WaterInsideBL => 58,
            TileKind::WaterInsideBR => 59,

            TileKind::WaterSideT => 54,
            TileKind::WaterSideR => 55,
            TileKind::WaterSideB => 63,
            TileKind::WaterSideL => 62,

            TileKind::WaterCornerTL => 52,
            TileKind::WaterCornerTR => 53,
            TileKind::WaterCornerBL => 60,
            TileKind::WaterCornerBR => 61,

            // Props
            TileKind::BigTreeTL => 0,
            TileKind::BigTreeTR => 1,
            TileKind::BigTreeBL => 8,
            TileKind::BigTreeBR => 9,

            TileKind::SmallTree => 23,

            TileKind::Plant1 => 20,
            TileKind::Plant2 => 21,
            TileKind::Plant3 => 22,

            TileKind::TreeStump1 => 38,
            TileKind::TreeStump2 => 39,
            TileKind::TreeStump3 => 48,

            TileKind::Rock1 => 32,
            TileKind::Rock2 => 33,
            TileKind::Rock3 => 34,
            TileKind::Rock4 => 35,
        }
    }
}