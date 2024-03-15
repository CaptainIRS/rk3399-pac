#[doc = "Register `ATE_TEST_STATUS` reader"]
pub type R = crate::R<AteTestStatusSpec>;
#[doc = "Register `ATE_TEST_STATUS` writer"]
pub type W = crate::W<AteTestStatusSpec>;
#[doc = "Field `PRBS7CHECKFSMSTATE` reader - PRBS7 check FSM state \\[15:12\\]:lane3,\\[11:8\\]:lane2,\\[7:4\\]:lane1,\\[3:0\\]:lane 0"]
pub type Prbs7checkfsmstateR = crate::FieldReader<u16>;
#[doc = "Field `PRBS7CHECKFSMSTATE` writer - PRBS7 check FSM state \\[15:12\\]:lane3,\\[11:8\\]:lane2,\\[7:4\\]:lane1,\\[3:0\\]:lane 0"]
pub type Prbs7checkfsmstateW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ERROR_INC` reader - ERROR indicator \\[19\\]:lane3,\\[18\\]:lane2,\\[17\\]:lane1,\\[16\\]:lane0"]
pub type ErrorIncR = crate::FieldReader;
#[doc = "Field `ERROR_INC` writer - ERROR indicator \\[19\\]:lane3,\\[18\\]:lane2,\\[17\\]:lane1,\\[16\\]:lane0"]
pub type ErrorIncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - PRBS7 check FSM state \\[15:12\\]:lane3,\\[11:8\\]:lane2,\\[7:4\\]:lane1,\\[3:0\\]:lane 0"]
    #[inline(always)]
    pub fn prbs7checkfsmstate(&self) -> Prbs7checkfsmstateR {
        Prbs7checkfsmstateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - ERROR indicator \\[19\\]:lane3,\\[18\\]:lane2,\\[17\\]:lane1,\\[16\\]:lane0"]
    #[inline(always)]
    pub fn error_inc(&self) -> ErrorIncR {
        ErrorIncR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - PRBS7 check FSM state \\[15:12\\]:lane3,\\[11:8\\]:lane2,\\[7:4\\]:lane1,\\[3:0\\]:lane 0"]
    #[inline(always)]
    #[must_use]
    pub fn prbs7checkfsmstate(&mut self) -> Prbs7checkfsmstateW<AteTestStatusSpec> {
        Prbs7checkfsmstateW::new(self, 0)
    }
    #[doc = "Bits 16:19 - ERROR indicator \\[19\\]:lane3,\\[18\\]:lane2,\\[17\\]:lane1,\\[16\\]:lane0"]
    #[inline(always)]
    #[must_use]
    pub fn error_inc(&mut self) -> ErrorIncW<AteTestStatusSpec> {
        ErrorIncW::new(self, 16)
    }
}
#[doc = "ATE test status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AteTestStatusSpec;
impl crate::RegisterSpec for AteTestStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ate_test_status::R`](R) reader structure"]
impl crate::Readable for AteTestStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ate_test_status::W`](W) writer structure"]
impl crate::Writable for AteTestStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets ATE_TEST_STATUS to value 0"]
impl crate::Resettable for AteTestStatusSpec {
    const RESET_VALUE: u32 = 0;
}
