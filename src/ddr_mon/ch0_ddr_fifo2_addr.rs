#[doc = "Register `CH0_DDR_FIFO2_ADDR` reader"]
pub type R = crate::R<Ch0DdrFifo2AddrSpec>;
#[doc = "Field `CH0_DDR_FIFO2_ADDR` reader - Channel 0 DDR controller interface address FIFO2"]
pub type Ch0DdrFifo2AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 DDR controller interface address FIFO2"]
    #[inline(always)]
    pub fn ch0_ddr_fifo2_addr(&self) -> Ch0DdrFifo2AddrR {
        Ch0DdrFifo2AddrR::new(self.bits)
    }
}
#[doc = "DDR Channel 0 Controller Interface Address FIFO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ddr_fifo2_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0DdrFifo2AddrSpec;
impl crate::RegisterSpec for Ch0DdrFifo2AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_ddr_fifo2_addr::R`](R) reader structure"]
impl crate::Readable for Ch0DdrFifo2AddrSpec {}
#[doc = "`reset()` method sets CH0_DDR_FIFO2_ADDR to value 0"]
impl crate::Resettable for Ch0DdrFifo2AddrSpec {
    const RESET_VALUE: u32 = 0;
}
