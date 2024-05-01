#[doc = "Register `DSP_HTOTAL_HS_END` reader"]
pub type R = crate::R<DspHtotalHsEndSpec>;
#[doc = "Register `DSP_HTOTAL_HS_END` writer"]
pub type W = crate::W<DspHtotalHsEndSpec>;
#[doc = "Field `DSP_HS_END` reader - Panel display scanning hsync pulse width"]
pub type DspHsEndR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HS_END` writer - Panel display scanning hsync pulse width"]
pub type DspHsEndW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_HTOTAL` reader - Panel display scanning horizontal period"]
pub type DspHtotalR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HTOTAL` writer - Panel display scanning horizontal period"]
pub type DspHtotalW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning hsync pulse width"]
    #[inline(always)]
    pub fn dsp_hs_end(&self) -> DspHsEndR {
        DspHsEndR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal period"]
    #[inline(always)]
    pub fn dsp_htotal(&self) -> DspHtotalR {
        DspHtotalR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning hsync pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_hs_end(&mut self) -> DspHsEndW<DspHtotalHsEndSpec> {
        DspHsEndW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal period"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_htotal(&mut self) -> DspHtotalW<DspHtotalHsEndSpec> {
        DspHtotalW::new(self, 16)
    }
}
#[doc = "Panel scanning horizontal width and hsync pulse end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_htotal_hs_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_htotal_hs_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspHtotalHsEndSpec;
impl crate::RegisterSpec for DspHtotalHsEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_htotal_hs_end::R`](R) reader structure"]
impl crate::Readable for DspHtotalHsEndSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_htotal_hs_end::W`](W) writer structure"]
impl crate::Writable for DspHtotalHsEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_HTOTAL_HS_END to value 0x014a_000a"]
impl crate::Resettable for DspHtotalHsEndSpec {
    const RESET_VALUE: u32 = 0x014a_000a;
}
