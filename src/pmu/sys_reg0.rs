#[doc = "Register `SYS_REG0` reader"]
pub type R = crate::R<SysReg0Spec>;
#[doc = "Register `SYS_REG0` writer"]
pub type W = crate::W<SysReg0Spec>;
#[doc = "Field `PMU_SYS_REG0` reader - system register 0"]
pub type PmuSysReg0R = crate::FieldReader<u32>;
#[doc = "Field `PMU_SYS_REG0` writer - system register 0"]
pub type PmuSysReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - system register 0"]
    #[inline(always)]
    pub fn pmu_sys_reg0(&self) -> PmuSysReg0R {
        PmuSysReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - system register 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_sys_reg0(&mut self) -> PmuSysReg0W<SysReg0Spec> {
        PmuSysReg0W::new(self, 0)
    }
}
#[doc = "pmu system register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysReg0Spec;
impl crate::RegisterSpec for SysReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_reg0::R`](R) reader structure"]
impl crate::Readable for SysReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_reg0::W`](W) writer structure"]
impl crate::Writable for SysReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_REG0 to value 0"]
impl crate::Resettable for SysReg0Spec {
    const RESET_VALUE: u32 = 0;
}
