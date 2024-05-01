#[doc = "Register `WIN0_DSP_ST` reader"]
pub type R = crate::R<Win0DspStSpec>;
#[doc = "Register `WIN0_DSP_ST` writer"]
pub type W = crate::W<Win0DspStSpec>;
#[doc = "Field `WIN0_DSP_XST` reader - Win0 horizontal start point(x) of the Panel scanning"]
pub type Win0DspXstR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_DSP_XST` writer - Win0 horizontal start point(x) of the Panel scanning"]
pub type Win0DspXstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WIN0_DSP_YST` reader - Win0 vertical start point(y) of the Panel scanning"]
pub type Win0DspYstR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_DSP_YST` writer - Win0 vertical start point(y) of the Panel scanning"]
pub type Win0DspYstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Win0 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    pub fn win0_dsp_xst(&self) -> Win0DspXstR {
        Win0DspXstR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Win0 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    pub fn win0_dsp_yst(&self) -> Win0DspYstR {
        Win0DspYstR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Win0 horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_xst(&mut self) -> Win0DspXstW<Win0DspStSpec> {
        Win0DspXstW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Win0 vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_yst(&mut self) -> Win0DspYstW<Win0DspStSpec> {
        Win0DspYstW::new(self, 16)
    }
}
#[doc = "Win0 display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0DspStSpec;
impl crate::RegisterSpec for Win0DspStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_dsp_st::R`](R) reader structure"]
impl crate::Readable for Win0DspStSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_dsp_st::W`](W) writer structure"]
impl crate::Writable for Win0DspStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_DSP_ST to value 0x000a_000a"]
impl crate::Resettable for Win0DspStSpec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
