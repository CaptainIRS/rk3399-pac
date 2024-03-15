#[doc = "Register `EFUSE_DOUT` reader"]
pub type R = crate::R<EfuseDoutSpec>;
#[doc = "Field `EFUSE_DOUT` reader - eFuse data output"]
pub type EfuseDoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFuse data output"]
    #[inline(always)]
    pub fn efuse_dout(&self) -> EfuseDoutR {
        EfuseDoutR::new(self.bits)
    }
}
#[doc = "e fuse data out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_dout::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseDoutSpec;
impl crate::RegisterSpec for EfuseDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_dout::R`](R) reader structure"]
impl crate::Readable for EfuseDoutSpec {}
#[doc = "`reset()` method sets EFUSE_DOUT to value 0"]
impl crate::Resettable for EfuseDoutSpec {
    const RESET_VALUE: u32 = 0;
}
