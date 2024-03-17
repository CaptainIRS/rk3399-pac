#[doc = "Register `DENALI_PHY_899` reader"]
pub type R = crate::R<DenaliPhy899Spec>;
#[doc = "Register `DENALI_PHY_899` writer"]
pub type W = crate::W<DenaliPhy899Spec>;
#[doc = "Field `PHY_CSLVL_ENABLE` reader - CS training enable. Set to 1 to enable CS training during CA training."]
pub type PhyCslvlEnableR = crate::BitReader;
#[doc = "Field `PHY_CSLVL_ENABLE` writer - CS training enable. Set to 1 to enable CS training during CA training."]
pub type PhyCslvlEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CSLVL_CS_MAP` reader - CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
pub type PhyCslvlCsMapR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_CS_MAP` writer - CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
pub type PhyCslvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CSLVL_START` reader - Defines the CS training DLL start value."]
pub type PhyCslvlStartR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_START` writer - Defines the CS training DLL start value."]
pub type PhyCslvlStartW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - CS training enable. Set to 1 to enable CS training during CA training."]
    #[inline(always)]
    pub fn phy_cslvl_enable(&self) -> PhyCslvlEnableR {
        PhyCslvlEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
    #[inline(always)]
    pub fn phy_cslvl_cs_map(&self) -> PhyCslvlCsMapR {
        PhyCslvlCsMapR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:26 - Defines the CS training DLL start value."]
    #[inline(always)]
    pub fn phy_cslvl_start(&self) -> PhyCslvlStartR {
        PhyCslvlStartR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CS training enable. Set to 1 to enable CS training during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_enable(&mut self) -> PhyCslvlEnableW<DenaliPhy899Spec> {
        PhyCslvlEnableW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_cs_map(&mut self) -> PhyCslvlCsMapW<DenaliPhy899Spec> {
        PhyCslvlCsMapW::new(self, 8)
    }
    #[doc = "Bits 16:26 - Defines the CS training DLL start value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_start(&mut self) -> PhyCslvlStartW<DenaliPhy899Spec> {
        PhyCslvlStartW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_899::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_899::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy899Spec;
impl crate::RegisterSpec for DenaliPhy899Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_899::R`](R) reader structure"]
impl crate::Readable for DenaliPhy899Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_899::W`](W) writer structure"]
impl crate::Writable for DenaliPhy899Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_899 to value 0"]
impl crate::Resettable for DenaliPhy899Spec {
    const RESET_VALUE: u32 = 0;
}
