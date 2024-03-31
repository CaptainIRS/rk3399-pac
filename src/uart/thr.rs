#[doc = "Register `THR` reader"]
pub type R = crate::R<ThrSpec>;
#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `DATA_OUTPUT` reader - Data to be transmitted on the serial output port (sout) in UART\n\nmode or the serial infrared output (sir_out_n) in infrared mode.\n\nData should only be written to the THR when the THR Empty\n\n(THRE) bit (LSR\\[5\\]) is set.\n\nIf in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
= 0) and\n\nTHRE is set, writing a single character to the THR clears the\n\nTHRE. Any additional writes to the THR before the THRE is set\n\nagain causes the THR data to be overwritten.\n\nIf in FIFO mode and FIFOs are enabled (FCR\\[0\\]
= 1) and THRE is\n\nset, x number of characters of data may be written to the THR\n\nbefore the FIFO is full. The number x (default=16) is determined\n\nby the value of FIFO Depth that you set during configuration. Any\n\nattempt to write data when the FIFO is full results in the write\n\ndata being lost."]
pub type DataOutputR = crate::FieldReader;
#[doc = "Field `DATA_OUTPUT` writer - Data to be transmitted on the serial output port (sout) in UART\n\nmode or the serial infrared output (sir_out_n) in infrared mode.\n\nData should only be written to the THR when the THR Empty\n\n(THRE) bit (LSR\\[5\\]) is set.\n\nIf in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
= 0) and\n\nTHRE is set, writing a single character to the THR clears the\n\nTHRE. Any additional writes to the THR before the THRE is set\n\nagain causes the THR data to be overwritten.\n\nIf in FIFO mode and FIFOs are enabled (FCR\\[0\\]
= 1) and THRE is\n\nset, x number of characters of data may be written to the THR\n\nbefore the FIFO is full. The number x (default=16) is determined\n\nby the value of FIFO Depth that you set during configuration. Any\n\nattempt to write data when the FIFO is full results in the write\n\ndata being lost."]
pub type DataOutputW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data to be transmitted on the serial output port (sout) in UART\n\nmode or the serial infrared output (sir_out_n) in infrared mode.\n\nData should only be written to the THR when the THR Empty\n\n(THRE) bit (LSR\\[5\\]) is set.\n\nIf in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
= 0) and\n\nTHRE is set, writing a single character to the THR clears the\n\nTHRE. Any additional writes to the THR before the THRE is set\n\nagain causes the THR data to be overwritten.\n\nIf in FIFO mode and FIFOs are enabled (FCR\\[0\\]
= 1) and THRE is\n\nset, x number of characters of data may be written to the THR\n\nbefore the FIFO is full. The number x (default=16) is determined\n\nby the value of FIFO Depth that you set during configuration. Any\n\nattempt to write data when the FIFO is full results in the write\n\ndata being lost."]
    #[inline(always)]
    pub fn data_output(&self) -> DataOutputR {
        DataOutputR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to be transmitted on the serial output port (sout) in UART\n\nmode or the serial infrared output (sir_out_n) in infrared mode.\n\nData should only be written to the THR when the THR Empty\n\n(THRE) bit (LSR\\[5\\]) is set.\n\nIf in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
= 0) and\n\nTHRE is set, writing a single character to the THR clears the\n\nTHRE. Any additional writes to the THR before the THRE is set\n\nagain causes the THR data to be overwritten.\n\nIf in FIFO mode and FIFOs are enabled (FCR\\[0\\]
= 1) and THRE is\n\nset, x number of characters of data may be written to the THR\n\nbefore the FIFO is full. The number x (default=16) is determined\n\nby the value of FIFO Depth that you set during configuration. Any\n\nattempt to write data when the FIFO is full results in the write\n\ndata being lost."]
    #[inline(always)]
    #[must_use]
    pub fn data_output(&mut self) -> DataOutputW<ThrSpec> {
        DataOutputW::new(self, 0)
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thr::R`](R) reader structure"]
impl crate::Readable for ThrSpec {}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {
    const RESET_VALUE: u32 = 0;
}
