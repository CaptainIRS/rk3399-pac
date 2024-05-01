#[doc = "Register `SRC_BG_COLOR` reader"]
pub type R = crate::R<SrcBgColorSpec>;
#[doc = "Register `SRC_BG_COLOR` writer"]
pub type W = crate::W<SrcBgColorSpec>;
#[doc = "Field `SW_SRC_BG_COLOR` reader - Source image background color\n\n('0' bit color for mono expansion.)"]
pub type SwSrcBgColorR = crate::FieldReader<u32>;
#[doc = "Field `SW_SRC_BG_COLOR` writer - Source image background color\n\n('0' bit color for mono expansion.)"]
pub type SwSrcBgColorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image background color\n\n('0' bit color for mono expansion.)"]
    #[inline(always)]
    pub fn sw_src_bg_color(&self) -> SwSrcBgColorR {
        SwSrcBgColorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image background color\n\n('0' bit color for mono expansion.)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_bg_color(&mut self) -> SwSrcBgColorW<SrcBgColorSpec> {
        SwSrcBgColorW::new(self, 0)
    }
}
#[doc = "RGA source image background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_bg_color::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_bg_color::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBgColorSpec;
impl crate::RegisterSpec for SrcBgColorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_bg_color::R`](R) reader structure"]
impl crate::Readable for SrcBgColorSpec {}
#[doc = "`write(|w| ..)` method takes [`src_bg_color::W`](W) writer structure"]
impl crate::Writable for SrcBgColorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_BG_COLOR to value 0"]
impl crate::Resettable for SrcBgColorSpec {
    const RESET_VALUE: u32 = 0;
}
