#[doc = "Register `DSP_VACT_ST_END_F1` reader"]
pub type R = crate::R<DspVactStEndF1Spec>;
#[doc = "Register `DSP_VACT_ST_END_F1` writer"]
pub type W = crate::W<DspVactStEndF1Spec>;
#[doc = "Field `DSP_VACT_END_F1` reader - Panel display scanning vertical active end point of 2nd field\n\n(interlace display mode)"]
pub type DspVactEndF1R = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_END_F1` writer - Panel display scanning vertical active end point of 2nd field\n\n(interlace display mode)"]
pub type DspVactEndF1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_VACT_ST_F1` reader - Panel display scanning vertical active start point of 2nd field\n\n(interlace display mode)"]
pub type DspVactStF1R = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_ST_F1` writer - Panel display scanning vertical active start point of 2nd field\n\n(interlace display mode)"]
pub type DspVactStF1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning vertical active end point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    pub fn dsp_vact_end_f1(&self) -> DspVactEndF1R {
        DspVactEndF1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical active start point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    pub fn dsp_vact_st_f1(&self) -> DspVactStF1R {
        DspVactStF1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning vertical active end point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_end_f1(&mut self) -> DspVactEndF1W<DspVactStEndF1Spec> {
        DspVactEndF1W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical active start point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_st_f1(&mut self) -> DspVactStF1W<DspVactStEndF1Spec> {
        DspVactStF1W::new(self, 16)
    }
}
#[doc = "Vertical scanning active start point and end point of even filed in interlace mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vact_st_end_f1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vact_st_end_f1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspVactStEndF1Spec;
impl crate::RegisterSpec for DspVactStEndF1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_vact_st_end_f1::R`](R) reader structure"]
impl crate::Readable for DspVactStEndF1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp_vact_st_end_f1::W`](W) writer structure"]
impl crate::Writable for DspVactStEndF1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_VACT_ST_END_F1 to value 0"]
impl crate::Resettable for DspVactStEndF1Spec {
    const RESET_VALUE: u32 = 0;
}
