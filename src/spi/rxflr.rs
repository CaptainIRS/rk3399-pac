#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Field `RXFLR` reader - Receive FIFO Level\n\nContains the number of valid data entries in the receive FIFO."]
pub type RxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO Level\n\nContains the number of valid data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxflr(&self) -> RxflrR {
        RxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RxflrSpec {
    const RESET_VALUE: u32 = 0;
}
