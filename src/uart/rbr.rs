#[doc = "Register `RBR` reader"]
pub type R = crate::R<RbrSpec>;
#[doc = "Register `RBR` writer"]
pub type W = crate::W<RbrSpec>;
#[doc = "Field `DATA_INPUT` reader - Data byte received on the serial input port (sin) in UART mode, or\n\nthe serial infrared input (sir_in) in infrared mode. The data in this\n\nregister is valid only if the Data Ready (DR) bit in the Line Status\n\nRegister (LCR) is set.\n\nIf in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are\n\ndisabled (FCR\\[0\\]
set to zero), the data in the RBR must be read\n\nbefore the next data arrives, otherwise it is overwritten, resulting\n\nin an over-run error.\n\nIf in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled\n\n(FCR\\[0\\]
set to one), this register accesses the head of the receive\n\nFIFO.\n\nIf the receive FIFO is full and this register is not read before the\n\nnext data character arrives, then the data already in the FIFO is\n\npreserved, but any incoming data are lost and an\n\nover-run error occurs."]
pub type DataInputR = crate::FieldReader;
#[doc = "Field `DATA_INPUT` writer - Data byte received on the serial input port (sin) in UART mode, or\n\nthe serial infrared input (sir_in) in infrared mode. The data in this\n\nregister is valid only if the Data Ready (DR) bit in the Line Status\n\nRegister (LCR) is set.\n\nIf in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are\n\ndisabled (FCR\\[0\\]
set to zero), the data in the RBR must be read\n\nbefore the next data arrives, otherwise it is overwritten, resulting\n\nin an over-run error.\n\nIf in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled\n\n(FCR\\[0\\]
set to one), this register accesses the head of the receive\n\nFIFO.\n\nIf the receive FIFO is full and this register is not read before the\n\nnext data character arrives, then the data already in the FIFO is\n\npreserved, but any incoming data are lost and an\n\nover-run error occurs."]
pub type DataInputW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte received on the serial input port (sin) in UART mode, or\n\nthe serial infrared input (sir_in) in infrared mode. The data in this\n\nregister is valid only if the Data Ready (DR) bit in the Line Status\n\nRegister (LCR) is set.\n\nIf in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are\n\ndisabled (FCR\\[0\\]
set to zero), the data in the RBR must be read\n\nbefore the next data arrives, otherwise it is overwritten, resulting\n\nin an over-run error.\n\nIf in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled\n\n(FCR\\[0\\]
set to one), this register accesses the head of the receive\n\nFIFO.\n\nIf the receive FIFO is full and this register is not read before the\n\nnext data character arrives, then the data already in the FIFO is\n\npreserved, but any incoming data are lost and an\n\nover-run error occurs."]
    #[inline(always)]
    pub fn data_input(&self) -> DataInputR {
        DataInputR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte received on the serial input port (sin) in UART mode, or\n\nthe serial infrared input (sir_in) in infrared mode. The data in this\n\nregister is valid only if the Data Ready (DR) bit in the Line Status\n\nRegister (LCR) is set.\n\nIf in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are\n\ndisabled (FCR\\[0\\]
set to zero), the data in the RBR must be read\n\nbefore the next data arrives, otherwise it is overwritten, resulting\n\nin an over-run error.\n\nIf in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled\n\n(FCR\\[0\\]
set to one), this register accesses the head of the receive\n\nFIFO.\n\nIf the receive FIFO is full and this register is not read before the\n\nnext data character arrives, then the data already in the FIFO is\n\npreserved, but any incoming data are lost and an\n\nover-run error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn data_input(&mut self) -> DataInputW<RbrSpec> {
        DataInputW::new(self, 0)
    }
}
#[doc = "Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrSpec;
impl crate::RegisterSpec for RbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RbrSpec {}
#[doc = "`write(|w| ..)` method takes [`rbr::W`](W) writer structure"]
impl crate::Writable for RbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RbrSpec {
    const RESET_VALUE: u32 = 0;
}
