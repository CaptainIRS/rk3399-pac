#[doc = "Register `FC_SPDVENDORNAME[%s]` reader"]
pub type R = crate::R<FcSpdvendornameSpec>;
#[doc = "Register `FC_SPDVENDORNAME[%s]` writer"]
pub type W = crate::W<FcSpdvendornameSpec>;
#[doc = "Field `FC_SPDVENDORNAME` reader - Frame Composer SPD Packet Data Vendor Name Register Array Configures the Source Product Descriptor infoFrame 8 bytes Vendor name. For more information, refer to the CEA-861-E specification."]
pub type FcSpdvendornameR = crate::FieldReader;
#[doc = "Field `FC_SPDVENDORNAME` writer - Frame Composer SPD Packet Data Vendor Name Register Array Configures the Source Product Descriptor infoFrame 8 bytes Vendor name. For more information, refer to the CEA-861-E specification."]
pub type FcSpdvendornameW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer SPD Packet Data Vendor Name Register Array Configures the Source Product Descriptor infoFrame 8 bytes Vendor name. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    pub fn fc_spdvendorname(&self) -> FcSpdvendornameR {
        FcSpdvendornameR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer SPD Packet Data Vendor Name Register Array Configures the Source Product Descriptor infoFrame 8 bytes Vendor name. For more information, refer to the CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_spdvendorname(&mut self) -> FcSpdvendornameW<FcSpdvendornameSpec> {
        FcSpdvendornameW::new(self, 0)
    }
}
#[doc = "Frame Composer SPD Packet Data Vendor Name Register Array Configures the Source Product Descriptor infoFrame 8 bytes Vendor name. For more information, refer to the CEA-861-E specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spdvendorname::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spdvendorname::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcSpdvendornameSpec;
impl crate::RegisterSpec for FcSpdvendornameSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_spdvendorname::R`](R) reader structure"]
impl crate::Readable for FcSpdvendornameSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_spdvendorname::W`](W) writer structure"]
impl crate::Writable for FcSpdvendornameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_SPDVENDORNAME[%s]
to value 0"]
impl crate::Resettable for FcSpdvendornameSpec {
    const RESET_VALUE: u8 = 0;
}
