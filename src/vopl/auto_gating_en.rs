#[doc = "Register `AUTO_GATING_EN` reader"]
pub type R = crate::R<AutoGatingEnSpec>;
#[doc = "Register `AUTO_GATING_EN` writer"]
pub type W = crate::W<AutoGatingEnSpec>;
#[doc = "Field `WIN0_ACLK_GATING_EN` reader - win0_aclk_gating_en"]
pub type Win0AclkGatingEnR = crate::BitReader;
#[doc = "Field `WIN0_ACLK_GATING_EN` writer - win0_aclk_gating_en"]
pub type Win0AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_ACLK_GATING_EN` reader - win1_aclk_gating_en"]
pub type Win1AclkGatingEnR = crate::BitReader;
#[doc = "Field `WIN1_ACLK_GATING_EN` writer - win1_aclk_gating_en"]
pub type Win1AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN2_ACLK_GATING_EN` reader - win2_aclk_gating_en"]
pub type Win2AclkGatingEnR = crate::BitReader;
#[doc = "Field `WIN2_ACLK_GATING_EN` writer - win2_aclk_gating_en"]
pub type Win2AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN3_ACLK_GATING_EN` reader - win3_aclk_gating_en"]
pub type Win3AclkGatingEnR = crate::BitReader;
#[doc = "Field `WIN3_ACLK_GATING_EN` writer - win3_aclk_gating_en"]
pub type Win3AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWC_ACLK_GATING_EN` reader - hwc_aclk_gating_en"]
pub type HwcAclkGatingEnR = crate::BitReader;
#[doc = "Field `HWC_ACLK_GATING_EN` writer - hwc_aclk_gating_en"]
pub type HwcAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERLAY_ACLK_GATING_EN` reader - overlay_aclk_gating_en"]
pub type OverlayAclkGatingEnR = crate::BitReader;
#[doc = "Field `OVERLAY_ACLK_GATING_EN` writer - overlay_aclk_gating_en"]
pub type OverlayAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_ACLK_GATING_EN` reader - gamma_aclk_gating_en"]
pub type GammaAclkGatingEnR = crate::BitReader;
#[doc = "Field `GAMMA_ACLK_GATING_EN` writer - gamma_aclk_gating_en"]
pub type GammaAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CABC_ACLK_GATING_EN` reader - cabc_aclk_gating_en"]
pub type CabcAclkGatingEnR = crate::BitReader;
#[doc = "Field `CABC_ACLK_GATING_EN` writer - cabc_aclk_gating_en"]
pub type CabcAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WB_ACLK_GATING_EN` reader - wb_aclk_gating_en"]
pub type WbAclkGatingEnR = crate::BitReader;
#[doc = "Field `WB_ACLK_GATING_EN` writer - wb_aclk_gating_en"]
pub type WbAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_PWMCLK_GATING_EN` reader - pwm_pwmclk_gating_en"]
pub type PwmPwmclkGatingEnR = crate::BitReader;
#[doc = "Field `PWM_PWMCLK_GATING_EN` writer - pwm_pwmclk_gating_en"]
pub type PwmPwmclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_PATH_ACLK_GATING_EN` reader - direct_path_aclk_gating_en"]
pub type DirectPathAclkGatingEnR = crate::BitReader;
#[doc = "Field `DIRECT_PATH_ACLK_GATING_EN` writer - direct_path_aclk_gating_en"]
pub type DirectPathAclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCD0_ACLK_GATING_EN` reader - fbcd0_aclk_gating_en"]
pub type Fbcd0AclkGatingEnR = crate::BitReader;
#[doc = "Field `FBCD0_ACLK_GATING_EN` writer - fbcd0_aclk_gating_en"]
pub type Fbcd0AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCD1_ACLK_GATING_EN` reader - fbcd1_aclk_gating_en"]
pub type Fbcd1AclkGatingEnR = crate::BitReader;
#[doc = "Field `FBCD1_ACLK_GATING_EN` writer - fbcd1_aclk_gating_en"]
pub type Fbcd1AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCD2_ACLK_GATING_EN` reader - fbcd2_aclk_gating_en"]
pub type Fbcd2AclkGatingEnR = crate::BitReader;
#[doc = "Field `FBCD2_ACLK_GATING_EN` writer - fbcd2_aclk_gating_en"]
pub type Fbcd2AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCD3_ACLK_GATING_EN` reader - fbcd3_aclk_gating_en"]
pub type Fbcd3AclkGatingEnR = crate::BitReader;
#[doc = "Field `FBCD3_ACLK_GATING_EN` writer - fbcd3_aclk_gating_en"]
pub type Fbcd3AclkGatingEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - win0_aclk_gating_en"]
    #[inline(always)]
    pub fn win0_aclk_gating_en(&self) -> Win0AclkGatingEnR {
        Win0AclkGatingEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - win1_aclk_gating_en"]
    #[inline(always)]
    pub fn win1_aclk_gating_en(&self) -> Win1AclkGatingEnR {
        Win1AclkGatingEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - win2_aclk_gating_en"]
    #[inline(always)]
    pub fn win2_aclk_gating_en(&self) -> Win2AclkGatingEnR {
        Win2AclkGatingEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - win3_aclk_gating_en"]
    #[inline(always)]
    pub fn win3_aclk_gating_en(&self) -> Win3AclkGatingEnR {
        Win3AclkGatingEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hwc_aclk_gating_en"]
    #[inline(always)]
    pub fn hwc_aclk_gating_en(&self) -> HwcAclkGatingEnR {
        HwcAclkGatingEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - overlay_aclk_gating_en"]
    #[inline(always)]
    pub fn overlay_aclk_gating_en(&self) -> OverlayAclkGatingEnR {
        OverlayAclkGatingEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - gamma_aclk_gating_en"]
    #[inline(always)]
    pub fn gamma_aclk_gating_en(&self) -> GammaAclkGatingEnR {
        GammaAclkGatingEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - cabc_aclk_gating_en"]
    #[inline(always)]
    pub fn cabc_aclk_gating_en(&self) -> CabcAclkGatingEnR {
        CabcAclkGatingEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - wb_aclk_gating_en"]
    #[inline(always)]
    pub fn wb_aclk_gating_en(&self) -> WbAclkGatingEnR {
        WbAclkGatingEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pwm_pwmclk_gating_en"]
    #[inline(always)]
    pub fn pwm_pwmclk_gating_en(&self) -> PwmPwmclkGatingEnR {
        PwmPwmclkGatingEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - direct_path_aclk_gating_en"]
    #[inline(always)]
    pub fn direct_path_aclk_gating_en(&self) -> DirectPathAclkGatingEnR {
        DirectPathAclkGatingEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - fbcd0_aclk_gating_en"]
    #[inline(always)]
    pub fn fbcd0_aclk_gating_en(&self) -> Fbcd0AclkGatingEnR {
        Fbcd0AclkGatingEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - fbcd1_aclk_gating_en"]
    #[inline(always)]
    pub fn fbcd1_aclk_gating_en(&self) -> Fbcd1AclkGatingEnR {
        Fbcd1AclkGatingEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - fbcd2_aclk_gating_en"]
    #[inline(always)]
    pub fn fbcd2_aclk_gating_en(&self) -> Fbcd2AclkGatingEnR {
        Fbcd2AclkGatingEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - fbcd3_aclk_gating_en"]
    #[inline(always)]
    pub fn fbcd3_aclk_gating_en(&self) -> Fbcd3AclkGatingEnR {
        Fbcd3AclkGatingEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - win0_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn win0_aclk_gating_en(&mut self) -> Win0AclkGatingEnW<AutoGatingEnSpec> {
        Win0AclkGatingEnW::new(self, 0)
    }
    #[doc = "Bit 1 - win1_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn win1_aclk_gating_en(&mut self) -> Win1AclkGatingEnW<AutoGatingEnSpec> {
        Win1AclkGatingEnW::new(self, 1)
    }
    #[doc = "Bit 2 - win2_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn win2_aclk_gating_en(&mut self) -> Win2AclkGatingEnW<AutoGatingEnSpec> {
        Win2AclkGatingEnW::new(self, 2)
    }
    #[doc = "Bit 3 - win3_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn win3_aclk_gating_en(&mut self) -> Win3AclkGatingEnW<AutoGatingEnSpec> {
        Win3AclkGatingEnW::new(self, 3)
    }
    #[doc = "Bit 4 - hwc_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_aclk_gating_en(&mut self) -> HwcAclkGatingEnW<AutoGatingEnSpec> {
        HwcAclkGatingEnW::new(self, 4)
    }
    #[doc = "Bit 5 - overlay_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn overlay_aclk_gating_en(&mut self) -> OverlayAclkGatingEnW<AutoGatingEnSpec> {
        OverlayAclkGatingEnW::new(self, 5)
    }
    #[doc = "Bit 6 - gamma_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_aclk_gating_en(&mut self) -> GammaAclkGatingEnW<AutoGatingEnSpec> {
        GammaAclkGatingEnW::new(self, 6)
    }
    #[doc = "Bit 7 - cabc_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_aclk_gating_en(&mut self) -> CabcAclkGatingEnW<AutoGatingEnSpec> {
        CabcAclkGatingEnW::new(self, 7)
    }
    #[doc = "Bit 8 - wb_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn wb_aclk_gating_en(&mut self) -> WbAclkGatingEnW<AutoGatingEnSpec> {
        WbAclkGatingEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pwm_pwmclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pwmclk_gating_en(&mut self) -> PwmPwmclkGatingEnW<AutoGatingEnSpec> {
        PwmPwmclkGatingEnW::new(self, 9)
    }
    #[doc = "Bit 10 - direct_path_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn direct_path_aclk_gating_en(&mut self) -> DirectPathAclkGatingEnW<AutoGatingEnSpec> {
        DirectPathAclkGatingEnW::new(self, 10)
    }
    #[doc = "Bit 12 - fbcd0_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn fbcd0_aclk_gating_en(&mut self) -> Fbcd0AclkGatingEnW<AutoGatingEnSpec> {
        Fbcd0AclkGatingEnW::new(self, 12)
    }
    #[doc = "Bit 13 - fbcd1_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn fbcd1_aclk_gating_en(&mut self) -> Fbcd1AclkGatingEnW<AutoGatingEnSpec> {
        Fbcd1AclkGatingEnW::new(self, 13)
    }
    #[doc = "Bit 14 - fbcd2_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn fbcd2_aclk_gating_en(&mut self) -> Fbcd2AclkGatingEnW<AutoGatingEnSpec> {
        Fbcd2AclkGatingEnW::new(self, 14)
    }
    #[doc = "Bit 15 - fbcd3_aclk_gating_en"]
    #[inline(always)]
    #[must_use]
    pub fn fbcd3_aclk_gating_en(&mut self) -> Fbcd3AclkGatingEnW<AutoGatingEnSpec> {
        Fbcd3AclkGatingEnW::new(self, 15)
    }
}
#[doc = "Auto gating enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_gating_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_gating_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoGatingEnSpec;
impl crate::RegisterSpec for AutoGatingEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_gating_en::R`](R) reader structure"]
impl crate::Readable for AutoGatingEnSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_gating_en::W`](W) writer structure"]
impl crate::Writable for AutoGatingEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTO_GATING_EN to value 0"]
impl crate::Resettable for AutoGatingEnSpec {
    const RESET_VALUE: u32 = 0;
}
