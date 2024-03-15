#[doc = "Register `DDRMON_CH1_DDR_FIFO1_ADDR` reader"]
pub type R = crate::R<DdrmonCh1DdrFifo1AddrSpec>;
#[doc = "Field `CH1_DDR_FIFO1_ADDR` reader - Channel 1 DDR controller interface address FIFO1"]
pub type Ch1DdrFifo1AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 DDR controller interface address FIFO1"]
    #[inline(always)]
    pub fn ch1_ddr_fifo1_addr(&self) -> Ch1DdrFifo1AddrR {
        Ch1DdrFifo1AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 1 Controller Interface Address FIFO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_ddr_fifo1_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh1DdrFifo1AddrSpec;
impl crate::RegisterSpec for DdrmonCh1DdrFifo1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch1_ddr_fifo1_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh1DdrFifo1AddrSpec {}
#[doc = "`reset()` method sets DDRMON_CH1_DDR_FIFO1_ADDR to value 0"]
impl crate::Resettable for DdrmonCh1DdrFifo1AddrSpec {
    const RESET_VALUE: u32 = 0;
}
