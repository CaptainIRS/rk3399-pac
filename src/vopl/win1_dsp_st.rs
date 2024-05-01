#[doc = "Register `WIN1_DSP_ST` reader"]
pub type R = crate::R<Win1DspStSpec>;
#[doc = "Register `WIN1_DSP_ST` writer"]
pub type W = crate::W<Win1DspStSpec>;
#[doc = "Field `WIN1_DSP_XST` reader - Win1 horizontal start point(x) of the Panel scanning"]
pub type Win1DspXstR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_DSP_XST` writer - Win1 horizontal start point(x) of the Panel scanning"]
pub type Win1DspXstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN1_DSP_YST` reader - Win1 vertical start point(y) of the Panel scanning"]
pub type Win1DspYstR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_DSP_YST` writer - Win1 vertical start point(y) of the Panel scanning"]
pub type Win1DspYstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win1 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    pub fn win1_dsp_xst(&self) -> Win1DspXstR {
        Win1DspXstR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win1 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    pub fn win1_dsp_yst(&self) -> Win1DspYstR {
        Win1DspYstR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win1 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_xst(&mut self) -> Win1DspXstW<Win1DspStSpec> {
        Win1DspXstW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win1 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_yst(&mut self) -> Win1DspYstW<Win1DspStSpec> {
        Win1DspYstW::new(self, 16)
    }
}
#[doc = "Win1 display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1DspStSpec;
impl crate::RegisterSpec for Win1DspStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_dsp_st::R`](R) reader structure"]
impl crate::Readable for Win1DspStSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_dsp_st::W`](W) writer structure"]
impl crate::Writable for Win1DspStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_DSP_ST to value 0x000a_000a"]
impl crate::Resettable for Win1DspStSpec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
