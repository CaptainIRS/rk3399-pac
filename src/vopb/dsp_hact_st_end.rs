#[doc = "Register `DSP_HACT_ST_END` reader"]
pub type R = crate::R<DspHactStEndSpec>;
#[doc = "Register `DSP_HACT_ST_END` writer"]
pub type W = crate::W<DspHactStEndSpec>;
#[doc = "Field `DSP_HACT_END` reader - Panel display scanning horizontal active end point"]
pub type DspHactEndR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HACT_END` writer - Panel display scanning horizontal active end point"]
pub type DspHactEndW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_HACT_ST` reader - Panel display scanning horizontal active start point"]
pub type DspHactStR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HACT_ST` writer - Panel display scanning horizontal active start point"]
pub type DspHactStW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    pub fn dsp_hact_end(&self) -> DspHactEndR {
        DspHactEndR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    pub fn dsp_hact_st(&self) -> DspHactStR {
        DspHactStR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_hact_end(&mut self) -> DspHactEndW<DspHactStEndSpec> {
        DspHactEndW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_hact_st(&mut self) -> DspHactStW<DspHactStEndSpec> {
        DspHactStW::new(self, 16)
    }
}
#[doc = "Panel active horizontal scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_hact_st_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_hact_st_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspHactStEndSpec;
impl crate::RegisterSpec for DspHactStEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_hact_st_end::R`](R) reader structure"]
impl crate::Readable for DspHactStEndSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_hact_st_end::W`](W) writer structure"]
impl crate::Writable for DspHactStEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_HACT_ST_END to value 0x000a_014a"]
impl crate::Resettable for DspHactStEndSpec {
    const RESET_VALUE: u32 = 0x000a_014a;
}
