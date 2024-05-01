#[doc = "Register `WIN3_DSP_INFO3` reader"]
pub type R = crate::R<Win3DspInfo3Spec>;
#[doc = "Register `WIN3_DSP_INFO3` writer"]
pub type W = crate::W<Win3DspInfo3Spec>;
#[doc = "Field `WIN3_DSP_WIDTH3` reader - Win3 display window width3\n\nwin3_dsp_width = size -1"]
pub type Win3DspWidth3R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_WIDTH3` writer - Win3 display window width3\n\nwin3_dsp_width = size -1"]
pub type Win3DspWidth3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN3_DSP_HEIGHT3` reader - Win3 display window height3\n\nwin3_dsp_height0 = size -1"]
pub type Win3DspHeight3R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_HEIGHT3` writer - Win3 display window height3\n\nwin3_dsp_height0 = size -1"]
pub type Win3DspHeight3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win3 display window width3\n\nwin3_dsp_width = size -1"]
    #[inline(always)]
    pub fn win3_dsp_width3(&self) -> Win3DspWidth3R {
        Win3DspWidth3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win3 display window height3\n\nwin3_dsp_height0 = size -1"]
    #[inline(always)]
    pub fn win3_dsp_height3(&self) -> Win3DspHeight3R {
        Win3DspHeight3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win3 display window width3\n\nwin3_dsp_width = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_width3(&mut self) -> Win3DspWidth3W<Win3DspInfo3Spec> {
        Win3DspWidth3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win3 display window height3\n\nwin3_dsp_height0 = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_height3(&mut self) -> Win3DspHeight3W<Win3DspInfo3Spec> {
        Win3DspHeight3W::new(self, 16)
    }
}
#[doc = "Win3 display width3/height3 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3DspInfo3Spec;
impl crate::RegisterSpec for Win3DspInfo3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_dsp_info3::R`](R) reader structure"]
impl crate::Readable for Win3DspInfo3Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_dsp_info3::W`](W) writer structure"]
impl crate::Writable for Win3DspInfo3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_DSP_INFO3 to value 0x00ef_013f"]
impl crate::Resettable for Win3DspInfo3Spec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
