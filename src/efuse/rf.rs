#[doc = "Register `RF` reader"]
pub type R = crate::R<RfSpec>;
#[doc = "Field `EFUSE_RF_R` reader - efuse redundancy bit used indicator register for RF3~RF0, Output\n\nhigh once the redundancy bit has been used."]
pub type EfuseRfRR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - efuse redundancy bit used indicator register for RF3~RF0, Output\n\nhigh once the redundancy bit has been used."]
    #[inline(always)]
    pub fn efuse_rf_r(&self) -> EfuseRfRR {
        EfuseRfRR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "e fuse redundancy bit used indicator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfSpec;
impl crate::RegisterSpec for RfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf::R`](R) reader structure"]
impl crate::Readable for RfSpec {}
#[doc = "`reset()` method sets RF to value 0"]
impl crate::Resettable for RfSpec {
    const RESET_VALUE: u32 = 0;
}
