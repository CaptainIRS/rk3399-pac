#[doc = "Register `FC_VSDIEEEID0` reader"]
pub type R = crate::R<FcVsdieeeid0Spec>;
#[doc = "Register `FC_VSDIEEEID0` writer"]
pub type W = crate::W<FcVsdieeeid0Spec>;
#[doc = "Field `IEEE` reader - This register configures the Vendor Specific InfoFrame IEEE registration identifier. For more information, refer to the CEA-861-E specification."]
pub type IeeeR = crate::FieldReader;
#[doc = "Field `IEEE` writer - This register configures the Vendor Specific InfoFrame IEEE registration identifier. For more information, refer to the CEA-861-E specification."]
pub type IeeeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register configures the Vendor Specific InfoFrame IEEE registration identifier. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    pub fn ieee(&self) -> IeeeR {
        IeeeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register configures the Vendor Specific InfoFrame IEEE registration identifier. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn ieee(&mut self) -> IeeeW<FcVsdieeeid0Spec> {
        IeeeW::new(self, 0)
    }
}
#[doc = "This register configures the Vendor Specific InfoFrame IEEE registration identifier. For more information, refer to the CEA-861-E specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdieeeid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdieeeid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsdieeeid0Spec;
impl crate::RegisterSpec for FcVsdieeeid0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsdieeeid0::R`](R) reader structure"]
impl crate::Readable for FcVsdieeeid0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsdieeeid0::W`](W) writer structure"]
impl crate::Writable for FcVsdieeeid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSDIEEEID0 to value 0"]
impl crate::Resettable for FcVsdieeeid0Spec {
    const RESET_VALUE: u8 = 0;
}
