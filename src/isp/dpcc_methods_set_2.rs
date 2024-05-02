#[doc = "Register `DPCC_METHODS_SET_2` reader"]
pub type R = crate::R<DpccMethodsSet2Spec>;
#[doc = "Register `DPCC_METHODS_SET_2` writer"]
pub type W = crate::W<DpccMethodsSet2Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PgGreen2Enable {
    #[doc = "1: enable Peak Gradient check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Peak Gradient check for green"]
    B0 = 0,
}
impl From<PgGreen2Enable> for bool {
    #[inline(always)]
    fn from(variant: PgGreen2Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG_GREEN2_ENABLE` reader - "]
pub type PgGreen2EnableR = crate::BitReader<PgGreen2Enable>;
impl PgGreen2EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PgGreen2Enable {
        match self.bits {
            true => PgGreen2Enable::B1,
            false => PgGreen2Enable::B0,
        }
    }
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PgGreen2Enable::B1
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PgGreen2Enable::B0
    }
}
#[doc = "Field `PG_GREEN2_ENABLE` writer - "]
pub type PgGreen2EnableW<'a, REG> = crate::BitWriter<'a, REG, PgGreen2Enable>;
impl<'a, REG> PgGreen2EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen2Enable::B1)
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen2Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcGreen2Enable {
    #[doc = "1: enable Line check for green"]
    B1 = 1,
    #[doc = "0: bypass Line check for green *Default*"]
    B0 = 0,
}
impl From<LcGreen2Enable> for bool {
    #[inline(always)]
    fn from(variant: LcGreen2Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LC_GREEN2_ENABLE` reader - "]
pub type LcGreen2EnableR = crate::BitReader<LcGreen2Enable>;
impl LcGreen2EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcGreen2Enable {
        match self.bits {
            true => LcGreen2Enable::B1,
            false => LcGreen2Enable::B0,
        }
    }
    #[doc = "enable Line check for green"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LcGreen2Enable::B1
    }
    #[doc = "bypass Line check for green *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LcGreen2Enable::B0
    }
}
#[doc = "Field `LC_GREEN2_ENABLE` writer - "]
pub type LcGreen2EnableW<'a, REG> = crate::BitWriter<'a, REG, LcGreen2Enable>;
impl<'a, REG> LcGreen2EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Line check for green"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen2Enable::B1)
    }
    #[doc = "bypass Line check for green *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen2Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoGreen2Enable {
    #[doc = "1: enable Rank Order check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Order check for green"]
    B0 = 0,
}
impl From<RoGreen2Enable> for bool {
    #[inline(always)]
    fn from(variant: RoGreen2Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO_GREEN2_ENABLE` reader - "]
pub type RoGreen2EnableR = crate::BitReader<RoGreen2Enable>;
impl RoGreen2EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RoGreen2Enable {
        match self.bits {
            true => RoGreen2Enable::B1,
            false => RoGreen2Enable::B0,
        }
    }
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RoGreen2Enable::B1
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RoGreen2Enable::B0
    }
}
#[doc = "Field `RO_GREEN2_ENABLE` writer - "]
pub type RoGreen2EnableW<'a, REG> = crate::BitWriter<'a, REG, RoGreen2Enable>;
impl<'a, REG> RoGreen2EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen2Enable::B1)
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen2Enable::B0)
    }
}
#[doc = "Field `RND_GREEN2_ENABLE` reader - 1: enable Rank Neighbor Difference check for green\n\n*Default* 0: bypass Rank Neighbor Difference check for\n\ngreen"]
pub type RndGreen2EnableR = crate::BitReader;
#[doc = "Field `RND_GREEN2_ENABLE` writer - 1: enable Rank Neighbor Difference check for green\n\n*Default* 0: bypass Rank Neighbor Difference check for\n\ngreen"]
pub type RndGreen2EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgGreen2Enable {
    #[doc = "1: enable Rank Gradient check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Gradient check for green"]
    B0 = 0,
}
impl From<RgGreen2Enable> for bool {
    #[inline(always)]
    fn from(variant: RgGreen2Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RG_GREEN2_ENABLE` reader - "]
pub type RgGreen2EnableR = crate::BitReader<RgGreen2Enable>;
impl RgGreen2EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgGreen2Enable {
        match self.bits {
            true => RgGreen2Enable::B1,
            false => RgGreen2Enable::B0,
        }
    }
    #[doc = "enable Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgGreen2Enable::B1
    }
    #[doc = "bypass Rank Gradient check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgGreen2Enable::B0
    }
}
#[doc = "Field `RG_GREEN2_ENABLE` writer - "]
pub type RgGreen2EnableW<'a, REG> = crate::BitWriter<'a, REG, RgGreen2Enable>;
impl<'a, REG> RgGreen2EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen2Enable::B1)
    }
    #[doc = "bypass Rank Gradient check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen2Enable::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pg_green2_enable(&self) -> PgGreen2EnableR {
        PgGreen2EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lc_green2_enable(&self) -> LcGreen2EnableR {
        LcGreen2EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_green2_enable(&self) -> RoGreen2EnableR {
        RoGreen2EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enable Rank Neighbor Difference check for green\n\n*Default* 0: bypass Rank Neighbor Difference check for\n\ngreen"]
    #[inline(always)]
    pub fn rnd_green2_enable(&self) -> RndGreen2EnableR {
        RndGreen2EnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rg_green2_enable(&self) -> RgGreen2EnableR {
        RgGreen2EnableR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pg_green2_enable(&mut self) -> PgGreen2EnableW<DpccMethodsSet2Spec> {
        PgGreen2EnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lc_green2_enable(&mut self) -> LcGreen2EnableW<DpccMethodsSet2Spec> {
        LcGreen2EnableW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ro_green2_enable(&mut self) -> RoGreen2EnableW<DpccMethodsSet2Spec> {
        RoGreen2EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: enable Rank Neighbor Difference check for green\n\n*Default* 0: bypass Rank Neighbor Difference check for\n\ngreen"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_green2_enable(&mut self) -> RndGreen2EnableW<DpccMethodsSet2Spec> {
        RndGreen2EnableW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rg_green2_enable(&mut self) -> RgGreen2EnableW<DpccMethodsSet2Spec> {
        RgGreen2EnableW::new(self, 4)
    }
}
#[doc = "Methods enable bits for SET_2\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccMethodsSet2Spec;
impl crate::RegisterSpec for DpccMethodsSet2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_methods_set_2::R`](R) reader structure"]
impl crate::Readable for DpccMethodsSet2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_methods_set_2::W`](W) writer structure"]
impl crate::Writable for DpccMethodsSet2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_METHODS_SET_2 to value 0"]
impl crate::Resettable for DpccMethodsSet2Spec {
    const RESET_VALUE: u32 = 0;
}
