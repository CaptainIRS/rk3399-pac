#[doc = "Register `CLKGATE_CON27` reader"]
pub type R = crate::R<ClkgateCon27Spec>;
#[doc = "Register `CLKGATE_CON27` writer"]
pub type W = crate::W<ClkgateCon27Spec>;
#[doc = "Field `HCLK_ISP0_NOC_EN` reader - hclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkIsp0NocEnR = crate::BitReader;
#[doc = "Field `HCLK_ISP0_NOC_EN` writer - hclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkIsp0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP0_NOC_EN` reader - aclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkIsp0NocEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP0_NOC_EN` writer - aclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkIsp0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_ISP1_NOC_EN` reader - hclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkIsp1NocEnR = crate::BitReader;
#[doc = "Field `HCLK_ISP1_NOC_EN` writer - hclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkIsp1NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP1_NOC_EN` reader - aclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkIsp1NocEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP1_NOC_EN` writer - aclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkIsp1NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_ISP0_WRAPPER_EN` reader - hclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIsp0WrapperEnR = crate::BitReader;
#[doc = "Field `HCLK_ISP0_WRAPPER_EN` writer - hclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIsp0WrapperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP0_WRAPPER_EN` reader - aclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIsp0WrapperEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP0_WRAPPER_EN` writer - aclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIsp0WrapperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLKIN_ISP1_WRAPPER_EN` reader - pclkin_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkinIsp1WrapperEnR = crate::BitReader;
#[doc = "Field `PCLKIN_ISP1_WRAPPER_EN` writer - pclkin_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkinIsp1WrapperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_ISP1_WRAPPER_EN` reader - hclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIsp1WrapperEnR = crate::BitReader;
#[doc = "Field `HCLK_ISP1_WRAPPER_EN` writer - hclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIsp1WrapperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP1_WRAPPER_EN` reader - aclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIsp1WrapperEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP1_WRAPPER_EN` writer - aclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIsp1WrapperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - hclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_isp0_noc_en(&self) -> HclkIsp0NocEnR {
        HclkIsp0NocEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_isp0_noc_en(&self) -> AclkIsp0NocEnR {
        AclkIsp0NocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_isp1_noc_en(&self) -> HclkIsp1NocEnR {
        HclkIsp1NocEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_isp1_noc_en(&self) -> AclkIsp1NocEnR {
        AclkIsp1NocEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_isp0_wrapper_en(&self) -> HclkIsp0WrapperEnR {
        HclkIsp0WrapperEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_isp0_wrapper_en(&self) -> AclkIsp0WrapperEnR {
        AclkIsp0WrapperEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclkin_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclkin_isp1_wrapper_en(&self) -> PclkinIsp1WrapperEnR {
        PclkinIsp1WrapperEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_isp1_wrapper_en(&self) -> HclkIsp1WrapperEnR {
        HclkIsp1WrapperEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_isp1_wrapper_en(&self) -> AclkIsp1WrapperEnR {
        AclkIsp1WrapperEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp0_noc_en(&mut self) -> HclkIsp0NocEnW<ClkgateCon27Spec> {
        HclkIsp0NocEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_isp0_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp0_noc_en(&mut self) -> AclkIsp0NocEnW<ClkgateCon27Spec> {
        AclkIsp0NocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp1_noc_en(&mut self) -> HclkIsp1NocEnW<ClkgateCon27Spec> {
        HclkIsp1NocEnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_isp1_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp1_noc_en(&mut self) -> AclkIsp1NocEnW<ClkgateCon27Spec> {
        AclkIsp1NocEnW::new(self, 3)
    }
    #[doc = "Bit 4 - hclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp0_wrapper_en(&mut self) -> HclkIsp0WrapperEnW<ClkgateCon27Spec> {
        HclkIsp0WrapperEnW::new(self, 4)
    }
    #[doc = "Bit 5 - aclk_isp0_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp0_wrapper_en(&mut self) -> AclkIsp0WrapperEnW<ClkgateCon27Spec> {
        AclkIsp0WrapperEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclkin_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclkin_isp1_wrapper_en(&mut self) -> PclkinIsp1WrapperEnW<ClkgateCon27Spec> {
        PclkinIsp1WrapperEnW::new(self, 6)
    }
    #[doc = "Bit 7 - hclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp1_wrapper_en(&mut self) -> HclkIsp1WrapperEnW<ClkgateCon27Spec> {
        HclkIsp1WrapperEnW::new(self, 7)
    }
    #[doc = "Bit 8 - aclk_isp1_wrapper clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp1_wrapper_en(&mut self) -> AclkIsp1WrapperEnW<ClkgateCon27Spec> {
        AclkIsp1WrapperEnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon27Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon27Spec;
impl crate::RegisterSpec for ClkgateCon27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con27::R`](R) reader structure"]
impl crate::Readable for ClkgateCon27Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con27::W`](W) writer structure"]
impl crate::Writable for ClkgateCon27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON27 to value 0"]
impl crate::Resettable for ClkgateCon27Spec {
    const RESET_VALUE: u32 = 0;
}
