#[doc = "Register `WIN2_DSP_ST2` reader"]
pub type R = crate::R<Win2DspSt2Spec>;
#[doc = "Register `WIN2_DSP_ST2` writer"]
pub type W = crate::W<Win2DspSt2Spec>;
#[doc = "Field `WIN2_DSP_XST2` reader - Win2 horizontal start point(x) of the Panel scanning"]
pub type Win2DspXst2R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_XST2` writer - Win2 horizontal start point(x) of the Panel scanning"]
pub type Win2DspXst2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN2_DSP_YST2` reader - Win2 vertical start point(y) of the Panel scanning"]
pub type Win2DspYst2R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_YST2` writer - Win2 vertical start point(y) of the Panel scanning"]
pub type Win2DspYst2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win2 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    pub fn win2_dsp_xst2(&self) -> Win2DspXst2R {
        Win2DspXst2R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win2 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    pub fn win2_dsp_yst2(&self) -> Win2DspYst2R {
        Win2DspYst2R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win2 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_xst2(&mut self) -> Win2DspXst2W<Win2DspSt2Spec> {
        Win2DspXst2W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win2 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_yst2(&mut self) -> Win2DspYst2W<Win2DspSt2Spec> {
        Win2DspYst2W::new(self, 16)
    }
}
#[doc = "Win2 display start point2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_st2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_st2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2DspSt2Spec;
impl crate::RegisterSpec for Win2DspSt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_dsp_st2::R`](R) reader structure"]
impl crate::Readable for Win2DspSt2Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_dsp_st2::W`](W) writer structure"]
impl crate::Writable for Win2DspSt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_DSP_ST2 to value 0x000a_000a"]
impl crate::Resettable for Win2DspSt2Spec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
