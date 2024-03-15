#[doc = "Register `DDRMON_CH0_DDR_FIFO3_ADDR` reader"]
pub type R = crate::R<DdrmonCh0DdrFifo3AddrSpec>;
#[doc = "Field `CH0_DDR_FIFO3_ADDR` reader - Channel 0 DDR controller interface address FIFO3"]
pub type Ch0DdrFifo3AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 DDR controller interface address FIFO3"]
    #[inline(always)]
    pub fn ch0_ddr_fifo3_addr(&self) -> Ch0DdrFifo3AddrR {
        Ch0DdrFifo3AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 0 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo3_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh0DdrFifo3AddrSpec;
impl crate::RegisterSpec for DdrmonCh0DdrFifo3AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch0_ddr_fifo3_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh0DdrFifo3AddrSpec {}
#[doc = "`reset()` method sets DDRMON_CH0_DDR_FIFO3_ADDR to value 0"]
impl crate::Resettable for DdrmonCh0DdrFifo3AddrSpec {
    const RESET_VALUE: u32 = 0;
}
