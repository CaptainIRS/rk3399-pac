#[doc = "Register `TX_GYDATA0` reader"]
pub type R = crate::R<TxGydata0Spec>;
#[doc = "Register `TX_GYDATA0` writer"]
pub type W = crate::W<TxGydata0Spec>;
#[doc = "Field `GYDATA` reader - This register defines the value of gydata\\[7:0\\]
when TX_INSTUFFING\\[0\\]
(gydata_stuffing) is set to 1b."]
pub type GydataR = crate::FieldReader;
#[doc = "Field `GYDATA` writer - This register defines the value of gydata\\[7:0\\]
when TX_INSTUFFING\\[0\\]
(gydata_stuffing) is set to 1b."]
pub type GydataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the value of gydata\\[7:0\\]
when TX_INSTUFFING\\[0\\]
(gydata_stuffing) is set to 1b."]
    #[inline(always)]
    pub fn gydata(&self) -> GydataR {
        GydataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the value of gydata\\[7:0\\]
when TX_INSTUFFING\\[0\\]
(gydata_stuffing) is set to 1b."]
    #[inline(always)]
    #[must_use]
    pub fn gydata(&mut self) -> GydataW<TxGydata0Spec> {
        GydataW::new(self, 0)
    }
}
#[doc = "This register defines the value of gydata\\[7:0\\]
when TX_INSTUFFING\\[0\\]
(gydata_stuffing) is set to 1b.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_gydata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_gydata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxGydata0Spec;
impl crate::RegisterSpec for TxGydata0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_gydata0::R`](R) reader structure"]
impl crate::Readable for TxGydata0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_gydata0::W`](W) writer structure"]
impl crate::Writable for TxGydata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_GYDATA0 to value 0"]
impl crate::Resettable for TxGydata0Spec {
    const RESET_VALUE: u8 = 0;
}
