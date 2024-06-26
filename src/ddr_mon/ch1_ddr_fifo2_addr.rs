#[doc = "Register `CH1_DDR_FIFO2_ADDR` reader"]
pub type R = crate::R<Ch1DdrFifo2AddrSpec>;
#[doc = "Field `CH1_DDR_FIFO2_ADDR` reader - Channel 1 DDR controller interface address FIFO2"]
pub type Ch1DdrFifo2AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 DDR controller interface address FIFO2"]
    #[inline(always)]
    pub fn ch1_ddr_fifo2_addr(&self) -> Ch1DdrFifo2AddrR {
        Ch1DdrFifo2AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 1 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ddr_fifo2_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DdrFifo2AddrSpec;
impl crate::RegisterSpec for Ch1DdrFifo2AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_ddr_fifo2_addr::R`](R) reader structure"]
impl crate::Readable for Ch1DdrFifo2AddrSpec {}
#[doc = "`reset()` method sets CH1_DDR_FIFO2_ADDR to value 0"]
impl crate::Resettable for Ch1DdrFifo2AddrSpec {
    const RESET_VALUE: u32 = 0;
}
