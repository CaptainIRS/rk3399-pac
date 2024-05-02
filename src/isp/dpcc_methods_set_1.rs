#[doc = "Register `DPCC_METHODS_SET_1` reader"]
pub type R = crate::R<DpccMethodsSet1Spec>;
#[doc = "Register `DPCC_METHODS_SET_1` writer"]
pub type W = crate::W<DpccMethodsSet1Spec>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PgGreen1Enable {
    #[doc = "1: enable Peak Gradient check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Peak Gradient check for green"]
    B0 = 0,
}
impl From<PgGreen1Enable> for bool {
    #[inline(always)]
    fn from(variant: PgGreen1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG_GREEN1_ENABLE` reader - "]
pub type PgGreen1EnableR = crate::BitReader<PgGreen1Enable>;
impl PgGreen1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PgGreen1Enable {
        match self.bits {
            true => PgGreen1Enable::B1,
            false => PgGreen1Enable::B0,
        }
    }
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PgGreen1Enable::B1
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PgGreen1Enable::B0
    }
}
#[doc = "Field `PG_GREEN1_ENABLE` writer - "]
pub type PgGreen1EnableW<'a, REG> = crate::BitWriter<'a, REG, PgGreen1Enable>;
impl<'a, REG> PgGreen1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen1Enable::B1)
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen1Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcGreen1Enable {
    #[doc = "1: enable Line check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Line check for green"]
    B0 = 0,
}
impl From<LcGreen1Enable> for bool {
    #[inline(always)]
    fn from(variant: LcGreen1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LC_GREEN1_ENABLE` reader - "]
pub type LcGreen1EnableR = crate::BitReader<LcGreen1Enable>;
impl LcGreen1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcGreen1Enable {
        match self.bits {
            true => LcGreen1Enable::B1,
            false => LcGreen1Enable::B0,
        }
    }
    #[doc = "enable Line check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LcGreen1Enable::B1
    }
    #[doc = "bypass Line check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LcGreen1Enable::B0
    }
}
#[doc = "Field `LC_GREEN1_ENABLE` writer - "]
pub type LcGreen1EnableW<'a, REG> = crate::BitWriter<'a, REG, LcGreen1Enable>;
impl<'a, REG> LcGreen1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Line check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen1Enable::B1)
    }
    #[doc = "bypass Line check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen1Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoGreen1Enable {
    #[doc = "1: enable Rank Order check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Order check for green"]
    B0 = 0,
}
impl From<RoGreen1Enable> for bool {
    #[inline(always)]
    fn from(variant: RoGreen1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO_GREEN1_ENABLE` reader - "]
pub type RoGreen1EnableR = crate::BitReader<RoGreen1Enable>;
impl RoGreen1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RoGreen1Enable {
        match self.bits {
            true => RoGreen1Enable::B1,
            false => RoGreen1Enable::B0,
        }
    }
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RoGreen1Enable::B1
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RoGreen1Enable::B0
    }
}
#[doc = "Field `RO_GREEN1_ENABLE` writer - "]
pub type RoGreen1EnableW<'a, REG> = crate::BitWriter<'a, REG, RoGreen1Enable>;
impl<'a, REG> RoGreen1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen1Enable::B1)
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen1Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RndGreen1Enable {
    #[doc = "1: enable Rank Neighbor Difference check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Neighbor Difference check for green"]
    B0 = 0,
}
impl From<RndGreen1Enable> for bool {
    #[inline(always)]
    fn from(variant: RndGreen1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RND_GREEN1_ENABLE` reader - "]
pub type RndGreen1EnableR = crate::BitReader<RndGreen1Enable>;
impl RndGreen1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RndGreen1Enable {
        match self.bits {
            true => RndGreen1Enable::B1,
            false => RndGreen1Enable::B0,
        }
    }
    #[doc = "enable Rank Neighbor Difference check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RndGreen1Enable::B1
    }
    #[doc = "bypass Rank Neighbor Difference check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RndGreen1Enable::B0
    }
}
#[doc = "Field `RND_GREEN1_ENABLE` writer - "]
pub type RndGreen1EnableW<'a, REG> = crate::BitWriter<'a, REG, RndGreen1Enable>;
impl<'a, REG> RndGreen1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Neighbor Difference check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RndGreen1Enable::B1)
    }
    #[doc = "bypass Rank Neighbor Difference check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RndGreen1Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgGreen1Enable {
    #[doc = "1: enable Rank Gradient check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Gradient check for green"]
    B0 = 0,
}
impl From<RgGreen1Enable> for bool {
    #[inline(always)]
    fn from(variant: RgGreen1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RG_GREEN1_ENABLE` reader - "]
pub type RgGreen1EnableR = crate::BitReader<RgGreen1Enable>;
impl RgGreen1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgGreen1Enable {
        match self.bits {
            true => RgGreen1Enable::B1,
            false => RgGreen1Enable::B0,
        }
    }
    #[doc = "enable Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgGreen1Enable::B1
    }
    #[doc = "bypass Rank Gradient check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgGreen1Enable::B0
    }
}
#[doc = "Field `RG_GREEN1_ENABLE` writer - "]
pub type RgGreen1EnableW<'a, REG> = crate::BitWriter<'a, REG, RgGreen1Enable>;
impl<'a, REG> RgGreen1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen1Enable::B1)
    }
    #[doc = "bypass Rank Gradient check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen1Enable::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pg_green1_enable(&self) -> PgGreen1EnableR {
        PgGreen1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lc_green1_enable(&self) -> LcGreen1EnableR {
        LcGreen1EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_green1_enable(&self) -> RoGreen1EnableR {
        RoGreen1EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rnd_green1_enable(&self) -> RndGreen1EnableR {
        RndGreen1EnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rg_green1_enable(&self) -> RgGreen1EnableR {
        RgGreen1EnableR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pg_green1_enable(&mut self) -> PgGreen1EnableW<DpccMethodsSet1Spec> {
        PgGreen1EnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lc_green1_enable(&mut self) -> LcGreen1EnableW<DpccMethodsSet1Spec> {
        LcGreen1EnableW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ro_green1_enable(&mut self) -> RoGreen1EnableW<DpccMethodsSet1Spec> {
        RoGreen1EnableW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_green1_enable(&mut self) -> RndGreen1EnableW<DpccMethodsSet1Spec> {
        RndGreen1EnableW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rg_green1_enable(&mut self) -> RgGreen1EnableW<DpccMethodsSet1Spec> {
        RgGreen1EnableW::new(self, 4)
    }
}
#[doc = "Methods enable bits for SET_1\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccMethodsSet1Spec;
impl crate::RegisterSpec for DpccMethodsSet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_methods_set_1::R`](R) reader structure"]
impl crate::Readable for DpccMethodsSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_methods_set_1::W`](W) writer structure"]
impl crate::Writable for DpccMethodsSet1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_METHODS_SET_1 to value 0x1d1d"]
impl crate::Resettable for DpccMethodsSet1Spec {
    const RESET_VALUE: u32 = 0x1d1d;
}
