#[doc = "Register `A_COREVERLSB` reader"]
pub type R = crate::R<ACoreverlsbSpec>;
#[doc = "Field `A_COREVERLSB` reader - HDCP Controller Version Register LSB"]
pub type ACoreverlsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - HDCP Controller Version Register LSB"]
    #[inline(always)]
    pub fn a_coreverlsb(&self) -> ACoreverlsbR {
        ACoreverlsbR::new(self.bits)
    }
}
#[doc = "HDCP Controller Version Register LSB Design ID number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_coreverlsb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACoreverlsbSpec;
impl crate::RegisterSpec for ACoreverlsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_coreverlsb::R`](R) reader structure"]
impl crate::Readable for ACoreverlsbSpec {}
#[doc = "`reset()` method sets A_COREVERLSB to value 0x02"]
impl crate::Resettable for ACoreverlsbSpec {
    const RESET_VALUE: u8 = 0x02;
}
