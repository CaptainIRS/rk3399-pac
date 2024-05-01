#[doc = "Register `VID_VACTIVE_LINES` reader"]
pub type R = crate::R<VidVactiveLinesSpec>;
#[doc = "Register `VID_VACTIVE_LINES` writer"]
pub type W = crate::W<VidVactiveLinesSpec>;
#[doc = "Field `V_ACTIVE_LINES` reader - v_active_lines\n\nThis field configures the Vertical Active period measured in number\n\nof horizontal lines."]
pub type VActiveLinesR = crate::FieldReader<u16>;
#[doc = "Field `V_ACTIVE_LINES` writer - v_active_lines\n\nThis field configures the Vertical Active period measured in number\n\nof horizontal lines."]
pub type VActiveLinesW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - v_active_lines\n\nThis field configures the Vertical Active period measured in number\n\nof horizontal lines."]
    #[inline(always)]
    pub fn v_active_lines(&self) -> VActiveLinesR {
        VActiveLinesR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - v_active_lines\n\nThis field configures the Vertical Active period measured in number\n\nof horizontal lines."]
    #[inline(always)]
    #[must_use]
    pub fn v_active_lines(&mut self) -> VActiveLinesW<VidVactiveLinesSpec> {
        VActiveLinesW::new(self, 0)
    }
}
#[doc = "Vertical Resolution Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vactive_lines::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vactive_lines::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidVactiveLinesSpec;
impl crate::RegisterSpec for VidVactiveLinesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vactive_lines::R`](R) reader structure"]
impl crate::Readable for VidVactiveLinesSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_vactive_lines::W`](W) writer structure"]
impl crate::Writable for VidVactiveLinesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_VACTIVE_LINES to value 0"]
impl crate::Resettable for VidVactiveLinesSpec {
    const RESET_VALUE: u32 = 0;
}
