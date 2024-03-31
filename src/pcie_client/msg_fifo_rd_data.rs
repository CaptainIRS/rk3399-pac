#[doc = "Register `MSG_FIFO_RD_DATA` reader"]
pub type R = crate::R<MsgFifoRdDataSpec>;
#[doc = "Field `RD_DATA` reader - Message fifo read data\n\nMessage fifo read data"]
pub type RdDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Message fifo read data\n\nMessage fifo read data"]
    #[inline(always)]
    pub fn rd_data(&self) -> RdDataR {
        RdDataR::new(self.bits)
    }
}
#[doc = "Message fifo read data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_fifo_rd_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgFifoRdDataSpec;
impl crate::RegisterSpec for MsgFifoRdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg_fifo_rd_data::R`](R) reader structure"]
impl crate::Readable for MsgFifoRdDataSpec {}
#[doc = "`reset()` method sets MSG_FIFO_RD_DATA to value 0"]
impl crate::Resettable for MsgFifoRdDataSpec {
    const RESET_VALUE: u32 = 0;
}
