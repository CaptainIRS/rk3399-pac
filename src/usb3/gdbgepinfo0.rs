#[doc = "Register `GDBGEPINFO0` reader"]
pub type R = crate::R<Gdbgepinfo0Spec>;
#[doc = "Field `EPDEBUG` reader - Endpoint Debug Information Low 32-bit\n\nEndpoint Debug Information Low 32-bit"]
pub type EpdebugR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Debug Information Low 32-bit\n\nEndpoint Debug Information Low 32-bit"]
    #[inline(always)]
    pub fn epdebug(&self) -> EpdebugR {
        EpdebugR::new(self.bits)
    }
}
#[doc = "Global Debug Endpoint Information Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgepinfo0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gdbgepinfo0Spec;
impl crate::RegisterSpec for Gdbgepinfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdbgepinfo0::R`](R) reader structure"]
impl crate::Readable for Gdbgepinfo0Spec {}
#[doc = "`reset()` method sets GDBGEPINFO0 to value 0"]
impl crate::Resettable for Gdbgepinfo0Spec {
    const RESET_VALUE: u32 = 0;
}
