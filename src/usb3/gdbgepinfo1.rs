#[doc = "Register `GDBGEPINFO1` reader"]
pub type R = crate::R<Gdbgepinfo1Spec>;
#[doc = "Field `EPDEBUG` reader - Endpoint Debug Information High 32-bit\n\nEndpoint Debug Information High 32-bit"]
pub type EpdebugR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Debug Information High 32-bit\n\nEndpoint Debug Information High 32-bit"]
    #[inline(always)]
    pub fn epdebug(&self) -> EpdebugR {
        EpdebugR::new(self.bits)
    }
}
#[doc = "Global Debug Endpoint Information Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgepinfo1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gdbgepinfo1Spec;
impl crate::RegisterSpec for Gdbgepinfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdbgepinfo1::R`](R) reader structure"]
impl crate::Readable for Gdbgepinfo1Spec {}
#[doc = "`reset()` method sets GDBGEPINFO1 to value 0x0080_0000"]
impl crate::Resettable for Gdbgepinfo1Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
