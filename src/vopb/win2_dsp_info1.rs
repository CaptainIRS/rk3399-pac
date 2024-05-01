#[doc = "Register `WIN2_DSP_INFO1` reader"]
pub type R = crate::R<Win2DspInfo1Spec>;
#[doc = "Register `WIN2_DSP_INFO1` writer"]
pub type W = crate::W<Win2DspInfo1Spec>;
#[doc = "Field `WIN2_DSP_WIDTH1` reader - Win2 display window width1\n\nwin2_dsp_width = size -1"]
pub type Win2DspWidth1R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_WIDTH1` writer - Win2 display window width1\n\nwin2_dsp_width = size -1"]
pub type Win2DspWidth1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN2_DSP_HEIGHT1` reader - Win2 display window height1\n\nwin2_dsp_height0 = size -1"]
pub type Win2DspHeight1R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_HEIGHT1` writer - Win2 display window height1\n\nwin2_dsp_height0 = size -1"]
pub type Win2DspHeight1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win2 display window width1\n\nwin2_dsp_width = size -1"]
    #[inline(always)]
    pub fn win2_dsp_width1(&self) -> Win2DspWidth1R {
        Win2DspWidth1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win2 display window height1\n\nwin2_dsp_height0 = size -1"]
    #[inline(always)]
    pub fn win2_dsp_height1(&self) -> Win2DspHeight1R {
        Win2DspHeight1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win2 display window width1\n\nwin2_dsp_width = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_width1(&mut self) -> Win2DspWidth1W<Win2DspInfo1Spec> {
        Win2DspWidth1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win2 display window height1\n\nwin2_dsp_height0 = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_height1(&mut self) -> Win2DspHeight1W<Win2DspInfo1Spec> {
        Win2DspHeight1W::new(self, 16)
    }
}
#[doc = "Win2 display width1/height1 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2DspInfo1Spec;
impl crate::RegisterSpec for Win2DspInfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_dsp_info1::R`](R) reader structure"]
impl crate::Readable for Win2DspInfo1Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_dsp_info1::W`](W) writer structure"]
impl crate::Writable for Win2DspInfo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_DSP_INFO1 to value 0x00ef_013f"]
impl crate::Resettable for Win2DspInfo1Spec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
