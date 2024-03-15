#[doc = "Register `FC_VSDPAYLOAD[%s]` reader"]
pub type R = crate::R<FcVsdpayloadSpec>;
#[doc = "Register `FC_VSDPAYLOAD[%s]` writer"]
pub type W = crate::W<FcVsdpayloadSpec>;
#[doc = "Field `FC_VSDPAYLOAD` reader - Frame Composer VSI Packet Data Payload Register Array Configures the Vendor Specific infoFrame 24 bytes specific payload. For more information, refer to the CEA-861-E specification."]
pub type FcVsdpayloadR = crate::FieldReader;
#[doc = "Field `FC_VSDPAYLOAD` writer - Frame Composer VSI Packet Data Payload Register Array Configures the Vendor Specific infoFrame 24 bytes specific payload. For more information, refer to the CEA-861-E specification."]
pub type FcVsdpayloadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer VSI Packet Data Payload Register Array Configures the Vendor Specific infoFrame 24 bytes specific payload. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    pub fn fc_vsdpayload(&self) -> FcVsdpayloadR {
        FcVsdpayloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer VSI Packet Data Payload Register Array Configures the Vendor Specific infoFrame 24 bytes specific payload. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_vsdpayload(&mut self) -> FcVsdpayloadW<FcVsdpayloadSpec> {
        FcVsdpayloadW::new(self, 0)
    }
}
#[doc = "Frame Composer VSI Packet Data Payload Register Array Configures the Vendor Specific infoFrame 24 bytes specific payload. For more information, refer to the CEA-861-E specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsdpayload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsdpayload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsdpayloadSpec;
impl crate::RegisterSpec for FcVsdpayloadSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsdpayload::R`](R) reader structure"]
impl crate::Readable for FcVsdpayloadSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsdpayload::W`](W) writer structure"]
impl crate::Writable for FcVsdpayloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSDPAYLOAD[%s]
to value 0"]
impl crate::Resettable for FcVsdpayloadSpec {
    const RESET_VALUE: u8 = 0;
}
