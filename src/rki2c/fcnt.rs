#[doc = "Register `FCNT` reader"]
pub type R = crate::R<FcntSpec>;
#[doc = "Field `FCNT` reader - finished count\n\nthe count of data which has been transmitted or received\n\nfor debug purpose"]
pub type FcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - finished count\n\nthe count of data which has been transmitted or received\n\nfor debug purpose"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "finished count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcntSpec;
impl crate::RegisterSpec for FcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcnt::R`](R) reader structure"]
impl crate::Readable for FcntSpec {}
#[doc = "`reset()` method sets FCNT to value 0"]
impl crate::Resettable for FcntSpec {
    const RESET_VALUE: u32 = 0;
}
