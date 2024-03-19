#[doc = "Register `LANE_MAP` reader"]
pub type R = crate::R<LaneMapSpec>;
#[doc = "Register `LANE_MAP` writer"]
pub type W = crate::W<LaneMapSpec>;
#[doc = "Control physical lane 0 will map to which logic lane:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lane0Map {
    #[doc = "3: Logic lane 3,"]
    B11 = 3,
    #[doc = "2: Logic lane 2,"]
    B10 = 2,
    #[doc = "1: Logic lane 1,"]
    B01 = 1,
    #[doc = "0: Logic lane 0,"]
    B00 = 0,
}
impl From<Lane0Map> for u8 {
    #[inline(always)]
    fn from(variant: Lane0Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lane0Map {
    type Ux = u8;
}
#[doc = "Field `LANE0_MAP` reader - Control physical lane 0 will map to which logic lane:"]
pub type Lane0MapR = crate::FieldReader<Lane0Map>;
impl Lane0MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane0Map {
        match self.bits {
            3 => Lane0Map::B11,
            2 => Lane0Map::B10,
            1 => Lane0Map::B01,
            0 => Lane0Map::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lane0Map::B11
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lane0Map::B10
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lane0Map::B01
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lane0Map::B00
    }
}
#[doc = "Field `LANE0_MAP` writer - Control physical lane 0 will map to which logic lane:"]
pub type Lane0MapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lane0Map>;
impl<'a, REG> Lane0MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lane0Map::B11)
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lane0Map::B10)
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lane0Map::B01)
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lane0Map::B00)
    }
}
#[doc = "Control physical lane 1 will map to which logic lane:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lane1Map {
    #[doc = "3: Logic lane 3,"]
    B11 = 3,
    #[doc = "2: Logic lane 2,"]
    B10 = 2,
    #[doc = "1: Logic lane 1,"]
    B01 = 1,
    #[doc = "0: Logic lane 0,"]
    B00 = 0,
}
impl From<Lane1Map> for u8 {
    #[inline(always)]
    fn from(variant: Lane1Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lane1Map {
    type Ux = u8;
}
#[doc = "Field `LANE1_MAP` reader - Control physical lane 1 will map to which logic lane:"]
pub type Lane1MapR = crate::FieldReader<Lane1Map>;
impl Lane1MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane1Map {
        match self.bits {
            3 => Lane1Map::B11,
            2 => Lane1Map::B10,
            1 => Lane1Map::B01,
            0 => Lane1Map::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lane1Map::B11
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lane1Map::B10
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lane1Map::B01
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lane1Map::B00
    }
}
#[doc = "Field `LANE1_MAP` writer - Control physical lane 1 will map to which logic lane:"]
pub type Lane1MapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lane1Map>;
impl<'a, REG> Lane1MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1Map::B11)
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1Map::B10)
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1Map::B01)
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1Map::B00)
    }
}
#[doc = "Control physical lane 2 will map to which logic lane:\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lane2Map {
    #[doc = "3: Logic lane 3,"]
    B11 = 3,
    #[doc = "2: Logic lane 2,"]
    B10 = 2,
    #[doc = "1: Logic lane 1,"]
    B01 = 1,
    #[doc = "0: Logic lane 0,"]
    B00 = 0,
}
impl From<Lane2Map> for u8 {
    #[inline(always)]
    fn from(variant: Lane2Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lane2Map {
    type Ux = u8;
}
#[doc = "Field `LANE2_MAP` reader - Control physical lane 2 will map to which logic lane:"]
pub type Lane2MapR = crate::FieldReader<Lane2Map>;
impl Lane2MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane2Map {
        match self.bits {
            3 => Lane2Map::B11,
            2 => Lane2Map::B10,
            1 => Lane2Map::B01,
            0 => Lane2Map::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lane2Map::B11
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lane2Map::B10
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lane2Map::B01
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lane2Map::B00
    }
}
#[doc = "Field `LANE2_MAP` writer - Control physical lane 2 will map to which logic lane:"]
pub type Lane2MapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lane2Map>;
impl<'a, REG> Lane2MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2Map::B11)
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2Map::B10)
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2Map::B01)
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2Map::B00)
    }
}
#[doc = "Control physical lane 3 will map to which logic lane:\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lane3Map {
    #[doc = "3: Logic lane 3,"]
    B11 = 3,
    #[doc = "2: Logic lane 2,"]
    B10 = 2,
    #[doc = "1: Logic lane 1,"]
    B01 = 1,
    #[doc = "0: Logic lane 0,"]
    B00 = 0,
}
impl From<Lane3Map> for u8 {
    #[inline(always)]
    fn from(variant: Lane3Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lane3Map {
    type Ux = u8;
}
#[doc = "Field `LANE3_MAP` reader - Control physical lane 3 will map to which logic lane:"]
pub type Lane3MapR = crate::FieldReader<Lane3Map>;
impl Lane3MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane3Map {
        match self.bits {
            3 => Lane3Map::B11,
            2 => Lane3Map::B10,
            1 => Lane3Map::B01,
            0 => Lane3Map::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lane3Map::B11
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lane3Map::B10
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lane3Map::B01
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lane3Map::B00
    }
}
#[doc = "Field `LANE3_MAP` writer - Control physical lane 3 will map to which logic lane:"]
pub type Lane3MapW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lane3Map>;
impl<'a, REG> Lane3MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Logic lane 3,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3Map::B11)
    }
    #[doc = "Logic lane 2,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3Map::B10)
    }
    #[doc = "Logic lane 1,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3Map::B01)
    }
    #[doc = "Logic lane 0,"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3Map::B00)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control physical lane 0 will map to which logic lane:"]
    #[inline(always)]
    pub fn lane0_map(&self) -> Lane0MapR {
        Lane0MapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Control physical lane 1 will map to which logic lane:"]
    #[inline(always)]
    pub fn lane1_map(&self) -> Lane1MapR {
        Lane1MapR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Control physical lane 2 will map to which logic lane:"]
    #[inline(always)]
    pub fn lane2_map(&self) -> Lane2MapR {
        Lane2MapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Control physical lane 3 will map to which logic lane:"]
    #[inline(always)]
    pub fn lane3_map(&self) -> Lane3MapR {
        Lane3MapR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control physical lane 0 will map to which logic lane:"]
    #[inline(always)]
    #[must_use]
    pub fn lane0_map(&mut self) -> Lane0MapW<LaneMapSpec> {
        Lane0MapW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Control physical lane 1 will map to which logic lane:"]
    #[inline(always)]
    #[must_use]
    pub fn lane1_map(&mut self) -> Lane1MapW<LaneMapSpec> {
        Lane1MapW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Control physical lane 2 will map to which logic lane:"]
    #[inline(always)]
    #[must_use]
    pub fn lane2_map(&mut self) -> Lane2MapW<LaneMapSpec> {
        Lane2MapW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Control physical lane 3 will map to which logic lane:"]
    #[inline(always)]
    #[must_use]
    pub fn lane3_map(&mut self) -> Lane3MapW<LaneMapSpec> {
        Lane3MapW::new(self, 6)
    }
}
#[doc = "Lane Map Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LaneMapSpec;
impl crate::RegisterSpec for LaneMapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lane_map::R`](R) reader structure"]
impl crate::Readable for LaneMapSpec {}
#[doc = "`write(|w| ..)` method takes [`lane_map::W`](W) writer structure"]
impl crate::Writable for LaneMapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LANE_MAP to value 0xe4"]
impl crate::Resettable for LaneMapSpec {
    const RESET_VALUE: u32 = 0xe4;
}
