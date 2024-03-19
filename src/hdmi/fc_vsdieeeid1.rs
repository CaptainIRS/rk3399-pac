#[doc = "Register `FC_VSDIEEEID1` reader"]
pub type R = crate::R<FcVsdieeeid1Spec>;
#[doc = "Register `FC_VSDIEEEID1` writer"]
pub type W = crate::W<FcVsdieeeid1Spec>;
#[doc = "Field `IEEE` reader - This register configures the Vendor Specific\n\nInfoFrame IEEE registration identifier. For more\n\ninformation, refer to the CEA-861-E specification."]
pub type IeeeR = crate::FieldReader;
#[doc = "Field `IEEE` writer - This register configures the Vendor Specific\n\nInfoFrame IEEE registration identifier. For more\n\ninformation, refer to the CEA-861-E specification."]
pub type IeeeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register configures the Vendor Specific\n\nInfoFrame IEEE registration identifier. For more\n\ninformation, refer to the CEA-861-E specification."]
    #[inline(always)]
    pub fn ieee(&self) -> IeeeR {
        IeeeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register configures the Vendor Specific\n\nInfoFrame IEEE registration identifier. For more\n\ninformation, refer to the CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn ieee(&mut self) -> IeeeW<FcVsdieeeid1Spec> {
        IeeeW::new(self, 0)
    }
}
#[doc = "Frame Composer VSI Packet Data IEEE Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdieeeid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdieeeid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsdieeeid1Spec;
impl crate::RegisterSpec for FcVsdieeeid1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsdieeeid1::R`](R) reader structure"]
impl crate::Readable for FcVsdieeeid1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsdieeeid1::W`](W) writer structure"]
impl crate::Writable for FcVsdieeeid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSDIEEEID1 to value 0"]
impl crate::Resettable for FcVsdieeeid1Spec {
    const RESET_VALUE: u8 = 0;
}
