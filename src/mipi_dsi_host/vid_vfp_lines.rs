#[doc = "Register `VID_VFP_LINES` reader"]
pub type R = crate::R<VidVfpLinesSpec>;
#[doc = "Register `VID_VFP_LINES` writer"]
pub type W = crate::W<VidVfpLinesSpec>;
#[doc = "Field `VFP_LINES` reader - vfp_lines\n\nThis field configures the Vertical Front Porch period measured in\n\nnumber of horizontal lines."]
pub type VfpLinesR = crate::FieldReader<u16>;
#[doc = "Field `VFP_LINES` writer - vfp_lines\n\nThis field configures the Vertical Front Porch period measured in\n\nnumber of horizontal lines."]
pub type VfpLinesW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - vfp_lines\n\nThis field configures the Vertical Front Porch period measured in\n\nnumber of horizontal lines."]
    #[inline(always)]
    pub fn vfp_lines(&self) -> VfpLinesR {
        VfpLinesR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - vfp_lines\n\nThis field configures the Vertical Front Porch period measured in\n\nnumber of horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn vfp_lines(&mut self) -> VfpLinesW<VidVfpLinesSpec> {
        VfpLinesW::new(self, 0)
    }
}
#[doc = "Vertical Front Porch Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vfp_lines::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vfp_lines::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVfpLinesSpec;
impl crate::RegisterSpec for VidVfpLinesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vfp_lines::R`](R) reader structure"]
impl crate::Readable for VidVfpLinesSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_vfp_lines::W`](W) writer structure"]
impl crate::Writable for VidVfpLinesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_VFP_LINES to value 0"]
impl crate::Resettable for VidVfpLinesSpec {
    const RESET_VALUE: u32 = 0;
}
