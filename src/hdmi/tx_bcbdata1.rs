#[doc = "Register `TX_BCBDATA1` reader"]
pub type R = crate::R<TxBcbdata1Spec>;
#[doc = "Register `TX_BCBDATA1` writer"]
pub type W = crate::W<TxBcbdata1Spec>;
#[doc = "Field `BCBDATA` reader - This register defines the value of bcbdata\\[15:8\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
pub type BcbdataR = crate::FieldReader;
#[doc = "Field `BCBDATA` writer - This register defines the value of bcbdata\\[15:8\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
pub type BcbdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the value of bcbdata\\[15:8\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
    #[inline(always)]
    pub fn bcbdata(&self) -> BcbdataR {
        BcbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the value of bcbdata\\[15:8\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
    #[inline(always)]
    #[must_use]
    pub fn bcbdata(&mut self) -> BcbdataW<TxBcbdata1Spec> {
        BcbdataW::new(self, 0)
    }
}
#[doc = "This register defines the value of bcbdata\\[15:8\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_bcbdata1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_bcbdata1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBcbdata1Spec;
impl crate::RegisterSpec for TxBcbdata1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_bcbdata1::R`](R) reader structure"]
impl crate::Readable for TxBcbdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_bcbdata1::W`](W) writer structure"]
impl crate::Writable for TxBcbdata1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_BCBDATA1 to value 0"]
impl crate::Resettable for TxBcbdata1Spec {
    const RESET_VALUE: u8 = 0;
}
