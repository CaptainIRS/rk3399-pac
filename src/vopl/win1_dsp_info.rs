#[doc = "Register `WIN1_DSP_INFO` reader"]
pub type R = crate::R<Win1DspInfoSpec>;
#[doc = "Register `WIN1_DSP_INFO` writer"]
pub type W = crate::W<Win1DspInfoSpec>;
#[doc = "Field `WIN1_DSP_WIDTH` reader - Win1 display window width\n\nwin1_dsp_width = (win1 horizontial size -1)"]
pub type Win1DspWidthR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_DSP_WIDTH` writer - Win1 display window width\n\nwin1_dsp_width = (win1 horizontial size -1)"]
pub type Win1DspWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN1_DSP_HEIGHT` reader - Win1 display window height\n\nwin1_dsp_height = (win1 vertical size -1)"]
pub type Win1DspHeightR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_DSP_HEIGHT` writer - Win1 display window height\n\nwin1_dsp_height = (win1 vertical size -1)"]
pub type Win1DspHeightW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win1 display window width\n\nwin1_dsp_width = (win1 horizontial size -1)"]
    #[inline(always)]
    pub fn win1_dsp_width(&self) -> Win1DspWidthR {
        Win1DspWidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win1 display window height\n\nwin1_dsp_height = (win1 vertical size -1)"]
    #[inline(always)]
    pub fn win1_dsp_height(&self) -> Win1DspHeightR {
        Win1DspHeightR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win1 display window width\n\nwin1_dsp_width = (win1 horizontial size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_width(&mut self) -> Win1DspWidthW<Win1DspInfoSpec> {
        Win1DspWidthW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win1 display window height\n\nwin1_dsp_height = (win1 vertical size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dsp_height(&mut self) -> Win1DspHeightW<Win1DspInfoSpec> {
        Win1DspHeightW::new(self, 16)
    }
}
#[doc = "Win1 display width/height on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dsp_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dsp_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1DspInfoSpec;
impl crate::RegisterSpec for Win1DspInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_dsp_info::R`](R) reader structure"]
impl crate::Readable for Win1DspInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_dsp_info::W`](W) writer structure"]
impl crate::Writable for Win1DspInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_DSP_INFO to value 0x00ef_013f"]
impl crate::Resettable for Win1DspInfoSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
