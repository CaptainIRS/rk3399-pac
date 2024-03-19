#[doc = "Register `USB3_GDBGLSP` reader"]
pub type R = crate::R<Usb3GdbglspSpec>;
#[doc = "Field `LSPDEBUG` reader - LSP Debug Information\n\nLSP Debug Information"]
pub type LspdebugR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - LSP Debug Information\n\nLSP Debug Information"]
    #[inline(always)]
    pub fn lspdebug(&self) -> LspdebugR {
        LspdebugR::new(self.bits)
    }
}
#[doc = "Global Debug LSP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglsp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdbglspSpec;
impl crate::RegisterSpec for Usb3GdbglspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdbglsp::R`](R) reader structure"]
impl crate::Readable for Usb3GdbglspSpec {}
#[doc = "`reset()` method sets USB3_GDBGLSP to value 0"]
impl crate::Resettable for Usb3GdbglspSpec {
    const RESET_VALUE: u32 = 0;
}
