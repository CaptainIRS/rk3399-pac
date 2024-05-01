#[doc = "Register `WIN3_DSP_INFO2` reader"]
pub type R = crate::R<Win3DspInfo2Spec>;
#[doc = "Register `WIN3_DSP_INFO2` writer"]
pub type W = crate::W<Win3DspInfo2Spec>;
#[doc = "Field `WIN3_DSP_WIDTH2` reader - Win3 display window width2\n\nwin3_dsp_width = size -1"]
pub type Win3DspWidth2R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_WIDTH2` writer - Win3 display window width2\n\nwin3_dsp_width = size -1"]
pub type Win3DspWidth2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN3_DSP_HEIGHT2` reader - Win3 display window height2\n\nwin3_dsp_height0 = size -1"]
pub type Win3DspHeight2R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_DSP_HEIGHT2` writer - Win3 display window height2\n\nwin3_dsp_height0 = size -1"]
pub type Win3DspHeight2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win3 display window width2\n\nwin3_dsp_width = size -1"]
    #[inline(always)]
    pub fn win3_dsp_width2(&self) -> Win3DspWidth2R {
        Win3DspWidth2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win3 display window height2\n\nwin3_dsp_height0 = size -1"]
    #[inline(always)]
    pub fn win3_dsp_height2(&self) -> Win3DspHeight2R {
        Win3DspHeight2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win3 display window width2\n\nwin3_dsp_width = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_width2(&mut self) -> Win3DspWidth2W<Win3DspInfo2Spec> {
        Win3DspWidth2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win3 display window height2\n\nwin3_dsp_height0 = size -1"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dsp_height2(&mut self) -> Win3DspHeight2W<Win3DspInfo2Spec> {
        Win3DspHeight2W::new(self, 16)
    }
}
#[doc = "Win3 display width2/height2 on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_dsp_info2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_dsp_info2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3DspInfo2Spec;
impl crate::RegisterSpec for Win3DspInfo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_dsp_info2::R`](R) reader structure"]
impl crate::Readable for Win3DspInfo2Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_dsp_info2::W`](W) writer structure"]
impl crate::Writable for Win3DspInfo2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_DSP_INFO2 to value 0x00ef_013f"]
impl crate::Resettable for Win3DspInfo2Spec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
