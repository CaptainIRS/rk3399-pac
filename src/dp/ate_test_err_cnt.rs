#[doc = "Register `ATE_TEST_ERR_CNT%s` reader"]
pub type R = crate::R<AteTestErrCntSpec>;
#[doc = "Register `ATE_TEST_ERR_CNT%s` writer"]
pub type W = crate::W<AteTestErrCntSpec>;
#[doc = "Field `ATE_TEST_ERR_CNT0_ATE_TEST_ERR_CNT3` reader - ATE test error counter register.0x080C—lane0, \n\n0x0810—lane1, 0x0814—lane2, 0x0818—lane3"]
pub type AteTestErrCnt0AteTestErrCnt3R = crate::FieldReader<u32>;
#[doc = "Field `ATE_TEST_ERR_CNT0_ATE_TEST_ERR_CNT3` writer - ATE test error counter register.0x080C—lane0, \n\n0x0810—lane1, 0x0814—lane2, 0x0818—lane3"]
pub type AteTestErrCnt0AteTestErrCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ATE test error counter register.0x080C—lane0, \n\n0x0810—lane1, 0x0814—lane2, 0x0818—lane3"]
    #[inline(always)]
    pub fn ate_test_err_cnt0_ate_test_err_cnt3(&self) -> AteTestErrCnt0AteTestErrCnt3R {
        AteTestErrCnt0AteTestErrCnt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ATE test error counter register.0x080C—lane0, \n\n0x0810—lane1, 0x0814—lane2, 0x0818—lane3"]
    #[inline(always)]
    #[must_use]
    pub fn ate_test_err_cnt0_ate_test_err_cnt3(
        &mut self,
    ) -> AteTestErrCnt0AteTestErrCnt3W<AteTestErrCntSpec> {
        AteTestErrCnt0AteTestErrCnt3W::new(self, 0)
    }
}
#[doc = "ATE test error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_err_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_err_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AteTestErrCntSpec;
impl crate::RegisterSpec for AteTestErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ate_test_err_cnt::R`](R) reader structure"]
impl crate::Readable for AteTestErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`ate_test_err_cnt::W`](W) writer structure"]
impl crate::Writable for AteTestErrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets ATE_TEST_ERR_CNT%s to value 0"]
impl crate::Resettable for AteTestErrCntSpec {
    const RESET_VALUE: u32 = 0;
}
