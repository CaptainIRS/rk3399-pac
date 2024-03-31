#[doc = "Register `UCV` reader"]
pub type R = crate::R<UcvSpec>;
#[doc = "Field `VER` reader - ASCII value for each number in the version"]
pub type VerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ASCII value for each number in the version"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(self.bits)
    }
}
#[doc = "UART Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcvSpec;
impl crate::RegisterSpec for UcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucv::R`](R) reader structure"]
impl crate::Readable for UcvSpec {}
#[doc = "`reset()` method sets UCV to value 0x0330_372a"]
impl crate::Resettable for UcvSpec {
    const RESET_VALUE: u32 = 0x0330_372a;
}
