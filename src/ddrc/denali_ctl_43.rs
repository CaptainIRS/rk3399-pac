#[doc = "Register `DENALI_CTL_43` reader"]
pub type R = crate::R<DenaliCtl43Spec>;
#[doc = "Register `DENALI_CTL_43` writer"]
pub type W = crate::W<DenaliCtl43Spec>;
#[doc = "Field `TMRZ_F2` reader - DRAM TMRZ value for frequency copy 2 in cycles."]
pub type TmrzF2R = crate::FieldReader;
#[doc = "Field `TMRZ_F2` writer - DRAM TMRZ value for frequency copy 2 in cycles."]
pub type TmrzF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AP` reader - Enable auto pre-charge mode of controller. Set to 1 to enable."]
pub type ApR = crate::BitReader;
#[doc = "Field `AP` writer - Enable auto pre-charge mode of controller. Set to 1 to enable."]
pub type ApW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONCURRENTAP` reader - IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
pub type ConcurrentapR = crate::BitReader;
#[doc = "Field `CONCURRENTAP` writer - IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
pub type ConcurrentapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAS_LOCKOUT` reader - IF the DRAM supports it, this allows the controller to execute auto pre- charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
pub type TrasLockoutR = crate::BitReader;
#[doc = "Field `TRAS_LOCKOUT` writer - IF the DRAM supports it, this allows the controller to execute auto pre- charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
pub type TrasLockoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - DRAM TMRZ value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tmrz_f2(&self) -> TmrzF2R {
        TmrzF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable auto pre-charge mode of controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
    #[inline(always)]
    pub fn concurrentap(&self) -> ConcurrentapR {
        ConcurrentapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - IF the DRAM supports it, this allows the controller to execute auto pre- charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
    #[inline(always)]
    pub fn tras_lockout(&self) -> TrasLockoutR {
        TrasLockoutR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - DRAM TMRZ value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrz_f2(&mut self) -> TmrzF2W<DenaliCtl43Spec> {
        TmrzF2W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable auto pre-charge mode of controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<DenaliCtl43Spec> {
        ApW::new(self, 8)
    }
    #[doc = "Bit 16 - IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn concurrentap(&mut self) -> ConcurrentapW<DenaliCtl43Spec> {
        ConcurrentapW::new(self, 16)
    }
    #[doc = "Bit 24 - IF the DRAM supports it, this allows the controller to execute auto pre- charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn tras_lockout(&mut self) -> TrasLockoutW<DenaliCtl43Spec> {
        TrasLockoutW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl43Spec;
impl crate::RegisterSpec for DenaliCtl43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_43::R`](R) reader structure"]
impl crate::Readable for DenaliCtl43Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_43::W`](W) writer structure"]
impl crate::Writable for DenaliCtl43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_43 to value 0"]
impl crate::Resettable for DenaliCtl43Spec {
    const RESET_VALUE: u32 = 0;
}
