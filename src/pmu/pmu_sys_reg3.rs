#[doc = "Register `PMU_SYS_REG3` reader"]
pub type R = crate::R<PmuSysReg3Spec>;
#[doc = "Register `PMU_SYS_REG3` writer"]
pub type W = crate::W<PmuSysReg3Spec>;
#[doc = "Field `PMU_SYS_REG3` reader - system register 3"]
pub type PmuSysReg3R = crate::FieldReader<u32>;
#[doc = "Field `PMU_SYS_REG3` writer - system register 3"]
pub type PmuSysReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - system register 3"]
    #[inline(always)]
    pub fn pmu_sys_reg3(&self) -> PmuSysReg3R {
        PmuSysReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - system register 3"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_sys_reg3(&mut self) -> PmuSysReg3W<PmuSysReg3Spec> {
        PmuSysReg3W::new(self, 0)
    }
}
#[doc = "pmu system register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuSysReg3Spec;
impl crate::RegisterSpec for PmuSysReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_sys_reg3::R`](R) reader structure"]
impl crate::Readable for PmuSysReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pmu_sys_reg3::W`](W) writer structure"]
impl crate::Writable for PmuSysReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_SYS_REG3 to value 0"]
impl crate::Resettable for PmuSysReg3Spec {
    const RESET_VALUE: u32 = 0;
}
