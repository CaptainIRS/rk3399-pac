#[doc = "Register `WIN0_DSP_INFO` reader"]
pub type R = crate::R<Win0DspInfoSpec>;
#[doc = "Register `WIN0_DSP_INFO` writer"]
pub type W = crate::W<Win0DspInfoSpec>;
#[doc = "Field `WIN0_DSP_WIDTH` reader - Win0 display window width\n\nwin0_dsp_width = (win0 horizontial size -1)"]
pub type Win0DspWidthR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_DSP_WIDTH` writer - Win0 display window width\n\nwin0_dsp_width = (win0 horizontial size -1)"]
pub type Win0DspWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WIN0_DSP_HEIGHT` reader - Win0 display window height\n\nwin0_dsp_height = (win0 vertical size -1)"]
pub type Win0DspHeightR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_DSP_HEIGHT` writer - Win0 display window height\n\nwin0_dsp_height = (win0 vertical size -1)"]
pub type Win0DspHeightW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Win0 display window width\n\nwin0_dsp_width = (win0 horizontial size -1)"]
    #[inline(always)]
    pub fn win0_dsp_width(&self) -> Win0DspWidthR {
        Win0DspWidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Win0 display window height\n\nwin0_dsp_height = (win0 vertical size -1)"]
    #[inline(always)]
    pub fn win0_dsp_height(&self) -> Win0DspHeightR {
        Win0DspHeightR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Win0 display window width\n\nwin0_dsp_width = (win0 horizontial size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_width(&mut self) -> Win0DspWidthW<Win0DspInfoSpec> {
        Win0DspWidthW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Win0 display window height\n\nwin0_dsp_height = (win0 vertical size -1)"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dsp_height(&mut self) -> Win0DspHeightW<Win0DspInfoSpec> {
        Win0DspHeightW::new(self, 16)
    }
}
#[doc = "Win0 display width/height on panel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dsp_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dsp_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0DspInfoSpec;
impl crate::RegisterSpec for Win0DspInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_dsp_info::R`](R) reader structure"]
impl crate::Readable for Win0DspInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_dsp_info::W`](W) writer structure"]
impl crate::Writable for Win0DspInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_DSP_INFO to value 0x00ef_013f"]
impl crate::Resettable for Win0DspInfoSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}
