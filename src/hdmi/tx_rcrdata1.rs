#[doc = "Register `TX_RCRDATA1` reader"]
pub type R = crate::R<TxRcrdata1Spec>;
#[doc = "Register `TX_RCRDATA1` writer"]
pub type W = crate::W<TxRcrdata1Spec>;
#[doc = "Field `RCRDATA` reader - This register defines the value of rcrydata\\[15:8\\]
when TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set to 1b."]
pub type RcrdataR = crate::FieldReader;
#[doc = "Field `RCRDATA` writer - This register defines the value of rcrydata\\[15:8\\]
when TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set to 1b."]
pub type RcrdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the value of rcrydata\\[15:8\\]
when TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set to 1b."]
    #[inline(always)]
    pub fn rcrdata(&self) -> RcrdataR {
        RcrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the value of rcrydata\\[15:8\\]
when TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set to 1b."]
    #[inline(always)]
    #[must_use]
    pub fn rcrdata(&mut self) -> RcrdataW<TxRcrdata1Spec> {
        RcrdataW::new(self, 0)
    }
}
#[doc = "This register defines the value of rcrydata\\[15:8\\]
when TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set to 1b.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rcrdata1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_rcrdata1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxRcrdata1Spec;
impl crate::RegisterSpec for TxRcrdata1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_rcrdata1::R`](R) reader structure"]
impl crate::Readable for TxRcrdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_rcrdata1::W`](W) writer structure"]
impl crate::Writable for TxRcrdata1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_RCRDATA1 to value 0"]
impl crate::Resettable for TxRcrdata1Spec {
    const RESET_VALUE: u8 = 0;
}
