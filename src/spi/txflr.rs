#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TxflrSpec>;
#[doc = "Field `TXFLR` reader - Transmit FIFO Level\n\nContains the number of valid data entries in the transmit FIFO."]
pub type TxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level\n\nContains the number of valid data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn txflr(&self) -> TxflrR {
        TxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxflrSpec;
impl crate::RegisterSpec for TxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TxflrSpec {}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TxflrSpec {
    const RESET_VALUE: u32 = 0;
}
