#[doc = "Register `CRU_LPLL_CON5` reader"]
pub type R = crate::R<CruLpllCon5Spec>;
#[doc = "Register `CRU_LPLL_CON5` writer"]
pub type W = crate::W<CruLpllCon5Spec>;
#[doc = "select external wave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsmodSelExtWave {
    #[doc = "0: select ext_wave"]
    B0 = 0,
    #[doc = "1: select ext_wave"]
    B1 = 1,
}
impl From<SsmodSelExtWave> for bool {
    #[inline(always)]
    fn from(variant: SsmodSelExtWave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSMOD_SEL_EXT_WAVE` reader - select external wave"]
pub type SsmodSelExtWaveR = crate::BitReader<SsmodSelExtWave>;
impl SsmodSelExtWaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsmodSelExtWave {
        match self.bits {
            false => SsmodSelExtWave::B0,
            true => SsmodSelExtWave::B1,
        }
    }
    #[doc = "select ext_wave"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SsmodSelExtWave::B0
    }
    #[doc = "select ext_wave"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SsmodSelExtWave::B1
    }
}
#[doc = "Field `SSMOD_SEL_EXT_WAVE` writer - select external wave"]
pub type SsmodSelExtWaveW<'a, REG> = crate::BitWriter<'a, REG, SsmodSelExtWave>;
impl<'a, REG> SsmodSelExtWaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select ext_wave"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodSelExtWave::B0)
    }
    #[doc = "select ext_wave"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SsmodSelExtWave::B1)
    }
}
#[doc = "Field `SSMOD_EXT_MAXADDR` reader - External wave table data inputs (0-255)"]
pub type SsmodExtMaxaddrR = crate::FieldReader;
#[doc = "Field `SSMOD_EXT_MAXADDR` writer - External wave table data inputs (0-255)"]
pub type SsmodExtMaxaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - select external wave"]
    #[inline(always)]
    pub fn ssmod_sel_ext_wave(&self) -> SsmodSelExtWaveR {
        SsmodSelExtWaveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - External wave table data inputs (0-255)"]
    #[inline(always)]
    pub fn ssmod_ext_maxaddr(&self) -> SsmodExtMaxaddrR {
        SsmodExtMaxaddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - select external wave"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_sel_ext_wave(&mut self) -> SsmodSelExtWaveW<CruLpllCon5Spec> {
        SsmodSelExtWaveW::new(self, 0)
    }
    #[doc = "Bits 8:15 - External wave table data inputs (0-255)"]
    #[inline(always)]
    #[must_use]
    pub fn ssmod_ext_maxaddr(&mut self) -> SsmodExtMaxaddrW<CruLpllCon5Spec> {
        SsmodExtMaxaddrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruLpllCon5Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "LPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruLpllCon5Spec;
impl crate::RegisterSpec for CruLpllCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_lpll_con5::R`](R) reader structure"]
impl crate::Readable for CruLpllCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_lpll_con5::W`](W) writer structure"]
impl crate::Writable for CruLpllCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_LPLL_CON5 to value 0x7f00"]
impl crate::Resettable for CruLpllCon5Spec {
    const RESET_VALUE: u32 = 0x7f00;
}
