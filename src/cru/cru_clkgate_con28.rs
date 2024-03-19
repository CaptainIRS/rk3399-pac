#[doc = "Register `CRU_CLKGATE_CON28` reader"]
pub type R = crate::R<CruClkgateCon28Spec>;
#[doc = "Register `CRU_CLKGATE_CON28` writer"]
pub type W = crate::W<CruClkgateCon28Spec>;
#[doc = "Field `HCLK_VOP0_NOC_EN` reader - hclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkVop0NocEnR = crate::BitReader;
#[doc = "Field `HCLK_VOP0_NOC_EN` writer - hclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkVop0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOP0_NOC_EN` reader - aclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVop0NocEnR = crate::BitReader;
#[doc = "Field `ACLK_VOP0_NOC_EN` writer - aclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVop0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VOP0_EN` reader - hclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVop0EnR = crate::BitReader;
#[doc = "Field `HCLK_VOP0_EN` writer - hclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVop0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOP0_EN` reader - aclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVop0EnR = crate::BitReader;
#[doc = "Field `ACLK_VOP0_EN` writer - aclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVop0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VOPB_NOC_EN` reader - hclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkVopbNocEnR = crate::BitReader;
#[doc = "Field `HCLK_VOPB_NOC_EN` writer - hclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkVopbNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOPB_NOC_EN` reader - aclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVopbNocEnR = crate::BitReader;
#[doc = "Field `ACLK_VOPB_NOC_EN` writer - aclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVopbNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VOPB_EN` reader - hclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVopbEnR = crate::BitReader;
#[doc = "Field `HCLK_VOPB_EN` writer - hclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVopbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOPB_EN` reader - aclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVopbEnR = crate::BitReader;
#[doc = "Field `ACLK_VOPB_EN` writer - aclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVopbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - hclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_vop0_noc_en(&self) -> HclkVop0NocEnR {
        HclkVop0NocEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_vop0_noc_en(&self) -> AclkVop0NocEnR {
        AclkVop0NocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vop0_en(&self) -> HclkVop0EnR {
        HclkVop0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vop0_en(&self) -> AclkVop0EnR {
        AclkVop0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_vopb_noc_en(&self) -> HclkVopbNocEnR {
        HclkVopbNocEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_vopb_noc_en(&self) -> AclkVopbNocEnR {
        AclkVopbNocEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vopb_en(&self) -> HclkVopbEnR {
        HclkVopbEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vopb_en(&self) -> AclkVopbEnR {
        AclkVopbEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vop0_noc_en(&mut self) -> HclkVop0NocEnW<CruClkgateCon28Spec> {
        HclkVop0NocEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_vop0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop0_noc_en(&mut self) -> AclkVop0NocEnW<CruClkgateCon28Spec> {
        AclkVop0NocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vop0_en(&mut self) -> HclkVop0EnW<CruClkgateCon28Spec> {
        HclkVop0EnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_vop0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop0_en(&mut self) -> AclkVop0EnW<CruClkgateCon28Spec> {
        AclkVop0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - hclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vopb_noc_en(&mut self) -> HclkVopbNocEnW<CruClkgateCon28Spec> {
        HclkVopbNocEnW::new(self, 4)
    }
    #[doc = "Bit 5 - aclk_vopb_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vopb_noc_en(&mut self) -> AclkVopbNocEnW<CruClkgateCon28Spec> {
        AclkVopbNocEnW::new(self, 5)
    }
    #[doc = "Bit 6 - hclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vopb_en(&mut self) -> HclkVopbEnW<CruClkgateCon28Spec> {
        HclkVopbEnW::new(self, 6)
    }
    #[doc = "Bit 7 - aclk_vopb clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vopb_en(&mut self) -> AclkVopbEnW<CruClkgateCon28Spec> {
        AclkVopbEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon28Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon28Spec;
impl crate::RegisterSpec for CruClkgateCon28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con28::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon28Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con28::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON28 to value 0"]
impl crate::Resettable for CruClkgateCon28Spec {
    const RESET_VALUE: u32 = 0;
}
