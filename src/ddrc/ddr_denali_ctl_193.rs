#[doc = "Register `DDR_DENALI_CTL_193` reader"]
pub type R = crate::R<DdrDenaliCtl193Spec>;
#[doc = "Register `DDR_DENALI_CTL_193` writer"]
pub type W = crate::W<DdrDenaliCtl193Spec>;
#[doc = "Field `BANK_SPLIT_EN` reader - Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
pub type BankSplitEnR = crate::BitReader;
#[doc = "Field `BANK_SPLIT_EN` writer - Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
pub type BankSplitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLACEMENT_EN` reader - Enable placement logic for command queue. Set to 1 to enable."]
pub type PlacementEnR = crate::BitReader;
#[doc = "Field `PLACEMENT_EN` writer - Enable placement logic for command queue. Set to 1 to enable."]
pub type PlacementEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIORITY_EN` reader - Enable priority as a rule for command queue placement. Set to 1 to enable."]
pub type PriorityEnR = crate::BitReader;
#[doc = "Field `PRIORITY_EN` writer - Enable priority as a rule for command queue placement. Set to 1 to enable."]
pub type PriorityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW_SAME_EN` reader - Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
pub type RwSameEnR = crate::BitReader;
#[doc = "Field `RW_SAME_EN` writer - Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
pub type RwSameEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn bank_split_en(&self) -> BankSplitEnR {
        BankSplitEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable placement logic for command queue. Set to 1 to enable."]
    #[inline(always)]
    pub fn placement_en(&self) -> PlacementEnR {
        PlacementEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable priority as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn priority_en(&self) -> PriorityEnR {
        PriorityEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn rw_same_en(&self) -> RwSameEnR {
        RwSameEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bank_split_en(&mut self) -> BankSplitEnW<DdrDenaliCtl193Spec> {
        BankSplitEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable placement logic for command queue. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn placement_en(&mut self) -> PlacementEnW<DdrDenaliCtl193Spec> {
        PlacementEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable priority as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn priority_en(&mut self) -> PriorityEnW<DdrDenaliCtl193Spec> {
        PriorityEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rw_same_en(&mut self) -> RwSameEnW<DdrDenaliCtl193Spec> {
        RwSameEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_193::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_193::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl193Spec;
impl crate::RegisterSpec for DdrDenaliCtl193Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_193::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl193Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_193::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl193Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_193 to value 0"]
impl crate::Resettable for DdrDenaliCtl193Spec {
    const RESET_VALUE: u32 = 0;
}
