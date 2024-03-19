#[doc = "Register `A_COREVERMSB` reader"]
pub type R = crate::R<ACorevermsbSpec>;
#[doc = "Field `A_COREVERMSB` reader - HDCP Controller Version Register MSB"]
pub type ACorevermsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - HDCP Controller Version Register MSB"]
    #[inline(always)]
    pub fn a_corevermsb(&self) -> ACorevermsbR {
        ACorevermsbR::new(self.bits)
    }
}
#[doc = "HDCP Controller Version Register MSB Revision ID number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_corevermsb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACorevermsbSpec;
impl crate::RegisterSpec for ACorevermsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_corevermsb::R`](R) reader structure"]
impl crate::Readable for ACorevermsbSpec {}
#[doc = "`reset()` method sets A_COREVERMSB to value 0x03"]
impl crate::Resettable for ACorevermsbSpec {
    const RESET_VALUE: u8 = 0x03;
}
