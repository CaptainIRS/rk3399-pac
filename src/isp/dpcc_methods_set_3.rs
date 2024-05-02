#[doc = "Register `DPCC_METHODS_SET_3` reader"]
pub type R = crate::R<DpccMethodsSet3Spec>;
#[doc = "Register `DPCC_METHODS_SET_3` writer"]
pub type W = crate::W<DpccMethodsSet3Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PgGreen3Enable {
    #[doc = "1: enable Peak Gradient check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Peak Gradient check for green"]
    B0 = 0,
}
impl From<PgGreen3Enable> for bool {
    #[inline(always)]
    fn from(variant: PgGreen3Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG_GREEN3_ENABLE` reader - "]
pub type PgGreen3EnableR = crate::BitReader<PgGreen3Enable>;
impl PgGreen3EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PgGreen3Enable {
        match self.bits {
            true => PgGreen3Enable::B1,
            false => PgGreen3Enable::B0,
        }
    }
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PgGreen3Enable::B1
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PgGreen3Enable::B0
    }
}
#[doc = "Field `PG_GREEN3_ENABLE` writer - "]
pub type PgGreen3EnableW<'a, REG> = crate::BitWriter<'a, REG, PgGreen3Enable>;
impl<'a, REG> PgGreen3EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Peak Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen3Enable::B1)
    }
    #[doc = "bypass Peak Gradient check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PgGreen3Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcGreen3Enable {
    #[doc = "1: enable Line check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Line check for green"]
    B0 = 0,
}
impl From<LcGreen3Enable> for bool {
    #[inline(always)]
    fn from(variant: LcGreen3Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LC_GREEN3_ENABLE` reader - "]
pub type LcGreen3EnableR = crate::BitReader<LcGreen3Enable>;
impl LcGreen3EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcGreen3Enable {
        match self.bits {
            true => LcGreen3Enable::B1,
            false => LcGreen3Enable::B0,
        }
    }
    #[doc = "enable Line check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LcGreen3Enable::B1
    }
    #[doc = "bypass Line check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LcGreen3Enable::B0
    }
}
#[doc = "Field `LC_GREEN3_ENABLE` writer - "]
pub type LcGreen3EnableW<'a, REG> = crate::BitWriter<'a, REG, LcGreen3Enable>;
impl<'a, REG> LcGreen3EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Line check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen3Enable::B1)
    }
    #[doc = "bypass Line check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LcGreen3Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoGreen3Enable {
    #[doc = "1: enable Rank Order check for green *Default*"]
    B1 = 1,
    #[doc = "0: bypass Rank Order check for green"]
    B0 = 0,
}
impl From<RoGreen3Enable> for bool {
    #[inline(always)]
    fn from(variant: RoGreen3Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO_GREEN3_ENABLE` reader - "]
pub type RoGreen3EnableR = crate::BitReader<RoGreen3Enable>;
impl RoGreen3EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RoGreen3Enable {
        match self.bits {
            true => RoGreen3Enable::B1,
            false => RoGreen3Enable::B0,
        }
    }
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RoGreen3Enable::B1
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RoGreen3Enable::B0
    }
}
#[doc = "Field `RO_GREEN3_ENABLE` writer - "]
pub type RoGreen3EnableW<'a, REG> = crate::BitWriter<'a, REG, RoGreen3Enable>;
impl<'a, REG> RoGreen3EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Order check for green *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen3Enable::B1)
    }
    #[doc = "bypass Rank Order check for green"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RoGreen3Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RndGreen3Enable {
    #[doc = "1: enable Rank Neighbor Difference check for green"]
    B1 = 1,
    #[doc = "0: bypass Rank Neighbor Difference check for green *Default*"]
    B0 = 0,
}
impl From<RndGreen3Enable> for bool {
    #[inline(always)]
    fn from(variant: RndGreen3Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RND_GREEN3_ENABLE` reader - "]
pub type RndGreen3EnableR = crate::BitReader<RndGreen3Enable>;
impl RndGreen3EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RndGreen3Enable {
        match self.bits {
            true => RndGreen3Enable::B1,
            false => RndGreen3Enable::B0,
        }
    }
    #[doc = "enable Rank Neighbor Difference check for green"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RndGreen3Enable::B1
    }
    #[doc = "bypass Rank Neighbor Difference check for green *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RndGreen3Enable::B0
    }
}
#[doc = "Field `RND_GREEN3_ENABLE` writer - "]
pub type RndGreen3EnableW<'a, REG> = crate::BitWriter<'a, REG, RndGreen3Enable>;
impl<'a, REG> RndGreen3EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Neighbor Difference check for green"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RndGreen3Enable::B1)
    }
    #[doc = "bypass Rank Neighbor Difference check for green *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RndGreen3Enable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgGreen3Enable {
    #[doc = "1: enable Rank Gradient check for green"]
    B1 = 1,
    #[doc = "0: bypass Rank Gradient check for green *Default*"]
    B0 = 0,
}
impl From<RgGreen3Enable> for bool {
    #[inline(always)]
    fn from(variant: RgGreen3Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RG_GREEN3_ENABLE` reader - "]
pub type RgGreen3EnableR = crate::BitReader<RgGreen3Enable>;
impl RgGreen3EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgGreen3Enable {
        match self.bits {
            true => RgGreen3Enable::B1,
            false => RgGreen3Enable::B0,
        }
    }
    #[doc = "enable Rank Gradient check for green"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgGreen3Enable::B1
    }
    #[doc = "bypass Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgGreen3Enable::B0
    }
}
#[doc = "Field `RG_GREEN3_ENABLE` writer - "]
pub type RgGreen3EnableW<'a, REG> = crate::BitWriter<'a, REG, RgGreen3Enable>;
impl<'a, REG> RgGreen3EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable Rank Gradient check for green"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen3Enable::B1)
    }
    #[doc = "bypass Rank Gradient check for green *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgGreen3Enable::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pg_green3_enable(&self) -> PgGreen3EnableR {
        PgGreen3EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lc_green3_enable(&self) -> LcGreen3EnableR {
        LcGreen3EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_green3_enable(&self) -> RoGreen3EnableR {
        RoGreen3EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rnd_green3_enable(&self) -> RndGreen3EnableR {
        RndGreen3EnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rg_green3_enable(&self) -> RgGreen3EnableR {
        RgGreen3EnableR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pg_green3_enable(&mut self) -> PgGreen3EnableW<DpccMethodsSet3Spec> {
        PgGreen3EnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lc_green3_enable(&mut self) -> LcGreen3EnableW<DpccMethodsSet3Spec> {
        LcGreen3EnableW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ro_green3_enable(&mut self) -> RoGreen3EnableW<DpccMethodsSet3Spec> {
        RoGreen3EnableW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_green3_enable(&mut self) -> RndGreen3EnableW<DpccMethodsSet3Spec> {
        RndGreen3EnableW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rg_green3_enable(&mut self) -> RgGreen3EnableW<DpccMethodsSet3Spec> {
        RgGreen3EnableW::new(self, 4)
    }
}
#[doc = "Methods enable bits for SET_3\n\nNote: different methods can be used in parallel, the result is the logical AND of all selected \n\n\n\nmethods \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_methods_set_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_methods_set_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccMethodsSet3Spec;
impl crate::RegisterSpec for DpccMethodsSet3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_methods_set_3::R`](R) reader structure"]
impl crate::Readable for DpccMethodsSet3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_methods_set_3::W`](W) writer structure"]
impl crate::Writable for DpccMethodsSet3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_METHODS_SET_3 to value 0"]
impl crate::Resettable for DpccMethodsSet3Spec {
    const RESET_VALUE: u32 = 0;
}
