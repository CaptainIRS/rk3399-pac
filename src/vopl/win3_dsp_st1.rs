#[doc = "Register `WIN3_DSP_ST1` reader"]
pub type R = crate::R<Win3DspSt1Spec>;
#[doc = "Register `WIN3_DSP_ST1` writer"]
pub type W = crate::W<Win3DspSt1Spec>;
#[doc = "Field `WIN3_DSP_XST1` reader - Win3 horizontal start point(x) of the Panel scanning"]
pub type Win3DspXst1R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_XST1` writer - Win3 horizontal start point(x) of the Panel scanning"]
pub type Win3DspXst1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN3_DSP_YST1` reader - Win3 vertical start point(y) of the Panel scanning"]
pub type Win3DspYst1R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_YST1` writer - Win3 vertical start point(y) of the Panel scanning"]
pub type Win3DspYst1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win3 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    pub fn win3_dsp_xst1(&self) -> Win3DspXst1R {
        Win3DspXst1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win3 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    pub fn win3_dsp_yst1(&self) -> Win3DspYst1R {
        Win3DspYst1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win3 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_xst1(&mut self) -> Win3DspXst1W<Win3DspSt1Spec> {
        Win3DspXst1W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win3 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_yst1(&mut self) -> Win3DspYst1W<Win3DspSt1Spec> {
        Win3DspYst1W::new(self, 16)
    }
}
#[doc = "Win3 display start point1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_st1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_st1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3DspSt1Spec;
impl crate::RegisterSpec for Win3DspSt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_dsp_st1::R`](R) reader structure"]
impl crate::Readable for Win3DspSt1Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_dsp_st1::W`](W) writer structure"]
impl crate::Writable for Win3DspSt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_DSP_ST1 to value 0x000a_000a"]
impl crate::Resettable for Win3DspSt1Spec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
