#[doc = "Register `CRU_GLB_RST_ST` reader"]
pub type R = crate::R<CruGlbRstStSpec>;
#[doc = "Register `CRU_GLB_RST_ST` writer"]
pub type W = crate::W<CruGlbRstStSpec>;
#[doc = "first global rst flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FstGlbRstSt {
    #[doc = "0: last hot reset is first global reset"]
    B0 = 0,
    #[doc = "1: last hot reset is first global reset"]
    B1 = 1,
}
impl From<FstGlbRstSt> for bool {
    #[inline(always)]
    fn from(variant: FstGlbRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FST_GLB_RST_ST` reader - first global rst flag"]
pub type FstGlbRstStR = crate::BitReader<FstGlbRstSt>;
impl FstGlbRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FstGlbRstSt {
        match self.bits {
            false => FstGlbRstSt::B0,
            true => FstGlbRstSt::B1,
        }
    }
    #[doc = "last hot reset is first global reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FstGlbRstSt::B0
    }
    #[doc = "last hot reset is first global reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FstGlbRstSt::B1
    }
}
#[doc = "Field `FST_GLB_RST_ST` writer - first global rst flag"]
pub type FstGlbRstStW<'a, REG> = crate::BitWriter1C<'a, REG, FstGlbRstSt>;
impl<'a, REG> FstGlbRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is first global reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbRstSt::B0)
    }
    #[doc = "last hot reset is first global reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbRstSt::B1)
    }
}
#[doc = "second global rst flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SndGlbRstSt {
    #[doc = "0: last hot reset is second global reset"]
    B0 = 0,
    #[doc = "1: last hot reset is second global reset"]
    B1 = 1,
}
impl From<SndGlbRstSt> for bool {
    #[inline(always)]
    fn from(variant: SndGlbRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SND_GLB_RST_ST` reader - second global rst flag"]
pub type SndGlbRstStR = crate::BitReader<SndGlbRstSt>;
impl SndGlbRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SndGlbRstSt {
        match self.bits {
            false => SndGlbRstSt::B0,
            true => SndGlbRstSt::B1,
        }
    }
    #[doc = "last hot reset is second global reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SndGlbRstSt::B0
    }
    #[doc = "last hot reset is second global reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SndGlbRstSt::B1
    }
}
#[doc = "Field `SND_GLB_RST_ST` writer - second global rst flag"]
pub type SndGlbRstStW<'a, REG> = crate::BitWriter1C<'a, REG, SndGlbRstSt>;
impl<'a, REG> SndGlbRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is second global reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbRstSt::B0)
    }
    #[doc = "last hot reset is second global reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbRstSt::B1)
    }
}
#[doc = "first global TSADC triggered reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FstGlbTsadcRstSt {
    #[doc = "0: last hot reset is first global TSADC triggered reset"]
    B0 = 0,
    #[doc = "1: last hot reset is first global TSADC triggered reset"]
    B1 = 1,
}
impl From<FstGlbTsadcRstSt> for bool {
    #[inline(always)]
    fn from(variant: FstGlbTsadcRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FST_GLB_TSADC_RST_ST` reader - first global TSADC triggered reset flag"]
pub type FstGlbTsadcRstStR = crate::BitReader<FstGlbTsadcRstSt>;
impl FstGlbTsadcRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FstGlbTsadcRstSt {
        match self.bits {
            false => FstGlbTsadcRstSt::B0,
            true => FstGlbTsadcRstSt::B1,
        }
    }
    #[doc = "last hot reset is first global TSADC triggered reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FstGlbTsadcRstSt::B0
    }
    #[doc = "last hot reset is first global TSADC triggered reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FstGlbTsadcRstSt::B1
    }
}
#[doc = "Field `FST_GLB_TSADC_RST_ST` writer - first global TSADC triggered reset flag"]
pub type FstGlbTsadcRstStW<'a, REG> = crate::BitWriter1C<'a, REG, FstGlbTsadcRstSt>;
impl<'a, REG> FstGlbTsadcRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is first global TSADC triggered reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbTsadcRstSt::B0)
    }
    #[doc = "last hot reset is first global TSADC triggered reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbTsadcRstSt::B1)
    }
}
#[doc = "second global TSADC triggered reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SndGlbTsadcRstSt {
    #[doc = "0: last hot reset is second global TSADC triggered reset"]
    B0 = 0,
    #[doc = "1: last hot reset is second global TSADC triggered reset"]
    B1 = 1,
}
impl From<SndGlbTsadcRstSt> for bool {
    #[inline(always)]
    fn from(variant: SndGlbTsadcRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SND_GLB_TSADC_RST_ST` reader - second global TSADC triggered reset flag"]
pub type SndGlbTsadcRstStR = crate::BitReader<SndGlbTsadcRstSt>;
impl SndGlbTsadcRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SndGlbTsadcRstSt {
        match self.bits {
            false => SndGlbTsadcRstSt::B0,
            true => SndGlbTsadcRstSt::B1,
        }
    }
    #[doc = "last hot reset is second global TSADC triggered reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SndGlbTsadcRstSt::B0
    }
    #[doc = "last hot reset is second global TSADC triggered reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SndGlbTsadcRstSt::B1
    }
}
#[doc = "Field `SND_GLB_TSADC_RST_ST` writer - second global TSADC triggered reset flag"]
pub type SndGlbTsadcRstStW<'a, REG> = crate::BitWriter1C<'a, REG, SndGlbTsadcRstSt>;
impl<'a, REG> SndGlbTsadcRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is second global TSADC triggered reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbTsadcRstSt::B0)
    }
    #[doc = "last hot reset is second global TSADC triggered reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbTsadcRstSt::B1)
    }
}
#[doc = "first global watch_dog triggered reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FstGlbWdtRstSt {
    #[doc = "0: last hot reset is first global watch_dog triggered reset"]
    B0 = 0,
    #[doc = "1: last hot reset is first global watch_dog triggered reset"]
    B1 = 1,
}
impl From<FstGlbWdtRstSt> for bool {
    #[inline(always)]
    fn from(variant: FstGlbWdtRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FST_GLB_WDT_RST_ST` reader - first global watch_dog triggered reset flag"]
pub type FstGlbWdtRstStR = crate::BitReader<FstGlbWdtRstSt>;
impl FstGlbWdtRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FstGlbWdtRstSt {
        match self.bits {
            false => FstGlbWdtRstSt::B0,
            true => FstGlbWdtRstSt::B1,
        }
    }
    #[doc = "last hot reset is first global watch_dog triggered reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FstGlbWdtRstSt::B0
    }
    #[doc = "last hot reset is first global watch_dog triggered reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FstGlbWdtRstSt::B1
    }
}
#[doc = "Field `FST_GLB_WDT_RST_ST` writer - first global watch_dog triggered reset flag"]
pub type FstGlbWdtRstStW<'a, REG> = crate::BitWriter1C<'a, REG, FstGlbWdtRstSt>;
impl<'a, REG> FstGlbWdtRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is first global watch_dog triggered reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbWdtRstSt::B0)
    }
    #[doc = "last hot reset is first global watch_dog triggered reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FstGlbWdtRstSt::B1)
    }
}
#[doc = "second global watch_dog triggered reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SndGlbWdtRstSt {
    #[doc = "0: last hot reset is second global watch_dog triggered reset"]
    B0 = 0,
    #[doc = "1: last hot reset is second global watch_dog triggered reset"]
    B1 = 1,
}
impl From<SndGlbWdtRstSt> for bool {
    #[inline(always)]
    fn from(variant: SndGlbWdtRstSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SND_GLB_WDT_RST_ST` reader - second global watch_dog triggered reset flag"]
pub type SndGlbWdtRstStR = crate::BitReader<SndGlbWdtRstSt>;
impl SndGlbWdtRstStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SndGlbWdtRstSt {
        match self.bits {
            false => SndGlbWdtRstSt::B0,
            true => SndGlbWdtRstSt::B1,
        }
    }
    #[doc = "last hot reset is second global watch_dog triggered reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SndGlbWdtRstSt::B0
    }
    #[doc = "last hot reset is second global watch_dog triggered reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SndGlbWdtRstSt::B1
    }
}
#[doc = "Field `SND_GLB_WDT_RST_ST` writer - second global watch_dog triggered reset flag"]
pub type SndGlbWdtRstStW<'a, REG> = crate::BitWriter1C<'a, REG, SndGlbWdtRstSt>;
impl<'a, REG> SndGlbWdtRstStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last hot reset is second global watch_dog triggered reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbWdtRstSt::B0)
    }
    #[doc = "last hot reset is second global watch_dog triggered reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SndGlbWdtRstSt::B1)
    }
}
impl R {
    #[doc = "Bit 0 - first global rst flag"]
    #[inline(always)]
    pub fn fst_glb_rst_st(&self) -> FstGlbRstStR {
        FstGlbRstStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - second global rst flag"]
    #[inline(always)]
    pub fn snd_glb_rst_st(&self) -> SndGlbRstStR {
        SndGlbRstStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - first global TSADC triggered reset flag"]
    #[inline(always)]
    pub fn fst_glb_tsadc_rst_st(&self) -> FstGlbTsadcRstStR {
        FstGlbTsadcRstStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - second global TSADC triggered reset flag"]
    #[inline(always)]
    pub fn snd_glb_tsadc_rst_st(&self) -> SndGlbTsadcRstStR {
        SndGlbTsadcRstStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - first global watch_dog triggered reset flag"]
    #[inline(always)]
    pub fn fst_glb_wdt_rst_st(&self) -> FstGlbWdtRstStR {
        FstGlbWdtRstStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - second global watch_dog triggered reset flag"]
    #[inline(always)]
    pub fn snd_glb_wdt_rst_st(&self) -> SndGlbWdtRstStR {
        SndGlbWdtRstStR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - first global rst flag"]
    #[inline(always)]
    #[must_use]
    pub fn fst_glb_rst_st(&mut self) -> FstGlbRstStW<CruGlbRstStSpec> {
        FstGlbRstStW::new(self, 0)
    }
    #[doc = "Bit 1 - second global rst flag"]
    #[inline(always)]
    #[must_use]
    pub fn snd_glb_rst_st(&mut self) -> SndGlbRstStW<CruGlbRstStSpec> {
        SndGlbRstStW::new(self, 1)
    }
    #[doc = "Bit 2 - first global TSADC triggered reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn fst_glb_tsadc_rst_st(&mut self) -> FstGlbTsadcRstStW<CruGlbRstStSpec> {
        FstGlbTsadcRstStW::new(self, 2)
    }
    #[doc = "Bit 3 - second global TSADC triggered reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn snd_glb_tsadc_rst_st(&mut self) -> SndGlbTsadcRstStW<CruGlbRstStSpec> {
        SndGlbTsadcRstStW::new(self, 3)
    }
    #[doc = "Bit 4 - first global watch_dog triggered reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn fst_glb_wdt_rst_st(&mut self) -> FstGlbWdtRstStW<CruGlbRstStSpec> {
        FstGlbWdtRstStW::new(self, 4)
    }
    #[doc = "Bit 5 - second global watch_dog triggered reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn snd_glb_wdt_rst_st(&mut self) -> SndGlbWdtRstStW<CruGlbRstStSpec> {
        SndGlbWdtRstStW::new(self, 5)
    }
}
#[doc = "Global reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_rst_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_rst_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruGlbRstStSpec;
impl crate::RegisterSpec for CruGlbRstStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_glb_rst_st::R`](R) reader structure"]
impl crate::Readable for CruGlbRstStSpec {}
#[doc = "`write(|w| ..)` method takes [`cru_glb_rst_st::W`](W) writer structure"]
impl crate::Writable for CruGlbRstStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets CRU_GLB_RST_ST to value 0"]
impl crate::Resettable for CruGlbRstStSpec {
    const RESET_VALUE: u32 = 0;
}
