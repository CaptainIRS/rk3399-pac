#[doc = "Register `VID_VBP_LINES` reader"]
pub type R = crate::R<VidVbpLinesSpec>;
#[doc = "Register `VID_VBP_LINES` writer"]
pub type W = crate::W<VidVbpLinesSpec>;
#[doc = "Field `VBP_LINES` reader - vbp_lines\n\nThis field configures the Vertical Back Porch period measured in\n\nnumber of horizontal lines."]
pub type VbpLinesR = crate::FieldReader<u16>;
#[doc = "Field `VBP_LINES` writer - vbp_lines\n\nThis field configures the Vertical Back Porch period measured in\n\nnumber of horizontal lines."]
pub type VbpLinesW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - vbp_lines\n\nThis field configures the Vertical Back Porch period measured in\n\nnumber of horizontal lines."]
    #[inline(always)]
    pub fn vbp_lines(&self) -> VbpLinesR {
        VbpLinesR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - vbp_lines\n\nThis field configures the Vertical Back Porch period measured in\n\nnumber of horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn vbp_lines(&mut self) -> VbpLinesW<VidVbpLinesSpec> {
        VbpLinesW::new(self, 0)
    }
}
#[doc = "Vertical Back Porch Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vbp_lines::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVbpLinesSpec;
impl crate::RegisterSpec for VidVbpLinesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vbp_lines::R`](R) reader structure"]
impl crate::Readable for VidVbpLinesSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_vbp_lines::W`](W) writer structure"]
impl crate::Writable for VidVbpLinesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_VBP_LINES to value 0"]
impl crate::Resettable for VidVbpLinesSpec {
    const RESET_VALUE: u32 = 0;
}
