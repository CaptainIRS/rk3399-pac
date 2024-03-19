#[doc = "Register `TX_RCRDATA0` reader"]
pub type R = crate::R<TxRcrdata0Spec>;
#[doc = "Register `TX_RCRDATA0` writer"]
pub type W = crate::W<TxRcrdata0Spec>;
#[doc = "Field `RCRDATA` reader - This register defines the value of rcrydata\\[7:0\\]\n\nwhen TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set\n\nto 1b."]
pub type RcrdataR = crate::FieldReader;
#[doc = "Field `RCRDATA` writer - This register defines the value of rcrydata\\[7:0\\]\n\nwhen TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set\n\nto 1b."]
pub type RcrdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the value of rcrydata\\[7:0\\]\n\nwhen TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set\n\nto 1b."]
    #[inline(always)]
    pub fn rcrdata(&self) -> RcrdataR {
        RcrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the value of rcrydata\\[7:0\\]\n\nwhen TX_INSTUFFING\\[1\\]
(rcrdata_stuffing) is set\n\nto 1b."]
    #[inline(always)]
    #[must_use]
    pub fn rcrdata(&mut self) -> RcrdataW<TxRcrdata0Spec> {
        RcrdataW::new(self, 0)
    }
}
#[doc = "Video Input rcr Data Channel Stuffing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_rcrdata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_rcrdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxRcrdata0Spec;
impl crate::RegisterSpec for TxRcrdata0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_rcrdata0::R`](R) reader structure"]
impl crate::Readable for TxRcrdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_rcrdata0::W`](W) writer structure"]
impl crate::Writable for TxRcrdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_RCRDATA0 to value 0"]
impl crate::Resettable for TxRcrdata0Spec {
    const RESET_VALUE: u8 = 0;
}
