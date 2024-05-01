#[doc = "Register `SRC_FG_COLOR` reader"]
pub type R = crate::R<SrcFgColorSpec>;
#[doc = "Register `SRC_FG_COLOR` writer"]
pub type W = crate::W<SrcFgColorSpec>;
#[doc = "Field `SW_SRC_FG_COLOR` reader - Source image foreground color\n\nSource image foreground color\n\n('1' bit color for mono expansion.)\n\nColor fill color, Pan color"]
pub type SwSrcFgColorR = crate::FieldReader<u32>;
#[doc = "Field `SW_SRC_FG_COLOR` writer - Source image foreground color\n\nSource image foreground color\n\n('1' bit color for mono expansion.)\n\nColor fill color, Pan color"]
pub type SwSrcFgColorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image foreground color\n\nSource image foreground color\n\n('1' bit color for mono expansion.)\n\nColor fill color, Pan color"]
    #[inline(always)]
    pub fn sw_src_fg_color(&self) -> SwSrcFgColorR {
        SwSrcFgColorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image foreground color\n\nSource image foreground color\n\n('1' bit color for mono expansion.)\n\nColor fill color, Pan color"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_fg_color(&mut self) -> SwSrcFgColorW<SrcFgColorSpec> {
        SwSrcFgColorW::new(self, 0)
    }
}
#[doc = "RGA source image foreground color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_fg_color::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_fg_color::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcFgColorSpec;
impl crate::RegisterSpec for SrcFgColorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_fg_color::R`](R) reader structure"]
impl crate::Readable for SrcFgColorSpec {}
#[doc = "`write(|w| ..)` method takes [`src_fg_color::W`](W) writer structure"]
impl crate::Writable for SrcFgColorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_FG_COLOR to value 0"]
impl crate::Resettable for SrcFgColorSpec {
    const RESET_VALUE: u32 = 0;
}
