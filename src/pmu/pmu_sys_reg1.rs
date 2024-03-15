#[doc = "Register `PMU_SYS_REG1` reader"]
pub type R = crate::R<PmuSysReg1Spec>;
#[doc = "Register `PMU_SYS_REG1` writer"]
pub type W = crate::W<PmuSysReg1Spec>;
#[doc = "Field `PMU_SYS_REG1` reader - system register 1"]
pub type PmuSysReg1R = crate::FieldReader<u32>;
#[doc = "Field `PMU_SYS_REG1` writer - system register 1"]
pub type PmuSysReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - system register 1"]
    #[inline(always)]
    pub fn pmu_sys_reg1(&self) -> PmuSysReg1R {
        PmuSysReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - system register 1"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_sys_reg1(&mut self) -> PmuSysReg1W<PmuSysReg1Spec> {
        PmuSysReg1W::new(self, 0)
    }
}
#[doc = "pmu system register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuSysReg1Spec;
impl crate::RegisterSpec for PmuSysReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_sys_reg1::R`](R) reader structure"]
impl crate::Readable for PmuSysReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_sys_reg1::W`](W) writer structure"]
impl crate::Writable for PmuSysReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_SYS_REG1 to value 0"]
impl crate::Resettable for PmuSysReg1Spec {
    const RESET_VALUE: u32 = 0;
}
