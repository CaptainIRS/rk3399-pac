#[doc = "Register `UART_SRBR` reader"]
pub type R = crate::R<UartSrbrSpec>;
#[doc = "Field `SHADOW_RBR` reader - This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO are preserved, but any incoming data is lost. An overrun error also occurs."]
pub type ShadowRbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO are preserved, but any incoming data is lost. An overrun error also occurs."]
    #[inline(always)]
    pub fn shadow_rbr(&self) -> ShadowRbrR {
        ShadowRbrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Shadow Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_srbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSrbrSpec;
impl crate::RegisterSpec for UartSrbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_srbr::R`](R) reader structure"]
impl crate::Readable for UartSrbrSpec {}
#[doc = "`reset()` method sets UART_SRBR to value 0"]
impl crate::Resettable for UartSrbrSpec {
    const RESET_VALUE: u32 = 0;
}
