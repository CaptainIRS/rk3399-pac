#[doc = "Register `RKI2C_FCNT` reader"]
pub type R = crate::R<Rki2cFcntSpec>;
#[doc = "Field `FCNT` reader - finished count the count of data which has been transmitted or received for debug purpose"]
pub type FcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - finished count the count of data which has been transmitted or received for debug purpose"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "finished count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_fcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cFcntSpec;
impl crate::RegisterSpec for Rki2cFcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_fcnt::R`](R) reader structure"]
impl crate::Readable for Rki2cFcntSpec {}
#[doc = "`reset()` method sets RKI2C_FCNT to value 0"]
impl crate::Resettable for Rki2cFcntSpec {
    const RESET_VALUE: u32 = 0;
}
