#[doc = "Register `DSP_VS_ST_END_F1` reader"]
pub type R = crate::R<DspVsStEndF1Spec>;
#[doc = "Register `DSP_VS_ST_END_F1` writer"]
pub type W = crate::W<DspVsStEndF1Spec>;
#[doc = "Field `DSP_VS_END_F1` reader - Panel display scanning vertical vsync end point of 2nd\n\nfield(interlace display mode)"]
pub type DspVsEndF1R = crate::FieldReader<u16>;
#[doc = "Field `DSP_VS_END_F1` writer - Panel display scanning vertical vsync end point of 2nd\n\nfield(interlace display mode)"]
pub type DspVsEndF1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_VS_ST_F1` reader - Panel display scanning vertical vsync start point of 2nd field\n\n(interlace display mode)"]
pub type DspVsStF1R = crate::FieldReader<u16>;
#[doc = "Field `DSP_VS_ST_F1` writer - Panel display scanning vertical vsync start point of 2nd field\n\n(interlace display mode)"]
pub type DspVsStF1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning vertical vsync end point of 2nd\n\nfield(interlace display mode)"]
    #[inline(always)]
    pub fn dsp_vs_end_f1(&self) -> DspVsEndF1R {
        DspVsEndF1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical vsync start point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    pub fn dsp_vs_st_f1(&self) -> DspVsStF1R {
        DspVsStF1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning vertical vsync end point of 2nd\n\nfield(interlace display mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vs_end_f1(&mut self) -> DspVsEndF1W<DspVsStEndF1Spec> {
        DspVsEndF1W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning vertical vsync start point of 2nd field\n\n(interlace display mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vs_st_f1(&mut self) -> DspVsStF1W<DspVsStEndF1Spec> {
        DspVsStF1W::new(self, 16)
    }
}
#[doc = "Vertical scanning start point and vsync pulse end point of even filed in interlace mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_vs_st_end_f1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_vs_st_end_f1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspVsStEndF1Spec;
impl crate::RegisterSpec for DspVsStEndF1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_vs_st_end_f1::R`](R) reader structure"]
impl crate::Readable for DspVsStEndF1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsp_vs_st_end_f1::W`](W) writer structure"]
impl crate::Writable for DspVsStEndF1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_VS_ST_END_F1 to value 0"]
impl crate::Resettable for DspVsStEndF1Spec {
    const RESET_VALUE: u32 = 0;
}
