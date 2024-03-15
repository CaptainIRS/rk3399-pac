#[doc = "Register `DDRMON_CH0_DDR_FIFO0_ADDR` reader"]
pub type R = crate::R<DdrmonCh0DdrFifo0AddrSpec>;
#[doc = "Field `CH0_DDR_FIFO0_ADDR` reader - Channel 0 DDR controller interface address FIFO0"]
pub type Ch0DdrFifo0AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 DDR controller interface address FIFO0"]
    #[inline(always)]
    pub fn ch0_ddr_fifo0_addr(&self) -> Ch0DdrFifo0AddrR {
        Ch0DdrFifo0AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 0 Controller Interface Address FIFO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_ddr_fifo0_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh0DdrFifo0AddrSpec;
impl crate::RegisterSpec for DdrmonCh0DdrFifo0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch0_ddr_fifo0_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh0DdrFifo0AddrSpec {}
#[doc = "`reset()` method sets DDRMON_CH0_DDR_FIFO0_ADDR to value 0"]
impl crate::Resettable for DdrmonCh0DdrFifo0AddrSpec {
    const RESET_VALUE: u32 = 0;
}
