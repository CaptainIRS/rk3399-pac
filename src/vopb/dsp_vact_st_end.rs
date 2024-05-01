#[doc = "Register `DSP_VACT_ST_END` reader"]
pub type R = crate::R<DspVactStEndSpec>;
#[doc = "Register `DSP_VACT_ST_END` writer"]
pub type W = crate::W<DspVactStEndSpec>;
#[doc = "Field `DSP_VACT_END` reader - Panel display scanning vertical active end point"]
pub type DspVactEndR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_END` writer - Panel display scanning vertical active end point"]
pub type DspVactEndW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_VACT_ST` reader - Panel display scanning vertical active start point"]
pub type DspVactStR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_ST` writer - Panel display scanning vertical active start point"]
pub type DspVactStW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning vertical active end point"]
    #[inline(always)]
    pub fn dsp_vact_end(&self) -> DspVactEndR {
        DspVactEndR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical active start point"]
    #[inline(always)]
    pub fn dsp_vact_st(&self) -> DspVactStR {
        DspVactStR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning vertical active end point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_end(&mut self) -> DspVactEndW<DspVactStEndSpec> {
        DspVactEndW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical active start point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_st(&mut self) -> DspVactStW<DspVactStEndSpec> {
        DspVactStW::new(self, 16)
    }
}
#[doc = "Panel active vertical scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vact_st_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vact_st_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspVactStEndSpec;
impl crate::RegisterSpec for DspVactStEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_vact_st_end::R`](R) reader structure"]
impl crate::Readable for DspVactStEndSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_vact_st_end::W`](W) writer structure"]
impl crate::Writable for DspVactStEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_VACT_ST_END to value 0x000a_00fa"]
impl crate::Resettable for DspVactStEndSpec {
    const RESET_VALUE: u32 = 0x000a_00fa;
}
