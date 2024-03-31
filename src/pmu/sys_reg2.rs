#[doc = "Register `SYS_REG2` reader"]
pub type R = crate::R<SysReg2Spec>;
#[doc = "Register `SYS_REG2` writer"]
pub type W = crate::W<SysReg2Spec>;
#[doc = "Field `PMU_SYS_REG2` reader - system register 2"]
pub type PmuSysReg2R = crate::FieldReader<u32>;
#[doc = "Field `PMU_SYS_REG2` writer - system register 2"]
pub type PmuSysReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - system register 2"]
    #[inline(always)]
    pub fn pmu_sys_reg2(&self) -> PmuSysReg2R {
        PmuSysReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - system register 2"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_sys_reg2(&mut self) -> PmuSysReg2W<SysReg2Spec> {
        PmuSysReg2W::new(self, 0)
    }
}
#[doc = "pmu system register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysReg2Spec;
impl crate::RegisterSpec for SysReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_reg2::R`](R) reader structure"]
impl crate::Readable for SysReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_reg2::W`](W) writer structure"]
impl crate::Writable for SysReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_REG2 to value 0"]
impl crate::Resettable for SysReg2Spec {
    const RESET_VALUE: u32 = 0;
}
