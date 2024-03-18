#[doc = "Register `EFUSE_RF` reader"]
pub type R = crate::R<EfuseRfSpec>;
#[doc = "Field `EFUSE_RF_R` reader - efuse redundancy bit used indicator register for RF3~RF0, Output high once the redundancy bit has been used."]
pub type EfuseRfRR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - efuse redundancy bit used indicator register for RF3~RF0, Output high once the redundancy bit has been used."]
    #[inline(always)]
    pub fn efuse_rf_r(&self) -> EfuseRfRR {
        EfuseRfRR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "e fuse redundancy bit used indicator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_rf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseRfSpec;
impl crate::RegisterSpec for EfuseRfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_rf::R`](R) reader structure"]
impl crate::Readable for EfuseRfSpec {}
#[doc = "`reset()` method sets EFUSE_RF to value 0"]
impl crate::Resettable for EfuseRfSpec {
    const RESET_VALUE: u32 = 0;
}
