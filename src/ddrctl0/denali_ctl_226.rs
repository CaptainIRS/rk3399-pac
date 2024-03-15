#[doc = "Register `DENALI_CTL_226` reader"]
pub type R = crate::R<DenaliCtl226Spec>;
#[doc = "Register `DENALI_CTL_226` writer"]
pub type W = crate::W<DenaliCtl226Spec>;
#[doc = "Field `WLMRD` reader - Delay from issuing MRS to first write leveling strobe."]
pub type WlmrdR = crate::FieldReader;
#[doc = "Field `WLMRD` writer - Delay from issuing MRS to first write leveling strobe."]
pub type WlmrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRLVL_EN` reader - Enable the MC write leveling module. Set to 1 to enable."]
pub type WrlvlEnR = crate::BitReader;
#[doc = "Field `WRLVL_EN` writer - Enable the MC write leveling module. Set to 1 to enable."]
pub type WrlvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_PHY_WRLVL_MODE` reader - Specifies the PHY support for DFI write leveling. Set to 1 for supported."]
pub type DfiPhyWrlvlModeR = crate::BitReader;
#[doc = "Field `DFI_PHY_WRLVL_MODE` writer - Specifies the PHY support for DFI write leveling. Set to 1 for supported."]
pub type DfiPhyWrlvlModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRLVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
pub type WrlvlPeriodicR = crate::BitReader;
#[doc = "Field `WRLVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
pub type WrlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Delay from issuing MRS to first write leveling strobe."]
    #[inline(always)]
    pub fn wlmrd(&self) -> WlmrdR {
        WlmrdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Enable the MC write leveling module. Set to 1 to enable."]
    #[inline(always)]
    pub fn wrlvl_en(&self) -> WrlvlEnR {
        WrlvlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI write leveling. Set to 1 for supported."]
    #[inline(always)]
    pub fn dfi_phy_wrlvl_mode(&self) -> DfiPhyWrlvlModeR {
        DfiPhyWrlvlModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
    #[inline(always)]
    pub fn wrlvl_periodic(&self) -> WrlvlPeriodicR {
        WrlvlPeriodicR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Delay from issuing MRS to first write leveling strobe."]
    #[inline(always)]
    #[must_use]
    pub fn wlmrd(&mut self) -> WlmrdW<DenaliCtl226Spec> {
        WlmrdW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable the MC write leveling module. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_en(&mut self) -> WrlvlEnW<DenaliCtl226Spec> {
        WrlvlEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI write leveling. Set to 1 for supported."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phy_wrlvl_mode(&mut self) -> DfiPhyWrlvlModeW<DenaliCtl226Spec> {
        DfiPhyWrlvlModeW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_periodic(&mut self) -> WrlvlPeriodicW<DenaliCtl226Spec> {
        WrlvlPeriodicW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_226::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_226::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl226Spec;
impl crate::RegisterSpec for DenaliCtl226Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_226::R`](R) reader structure"]
impl crate::Readable for DenaliCtl226Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_226::W`](W) writer structure"]
impl crate::Writable for DenaliCtl226Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_226 to value 0"]
impl crate::Resettable for DenaliCtl226Spec {
    const RESET_VALUE: u32 = 0;
}
