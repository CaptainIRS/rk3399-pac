#[doc = "Register `DSP_BG` reader"]
pub type R = crate::R<DspBgSpec>;
#[doc = "Register `DSP_BG` writer"]
pub type W = crate::W<DspBgSpec>;
#[doc = "Field `DSP_BG_BLUE` reader - Background Blue color\n\n8bit blue color"]
pub type DspBgBlueR = crate::FieldReader;
#[doc = "Field `DSP_BG_BLUE` writer - Background Blue color\n\n8bit blue color"]
pub type DspBgBlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP_BG_GREEN` reader - Background Green color\n\n8bit green color"]
pub type DspBgGreenR = crate::FieldReader;
#[doc = "Field `DSP_BG_GREEN` writer - Background Green color\n\n8bit green color"]
pub type DspBgGreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DSP_BG_RED` reader - Background Red color\n\n8bit red color"]
pub type DspBgRedR = crate::FieldReader;
#[doc = "Field `DSP_BG_RED` writer - Background Red color\n\n8bit red color"]
pub type DspBgRedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Background Blue color\n\n8bit blue color"]
    #[inline(always)]
    pub fn dsp_bg_blue(&self) -> DspBgBlueR {
        DspBgBlueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background Green color\n\n8bit green color"]
    #[inline(always)]
    pub fn dsp_bg_green(&self) -> DspBgGreenR {
        DspBgGreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background Red color\n\n8bit red color"]
    #[inline(always)]
    pub fn dsp_bg_red(&self) -> DspBgRedR {
        DspBgRedR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background Blue color\n\n8bit blue color"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_bg_blue(&mut self) -> DspBgBlueW<DspBgSpec> {
        DspBgBlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Background Green color\n\n8bit green color"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_bg_green(&mut self) -> DspBgGreenW<DspBgSpec> {
        DspBgGreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Background Red color\n\n8bit red color"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_bg_red(&mut self) -> DspBgRedW<DspBgSpec> {
        DspBgRedW::new(self, 16)
    }
}
#[doc = "Background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspBgSpec;
impl crate::RegisterSpec for DspBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_bg::R`](R) reader structure"]
impl crate::Readable for DspBgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_bg::W`](W) writer structure"]
impl crate::Writable for DspBgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_BG to value 0"]
impl crate::Resettable for DspBgSpec {
    const RESET_VALUE: u32 = 0;
}
