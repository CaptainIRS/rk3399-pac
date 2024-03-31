#[doc = "Register `DOUT` reader"]
pub type R = crate::R<DoutSpec>;
#[doc = "Field `EFUSE_DOUT` reader - eFuse data output"]
pub type EfuseDoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFuse data output"]
    #[inline(always)]
    pub fn efuse_dout(&self) -> EfuseDoutR {
        EfuseDoutR::new(self.bits)
    }
}
#[doc = "e fuse data out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutSpec;
impl crate::RegisterSpec for DoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout::R`](R) reader structure"]
impl crate::Readable for DoutSpec {}
#[doc = "`reset()` method sets DOUT to value 0"]
impl crate::Resettable for DoutSpec {
    const RESET_VALUE: u32 = 0;
}
