#[doc = "Register `WIN2_DSP_INFO0` reader"]
pub type R = crate::R<Win2DspInfo0Spec>;
#[doc = "Register `WIN2_DSP_INFO0` writer"]
pub type W = crate::W<Win2DspInfo0Spec>;
#[doc = "Field `WIN2_DSP_WIDTH0` reader - Win2 display window width0\n\nwin2_dsp_width = size -1"]
pub type Win2DspWidth0R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_WIDTH0` writer - Win2 display window width0\n\nwin2_dsp_width = size -1"]
pub type Win2DspWidth0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN2_DSP_HEIGHT0` reader - Win2 display window height0\n\nwin2_dsp_height0 = size -1"]
pub type Win2DspHeight0R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_DSP_HEIGHT0` writer - Win2 display window height0\n\nwin2_dsp_height0 = size -1"]
pub type Win2DspHeight0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win2 display window width0\n\nwin2_dsp_width = size -1"]
    #[inline(always)]
    pub fn win2_dsp_width0(&self) -> Win2DspWidth0R {
        Win2DspWidth0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win2 display window height0\n\nwin2_dsp_height0 = size -1"]
    #[inline(always)]
    pub fn win2_dsp_height0(&self) -> Win2DspHeight0R {
        Win2DspHeight0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win2 display window width0\n\nwin2_dsp_width = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_width0(&mut self) -> Win2DspWidth0W<Win2DspInfo0Spec> {
        Win2DspWidth0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win2 display window height0\n\nwin2_dsp_height0 = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dsp_height0(&mut self) -> Win2DspHeight0W<Win2DspInfo0Spec> {
        Win2DspHeight0W::new(self, 16)
    }
}
#[doc = "Win2 display width0/height0 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_dsp_info0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_dsp_info0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2DspInfo0Spec;
impl crate::RegisterSpec for Win2DspInfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_dsp_info0::R`](R) reader structure"]
impl crate::Readable for Win2DspInfo0Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_dsp_info0::W`](W) writer structure"]
impl crate::Writable for Win2DspInfo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_DSP_INFO0 to value 0x00ef_013f"]
impl crate::Resettable for Win2DspInfo0Spec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
