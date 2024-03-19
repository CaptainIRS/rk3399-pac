#[doc = "Register `FC_VSYNCINWIDTH` reader"]
pub type R = crate::R<FcVsyncinwidthSpec>;
#[doc = "Register `FC_VSYNCINWIDTH` writer"]
pub type W = crate::W<FcVsyncinwidthSpec>;
#[doc = "Field `V_IN_WIDTH` reader - Description: Input video Vsync active pulse width.\n\nInteger number of video lines \\[0...63\\]."]
pub type VInWidthR = crate::FieldReader;
#[doc = "Field `V_IN_WIDTH` writer - Description: Input video Vsync active pulse width.\n\nInteger number of video lines \\[0...63\\]."]
pub type VInWidthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Description: Input video Vsync active pulse width.\n\nInteger number of video lines \\[0...63\\]."]
    #[inline(always)]
    pub fn v_in_width(&self) -> VInWidthR {
        VInWidthR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Description: Input video Vsync active pulse width.\n\nInteger number of video lines \\[0...63\\]."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_width(&mut self) -> VInWidthW<FcVsyncinwidthSpec> {
        VInWidthW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video VSync Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsyncinwidth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsyncinwidth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsyncinwidthSpec;
impl crate::RegisterSpec for FcVsyncinwidthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsyncinwidth::R`](R) reader structure"]
impl crate::Readable for FcVsyncinwidthSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsyncinwidth::W`](W) writer structure"]
impl crate::Writable for FcVsyncinwidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSYNCINWIDTH to value 0"]
impl crate::Resettable for FcVsyncinwidthSpec {
    const RESET_VALUE: u8 = 0;
}
