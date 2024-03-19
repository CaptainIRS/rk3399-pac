#[doc = "Register `UART_IIR` reader"]
pub type R = crate::R<UartIirSpec>;
#[doc = "Field `INT_ID` reader - Interrupt ID\n\nThis indicates the highest priority pending interrupt which can be\n\none of the following types:\n\n0000 = modem status\n\n0001 = no interrupt pending\n\n0010 = THR empty\n\n0100 = received data available\n\n0110 = receiver line status\n\n0111 = busy detect\n\n1100 = character timeout"]
pub type IntIdR = crate::FieldReader;
#[doc = "Field `FIFOS_EN` reader - FIFOs Enabled.\n\nThis is used to indicate whether the FIFOs are enabled or\n\ndisabled.\n\n00 = disabled\n\n11 = enabled"]
pub type FifosEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Interrupt ID\n\nThis indicates the highest priority pending interrupt which can be\n\none of the following types:\n\n0000 = modem status\n\n0001 = no interrupt pending\n\n0010 = THR empty\n\n0100 = received data available\n\n0110 = receiver line status\n\n0111 = busy detect\n\n1100 = character timeout"]
    #[inline(always)]
    pub fn int_id(&self) -> IntIdR {
        IntIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enabled.\n\nThis is used to indicate whether the FIFOs are enabled or\n\ndisabled.\n\n00 = disabled\n\n11 = enabled"]
    #[inline(always)]
    pub fn fifos_en(&self) -> FifosEnR {
        FifosEnR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_iir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIirSpec;
impl crate::RegisterSpec for UartIirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_iir::R`](R) reader structure"]
impl crate::Readable for UartIirSpec {}
#[doc = "`reset()` method sets UART_IIR to value 0"]
impl crate::Resettable for UartIirSpec {
    const RESET_VALUE: u32 = 0;
}
