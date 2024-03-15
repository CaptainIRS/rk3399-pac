#[doc = "Register `PMUGRF_OS_REG1` reader"]
pub type R = crate::R<PmugrfOsReg1Spec>;
#[doc = "Register `PMUGRF_OS_REG1` writer"]
pub type W = crate::W<PmugrfOsReg1Spec>;
#[doc = "Field `OS_REG1` reader - os register"]
pub type OsReg1R = crate::FieldReader<u32>;
#[doc = "Field `OS_REG1` writer - os register"]
pub type OsReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    pub fn os_reg1(&self) -> OsReg1R {
        OsReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    #[must_use]
    pub fn os_reg1(&mut self) -> OsReg1W<PmugrfOsReg1Spec> {
        OsReg1W::new(self, 0)
    }
}
#[doc = "os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfOsReg1Spec;
impl crate::RegisterSpec for PmugrfOsReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_os_reg1::R`](R) reader structure"]
impl crate::Readable for PmugrfOsReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_os_reg1::W`](W) writer structure"]
impl crate::Writable for PmugrfOsReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_OS_REG1 to value 0"]
impl crate::Resettable for PmugrfOsReg1Spec {
    const RESET_VALUE: u32 = 0;
}
