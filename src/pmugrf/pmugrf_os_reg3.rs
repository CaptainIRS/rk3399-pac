#[doc = "Register `PMUGRF_OS_REG3` reader"]
pub type R = crate::R<PmugrfOsReg3Spec>;
#[doc = "Register `PMUGRF_OS_REG3` writer"]
pub type W = crate::W<PmugrfOsReg3Spec>;
#[doc = "Field `OS_REG3` reader - os register"]
pub type OsReg3R = crate::FieldReader<u32>;
#[doc = "Field `OS_REG3` writer - os register"]
pub type OsReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    pub fn os_reg3(&self) -> OsReg3R {
        OsReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    #[must_use]
    pub fn os_reg3(&mut self) -> OsReg3W<PmugrfOsReg3Spec> {
        OsReg3W::new(self, 0)
    }
}
#[doc = "os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfOsReg3Spec;
impl crate::RegisterSpec for PmugrfOsReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_os_reg3::R`](R) reader structure"]
impl crate::Readable for PmugrfOsReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_os_reg3::W`](W) writer structure"]
impl crate::Writable for PmugrfOsReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_OS_REG3 to value 0"]
impl crate::Resettable for PmugrfOsReg3Spec {
    const RESET_VALUE: u32 = 0;
}
