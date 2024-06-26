#[doc = "Register `RX_ERR_CNT` reader"]
pub type R = crate::R<RxErrCntSpec>;
#[doc = "Register `RX_ERR_CNT` writer"]
pub type W = crate::W<RxErrCntSpec>;
#[doc = "Field `RX_Error_Counter` reader - RX Error Counter"]
pub type RxErrorCounterR = crate::FieldReader<u16>;
#[doc = "Field `RX_Error_Counter` writer - RX Error Counter"]
pub type RxErrorCounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RX Error Counter"]
    #[inline(always)]
    pub fn rx_error_counter(&self) -> RxErrorCounterR {
        RxErrorCounterR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RX Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rx_error_counter(&mut self) -> RxErrorCounterW<RxErrCntSpec> {
        RxErrorCounterW::new(self, 0)
    }
}
#[doc = "RX Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_err_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrCntSpec;
impl crate::RegisterSpec for RxErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_err_cnt::R`](R) reader structure"]
impl crate::Readable for RxErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_err_cnt::W`](W) writer structure"]
impl crate::Writable for RxErrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ERR_CNT to value 0"]
impl crate::Resettable for RxErrCntSpec {
    const RESET_VALUE: u32 = 0;
}
