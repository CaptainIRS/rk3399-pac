#[doc = "Register `BASE_ADDRESS_2` reader"]
pub type R = crate::R<BaseAddress2Spec>;
#[doc = "Field `NI` reader - Not Implemented \\[NI\\]
(no description)"]
pub type NiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not Implemented \\[NI\\]
(no description)"]
    #[inline(always)]
    pub fn ni(&self) -> NiR {
        NiR::new(self.bits)
    }
}
#[doc = "Base Address Register 2 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddress2Spec;
impl crate::RegisterSpec for BaseAddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_address_2::R`](R) reader structure"]
impl crate::Readable for BaseAddress2Spec {}
#[doc = "`reset()` method sets BASE_ADDRESS_2 to value 0"]
impl crate::Resettable for BaseAddress2Spec {
    const RESET_VALUE: u32 = 0;
}
