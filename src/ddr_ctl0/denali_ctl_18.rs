#[doc = "Register `DENALI_CTL_18` reader"]
pub type R = crate::R<DenaliCtl18Spec>;
#[doc = "Register `DENALI_CTL_18` writer"]
pub type W = crate::W<DenaliCtl18Spec>;
#[doc = "Field `NO_PHY_IND_TRAIN_INIT` reader - Disable PHY Independent Training during initialization. Set to 1 to disable."]
pub type NoPhyIndTrainInitR = crate::BitReader;
#[doc = "Field `NO_PHY_IND_TRAIN_INIT` writer - Disable PHY Independent Training during initialization. Set to 1 to disable."]
pub type NoPhyIndTrainInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_INDEP_TRAIN_MODE` reader - Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepTrainModeR = crate::BitReader;
#[doc = "Field `PHY_INDEP_TRAIN_MODE` writer - Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepTrainModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIBUS_FREQ_INIT` reader - Defines the initial DFI bus frequency."]
pub type DfibusFreqInitR = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_INIT` writer - Defines the initial DFI bus frequency."]
pub type DfibusFreqInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFIBUS_BOOT_FREQ` reader - Defines the DFI bus boot frequency."]
pub type DfibusBootFreqR = crate::FieldReader;
#[doc = "Field `DFIBUS_BOOT_FREQ` writer - Defines the DFI bus boot frequency."]
pub type DfibusBootFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Disable PHY Independent Training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_phy_ind_train_init(&self) -> NoPhyIndTrainInitR {
        NoPhyIndTrainInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_indep_train_mode(&self) -> PhyIndepTrainModeR {
        PhyIndepTrainModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Defines the initial DFI bus frequency."]
    #[inline(always)]
    pub fn dfibus_freq_init(&self) -> DfibusFreqInitR {
        DfibusFreqInitR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Defines the DFI bus boot frequency."]
    #[inline(always)]
    pub fn dfibus_boot_freq(&self) -> DfibusBootFreqR {
        DfibusBootFreqR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable PHY Independent Training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_phy_ind_train_init(&mut self) -> NoPhyIndTrainInitW<DenaliCtl18Spec> {
        NoPhyIndTrainInitW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_indep_train_mode(&mut self) -> PhyIndepTrainModeW<DenaliCtl18Spec> {
        PhyIndepTrainModeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Defines the initial DFI bus frequency."]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_init(&mut self) -> DfibusFreqInitW<DenaliCtl18Spec> {
        DfibusFreqInitW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Defines the DFI bus boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_boot_freq(&mut self) -> DfibusBootFreqW<DenaliCtl18Spec> {
        DfibusBootFreqW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl18Spec;
impl crate::RegisterSpec for DenaliCtl18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_18::R`](R) reader structure"]
impl crate::Readable for DenaliCtl18Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_18::W`](W) writer structure"]
impl crate::Writable for DenaliCtl18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_18 to value 0"]
impl crate::Resettable for DenaliCtl18Spec {
    const RESET_VALUE: u32 = 0;
}
