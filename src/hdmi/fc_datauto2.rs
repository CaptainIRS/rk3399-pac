#[doc = "Register `FC_DATAUTO2` reader"]
pub type R = crate::R<FcDatauto2Spec>;
#[doc = "Register `FC_DATAUTO2` writer"]
pub type W = crate::W<FcDatauto2Spec>;
#[doc = "Field `AUTO_LINE_SPACING` reader - Packets line spacing, for automatic packet scheduling"]
pub type AutoLineSpacingR = crate::FieldReader;
#[doc = "Field `AUTO_LINE_SPACING` writer - Packets line spacing, for automatic packet scheduling"]
pub type AutoLineSpacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AUTO_FRAME_PACKETS` reader - Packets per frame, for automatic packet scheduling"]
pub type AutoFramePacketsR = crate::FieldReader;
#[doc = "Field `AUTO_FRAME_PACKETS` writer - Packets per frame, for automatic packet scheduling"]
pub type AutoFramePacketsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Packets line spacing, for automatic packet scheduling"]
    #[inline(always)]
    pub fn auto_line_spacing(&self) -> AutoLineSpacingR {
        AutoLineSpacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Packets per frame, for automatic packet scheduling"]
    #[inline(always)]
    pub fn auto_frame_packets(&self) -> AutoFramePacketsR {
        AutoFramePacketsR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Packets line spacing, for automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn auto_line_spacing(&mut self) -> AutoLineSpacingW<FcDatauto2Spec> {
        AutoLineSpacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Packets per frame, for automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn auto_frame_packets(&mut self) -> AutoFramePacketsW<FcDatauto2Spec> {
        AutoFramePacketsW::new(self, 4)
    }
}
#[doc = "Packets line spacing, for automatic packet scheduling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDatauto2Spec;
impl crate::RegisterSpec for FcDatauto2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_datauto2::R`](R) reader structure"]
impl crate::Readable for FcDatauto2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_datauto2::W`](W) writer structure"]
impl crate::Writable for FcDatauto2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DATAUTO2 to value 0"]
impl crate::Resettable for FcDatauto2Spec {
    const RESET_VALUE: u8 = 0;
}
