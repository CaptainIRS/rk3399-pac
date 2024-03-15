#[doc = "Register `CEC_TX_CNT` reader"]
pub type R = crate::R<CecTxCntSpec>;
#[doc = "Register `CEC_TX_CNT` writer"]
pub type W = crate::W<CecTxCntSpec>;
#[doc = "Field `CEC_TX_CNT` reader - CEC Transmitter Counter register 5'd0: No data needs to be transmitted 5'd1: Frame size is 1 byte 5'd16: Frame size is 16 bytes"]
pub type CecTxCntR = crate::FieldReader;
#[doc = "Field `CEC_TX_CNT` writer - CEC Transmitter Counter register 5'd0: No data needs to be transmitted 5'd1: Frame size is 1 byte 5'd16: Frame size is 16 bytes"]
pub type CecTxCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CEC Transmitter Counter register 5'd0: No data needs to be transmitted 5'd1: Frame size is 1 byte 5'd16: Frame size is 16 bytes"]
    #[inline(always)]
    pub fn cec_tx_cnt(&self) -> CecTxCntR {
        CecTxCntR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CEC Transmitter Counter register 5'd0: No data needs to be transmitted 5'd1: Frame size is 1 byte 5'd16: Frame size is 16 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn cec_tx_cnt(&mut self) -> CecTxCntW<CecTxCntSpec> {
        CecTxCntW::new(self, 0)
    }
}
#[doc = "CEC Transmitter Counter register 5'd0: No data needs to be transmitted 5'd1: Frame size is 1 byte 5'd16: Frame size is 16 bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_tx_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_tx_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecTxCntSpec;
impl crate::RegisterSpec for CecTxCntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_tx_cnt::R`](R) reader structure"]
impl crate::Readable for CecTxCntSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_tx_cnt::W`](W) writer structure"]
impl crate::Writable for CecTxCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_TX_CNT to value 0"]
impl crate::Resettable for CecTxCntSpec {
    const RESET_VALUE: u8 = 0;
}
