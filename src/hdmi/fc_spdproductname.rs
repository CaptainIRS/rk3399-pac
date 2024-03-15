#[doc = "Register `FC_SPDPRODUCTNAME[%s]` reader"]
pub type R = crate::R<FcSpdproductnameSpec>;
#[doc = "Register `FC_SPDPRODUCTNAME[%s]` writer"]
pub type W = crate::W<FcSpdproductnameSpec>;
#[doc = "Field `FC_SPDPRODUCTNAME` reader - Frame Composer SPD packet Data Product Name Register Array Configures the Source Product Descriptor infoFrame 16 bytes Product name. For more information, refer to the CEA-861-E specification."]
pub type FcSpdproductnameR = crate::FieldReader;
#[doc = "Field `FC_SPDPRODUCTNAME` writer - Frame Composer SPD packet Data Product Name Register Array Configures the Source Product Descriptor infoFrame 16 bytes Product name. For more information, refer to the CEA-861-E specification."]
pub type FcSpdproductnameW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer SPD packet Data Product Name Register Array Configures the Source Product Descriptor infoFrame 16 bytes Product name. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    pub fn fc_spdproductname(&self) -> FcSpdproductnameR {
        FcSpdproductnameR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer SPD packet Data Product Name Register Array Configures the Source Product Descriptor infoFrame 16 bytes Product name. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_spdproductname(&mut self) -> FcSpdproductnameW<FcSpdproductnameSpec> {
        FcSpdproductnameW::new(self, 0)
    }
}
#[doc = "Frame Composer SPD packet Data Product Name Register Array Configures the Source Product Descriptor infoFrame 16 bytes Product name. For more information, refer to the CEA-861-E specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spdproductname::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spdproductname::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcSpdproductnameSpec;
impl crate::RegisterSpec for FcSpdproductnameSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_spdproductname::R`](R) reader structure"]
impl crate::Readable for FcSpdproductnameSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_spdproductname::W`](W) writer structure"]
impl crate::Writable for FcSpdproductnameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_SPDPRODUCTNAME[%s]
to value 0"]
impl crate::Resettable for FcSpdproductnameSpec {
    const RESET_VALUE: u8 = 0;
}
