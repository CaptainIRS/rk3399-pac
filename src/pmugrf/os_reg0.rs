#[doc = "Register `OS_REG0` reader"]
pub type R = crate::R<OsReg0Spec>;
#[doc = "Register `OS_REG0` writer"]
pub type W = crate::W<OsReg0Spec>;
#[doc = "Field `OS_REG0` reader - os register"]
pub type OsReg0R = crate::FieldReader<u32>;
#[doc = "Field `OS_REG0` writer - os register"]
pub type OsReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    pub fn os_reg0(&self) -> OsReg0R {
        OsReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - os register"]
    #[inline(always)]
    #[must_use]
    pub fn os_reg0(&mut self) -> OsReg0W<OsReg0Spec> {
        OsReg0W::new(self, 0)
    }
}
#[doc = "os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsReg0Spec;
impl crate::RegisterSpec for OsReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`os_reg0::R`](R) reader structure"]
impl crate::Readable for OsReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`os_reg0::W`](W) writer structure"]
impl crate::Writable for OsReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OS_REG0 to value 0"]
impl crate::Resettable for OsReg0Spec {
    const RESET_VALUE: u32 = 0;
}
