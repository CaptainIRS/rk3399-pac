#[doc = "Register `DENALI_PHY_957` reader"]
pub type R = crate::R<DenaliPhy957Spec>;
#[doc = "Register `DENALI_PHY_957` writer"]
pub type W = crate::W<DenaliPhy957Spec>;
#[doc = "Field `PHY_DDL_AC_MODE` reader - PHY Address/Control DDL BIST mode."]
pub type PhyDdlAcModeR = crate::FieldReader<u16>;
#[doc = "Field `PHY_DDL_AC_MODE` writer - PHY Address/Control DDL BIST mode."]
pub type PhyDdlAcModeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_PAD_BACKGROUND_CAL` reader - PHY background pad calibration enable"]
pub type PhyPadBackgroundCalR = crate::BitReader;
#[doc = "Field `PHY_PAD_BACKGROUND_CAL` writer - PHY background pad calibration enable"]
pub type PhyPadBackgroundCalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DLL_RST_EN` reader - PHY DLL reset software interface enable."]
pub type PhyDllRstEnR = crate::FieldReader;
#[doc = "Field `PHY_DLL_RST_EN` writer - PHY DLL reset software interface enable."]
pub type PhyDllRstEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:10 - PHY Address/Control DDL BIST mode."]
    #[inline(always)]
    pub fn phy_ddl_ac_mode(&self) -> PhyDdlAcModeR {
        PhyDdlAcModeR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - PHY background pad calibration enable"]
    #[inline(always)]
    pub fn phy_pad_background_cal(&self) -> PhyPadBackgroundCalR {
        PhyPadBackgroundCalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - PHY DLL reset software interface enable."]
    #[inline(always)]
    pub fn phy_dll_rst_en(&self) -> PhyDllRstEnR {
        PhyDllRstEnR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - PHY Address/Control DDL BIST mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_ac_mode(&mut self) -> PhyDdlAcModeW<DenaliPhy957Spec> {
        PhyDdlAcModeW::new(self, 0)
    }
    #[doc = "Bit 16 - PHY background pad calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_background_cal(&mut self) -> PhyPadBackgroundCalW<DenaliPhy957Spec> {
        PhyPadBackgroundCalW::new(self, 16)
    }
    #[doc = "Bits 24:25 - PHY DLL reset software interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dll_rst_en(&mut self) -> PhyDllRstEnW<DenaliPhy957Spec> {
        PhyDllRstEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_957::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_957::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy957Spec;
impl crate::RegisterSpec for DenaliPhy957Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_957::R`](R) reader structure"]
impl crate::Readable for DenaliPhy957Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_957::W`](W) writer structure"]
impl crate::Writable for DenaliPhy957Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_957 to value 0"]
impl crate::Resettable for DenaliPhy957Spec {
    const RESET_VALUE: u32 = 0;
}
