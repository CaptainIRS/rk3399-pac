#[doc = "Register `CEC_TX_DATA[%s]` reader"]
pub type R = crate::R<CecTxDataSpec>;
#[doc = "Register `CEC_TX_DATA[%s]` writer"]
pub type W = crate::W<CecTxDataSpec>;
#[doc = "Field `DATABYTE` reader - Data byte\\[x\\], where x is 0 to 15"]
pub type DatabyteR = crate::FieldReader;
#[doc = "Field `DATABYTE` writer - Data byte\\[x\\], where x is 0 to 15"]
pub type DatabyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte\\[x\\], where x is 0 to 15"]
    #[inline(always)]
    pub fn databyte(&self) -> DatabyteR {
        DatabyteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte\\[x\\], where x is 0 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn databyte(&mut self) -> DatabyteW<CecTxDataSpec> {
        DatabyteW::new(self, 0)
    }
}
#[doc = "Data byte\\[x\\], where x is 0 to 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_tx_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_tx_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecTxDataSpec;
impl crate::RegisterSpec for CecTxDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_tx_data::R`](R) reader structure"]
impl crate::Readable for CecTxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_tx_data::W`](W) writer structure"]
impl crate::Writable for CecTxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_TX_DATA[%s]
to value 0"]
impl crate::Resettable for CecTxDataSpec {
    const RESET_VALUE: u8 = 0;
}
