#[doc = "Register `DSP_VTOTAL_VS_END` reader"]
pub type R = crate::R<DspVtotalVsEndSpec>;
#[doc = "Register `DSP_VTOTAL_VS_END` writer"]
pub type W = crate::W<DspVtotalVsEndSpec>;
#[doc = "Field `DSP_VS_END` reader - Panel display scanning vsync pulse width"]
pub type DspVsEndR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VS_END` writer - Panel display scanning vsync pulse width"]
pub type DspVsEndW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "dsp vtotal number valid immediately enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDspVtotalImd {
    #[doc = "0: valid after frame start"]
    B0 = 0,
    #[doc = "1: valid immediately"]
    B1 = 1,
}
impl From<SwDspVtotalImd> for bool {
    #[inline(always)]
    fn from(variant: SwDspVtotalImd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DSP_VTOTAL_IMD` reader - dsp vtotal number valid immediately enable."]
pub type SwDspVtotalImdR = crate::BitReader<SwDspVtotalImd>;
impl SwDspVtotalImdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDspVtotalImd {
        match self.bits {
            false => SwDspVtotalImd::B0,
            true => SwDspVtotalImd::B1,
        }
    }
    #[doc = "valid after frame start"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDspVtotalImd::B0
    }
    #[doc = "valid immediately"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDspVtotalImd::B1
    }
}
#[doc = "Field `SW_DSP_VTOTAL_IMD` writer - dsp vtotal number valid immediately enable."]
pub type SwDspVtotalImdW<'a, REG> = crate::BitWriter<'a, REG, SwDspVtotalImd>;
impl<'a, REG> SwDspVtotalImdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "valid after frame start"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDspVtotalImd::B0)
    }
    #[doc = "valid immediately"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDspVtotalImd::B1)
    }
}
#[doc = "Field `DSP_VTOTAL` reader - Panel display scanning vertical period."]
pub type DspVtotalR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VTOTAL` writer - Panel display scanning vertical period."]
pub type DspVtotalW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning vsync pulse width"]
    #[inline(always)]
    pub fn dsp_vs_end(&self) -> DspVsEndR {
        DspVsEndR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - dsp vtotal number valid immediately enable."]
    #[inline(always)]
    pub fn sw_dsp_vtotal_imd(&self) -> SwDspVtotalImdR {
        SwDspVtotalImdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical period."]
    #[inline(always)]
    pub fn dsp_vtotal(&self) -> DspVtotalR {
        DspVtotalR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning vsync pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vs_end(&mut self) -> DspVsEndW<DspVtotalVsEndSpec> {
        DspVsEndW::new(self, 0)
    }
    #[doc = "Bit 15 - dsp vtotal number valid immediately enable."]
    #[inline(always)]
    #[must_use]
    pub fn sw_dsp_vtotal_imd(&mut self) -> SwDspVtotalImdW<DspVtotalVsEndSpec> {
        SwDspVtotalImdW::new(self, 15)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical period."]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vtotal(&mut self) -> DspVtotalW<DspVtotalVsEndSpec> {
        DspVtotalW::new(self, 16)
    }
}
#[doc = "Panel scanning vertical height and vsync pulse end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vtotal_vs_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vtotal_vs_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspVtotalVsEndSpec;
impl crate::RegisterSpec for DspVtotalVsEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_vtotal_vs_end::R`](R) reader structure"]
impl crate::Readable for DspVtotalVsEndSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_vtotal_vs_end::W`](W) writer structure"]
impl crate::Writable for DspVtotalVsEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_VTOTAL_VS_END to value 0x00fa_000a"]
impl crate::Resettable for DspVtotalVsEndSpec {
    const RESET_VALUE: u32 = 0x00fa_000a;
}
