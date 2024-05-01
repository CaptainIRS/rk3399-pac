#[doc = "Register `HWC_DSP_ST` reader"]
pub type R = crate::R<HwcDspStSpec>;
#[doc = "Register `HWC_DSP_ST` writer"]
pub type W = crate::W<HwcDspStSpec>;
#[doc = "Field `HWC_DSP_XST` reader - HWC horizontal start point(x) of the Panel scanning"]
pub type HwcDspXstR = crate::FieldReader<u16>;
#[doc = "Field `HWC_DSP_XST` writer - HWC horizontal start point(x) of the Panel scanning"]
pub type HwcDspXstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HWC_DSP_YST` reader - HWC vertical start point(y) of the Panel scanning"]
pub type HwcDspYstR = crate::FieldReader<u16>;
#[doc = "Field `HWC_DSP_YST` writer - HWC vertical start point(y) of the Panel scanning"]
pub type HwcDspYstW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - HWC horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    pub fn hwc_dsp_xst(&self) -> HwcDspXstR {
        HwcDspXstR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - HWC vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    pub fn hwc_dsp_yst(&self) -> HwcDspYstR {
        HwcDspYstR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - HWC horizontal start point(x) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_dsp_xst(&mut self) -> HwcDspXstW<HwcDspStSpec> {
        HwcDspXstW::new(self, 0)
    }
    #[doc = "Bits 16:28 - HWC vertical start point(y) of the Panel scanning"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_dsp_yst(&mut self) -> HwcDspYstW<HwcDspStSpec> {
        HwcDspYstW::new(self, 16)
    }
}
#[doc = "Hwc display start point on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_dsp_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_dsp_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcDspStSpec;
impl crate::RegisterSpec for HwcDspStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_dsp_st::R`](R) reader structure"]
impl crate::Readable for HwcDspStSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_dsp_st::W`](W) writer structure"]
impl crate::Writable for HwcDspStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_DSP_ST to value 0x000a_000a"]
impl crate::Resettable for HwcDspStSpec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
