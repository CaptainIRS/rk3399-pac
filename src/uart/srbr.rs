#[doc = "Register `SRBR` reader"]
pub type R = crate::R<SrbrSpec>;
#[doc = "Field `SHADOW_RBR` reader - This is a shadow register for the RBR and has been allocated\n\nsixteen 32-bit locations so as to accommodate burst accesses\n\nfrom the master. This register contains the data byte received on\n\nthe serial input port (sin) in UART mode or the serial infrared\n\ninput (sir_in) in infrared mode. The data in this register is valid\n\nonly if the Data Ready (DR) bit in the Line status Register (LSR)\n\nis set.\n\nIf FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR\n\nmust be read before the next data arrives, otherwise it is\n\noverwritten, resulting in an overrun error.\n\nIf FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses\n\nthe head of the receive FIFO. If the receive FIFO is full and this\n\nregister is not read before the next data character arrives, then\n\nthe data already in the FIFO are preserved, but any incoming\n\ndata is lost. An overrun error also occurs."]
pub type ShadowRbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This is a shadow register for the RBR and has been allocated\n\nsixteen 32-bit locations so as to accommodate burst accesses\n\nfrom the master. This register contains the data byte received on\n\nthe serial input port (sin) in UART mode or the serial infrared\n\ninput (sir_in) in infrared mode. The data in this register is valid\n\nonly if the Data Ready (DR) bit in the Line status Register (LSR)\n\nis set.\n\nIf FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR\n\nmust be read before the next data arrives, otherwise it is\n\noverwritten, resulting in an overrun error.\n\nIf FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses\n\nthe head of the receive FIFO. If the receive FIFO is full and this\n\nregister is not read before the next data character arrives, then\n\nthe data already in the FIFO are preserved, but any incoming\n\ndata is lost. An overrun error also occurs."]
    #[inline(always)]
    pub fn shadow_rbr(&self) -> ShadowRbrR {
        ShadowRbrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Shadow Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrbrSpec;
impl crate::RegisterSpec for SrbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srbr::R`](R) reader structure"]
impl crate::Readable for SrbrSpec {}
#[doc = "`reset()` method sets SRBR to value 0"]
impl crate::Resettable for SrbrSpec {
    const RESET_VALUE: u32 = 0;
}
