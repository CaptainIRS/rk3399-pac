#[doc = "Register `TX_BCBDATA0` reader"]
pub type R = crate::R<TxBcbdata0Spec>;
#[doc = "Register `TX_BCBDATA0` writer"]
pub type W = crate::W<TxBcbdata0Spec>;
#[doc = "Field `BCBDATA` reader - This register defines the value of bcbdata\\[7:0\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
pub type BcbdataR = crate::FieldReader;
#[doc = "Field `BCBDATA` writer - This register defines the value of bcbdata\\[7:0\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
pub type BcbdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the value of bcbdata\\[7:0\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
    #[inline(always)]
    pub fn bcbdata(&self) -> BcbdataR {
        BcbdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the value of bcbdata\\[7:0\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b."]
    #[inline(always)]
    #[must_use]
    pub fn bcbdata(&mut self) -> BcbdataW<TxBcbdata0Spec> {
        BcbdataW::new(self, 0)
    }
}
#[doc = "This register defines the value of bcbdata\\[7:0\\]
when TX_INSTUFFING\\[2\\]
(bcbdata_stuffing) is set to 1b.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_bcbdata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_bcbdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBcbdata0Spec;
impl crate::RegisterSpec for TxBcbdata0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_bcbdata0::R`](R) reader structure"]
impl crate::Readable for TxBcbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_bcbdata0::W`](W) writer structure"]
impl crate::Writable for TxBcbdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_BCBDATA0 to value 0"]
impl crate::Resettable for TxBcbdata0Spec {
    const RESET_VALUE: u8 = 0;
}
