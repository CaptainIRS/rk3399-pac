#[doc = "Register `CEC_RX_DATA[%s]` reader"]
pub type R = crate::R<CecRxDataSpec>;
#[doc = "Field `DATABYTE` reader - Data byte\\[x\\], where x is 0 to 15"]
pub type DatabyteR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data byte\\[x\\], where x is 0 to 15"]
    #[inline(always)]
    pub fn databyte(&self) -> DatabyteR {
        DatabyteR::new(self.bits)
    }
}
#[doc = "CEC RX Data Register Array Address offset: i =0 to 15\n\nThese registers (8 bit each) are the buffers used for storing the received data (including\n\nheader and data blocks).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_rx_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecRxDataSpec;
impl crate::RegisterSpec for CecRxDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_rx_data::R`](R) reader structure"]
impl crate::Readable for CecRxDataSpec {}
#[doc = "`reset()` method sets CEC_RX_DATA[%s]
to value 0"]
impl crate::Resettable for CecRxDataSpec {
    const RESET_VALUE: u8 = 0;
}
