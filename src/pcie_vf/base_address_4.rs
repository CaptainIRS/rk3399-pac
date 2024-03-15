#[doc = "Register `BASE_ADDRESS_4` reader"]
pub type R = crate::R<BaseAddress4Spec>;
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
#[doc = "Base Address Register 4 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddress4Spec;
impl crate::RegisterSpec for BaseAddress4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_address_4::R`](R) reader structure"]
impl crate::Readable for BaseAddress4Spec {}
#[doc = "`reset()` method sets BASE_ADDRESS_4 to value 0"]
impl crate::Resettable for BaseAddress4Spec {
    const RESET_VALUE: u32 = 0;
}
