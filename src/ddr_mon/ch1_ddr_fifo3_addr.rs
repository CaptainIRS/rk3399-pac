#[doc = "Register `CH1_DDR_FIFO3_ADDR` reader"]
pub type R = crate::R<Ch1DdrFifo3AddrSpec>;
#[doc = "Field `CH1_DDR_FIFO3_ADDR` reader - Channel 1 DDR controller interface address FIFO3"]
pub type Ch1DdrFifo3AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 DDR controller interface address FIFO3"]
    #[inline(always)]
    pub fn ch1_ddr_fifo3_addr(&self) -> Ch1DdrFifo3AddrR {
        Ch1DdrFifo3AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 1 Controller Interface Address FIFO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo3_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DdrFifo3AddrSpec;
impl crate::RegisterSpec for Ch1DdrFifo3AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_ddr_fifo3_addr::R`](R) reader structure"]
impl crate::Readable for Ch1DdrFifo3AddrSpec {}
#[doc = "`reset()` method sets CH1_DDR_FIFO3_ADDR to value 0"]
impl crate::Resettable for Ch1DdrFifo3AddrSpec {
    const RESET_VALUE: u32 = 0;
}
