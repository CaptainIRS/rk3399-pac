#[doc = "Register `DDRIO_PWRON_CNT` reader"]
pub type R = crate::R<DdrioPwronCntSpec>;
#[doc = "Register `DDRIO_PWRON_CNT` writer"]
pub type W = crate::W<DdrioPwronCntSpec>;
#[doc = "Field `PMU_DDRIO_PWRON_CNT` reader - pmu ddrio power on counter value"]
pub type PmuDdrioPwronCntR = crate::FieldReader<u32>;
#[doc = "Field `PMU_DDRIO_PWRON_CNT` writer - pmu ddrio power on counter value"]
pub type PmuDdrioPwronCntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - pmu ddrio power on counter value"]
    #[inline(always)]
    pub fn pmu_ddrio_pwron_cnt(&self) -> PmuDdrioPwronCntR {
        PmuDdrioPwronCntR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - pmu ddrio power on counter value"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_ddrio_pwron_cnt(&mut self) -> PmuDdrioPwronCntW<DdrioPwronCntSpec> {
        PmuDdrioPwronCntW::new(self, 0)
    }
}
#[doc = "pmu ddrio power on count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrio_pwron_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrio_pwron_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrioPwronCntSpec;
impl crate::RegisterSpec for DdrioPwronCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrio_pwron_cnt::R`](R) reader structure"]
impl crate::Readable for DdrioPwronCntSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrio_pwron_cnt::W`](W) writer structure"]
impl crate::Writable for DdrioPwronCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRIO_PWRON_CNT to value 0"]
impl crate::Resettable for DdrioPwronCntSpec {
    const RESET_VALUE: u32 = 0;
}
