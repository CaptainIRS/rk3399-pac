#[doc = "Register `DENALI_CTL_194` reader"]
pub type R = crate::R<DenaliCtl194Spec>;
#[doc = "Register `DENALI_CTL_194` writer"]
pub type W = crate::W<DenaliCtl194Spec>;
#[doc = "Field `RW_SAME_PAGE_EN` reader - Enable page grouping when read/ write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
pub type RwSamePageEnR = crate::BitReader;
#[doc = "Field `RW_SAME_PAGE_EN` writer - Enable page grouping when read/ write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
pub type RwSamePageEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SAME_EN` reader - Enable chip select grouping when read/write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
pub type CsSameEnR = crate::BitReader;
#[doc = "Field `CS_SAME_EN` writer - Enable chip select grouping when read/write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
pub type CsSameEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W2R_SPLIT_EN` reader - Enable splitting of commands to the same chip select from a write to a read command as a rule for command queue placement."]
pub type W2rSplitEnR = crate::BitReader;
#[doc = "Field `W2R_SPLIT_EN` writer - Enable splitting of commands to the same chip select from a write to a read command as a rule for command queue placement."]
pub type W2rSplitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_RW_GROUP_W_BNK_CONFLICT` reader - Disables placement to read/write group when grouping creates a bank collision. Bit (0) controls placement next to bank conflict command and bit (1) controls placement 2 away from bank conflict command. Set each bit to 1 to disable."]
pub type DisableRwGroupWBnkConflictR = crate::FieldReader;
#[doc = "Field `DISABLE_RW_GROUP_W_BNK_CONFLICT` writer - Disables placement to read/write group when grouping creates a bank collision. Bit (0) controls placement next to bank conflict command and bit (1) controls placement 2 away from bank conflict command. Set each bit to 1 to disable."]
pub type DisableRwGroupWBnkConflictW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable page grouping when read/ write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
    #[inline(always)]
    pub fn rw_same_page_en(&self) -> RwSamePageEnR {
        RwSamePageEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable chip select grouping when read/write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
    #[inline(always)]
    pub fn cs_same_en(&self) -> CsSameEnR {
        CsSameEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable splitting of commands to the same chip select from a write to a read command as a rule for command queue placement."]
    #[inline(always)]
    pub fn w2r_split_en(&self) -> W2rSplitEnR {
        W2rSplitEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Disables placement to read/write group when grouping creates a bank collision. Bit (0) controls placement next to bank conflict command and bit (1) controls placement 2 away from bank conflict command. Set each bit to 1 to disable."]
    #[inline(always)]
    pub fn disable_rw_group_w_bnk_conflict(&self) -> DisableRwGroupWBnkConflictR {
        DisableRwGroupWBnkConflictR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable page grouping when read/ write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rw_same_page_en(&mut self) -> RwSamePageEnW<DenaliCtl194Spec> {
        RwSamePageEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable chip select grouping when read/write grouping as a rule for command queue placement. This is only valid when the RW_SAME_EN parameter is set. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn cs_same_en(&mut self) -> CsSameEnW<DenaliCtl194Spec> {
        CsSameEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable splitting of commands to the same chip select from a write to a read command as a rule for command queue placement."]
    #[inline(always)]
    #[must_use]
    pub fn w2r_split_en(&mut self) -> W2rSplitEnW<DenaliCtl194Spec> {
        W2rSplitEnW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Disables placement to read/write group when grouping creates a bank collision. Bit (0) controls placement next to bank conflict command and bit (1) controls placement 2 away from bank conflict command. Set each bit to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn disable_rw_group_w_bnk_conflict(
        &mut self,
    ) -> DisableRwGroupWBnkConflictW<DenaliCtl194Spec> {
        DisableRwGroupWBnkConflictW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_194::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_194::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl194Spec;
impl crate::RegisterSpec for DenaliCtl194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_194::R`](R) reader structure"]
impl crate::Readable for DenaliCtl194Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_194::W`](W) writer structure"]
impl crate::Writable for DenaliCtl194Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_194 to value 0"]
impl crate::Resettable for DenaliCtl194Spec {
    const RESET_VALUE: u32 = 0;
}
